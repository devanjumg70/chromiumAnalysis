# Chromium HTTP Client Analysis

This report analyzes three core components of Chromium's network stack to aid in building a similar HTTP client in Rust.

## 1. net/http/http_network_transaction.cc

### Purpose
`HttpNetworkTransaction` is the driver for a single HTTP network request/response transaction. It sits above the socket and stream layers but below the URLRequest layer. Its primary job is to:
1.  Obtain an `HttpStream` (via `HttpStreamFactory`).
2.  Send the HTTP request (headers and body).
3.  Read the HTTP response headers and body.
4.  Handle network-level logic like authentication challenges (401/407), SSL client auth, and retries on connection failures.

### Key Classes
*   **Main Class**: `HttpNetworkTransaction`
    *   Inherits from: `HttpTransaction` (interface for all transaction types like cache, network, etc.), `HttpStreamRequest::Delegate` (callbacks for stream creation).
*   **Key Member Variables**:
    *   `HttpNetworkSession* session_`: Shared session state (pools, cache, etc.).
    *   `std::unique_ptr<HttpStream> stream_`: The underlying protocol implementation (HTTP/1.1, H2, QUIC).
    *   `State next_state_`: Current state of the internal state machine.
    *   `HttpResponseInfo response_`: Storage for response headers and metadata.

### Dependencies
*   **Includes**:
    *   **Standard**: `<vector>`, `<string>`, `<memory>`, `<utility>`.
    *   **Chromium Base**: `base/time/time.h`, `base/callback.h`, `base/metrics/histogram_macros.h`.
    *   **Net Core**: `net/base/io_buffer.h`, `net/base/net_errors.h`, `net/http/http_stream_factory.h`, `net/socket/client_socket_pool.h`.
*   **Dependent Files** (Reverse Dependencies):
    *   `net/http/http_network_layer.cc`: Uses it to execute transactions.
    *   `net/url_request/url_request_http_job.cc`: Indirectly uses it via `HttpNetworkLayer` or `HttpCache`.
    *   `net/http/http_proxy_client_socket.cc`: May interact during proxy tunnels.

### Main Methods
1.  **`Start(...)`**: Initializes the transaction, configures `HttpRequestInfo`, and kicks off the state machine (`DoLoop`).
2.  **`DoLoop(int result)`**: The core driver loop. It switches on `next_state_` and calls the corresponding `Do*` method. If a method returns `ERR_IO_PENDING`, the loop breaks and waits for a callback.
3.  **`DoCreateStream()`**: calls `session_->http_stream_factory()->RequestStream(...)` to get a connection.
4.  **`DoSendRequest()`**: Calls `stream_->SendRequest(...)` to write headers and upload body.
5.  **`Read(...)`**: Reads response body data into a provided buffer.

### Configuration Constants
*   `kDrainBodyBufferSize = 1024`: Size of buffer used when draining a body (e.g. before auth restart).
*   `kMaxRetryAttempts = 2`: Max retries for network errors (excluding initial request).
*   `kMaxRestarts = 32`: Safety limit for auth/redirect/certificate restarts to prevent infinite loops.

### Design Patterns
*   **State Machine**: The entire class is a state machine (`DoLoop`, `next_state_`, `STATE_CREATE_STREAM` enum). This allows non-blocking async IO in C++.
*   **Delegate**: Implements `HttpStreamRequest::Delegate` to react to asynchronous stream creation events.
*   **Strategy**: The `HttpStream` is a strategy representing the specific protocol version (H1, H2, H3). `HttpNetworkTransaction` doesn't care which one satisfies the interface.

### Rust Translation Notes
*   **Async/Await**: In Rust, the explicit `DoLoop` state machine should be replaced by `async` functions. The compiler builds the state machine for you.
    ```rust
    // Conceptually in Rust
    async fn execute_transaction(&mut self) -> Result<Response, Error> {
        let stream = self.create_stream().await?;
        stream.send_request(&self.headers).await?;
        let response = stream.read_response_headers().await?;
        Ok(response)
    }
    ```
*   **Libraries**: Use `reqwest` for a high-level equivalent, or `hyper::client::conn` for building the low-level transaction logic.
*   **Pinning**: You will likely need `Pin<Box<dyn Future...>>` if you are storing the future or implementing a trait for it manually, but `async fn` in traits is now supported (with caveats).

---

## 2. net/http/http_stream_factory.cc

### Purpose
Responsible for churning out `HttpStream` instances. It acts as the "brain" for connection establishment, deciding whether to:
*   Reuse an existing idle socket.
*   Connect to a proxy.
*   Race IPv4 vs IPv6.
*   Race TCP vs QUIC (Alternative Services).
*   Handle Preconnects.

### Key Classes
*   **Main Class**: `HttpStreamFactory`.
*   **Helper Classes**:
    *   `JobController`: Manages a single stream request, often spawning multiple `Job`s (e.g. one for kHTP and one for kQUIC) to race them.
    *   `Job`: Represents a single candidate connection attempt.

### Dependencies
*   **Includes**: `net/http/http_network_session.h`, `net/http/http_stream_request.h`, `net/socket/client_socket_pool.h`, `net/spdy/spdy_session.h`.
*   **Dependent Files**:
    *   `net/http/http_network_session.h`: Holds the factory instance.
    *   `net/http/http_network_transaction.cc`: Calls `RequestStream`.

### Main Methods
1.  **`RequestStream(...)`**: The public API called by transactions. Creates a `JobController` and starts it.
2.  **`ProcessAlternativeServices(...)`**: Analysis `Alt-Svc` headers to update the `HttpServerProperties` map (used for upgrading to HTTP/3).
3.  **`PreconnectStreams(...)`**: Initiates pre-connection to reduce latency for future requests.
4.  **`RequestStreamInternal(...)`**: Common implementation for requesting streams (normal, websocket, bidirectional).
5.  **`GetSpdySessionKey(...)`**: Static helper to compute the key used for pooling HTTP/2 sessions.

### Configuration Constants
*   `kAlternativeServiceHeader = "Alt-Svc"`: The header field name for HTTP/3 upgrades.

### Design Patterns
*   **Factory**: Abstract factory pattern for creating streams.
*   **Job/Worker**: The `JobController` spawns `Job` workers. This is essentially a "Happy Eyeballs" implementation pattern where multiple paths are tried in parallel or staggered.
*   **Observer**: `OnJobControllerComplete` allows the factory to track active jobs.

### Rust Translation Notes
*   **Service Trait**: In the `tower` ecosystem, this maps to a `Service<Request>` that yields a `Connection`.
*   **Happy Eyeballs**: Rust's `hyper` or `reqwest` handles this in the connector layer. Look at `hyper-util::client::legacy::connect::dns::GaiResolver` and the connector logic for racing.
*   **Racing Futures**: Use `tokio::select!` or `futures::future::select_ok` to race multiple connection attempts (e.g. QUIC and TCP) and take the winner.

---

## 3. net/socket/client_socket_pool.cc

### Purpose
Manages a pool of transport sockets (TCP, SSL, Proxy-tunneled). It enforces limits on the number of concurrent connections per "Group" (usually Origin + Privacy settings) and global limits. It handles handing out idle sockets and creating new ones via `ConnectJob`s.

### Key Classes
*   **Main Class**: `ClientSocketPool` (Abstract Base). Concrete implementations include `TransportClientSocketPool`.
*   **Key Inner Classes**:
    *   `GroupId`: Uniquely identifies a connection group (scheme, host, port, privacy mode, etc.).
    *   `SocketParams`: Parameters required to connect.
    *   `ClientSocketHandle`: A handle held by the consumer (Transaction) representing a leased socket.

### Dependencies
*   **Includes**: `net/socket/connect_job.h`, `net/socket/stream_socket.h`, `net/base/proxy_chain.h`.
*   **Dependent Files**:
    *   `net/socket/transport_client_socket_pool.cc`: Concrete implementation.
    *   `net/http/http_stream_factory_job.cc`: Requests sockets from the pool.

### Main Methods
1.  **`RequestSocket(...)`**: The core method. If an idle socket exists in the group, returns it. Else, checks limits and creates a `ConnectJob`.
2.  **`RequestSockets(...)`**: Used for preconnecting multiple sockets.
3.  **`ReleaseSocket(...)`**: Called when a transaction finishes. Decides whether to close the socket or keep it idle for reuse.
4.  **`CloseIdleSockets(...)`**: Maintenance method to aggressively flush idle sockets (e.g. on network change).
5.  **`IdleSocketCount()`**: Metrics/info.

### Configuration Constants
*   `g_used_idle_socket_timeout_s = 300`: Idle sockets are closed after 5 minutes.
*   `kMaxConnectRetryIntervalMs = 250`: Backoff timing.

### Design Patterns
*   **Object Pool**: Classic resource pooling to save handshake overhead.
*   **Handle/owner**: `ClientSocketHandle` acts as a smart pointer/lease for the underlying `StreamSocket`.
*   **Layered Architecture**: Pools can be layered (though in Chromium this is often modeled by `ConnectJob` layering now, the pool interface still supports `LowerLayeredPool`).

### Rust Translation Notes
*   **Crates**: `deadpool`, `bb8`, or `mobc` are generic object pools.
*   **Hyper**: `hyper-util` has a built-in `Pool` for HTTP connections.
*   **Implementation**: A Rust pool would store `Vec<T>` or `VecDeque<T>` of idle connections, keyed by a `Key` (equivalent to `GroupId`).
*   **Drop Trait**: Rust's `Drop` is perfect for `ReleaseSocket`. When the socket handle is dropped, it can automatically return to the pool.
