# Chromium HTTP/2 and Connection Pool Configuration

## Executive Summary

This document covers Chromium's HTTP/2 (SPDY) configuration and connection pool management settings. These are critical for implementing a Chrome-like networking stack.

**Source Files:** Based on Chromium source code analysis (chromium/net/spdy/ and chromium/net/socket/)

---

## 1. Connection Pool Limits

### Socket Pool Configuration
```cpp
// Source: net/socket/client_socket_pool_manager.cc:36-56

// Total soft limit of sockets across all hosts
auto g_socket_soft_cap_per_pool = std::to_array<size_t>({
    256,  // NORMAL_SOCKET_POOL
    256   // WEBSOCKET_SOCKET_POOL
});

// Maximum connections per host (group)
auto g_max_sockets_per_group = std::to_array<size_t>({
    6,   // NORMAL_SOCKET_POOL
    255  // WEBSOCKET_SOCKET_POOL
});

// Maximum connections per proxy chain
auto g_max_sockets_per_proxy_chain = std::to_array<size_t>({
    32,  // NORMAL_SOCKET_POOL (kDefaultMaxSocketsPerProxyChain)
    32   // WEBSOCKET_SOCKET_POOL  
});
```

**Key Configuration Values:**
| Setting | Normal HTTP | WebSocket |
|---------|-------------|-----------|
| **Total Pool Size** | 256 sockets | 256 sockets |
| **Per Host (Group)** | 6 connections | 255 connections |
| **Per Proxy Chain** | 32 connections | 32 connections |

**Design Rationale:**
- **6 connections per host**: 
  - Based on HTTP/1.1 best practices
  - Avoids overwhelming home routers (see [crbug.com/12066](http://crbug.com/12066))
  - Allows reasonable parallelism for HTTP/1.1
  - For HTTP/2, typically only 1-2 connections are used per host
  
- **255 for WebSocket**: 
  - WebSockets are long-lived, full-duplex connections
  - Similar to Firefox's limit of 200
  - Higher limit because each WebSocket represents one application channel

### Idle Socket Timeout
```cpp
// Source: net/socket/client_socket_pool_manager.cc:204-208
base::TimeDelta ClientSocketPoolManager::unused_idle_socket_timeout(
    HttpNetworkSession::SocketPoolType pool_type) {
  constexpr int kPreconnectIntervalSec = 60;
  return base::Seconds(kPreconnectIntervalSec);
}
```

**Idle Timeout:** 60 seconds for unused sockets

---

## 2. HTTP/2 (SPDY) Settings

### Default Window Sizes
```cpp
// Source: net/spdy/spdy_session.h:74-78

// Max frame data chunk size (before framing overhead)
const int kMaxSpdyFrameChunkSize = (16 * 1024) - 9;  // 16KB - 9 bytes

// Initial flow control window size (RFC 7540 default)
const int32_t kDefaultInitialWindowSize = 65535;  // 64KB - 1 byte
```

**Flow Control:**
- **Initial Window Size:** 65,535 bytes (RFC 7540 default)
- **Max Frame Size:** 16,375 bytes (16KB - 9 bytes for frame header)

### Concurrent Streams
```cpp
// Source: net/spdy/spdy_session.h:80-82
// Maximum number of concurrent streams we will create, unless the server
// sends a SETTINGS frame with a different value.
const size_t kInitialMaxConcurrentStreams = 100;
```

**Initial Max Concurrent Streams:** 100 streams

**Note:** The server can override this with `SETTINGS_MAX_CONCURRENT_STREAMS`

### Frame Queue Limits
```cpp
// Source: net/spdy/spdy_session.h:94-102
// Maximum number of capped frames that can be queued at any time.
const int kSpdySessionMaxQueuedCappedFrames = 10000;
```

**Queue Limit:** 10,000 capped frames to prevent unbounded growth

### Window Update Buffering
```cpp
// Source: net/spdy/spdy_session.h:104-111
// Default time to delay sending small receive window updates
constexpr base::TimeDelta kDefaultTimeToBufferSmallWindowUpdates =
    base::Seconds(5);
```

**Window Update Delay:** 5 seconds
- Prevents excessive WINDOW_UPDATE frames for slow consumers
- Batches small window updates together

### Stream and Session IDs
```cpp
// Source: net/spdy/spdy_session.h:89-92
// First and last valid stream IDs. As we always act as the client,
// start at 1 for the first stream id.
const spdy::SpdyStreamId kFirstStreamId = 1;
const spdy::SpdyStreamId kLastStreamId = 0x7fffffff;
```

**Stream ID Range:**
- **First Stream ID:** 1 (client-initiated streams are odd)
- **Last Stream ID:** 0x7FFFFFFF (2^31 - 1)

### Read Loop Yield Thresholds
```cpp
// Source: net/spdy/spdy_session.h:84-87
// If more than this many bytes have been read or more than that many
// milliseconds have passed, return ERR_IO_PENDING from ReadLoop.
const int kYieldAfterBytesRead = 32 * 1024;  // 32 KB
const int kYieldAfterDurationMilliseconds = 20;  // 20ms
```

**Read Loop Yielding:**
- **Bytes Threshold:** Yield after reading 32 KB
- **Time Threshold:** Yield after 20 milliseconds

**Purpose:** Prevents a single HTTP/2 session from monopolizing the event loop

---

## 3. HTTP/2 SETTINGS Frame

Chromium sends the following settings in the initial SETTINGS frame:

### Standard Settings

```cpp
// Based on analysis of net/spdy/spdy_session.cc and test code

SETTINGS_HEADER_TABLE_SIZE:      (depends on HPACK dynamic table config)
SETTINGS_ENABLE_PUSH:             0  (Server push disabled)
SETTINGS_MAX_CONCURRENT_STREAMS:  (not sent initially, waits for server)
SETTINGS_INITIAL_WINDOW_SIZE:     65535 (kDefaultInitialWindowSize)  
SETTINGS_MAX_FRAME_SIZE:          (uses default 16384)
SETTINGS_MAX_HEADER_LIST_SIZE:    (configurable, often not sent)
```

**Key Behaviors:**
1. **ENABLE_PUSH = 0**: Chromium disables HTTP/2 server push
2. **INITIAL_WINDOW_SIZE**: Set to 65535 bytes (can be adjusted by server)
3. **MAX_CONCURRENT_STREAMS**: Client doesn't send initially; uses default 100 until server specifies

### Experimental/Optional Settings

```cpp
// SETTINGS_ENABLE_CONNECT_PROTOCOL
// Sent when WebSocket over HTTP/2 is supported
SETTINGS_ENABLE_CONNECT_PROTOCOL: 1

// SETTINGS_DEPRECATE_HTTP2_PRIORITIES  
// Indicates support for RFC 9218 Extensible Priorities
SETTINGS_DEPRECATE_HTTP2_PRIORITIES: (if enabled)
```

---

## 4. HTTP/2 Priorities

### Priority Scheme

Chromium supports two priority mechanisms:

1. **HTTP/2 Stream Dependencies & Weights** (legacy)
   - Uses PRIORITY frames
   - Stream dependency tree
   - 1-256 weight values

2. **RFC 9218 Extensible Priorities** (newer)
   - Uses PRIORITY_UPDATE frames
   - Replaces stream dependencies
   - Activated when server sends `SETTINGS_DEPRECATE_HTTP2_PRIORITIES = 1`

```cpp
// Source: net/spdy/spdy_session.h:417-430

// Returns whether HTTP/2 style priority information should be sent.
// True unless SETTINGS_DEPRECATE_HTTP2_PRIORITIES = 1 received from server.
bool ShouldSendHttp2Priority() const;

// Returns whether PRIORITY_UPDATE frames should be sent.
// True if SETTINGS_DEPRECATE_HTTP2_PRIORITIES = 1 received from server.
bool ShouldSendPriorityUpdate() const;
```

---

## 5. HTTP/2 Features & Extensions

### GREASE for HTTP/2
```cpp
// Chromium sends greased (reserved) frame types to prevent ossification
// See: https://tools.ietf.org/html/draft-bishop-httpbis-grease-00

bool GreasedFramesEnabled() const;
void EnqueueGreasedFrame(const base::WeakPtr<SpdyStream>& stream);
```

**Purpose:** Sends frames with reserved type values to ensure middleboxes don't break on unknown frame types

### Extended CONNECT (WebSocket over HTTP/2)
```cpp
// SETTINGS_ENABLE_CONNECT_PROTOCOL enables extended CONNECT method
// Used for WebSocket over HTTP/2 (RFC 8441)
```

### ALPS (Application-Layer Protocol Settings)
```cpp
// Chromium can receive ALPS data during TLS handshake
// Allows server to send HTTP/2 SETTINGS before completing handshake

int ParseAlps();  // Parses ALPS data from TLS
```

**ALPS Support:** Chromium parses ALPS data received via TLS extension

---

## 6. Connection Pooling and Reuse

### Connection Pooling
```cpp
// HTTP/2 connections can be pooled across multiple origins if:
// 1. Server certificate is valid for both hostnames
// 2. No client certificate was sent
// 3. TLS parameters are compatible
// 4. Same proxy chain

bool CanPool(TransportSecurityState* transport_security_state,
             const SSLInfo& ssl_info,
             const SSLConfigService& ssl_config_service,
             std::string_view old_hostname,
             std::string_view new_hostname);
```

**Pooling Rules:**
- Connections can be reused across different hostnames if certificate matches
- Helps reduce connection overhead
- Subject to HSTS and other security policies

### Connection Lifetime
```cpp
// Connections can be in several states:
// - AVAILABLE: Accepting new streams
// - GOING_AWAY: No new streams, but existing streams continue
// - DRAINING: Closing, no new data

enum AvailabilityState {
    STATE_AVAILABLE,
    STATE_GOING_AWAY,
    STATE_DRAINING,
};
```

---

## 7. QUIC (HTTP/3) Settings

### QUIC Configuration
QUIC configuration is extensive and warrants its own document. Key points:

- **QUIC Version**: Chromium supports multiple QUIC versions
- **0-RTT**: Enabled for qualifying requests
- **Connection Migration**: Supported for mobile networks
- **UDP Packet Size**: Typically 1200-1350 bytes

**Location:** `net/quic/` directory contains QUIC-specific configuration

---

## 8. Platform Differences

### Mobile Optimizations
- **Android/iOS**: May use different connection limits based on network type (WiFi vs. Cellular)
- **Connection Migration**: QUIC connection migration is more aggressive on mobile

### Desktop
- **Standard Limits**: Uses the default 6 connections per host
- **Preconnect**: More aggressive preconnecting based on predictions

---

## Summary Table: Key HTTP/2 & Connection Values

| Setting | Value | Source |
|---------|-------|--------|
| **Max Connections Per Host** | 6 | `client_socket_pool_manager.cc:54` |
| **Total Socket Pool Size** | 256 | `client_socket_pool_manager.cc:37` |
| **Initial Window Size** | 65,535 bytes | `spdy_session.h:78` |
| **Max Frame Size** | 16,375 bytes | `spdy_session.h:74` |
| **Max Concurrent Streams** | 100 | `spdy_session.h:82` |
| **Max Queued Frames** | 10,000 | `spdy_session.h:102` |
| **Window Update Buffer** | 5 seconds | `spdy_session.h:110` |
| **Idle Socket Timeout** | 60 seconds | `client_socket_pool_manager.cc:206` |
| **Yield After Bytes** | 32 KB | `spdy_session.h:86` |
| **Yield After Time** | 20 ms | `spdy_session.h:87` |
| **Server Push** | Disabled (0) | Test code analysis |

---

## Implementation Notes for `chromenet`

1. **Connection Pooling:** Implement strict 6-connection-per-host limit for HTTP/1.1
2. **HTTP/2 Multiplexing:** Use single connection per host for HTTP/2
3. **Flow Control:** Implement 65KB initial window size, respect WINDOW_UPDATE
4. **Settings Frame:** Send proper SETTINGS with ENABLE_PUSH=0
5. **Prioritization:** Support both legacy and extensible priorities
6. **ALPN Negotiation:** Properly negotiate h2, http/1.1 via TLS ALPN

---

**Document Version:** 1.0  
**Last Updated:** 2026-01-31  
**Coverage:** Chromium HTTP/2 and connection pool configuration
