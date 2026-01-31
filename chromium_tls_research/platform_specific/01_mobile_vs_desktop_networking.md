# Mobile vs. Desktop Networking Configuration in Chromium

## Executive Summary

Chromium's networking stack behavior varies significantly between Desktop (Windows, macOS, Linux) and Mobile (Android, iOS) platforms. These differences are primarily driven by **battery conservation**, **network mobility**, and **OS-specific APIs**.

**Key Rule:** `chromenet` must emulate these platform-specific behaviors to match Chromium's fingerprint and performance characteristics correctly.

## 1. TCP Socket Configuration

### TCP Keep-Alive
*   **Desktop:** **ENABLED**
    *   **Delay:** 45 seconds (`kTCPKeepAliveSeconds`)
    *   **Rationale:** Keeps middleboxes from dropping idle connections.
*   **Mobile (Android/iOS):** **DISABLED**
    *   **Rationale:** "TCP keep alive wakes up the radio, which is expensive on mobile." (Source: `tcp_socket_posix.cc`)
    *   **Impact:** Mobile connections rely on higher-level heartbeats or re-establishment if middleboxes drop them.

### Socket Options (Don't Fragment)
*   **macOS / iOS:** Uses `IP_DONTFRAG` / `IPV6_DONTFRAG`.
*   **Linux / Android / ChromeOS:** Uses `IP_MTU_DISCOVER` with `IP_PMTUDISC_DO` / `IPV6_MTU_DISCOVER` with `IPV6_PMTUDISC_DO`.

### Socket Service Type (iOS Only)
*   **iOS:** Sets `SO_NET_SERVICE_TYPE` (e.g., `BK`, `BE`, `VI`, `VO`) to classify traffic for the OS scheduler.

---

## 2. Android-Specific Networking (`net/android`)

Android requires explicit binding of sockets to networks to handle multi-network scenarios (WiFi + Cellular active simultaneously) and to prevent traffic from leaking on the wrong interface.

### Network Binding
*   **Implementation:** Calls `android_setsocknetwork()` (via dynamic loading from `libandroid.so`).
*   **Function:** `BindToNetwork(SocketDescriptor socket, handles::NetworkHandle network)`
*   **Behavior:**
    *   Binds a specific socket to a specific Android Network Handle.
    *   Essential for QUIC connection migration and ensuring traffic uses the intended path.
    *   Returns `ERR_NETWORK_CHANGED` if the network disconnects.

### QUIC Optimization (Battery Saver)
*   **Feature:** `RegisterQuicConnectionClosePayload`
*   **Mechanism:** Registers a UDP "Connection Close" packet with the Android System Server.
*   **Benefit:** If the app is frozen or loses network access, the System Server sends the close packet. This avoids waking up the app and the modem just to say "goodbye," saving battery.

### DNS Resolution
*   **Implementation:** Calls `android_getaddrinfofornetwork()` (NDK API).
*   **Private DNS:** Detects "DNS over TLS" (DoT) state from Android system settings to configure internal resolution behavior.

---

## 3. iOS-Specific Networking

### DNS Configuration
*   **Watcher:** **DISABLED** (`DnsConfigServicePosix::CreateSystemService` returns `nullptr`).
*   **Reason:** iOS provides no public API to watch for system DNS configuration changes.
*   **Impact:** config changes might not be detected immediately without network change events.

### Multicast/Broadcast
*   **Broadcast:** Uses `SO_REUSEPORT` to allow multiple processes to receive broadcast/multicast packets (unlike Linux which typically uses `SO_REUSEADDR` for multicast).

---

## 4. SSL/TLS Platform Differences

### Client Certificates
*   **Windows:** Uses CAPI/CNG (Cryptographic API).
*   **macOS / iOS:** Uses `Security.framework` (Keychain).
*   **Android:** Java-based KeyChain API calls via JNI (`AndroidNetworkLibrary`).
*   **Linux:** Uses NSS (Network Security Services) or file-based stores.

### Root Certificate Trust
*   **Android:**
    *   Fetches user-added roots via `GetUserAddedRoots()` (JNI).
    *   Verifies chains using `AndroidNetworkLibrary_verifyServerCertificates` (delegates to Android system verifier).
    *   Supports Certificate Transparency checks in the verification callback.
*   **Desktop:**
    *   Typically uses the built-in Chrome Root Store (newer versions) or OS trust store (legacy/enterprise).

---

## 5. Summary Matrix for `chromenet`

| Feature | Desktop (Linux/Mac/Win) | Mobile (Android/iOS) |
| :--- | :--- | :--- |
| **TCP Keep-Alive** | **Enabled (45s)** | **Disabled** |
| **Network Binding** | Implicit (OS Routing) | **Explicit (`BindToNetwork`)** |
| **DNS Config** | Watched File/Registry | API (Android) / None (iOS) |
| **QUIC Close** | App-initiated | **Offloaded to System (Android)** |
| **Cert Verification** | Chrome Root Store + OS | **OS API (Android/iOS)** |

## Implementation Plan
1.  **Defaults:** Start with Desktop behavior (easier to implement in Rust).
2.  **Android Support:** Critical to implement `BindToNetwork` logic using the `ndk-sys` crate or JNI if targeting Android.
3.  **Battery:** Ensure `TCP_KEEPALIVE` is configurable and defaults to `false` for mobile targets.

---
**Document Version:** 1.0
**Last Updated:** 2026-01-31
