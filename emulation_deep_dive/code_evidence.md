# Code Evidence: Emulation vs. TLS Implementation

## 1. Header Emulation (The "Surface" Layer)
**File**: `content/browser/devtools/protocol/emulation_handler.cc`
**Mechanism**: The `ApplyOverrides` function directly modifies the `net::HttpRequestHeaders` object. This is late-binding, happening just before the request is sent.

```cpp
// content/browser/devtools/protocol/emulation_handler.cc

void EmulationHandler::ApplyOverrides(net::HttpRequestHeaders* headers,
                                      bool* user_agent_overridden,
                                      bool* accept_language_overridden) {
  // 1. User Agent Override
  if (!user_agent_.empty()) {
    headers->SetHeader(net::HttpRequestHeaders::kUserAgent, user_agent_);
  }
  *user_agent_overridden = !user_agent_.empty();

  // 2. Accept Language Override
  if (!accept_language_.empty()) {
    headers->SetHeader(
        net::HttpRequestHeaders::kAcceptLanguage,
        net::HttpUtil::GenerateAcceptLanguageHeader(accept_language_));
  }
  *accept_language_overridden = !accept_language_.empty();

  // 3. Client Hints (Sec-CH-UA-*)
  if (!prefers_color_scheme_.empty()) {
    // ... finds the client hint name and sets it ...
    headers->SetHeader(prefers_color_scheme_client_hint_name, prefers_color_scheme_);
  }
  // ... similar logic for other client hints ...
}
```

## 2. TLS Implementation (The "Deep" Layer)
**File**: `net/socket/ssl_client_socket_impl.cc`
**Mechanism**: TLS Context is a Global Singleton (`SSLContext::GetInstance`) initialized with hardcoded BoringSSL defaults. It does NOT read from DevTools.

```cpp
// net/socket/ssl_client_socket_impl.cc

// Singleton initialization - Happens ONCE for the browser process.
SSLClientSocketImpl::SSLContext::SSLContext() {
  // ...
  ssl_ctx_.reset(SSL_CTX_new(TLS_with_buffers_method()));
  
  // Hardcoded Cipher Suite rules
  // "Use BoringSSL defaults, but disable 3DES and HMAC-SHA1..."
  std::string command("ALL:!aPSK:!ECDSA+SHA1:!3DES");
  
  // This confirms that "Cipher Suites" are global and static.
  // They do NOT change based on the tab's Emulation settings.
  if (!SSL_set_strict_cipher_list(ssl_ctx_.get(), command.c_str())) {
     LOG(ERROR) << "SSL_set_cipher_list('" << command << "') failed";
  }
  // ...
}

// Socket Creation - Happens per connection.
int SSLClientSocketImpl::Init() {
  // ...
  // Creates the SSL object from the global context
  ssl_.reset(SSL_new(context->ssl_ctx())); 
  
  // Extensions (ALPN, etc) come from ssl_config_ 
  // ssl_config_ comes from HttpNetworkSession, NOT EmulationHandler.
  if (!ssl_config_.alpn_protos.empty()) {
     // ...
     SSL_set_alpn_protos(ssl_.get(), wire_protos.data(), wire_protos.size());
  }
  // ...
}
```

## 3. The "Gap"
The disconnection is structural:
1.  **DevTools** (`EmulationHandler`) talks to the **high-level** `URLRequest` via `HttpRequestHeaders`.
2.  **TLS** (`SSLClientSocket`) is created by the **low-level** `ClientSocketPool`.
3.  There is **no mechanism** to pass "Emulated Device Protocol Settings" (like JA3 hash, specific cipher orders, or extension permutations) from the DevTools UI down to the `SSLConfig` used by the socket pool.
