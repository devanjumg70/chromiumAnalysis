# Deep Dive: Chrome Emulation Implementation & The TLS Gap

## 1. Executive Summary
This research confirms that Chrome's "Device Emulation" is fundamentally a **Layer 7 (Application Layer) simulation**. It modifies HTTP headers to *announce* itself as a different device, but it uses the standard Chrome **Layer 6/5 (Presentation/Session Layer)** stack for the actual connection.

**Result**: A server analyzing the TLS Client Hello (JA3 fingerprint) will see "Desktop Chrome on Linux", even if the User-Agent says "Android Mobile".

## 2. Architecture of Emulation
The implementation is split into completely isolated subsystems.

### The "Emulation" Path (Headers)
*   **Location**: `content/browser/devtools/protocol/`
*   **Key Component**: `EmulationHandler`
*   **Data Flow**:
    1.  User selects device in DevTools.
    2.  `EmulationHandler::SetUserAgentOverride` stores the string.
    3.  `DevToolsInstrumentation` hooks into `NavigationRequest` and `ResourceRequest`.
    4.  `EmulationHandler::ApplyOverrides` writes to `net::HttpRequestHeaders`.
    *   **Scope**: This only touches the text headers sent *inside* the encrypted tunnel.

### The "Networking" Path (TLS)
*   **Location**: `net/socket/` and `net/ssl/`
*   **Key Component**: `SSLClientSocketImpl`
*   **Data Flow**:
    1.  `ClientSocketPool` requests a socket.
    2.  `SSLConnectJob` creates `SSLClientSocketImpl`.
    3.  `SSLClientSocketImpl` initializes `BoringSSL` (OpenSSL fork).
    4.  **Crucially**: It initializes using a `SSLContext` Singleton.
        *   This Singleton is configured ONCE at startup with Chrome's standard cipher suites, extension orders, and curves.
    *   **Scope**: This defines the handshake *establishing* the tunnel.

## 3. The "Gap" Analysis
Why doesn't Chrome emulate the TLS fingerprint?

1.  **Dependency Isolation**: The `net/` stack (where SSL lives) is designed to be independent of the `content/` layer (where DevTools lives). Adding a "Device Profile" parameter to `SSLConfig` just for DevTools would violate strict layering principles in Chromium.
2.  **BoringSSL Design**: The `SSLContext` is a heavy, shared object. Reconfiguring it per-request or creating thousands of different Contexts (one for every possible emulated device) would be performant-prohibitive.
3.  **Use Case**: The primary use case for DevTools is **Responsive Design** (does the site look right?) and **Feature Detection** (do touch events work?). It was never intended as a "Stealth Scraper" or "Penetration Testing" tool that needs to fool deep-packet inspection.

## 4. Implementation Details for Your Rust Client

To build a client that *actually* mimics the device (fixing what Chrome misses), you must control the bottom layer.

### What You Need to Implement
You need to create a **TLS Factory** that accepts a "Device Profile" and configures the underlying TLS library (Rustls, OpenSSL, or Boring) to match.

#### 1. The Device Profile (Extended)
Your `EmulatedDevice` struct needs more than just screen size. It needs a TLS Fingerprint spec:
```rust
struct TlsFingerprint {
    ciphers: Vec<u16>,             // Order matters! e.g., GREASE, AES_128_GCM...
    extensions: Vec<u16>,          // Order matters!
    supported_groups: Vec<u16>,    // e.g., X25519, P-256
    ec_point_formats: Vec<u8>,
    // ...
}
```

#### 2. The Implementation Map
| Feature | Chrome DevTools uses... | You must use... |
| :--- | :--- | :--- |
| **User-Agent** | `EmulationHandler` / Headers | `reqwest` / `hyper` Headers |
| **Client Hints** | `EmulationHandler` / Headers | `reqwest` / `hyper` Headers |
| **Cipher Suites** | Default BoringSSL (Static) | **Custom TLS Config** (Per-Device) |
| **Extensions** | Default BoringSSL (Static) | **Custom TLS Config** (Per-Device) |
| **ALPN** | Default (h2, http/1.1) | **Custom TLS Config** (Some apps use h2 only) |

## 5. Artifacts
*   **Code Evidence**: See `chromium_analysis/emulation_deep_dive/code_evidence.md` for the exact C++ lines proving this separation.
*   **Device Data**: See `chromium_analysis/all_devices.json` for the header data you *can* use immediately.
