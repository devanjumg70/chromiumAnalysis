# Device Emulation & Request Construction Analysis

## 1. Full Device Database
I have extracted the complete list of emulated devices (55 devices) from the official source `EmulatedDevices.ts`.
*   **File**: `chromium_analysis/all_devices.json`
*   **Data Points**: Title, Screen metrics (width, height, DPR), User Agent strings, and Capabilities (Touch, Mobile).

## 2. How Requests Are Built (Emulation Logic)

The "Device Emulation" feature in Chrome DevTools primarily operates by **overriding HTTP headers** at the network layer. It does *not* fundamentally change the network stack's behavior (e.g., TCP congestion control, TLS handshake) to match the emulated device.

### The Pipeline
1.  **Command**: User selects a device (e.g., "Pixel 7").
2.  **Handler**: `EmulationHandler::SetUserAgentOverride` (in `content/browser/devtools/protocol/emulation_handler.cc`) receives the command.
    *   It updates internal state: `user_agent_`, `user_agent_metadata_` (Client Hints), `accept_language_`.
3.  **Instrumentation**: When a network request begins (Navigation or Subresource), `devtools_instrumentation::ApplyNetworkRequestOverrides` is triggered.
4.  **Injection**:
    *   It calls `EmulationHandler::ApplyOverrides`.
    *   This method modifies the `net::HttpRequestHeaders` object directly:
        *   Sets `User-Agent`.
        *   Sets `Accept-Language`.
        *   Sets Client Hints headers (`Sec-CH-UA`, `Sec-CH-UA-Platform`, `Sec-CH-UA-Mobile`, `Sec-CH-UA-Model`).

### Code Reference
*   **State Storage**: `EmulationHandler::SetUserAgentOverride` (lines 851-974)
*   **Header Injection**: `EmulationHandler::ApplyOverrides` (lines 1193-1233)
*   **Trigger Point**: `content/browser/devtools/devtools_instrumentation.cc`

## 3. TLS Impersonation & Fingerprinting

**Critical Finding**: Chrome's built-in emulation is "Surface-Level".
*   **Headers**: Match the emulated device perfectly.
*   **TLS/SSL**: **Does NOT match.** The TLS handshake (Cipher Suites, Extensions, JA3 hash) remains that of the *host* browser (Desktop Chrome).
*   **HTTP/2**: **Does NOT match.** Frame settings and window sizes remain that of Desktop Chrome.

### Implication for Your Rust Client
If you build a Rust implementation and only change the `User-Agent` and `Client-Hints` to match a "Pixel 7", but use a standard `rustls` or `OpenSSL` configuration, you will have the same flaw as Chrome DevTools:
*   **Server View**: "User-Agent says Android, but TLS Client Hello says Rust/Linux." -> **Flagged as Bot/Scraper.**

### Recommendation
To build a "Better" emulator than Chrome:
1.  **Header Emulation**: Use the data from `all_devices.json` to set `User-Agent` and `Sec-CH-*` headers (just like Chrome).
2.  **TLS Impersonation (The "missing piece")**:
    *   You must configure your TLS library to mimic the *actual* handshake of the target device.
    *   **Android**: Uses specific ciphers, extension orders, and curves (often Boringssl based).
    *   **iOS**: Uses different extension orders and ciphers (SecureTransport/Network.framework).
    *   **Rust Tools**: Look into crates like `reqwest-impersonate` or `boring` that allow setting the "Client Hello" fingerprint effectively.

## 4. Derived Schema (for Rust)
Your `MobileDevice` struct in `device_struct.rs` is accurate for holding the data needed for Step 1 (Header Emulation).

```rust
// Use this data to populate:
// 1. User-Agent header
// 2. Sec-CH-UA, Sec-CH-UA-Mobile, Sec-CH-UA-Platform headers
struct EmulatedDevice {
    pub title: String,
    pub user_agent: String,
    pub user_agent_metadata: Option<UserAgentMetadata>,
    // ...
}
```
