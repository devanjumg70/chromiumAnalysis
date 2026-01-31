# Chromium TLS/BoringSSL Configuration Research

## Executive Summary

This document provides comprehensive documentation of how Chromium configures BoringSSL for TLS connections across all platforms. This research is intended to support the development of `chromenet`, a Rust crate that exactly replicates Chrome's networking behavior.

**Source Files:** Based on Chromium source code analysis (chromium/net/)

---

## 1. TLS Protocol Versions

### Default Configuration
```cpp
// Source: net/ssl/ssl_config.cc
const uint16_t kDefaultSSLVersionMin = SSL_PROTOCOL_VERSION_TLS1_2;  // 0x0303
const uint16_t kDefaultSSLVersionMax = SSL_PROTOCOL_VERSION_TLS1_3;  // 0x0304
```

**Key Points:**
- **Minimum TLS Version:** TLS 1.2 (0x0303)
- **Maximum TLS Version:** TLS 1.3 (0x0304)
- **Deprecated:** SSL 2.0, SSL 3.0, TLS 1.0, and TLS 1.1 are NOT supported
- Code enforces TLS 1.2+ in `ssl_client_socket_impl.cc` lines 731-733

### Runtime Configuration
```cpp
// Source: net/socket/ssl_client_socket_impl.cc:727-739
uint16_t version_min = ssl_config_.version_min_override.value_or(
    context_->config().version_min);
uint16_t version_max = ssl_config_.version_max_override.value_or(
    context_->config().version_max);

if (version_min < TLS1_2_VERSION || version_max < TLS1_2_VERSION) {
    // TLS versions before TLS 1.2 are no longer supported.
    return ERR_UNEXPECTED;
}

SSL_set_min_proto_version(ssl_.get(), version_min);
SSL_set_max_proto_version(ssl_.get(), version_max);
```

---

## 2. Named Groups (Supported Curves)

### Default Supported Groups
```cpp
// Source: net/ssl/ssl_config_service.cc:26-31
const SSLNamedGroupInfo kDefaultSSLSupportedGroups[] = {
    {.group_id = SSL_GROUP_X25519_MLKEM768, .send_key_share = true},   // Post-quantum hybrid
    {.group_id = SSL_GROUP_X25519,          .send_key_share = true},   // Curve25519
    {.group_id = SSL_GROUP_SECP256R1,       .send_key_share = false},  // P-256
    {.group_id = SSL_GROUP_SECP384R1,       .send_key_share = false},  // P-384
};
```

**Configuration Details:**
- **Order:** Groups are listed in preference order
- **Key Shares:** Only `X25519_MLKEM768` and `X25519` are sent in the initial ClientHello `key_share` extension
- **Post-Quantum:** X25519MLKEM768 is a hybrid post-quantum group (default since ~2024)
- **P-256/P-384:** Included in `supported_groups` but not in initial `key_share`

### Post-Quantum Support
```cpp
// Source: net/ssl/ssl_config_service.cc:65-68
bool SSLNamedGroupInfo::IsPostQuantum() const {
  return group_id == SSL_GROUP_X25519_MLKEM768 ||
         group_id == SSL_GROUP_MLKEM1024;
}
```

### Application in SSL Context
```cpp
// Source: net/socket/ssl_client_socket_impl.cc:681-693
const std::vector<uint16_t> supported_groups =
    context_->config().GetSupportedGroups();
SSL_set1_group_ids(ssl_.get(), supported_groups.data(),
                   supported_groups.size());

const std::vector<uint16_t> key_shares =
    context_->config().GetSupportedGroups(/*key_shares_only=*/true);
if (!key_shares.empty()) {
    SSL_set1_client_key_shares(ssl_.get(), key_shares.data(),
                               key_shares.size());
}
```

---

## 3. Cipher Suites

### Cipher Configuration String
```cpp
// Source: net/socket/ssl_client_socket_impl.cc:750-764
// Use BoringSSL defaults, but disable 3DES and HMAC-SHA1 ciphers in ECDSA.
// These are the remaining CBC-mode ECDSA ciphers.
std::string command("ALL:!aPSK:!ECDSA+SHA1:!3DES");

if (ssl_config_.require_ecdhe)
    command.append(":!kRSA");

// Remove any disabled ciphers.
for (uint16_t id : context_->config().disabled_cipher_suites) {
    const SSL_CIPHER* cipher = SSL_get_cipher_by_value(id);
    if (cipher) {
        command.append(":!");
        command.append(SSL_CIPHER_get_name(cipher));
    }
}

SSL_set_strict_cipher_list(ssl_.get(), command.c_str());
```

**Cipher Suite Policy:**
- **Base:** Start with BoringSSL's default `ALL` ciphers
- **Excluded:** Anonymous PSK (`!aPSK`), ECDSA+SHA1 (`!ECDSA+SHA1`), 3DES (`!3DES`)
- **Optional:** Disable RSA key exchange if `require_ecdhe` is set (`!kRSA`)
- **Additional Exclusions:** User-configured disabled ciphers from `disabled_cipher_suites`

### TLS 1.3 Cipher Preference
```cpp
// Source: net/ssl/ssl_config_service.h:96-98
// This configures a compliance policy that sets the cipher order for
// TLS 1.3 to prefer AES-256-GCM over AES-128-GCM over ChaCha20-Poly1305.
bool tls13_cipher_prefer_aes_256 = false;
```

```cpp
// Source: net/socket/ssl_client_socket_impl.cc:857-863
// The compliance policy must be the last thing configured in order to have
// defined behavior.
if (context_->config().tls13_cipher_prefer_aes_256 &&
    !SSL_set_compliance_policy(ssl_.get(),
                               ssl_compliance_policy_cnsa_202407)) {
    return ERR_UNEXPECTED;
} 
```

**Note:** When enabled, TLS 1.3 prefers AES-256-GCM > AES-128-GCM > ChaCha20-Poly1305

### Signature Algorithms (Verify Preferences)
```cpp
// Source: net/socket/ssl_client_socket_impl.cc:771-783
// Disable SHA-1 server signatures.
static const uint16_t kVerifyPrefs[] = {
    SSL_SIGN_ECDSA_SECP256R1_SHA256, SSL_SIGN_RSA_PSS_RSAE_SHA256,
    SSL_SIGN_RSA_PKCS1_SHA256,       SSL_SIGN_ECDSA_SECP384R1_SHA384,
    SSL_SIGN_RSA_PSS_RSAE_SHA384,    SSL_SIGN_RSA_PKCS1_SHA384,
    SSL_SIGN_RSA_PSS_RSAE_SHA512,    SSL_SIGN_RSA_PKCS1_SHA512,
};
SSL_set_verify_algorithm_prefs(ssl_.get(), kVerifyPrefs,
                               std::size(kVerifyPrefs));
```

**Key Points:**
- **SHA-1 Excluded:** No SHA-1 signature algorithms
- **Preferred:** ECDSA & RSA-PSS with SHA-256, then SHA-384, then SHA-512
- **Includes:** Both modern (RSA-PSS) and legacy (PKCS#1) RSA signatures

---

## 4. ALPN (Application-Layer Protocol Negotiation)

### Configuration
```cpp
// Source: net/socket/ssl_client_socket_impl.cc:789-806
if (!ssl_config_.alpn_protos.empty()) {
    std::vector<uint8_t> wire_protos =
        SerializeNextProtos(ssl_config_.alpn_protos);
    SSL_set_alpn_protos(ssl_.get(), wire_protos.data(), wire_protos.size());

    // ALPS (Application-Layer Protocol Settings)
    for (NextProto proto : ssl_config_.alpn_protos) {
        auto iter = ssl_config_.application_settings.find(proto);
        if (iter != ssl_config_.application_settings.end()) {
            const char* proto_string = NextProtoToString(proto);
            SSL_add_application_settings(
                ssl_.get(), 
                reinterpret_cast<const uint8_t*>(proto_string),
                strlen(proto_string), 
                iter->second.data(),
                iter->second.size());
        }
    }
}
```

**ALPN Support:**
- **Protocols:** Configurable list of supported protocols (HTTP/2, HTTP/3, etc.)
- **Wire Format:** Protocols are serialized to wire format before setting
- **ALPS:** Application-Layer Protocol Settings can be configured per protocol

### ALPS Codepoint
```cpp
// Source: net/socket/ssl_client_socket_impl.cc:785-787
SSL_set_alps_use_new_codepoint(
    ssl_.get(),
    base::FeatureList::IsEnabled(features::kUseNewAlpsCodepointHttp2));
```

---

## 5. Certificate Verification & Security

### Custom Verification Callback
```cpp
// Source: net/socket/ssl_client_socket_impl.cc:192-195
// Verifies the server certificate even on resumed sessions.
SSL_CTX_set_reverify_on_resume(ssl_ctx_.get(), 1);
SSL_CTX_set_custom_verify(ssl_ctx_.get(), SSL_VERIFY_PEER,
                          VerifyCertCallback);
```

**Key Points:**
- **Always Verify:** Even resumed sessions are re-verified
- **Custom Callback:** Uses Chrome's own certificate verification logic
- **Mode:** `SSL_VERIFY_PEER` (verify server certificate)

### OCSP & Certificate Transparency
```cpp
// Source: net/socket/ssl_client_socket_impl.cc:808-809
SSL_enable_signed_cert_timestamps(ssl_.get());
SSL_enable_ocsp_stapling(ssl_.get());
```

**Enabled Features:**
- **OCSP Stapling:** Enabled for certificate revocation checking
- **SCT (Signed Certificate Timestamps):** Enabled for Certificate Transparency

### Certificate Compression
```cpp
// Source: net/socket/ssl_client_socket_impl.cc:210
ConfigureCertificateCompression(ssl_ctx_.get());
```

**Note:** Chromium supports certificate compression (Brotli, Zstandard) to reduce handshake size

---

## 6. Session Resumption & Caching

### Session Cache Configuration
```cpp
// Source: net/socket/ssl_client_socket_impl.cc:196-201
// Disable the internal session cache. Session caching is handled
// externally (i.e. by SSLClientSessionCache).
SSL_CTX_set_session_cache_mode(
    ssl_ctx_.get(), 
    SSL_SESS_CACHE_CLIENT | SSL_SESS_CACHE_NO_INTERNAL);
SSL_CTX_sess_set_new_cb(ssl_ctx_.get(), NewSessionCallback);
SSL_CTX_set_timeout(ssl_ctx_.get(), 1 * 60 * 60 /* one hour */);
```

**Configuration:**
- **External Cache:** Uses `SSLClientSessionCache` instead of OpenSSL's internal cache
- **Cache Mode:** Client-side caching only, no internal BoringSSL cache
- **Session Timeout:** 1 hour (3600 seconds)
- **Callback:** Custom callback to store new sessions

### Session Lookup
```cpp
// Source: net/socket/ssl_client_socket_impl.cc:695-714
if (IsCachingEnabled()) {
    bssl::UniquePtr<SSL_SESSION> session =
        context_->ssl_client_session_cache()->Lookup(
            GetSessionCacheKey(/*dest_ip_addr=*/std::nullopt));
    
    // Fallback: Try IP-based cache key for RSA cipher suites
    if (!session) {
        IPEndPoint peer_address;
        if (stream_socket_->GetPeerAddress(&peer_address) == OK) {
            session = context_->ssl_client_session_cache()->Lookup(
                GetSessionCacheKey(peer_address.address()));
        }
    }
    
    if (session)
        SSL_set_session(ssl_.get(), session.get());
}
```

**Cache Key Partitioning:**
- **Primary Key:** Hostname-based (with privacy mode and network anonymization)
- **Fallback Key:** IP address (for legacy RSA cipher suite compatibility)

---

## 7. TLS Extensions & Features

### GREASE (Generate Random Extensions And Sustain Extensibility)
```cpp
// Source: net/socket/ssl_client_socket_impl.cc:203
SSL_CTX_set_grease_enabled(ssl_ctx_.get(), 1);
```

**Enabled:** GREASE is enabled to prevent ossification of the TLS ecosystem

### ECH (Encrypted Client Hello)
```cpp
// Source: net/socket/ssl_client_socket_impl.cc:830-845
if (context_->config().ech_enabled) {
    SSL_set_enable_ech_grease(ssl_.get(), 1);
}

if (!ssl_config_.ech_config_list.empty()) {
    DCHECK(context_->config().ech_enabled);
    SSL_set1_ech_config_list(ssl_.get(),
                             ssl_config_.ech_config_list.data(),
                             ssl_config_.ech_config_list.size());
}
```

**ECH Configuration:**
- **GREASE:** ECH GREASE is enabled when `ech_enabled` is true
- **Config List:** Uses provided ECHConfigList if available
- **Privacy:** Encrypts the SNI and other sensitive ClientHello fields

### SNI (Server Name Indication)
```cpp
// Source: net/socket/ssl_client_socket_impl.cc:668-679
const bool host_is_ip_address =
    HostIsIPAddressNoBrackets(host_and_port_.host());

// SNI should only contain valid DNS hostnames, not IP addresses
if (!host_is_ip_address &&
    !SSL_set_tlsext_host_name(ssl_.get(), host_and_port_.host().c_str())) {
    return ERR_UNEXPECTED;
}
```

**SNI Rules:**
- **DNS Names Only:** SNI is only set for DNS hostnames, not IP addresses
- **RFC 6066 Compliant:** Follows RFC 6066, Section 3

### Extension Permutation
```cpp
// Source: net/socket/ssl_client_socket_impl.cc:847
SSL_set_permute_extensions(ssl_.get(), 1);
```

**Randomization:** TLS extensions are permuted to prevent fingerprinting

### Trust Anchor IDs
```cpp
// Source: net/socket/ssl_client_socket_impl.cc:849-855
// Configure BoringSSL to send Trust Anchor IDs, if provided.
if (ssl_config_.trust_anchor_ids.has_value() &&
    !SSL_set1_requested_trust_anchors(ssl_.get(),
                                      ssl_config_.trust_anchor_ids->data(),
                                      ssl_config_.trust_anchor_ids->size())) {
    return ERR_UNEXPECTED;
}
```

**Purpose:** Helps servers select appropriate certificate chains

---

## 8. Early Data (0-RTT)

### Configuration
```cpp
// Source: net/socket/ssl_client_socket_impl.cc:741
SSL_set_early_data_enabled(ssl_.get(), ssl_config_.early_data_enabled);
```

```cpp
// Source: net/ssl/ssl_config.h:65-77
// Whether early data is enabled on this connection. Note that early data has
// weaker security properties than normal data and changes the
// SSLClientSocket's behavior. The caller must only send replayable data prior
// to handshake confirmation.
//
// Additionally, early data may be rejected by the server, resulting in some
// socket operation failing with ERR_EARLY_DATA_REJECTED or
// ERR_WRONG_VERSION_ON_EARLY_DATA before any data is returned from the
// server.
bool early_data_enabled = false;
```

**Key Points:**
- **Default:** Disabled (`false`)
- **Security Warning:** Early data has weaker replay protection
- **Use Case:** Only for replayable requests (e.g., idempotent HTTP GET)

---

## 9. Additional SSL/TLS Options

### SSL Modes
```cpp
// Source: net/socket/ssl_client_socket_impl.cc:747-748
SSL_set_mode(ssl_.get(),
             SSL_MODE_CBC_RECORD_SPLITTING | SSL_MODE_ENABLE_FALSE_START);
```

**Modes:**
- **`SSL_MODE_CBC_RECORD_SPLITTING`:** Mitigates BEAST attack in TLS 1.0
- **`SSL_MODE_ENABLE_FALSE_START`:** Allows TLS False Start optimization

### Legacy Server Connect
```cpp
// Source: net/socket/ssl_client_socket_impl.cc:743-745
// TODO(crbug.com/41393419): Make this option not a no-op in BoringSSL and
// then disable it.
SSL_set_options(ssl_.get(), SSL_OP_LEGACY_SERVER_CONNECT);
```

**Note:** Currently a no-op in BoringSSL

### Renegotiation
```cpp
// Source: net/socket/ssl_client_socket_impl.cc:811-817
// Configure BoringSSL to allow renegotiations. Once the initial handshake
// completes, if renegotiations are not allowed, the default reject value will
// be restored. Use ssl_renegotiate_explicit rather than ssl_renegotiate_freely
// so DoPeek() does not trigger renegotiations.
SSL_set_renegotiate_mode(ssl_.get(), ssl_renegotiate_explicit);
```

**Policy:**
- **Initial:** Allow explicit renegotiation during handshake
- **Post-Handshake:** Configurable based on ALPN protocol

### Handshake Config Shedding
```cpp
// Source: net/socket/ssl_client_socket_impl.cc:819
SSL_set_shed_handshake_config(ssl_.get(), 1);
```

**Optimization:** Free handshake-only memory after handshake completion

---

## 10. BoringSSL Context Configuration

### Context-Wide Settings
```cpp
// Source: net/socket/ssl_client_socket_impl.cc:185-210
SSLContext() {
    ssl_ctx_.reset(SSL_CTX_new(TLS_with_buffers_method()));
    
    // Certificate callback for client certificates
    SSL_CTX_set_cert_cb(ssl_ctx_.get(), ClientCertRequestCallback, nullptr);
    
    // Reverify certificates on session resumption
    SSL_CTX_set_reverify_on_resume(ssl_ctx_.get(), 1);
    
    // Custom verification callback
    SSL_CTX_set_custom_verify(ssl_ctx_.get(), SSL_VERIFY_PEER,
                              VerifyCertCallback);
    
    // External session cache (disable internal)
    SSL_CTX_set_session_cache_mode(
        ssl_ctx_.get(), SSL_SESS_CACHE_CLIENT | SSL_SESS_CACHE_NO_INTERNAL);
    SSL_CTX_sess_set_new_cb(ssl_ctx_.get(), NewSessionCallback);
    
    // Session timeout: 1 hour
    SSL_CTX_set_timeout(ssl_ctx_.get(), 1 * 60 * 60);
    
    // Enable GREASE
    SSL_CTX_set_grease_enabled(ssl_ctx_.get(), 1);
    
    // Buffer pool for certificate deduplication
    SSL_CTX_set0_buffer_pool(ssl_ctx_.get(), x509_util::GetBufferPool());
    
    // Message callback for logging
    SSL_CTX_set_msg_callback(ssl_ctx_.get(), MessageCallback);
    
    // Certificate compression (Brotli, Zstandard)
    ConfigureCertificateCompression(ssl_ctx_.get());
}
```

### Buffer Sizes
```cpp
// Source: net/socket/ssl_client_socket_impl.cc:83-84
// Default size of the internal BoringSSL buffers.
const int kDefaultOpenSSLBufferSize = 17 * 1024;  // 17 KB
```

---

## 11. Privacy & Partitioning

### Privacy Mode
```cpp
// Source: net/ssl/ssl_config.h:141-148
// An additional boolean to partition the session cache by.
// If PRIVACY_MODE_ENABLED_WITHOUT_CLIENT_CERTS, client certificates are
// disabled (or will be in the future).
PrivacyMode privacy_mode = PRIVACY_MODE_DISABLED;
```

### Network Anonymization
```cpp
// Source: net/ssl/ssl_config.h:130-132
// If the PartitionConnectionsByNetworkIsolationKey feature is enabled, the
// session cache is partitioned by this value.
NetworkAnonymizationKey network_anonymization_key;
```

**Key Points:**
- **Session Cache Partitioning:** Prevents cross-site tracking via session resume
- **Client Certificates:** Can be disabled in privacy mode

---

## 12. Platform-Specific Notes

### General
- Most TLS configuration is **platform-independent** (handled by BoringSSL)
- Platform differences mainly in:
  - **Client certificate stores** (Windows CryptoAPI, macOS Keychain, NSS on Linux)
  - **Platform key operations** (signing with hardware-backed keys)

### Client Certificate Platforms
```cpp
// Source: net/ssl/ directory
// - client_cert_store_win.cc/h     (Windows)
// - client_cert_store_mac.cc/h     (macOS)
// - client_cert_store_nss.cc/h     (Linux/NSS)
// - ssl_platform_key_win.cc        (Windows CNG/CryptoAPI)
// - ssl_platform_key_mac.cc        (macOS SecKey)
// - ssl_platform_key_android.cc    (Android KeyStore)
```

**Mobile Platforms:**
- **Android:** Uses Android KeyStore for client certificates
- **iOS:** Uses iOS Keychain (similar to macOS)

---

## Summary: Key Configuration Values

| Setting | Value | Source |
|---------|-------|--------|
| **Min TLS Version** | TLS 1.2 (0x0303) | `ssl_config.cc:13` |
| **Max TLS Version** | TLS 1.3 (0x0304) | `ssl_config.cc:14` |
| **Default Groups** | X25519MLKEM768, X25519, P-256, P-384 | `ssl_config_service.cc:26-31` |
| **Key Shares** | X25519MLKEM768, X25519 | `ssl_config_service.cc:27-28` |
| **Cipher String** | `ALL:!aPSK:!ECDSA+SHA1:!3DES` | `ssl_client_socket_impl.cc:752` |
| **Session Timeout** | 1 hour (3600s) | `ssl_client_socket_impl.cc:201` |
| **Buffer Size** | 17 KB | `ssl_client_socket_impl.cc:84` |
| **GREASE** | Enabled | `ssl_client_socket_impl.cc:203` |
| **OCSP Stapling** | Enabled | `ssl_client_socket_impl.cc:809` |
| **SCT** | Enabled | `ssl_client_socket_impl.cc:808` |
| **ECH** | Configurable (default enabled) | `ssl_config_service.h:106` |
| **Early Data** | Configurable (default disabled) | `ssl_config.h:77` |
| **Extension Permutation** | Enabled | `ssl_client_socket_impl.cc:847` |

---

## Next Steps for `chromenet` Implementation

1. **BoringSSL Rust Bindings:** Use the `boring` crate for BoringSSL FFI
2. **Match Configuration Exactly:** Apply all settings documented above
3. **Test Against Chromium:** Use TLS fingerprinting tools to verify matching behavior
4. **Platform Abstraction:** Abstract platform-specific certificate handling
5. **Session Cache:** Implement external session cache matching Chrome's logic

---

**Document Version:** 1.0  
**Last Updated:** 2026-01-31  
**Coverage:** Chromium net/ssl and net/socket SSL/TLS configuration
