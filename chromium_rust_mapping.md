# Chromium C++ to Rust "Raw" Mapping

This document defines the strict mapping between Chromium's `//net` stack (C++) and our custom Rust implementation (`chromium_net`).

## Core Principles
1.  **No High-Level Clients**: We do NOT use `reqwest`, `surf`, or `ureq`.
2.  **Raw BoringSSL**: We use `boring` (Rust bindings to Google's BoringSSL) for all crypto.
3.  **Manual State Management**: We implement the transaction lifecycles manually, using `hyper` only for wire-format parsing (comparable to Chromium's internal parsers).

## Component Mapping

| Component | Chromium (C++) | Rust (`chromium_net`) | Implementation Strategy |
| :--- | :--- | :--- | :--- |
| **TLS Library** | `//third_party/boringssl` | `boring` (crate) | Bindings to same C library. |
| **TCP Socket** | `net::TCPClientSocket` | `tokio::net::TcpStream` | Async non-blocking TCP. |
| **SSL Socket** | `net::SSLClientSocketImpl` | `boring::ssl::SslStream` | Wraps TCP stream. |
| **HTTP Parser** | `net::HttpStreamParser` | `hyper::client::conn` | Low-level connection handling. |
| **Connection Pool** | `net::ClientSocketPool` | `src/socket/pool.rs` | **Custom**. `DashMap<GroupId, VecDeque<Connection>>`. |
| **Transaction** | `net::HttpNetworkTransaction` | `src/http/transaction.rs` | **Custom**. State machine enum (`CreateStream`, `Send`, `Read`). |
| **Stream Factory** | `net::HttpStreamFactory` | `src/http/stream_factory.rs` | Logic to race TCP vs QUIC (Future). |
| **URL Request** | `net::URLRequest` | `src/url_request/request.rs` | Public API facade. |
| **Extractors** | `services/video_capture` | `src/extractor/` | Custom logic for specific sites. |

## Detailed Struct Mapping

### 1. Connection Pool (`net/socket/client_socket_pool.h`)
Chromium enforces strict limits (Default: 6 per host).

**Rust Implementation**:
```rust
// src/socket/pool.rs
pub struct ClientSocketPool {
    // Limits
    max_sockets_per_group: usize, // 6
    max_sockets_total: usize,     // 256
    
    // State
    idle_sockets: DashMap<GroupId, VecDeque<IdleSocket>>,
    active_socket_count: AtomicUsize,
}
```

### 2. Transaction State Machine (`net/http/http_network_transaction.cc`)
Chromium's `DoLoop` method drives the request.

**Rust Implementation**:
```rust
// src/http/transaction.rs
enum State {
    CreateStream,
    SendRequest,
    ReadHeaders,
    ReadBody,
    Complete,
}

impl Transaction {
    async fn step(&mut self) -> Result<StepResult> {
        match self.state {
            State::CreateStream => self.do_create_stream().await,
            State::SendRequest => self.do_send_request().await,
            // ...
        }
    }
}
```

### 3. Connect Job (`net/socket/connect_job.h`)
Responsible for establishing a connection (DNS -> TCP -> SSL).

**Rust Implementation**:
```rust
// src/socket/connect_job.rs
pub async fn connect(host: &str) -> Result<SslStream<TcpStream>> {
    let addrs = resolve(host).await?;
    let tcp = TcpStream::connect(&addrs).await?;
    let connector = SslConnector::builder(...).build();
    let ssl = connector.connect(host, tcp).await?;
    Ok(ssl)
}
```

## Dependency Manifest (`Cargo.toml`)

```toml
[dependencies]
# Async Runtime (Event Loop)
tokio = { version = "1", features = ["full"] }

# Crypto (The "Raw" Requirement)
boring = "4.0"
foreign-types = "0.3" # Required for boring interop

# HTTP Parsing (Low-level only)
hyper = { version = "1", features = ["client", "http1", "http2"] }
http = "1.0"
bytes = "1.5"

# Utils
thiserror = "1.0"
dashmap = "5.5" # Concurrent Map for Pool
url = "2.5"
tracing = "0.1"
```
