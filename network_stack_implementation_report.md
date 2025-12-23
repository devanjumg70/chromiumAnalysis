# Chromium Network Stack Implementation Analysis

## Executive Summary
Chromium's network stack is a **custom, "raw" implementation** located primarily in `//net`. It does **not** use high-level external HTTP libraries (like `curl`, `reqwest`, or `libwww`). Instead, it implements HTTP/1.1, HTTP/2, HTTP/3 (QUIC), and the entire transaction lifecycle from scratch on top of low-level socket APIs and its own BoringSSL library.

## "Lib vs Raw" Findings

*   **HTTP/1.1 & HTTP/2**: Implemented from scratch in `net/http/`.
    *   `HttpBasicStream` handles HTTP/1.1.
    *   `SpdySession` / `SpdyStream` handle HTTP/2.
*   **TLS/SSL**: Uses **BoringSSL** (Google's fork of OpenSSL).
    *   This is an internal library (`//third_party/boringssl`), not a system dependency.
    *   Chromium implements the "glue" in `net/socket/ssl_client_socket_impl.cc`.
*   **QUIC (HTTP/3)**: Implemented completely from scratch in `net/quic/`.
*   **DNS**: Custom asynchronous DNS resolver (`net/dns/`), bypassing the system resolver for performance and privacy.

## Key Architectural Components

### 1. `HttpNetworkTransaction` (The Driver)
Manages the lifecycle of a request. It is a **State Machine** designed to be non-blocking.
*   **Responsibility**: Auth challenges, HSTS upgrades, retry logic, and driving the `HttpStream`.
*   **Rust Equivalent**: The `Client` logic in `wreq` or `rquest`, but extended with custom retry/auth middleware.

### 2. `ClientSocketPool` (Connection Management)
A complex pooling system that manages TCP and SSL sockets.
*   **Limits**: Enforces 6 connections per host, 256 total.
*   **Logic**: "Late binding" - sockets are requested when needed, reused if idle, or created if allowed.
*   **Rust Equivalent**: `mobc` or `deadpool` managing `TcpStream`/`TlsStream`. `wreq` has internal pooling, but a custom wrapper is needed to match Chromium's strict per-group limits.

### 3. `HttpStreamFactory` (Protocol Selection)
Decides *how* to connect (IPv4 vs IPv6, TCP vs QUIC).
*   **Happy Eyeballs**: Races different connection methods.
*   **Rust Equivalent**: `hyper`'s connector or `wreq`'s internal connector logic.

## Dependencies (Third Party)
Chromium maintains its dependencies in `//third_party`. Key network-related ones:
*   `boringssl`: TLS implementation.
*   `zlib` / `brotli`: Compression.
*   `protobuf`: For some structured data (though standard HTTP headers are often parsed manually).

## Impact on `ghttp` Design

Since we cannot use Chromium's C++ code directly, and "porting" 10 million lines of code is unfeasible, the **Wrapper Strategy** is correct:

1.  **Core HTTP/TLS**: Use `wreq` (which uses `boring` - the Rust binding to BoringSSL). This gives us the **same TLS stack** as Chromium.
2.  **Logic Layer**: Implement `HttpNetworkTransaction`'s state machine logic (retries, auth) *on top* of `wreq`.
3.  **Connection Management**: Use a custom `DashMap`-based pool (as planned) to enforce strict Chromium-like limits, as `wreq`'s default pool might be too generic.

## Conclusion
Chromium builds from raw sockets up. Our Rust implementation should use `wreq` + `boring` to match the **cryptographic implementation** (crucial for fingerprinting) but re-implement the **high-level management logic** (pooling, cookies, auth) to match Chromium's behavior.
