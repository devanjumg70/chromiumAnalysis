
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, Clone, Copy)]
pub enum HttpError {
    // Connection Errors
    #[error("Connection closed (TCP FIN)")]
    ConnectionClosed,
    #[error("Connection reset (TCP RST)")]
    ConnectionReset,
    #[error("Connection refused")]
    ConnectionRefused,
    #[error("Connection aborted")]
    ConnectionAborted,
    #[error("Connection failed")]
    ConnectionFailed,
    #[error("Name not resolved")]
    NameNotResolved,
    #[error("Internet disconnected")]
    InternetDisconnected,
    #[error("SSL protocol error")]
    SslProtocolError,
    #[error("Address invalid")]
    AddressInvalid,
    #[error("Address unreachable")]
    AddressUnreachable,
    #[error("SSL client auth cert needed")]
    SslClientAuthCertNeeded,
    #[error("Tunnel connection failed")]
    TunnelConnectionFailed,
    #[error("SSL version or cipher mismatch")]
    SslVersionOrCipherMismatch,
    #[error("SSL renegotiation requested")]
    SslRenegotiationRequested,
    #[error("Proxy auth unsupported")]
    ProxyAuthUnsupported,
    #[error("Bad SSL client auth cert")]
    BadSslClientAuthCert,
    #[error("Connection timed out")]
    ConnectionTimedOut,
    #[error("Host resolver queue too large")]
    HostResolverQueueTooLarge,
    #[error("SOCKS connection failed")]
    SocksConnectionFailed,
    #[error("SOCKS connection host unreachable")]
    SocksConnectionHostUnreachable,
    #[error("ALPN negotiation failed")]
    AlpnNegotiationFailed,
    #[error("SSL no renegotiation")]
    SslNoRenegotiation,
    #[error("Winsock unexpected written bytes")]
    WinsockUnexpectedWrittenBytes,
    #[error("SSL decompression failure alert")]
    SslDecompressionFailureAlert,
    #[error("SSL bad record MAC alert")]
    SslBadRecordMacAlert,
    #[error("Proxy auth requested")]
    ProxyAuthRequested,
    #[error("Proxy connection failed")]
    ProxyConnectionFailed,
    #[error("Mandatory proxy configuration failed")]
    MandatoryProxyConfigurationFailed,
    #[error("Preconnect max socket limit")]
    PreconnectMaxSocketLimit,
    #[error("SSL client auth private key access denied")]
    SslClientAuthPrivateKeyAccessDenied,
    #[error("SSL client auth cert no private key")]
    SslClientAuthCertNoPrivateKey,
    #[error("Proxy certificate invalid")]
    ProxyCertificateInvalid,
    #[error("Name resolution failed")]
    NameResolutionFailed,
    #[error("Network access denied")]
    NetworkAccessDenied,
    #[error("Temporarily throttled")]
    TemporarilyThrottled,
    #[error("SSL client auth signature failed")]
    SslClientAuthSignatureFailed,
    #[error("Message too big")]
    MsgTooBig,
    #[error("WebSocket protocol error")]
    WsProtocolError,
    #[error("Address in use")]
    AddressInUse,
    #[error("SSL pinned key not in cert chain")]
    SslPinnedKeyNotInCertChain,
    #[error("Client auth cert type unsupported")]
    ClientAuthCertTypeUnsupported,
    #[error("SSL decrypt error alert")]
    SslDecryptErrorAlert,
    #[error("WebSocket throttle queue too large")]
    WsThrottleQueueTooLarge,
    #[error("SSL server cert changed")]
    SslServerCertChanged,
    #[error("SSL unrecognized name alert")]
    SslUnrecognizedNameAlert,
    #[error("Socket set receive buffer size error")]
    SocketSetReceiveBufferSizeError,
    #[error("Socket set send buffer size error")]
    SocketSetSendBufferSizeError,
    #[error("Socket receive buffer size unchangeable")]
    SocketReceiveBufferSizeUnchangeable,
    #[error("Socket send buffer size unchangeable")]
    SocketSendBufferSizeUnchangeable,
    #[error("SSL client auth cert bad format")]
    SslClientAuthCertBadFormat,
    #[error("ICANN name collision")]
    IcannNameCollision,
    #[error("SSL server cert bad format")]
    SslServerCertBadFormat,
    #[error("CT STH parsing failed")]
    CtSthParsingFailed,
    #[error("CT STH incomplete")]
    CtSthIncomplete,
    #[error("Unable to reuse connection for proxy auth")]
    UnableToReuseConnectionForProxyAuth,
    #[error("CT consistency proof parsing failed")]
    CtConsistencyProofParsingFailed,
    #[error("SSL obsolete cipher")]
    SslObsoleteCipher,
    #[error("WebSocket upgrade")]
    WsUpgrade,
    #[error("ReadIfReady not implemented")]
    ReadIfReadyNotImplemented,
    #[error("No buffer space")]
    NoBufferSpace,
    #[error("SSL client auth no common algorithms")]
    SslClientAuthNoCommonAlgorithms,
    #[error("Early data rejected")]
    EarlyDataRejected,
    #[error("Wrong version on early data")]
    WrongVersionOnEarlyData,
    #[error("TLS 1.3 downgrade detected")]
    Tls13DowngradeDetected,
    #[error("SSL key usage incompatible")]
    SslKeyUsageIncompatible,
    #[error("Invalid ECH config list")]
    InvalidEchConfigList,
    #[error("ECH not negotiated")]
    EchNotNegotiated,
    #[error("ECH fallback certificate invalid")]
    EchFallbackCertificateInvalid,
    #[error("Proxy unable to connect to destination")]
    ProxyUnableToConnectToDestination,
    #[error("Proxy delegate canceled connect request")]
    ProxyDelegateCanceledConnectRequest,
    #[error("Proxy delegate canceled connect response")]
    ProxyDelegateCanceledConnectResponse,

    // HTTP Errors
    #[error("Invalid URL")]
    InvalidUrl,
    #[error("Disallowed URL scheme")]
    DisallowedUrlScheme,
    #[error("Unknown URL scheme")]
    UnknownUrlScheme,
    #[error("Invalid redirect")]
    InvalidRedirect,
    #[error("Too many redirects")]
    TooManyRedirects,
    #[error("Unsafe redirect")]
    UnsafeRedirect,
    #[error("Unsafe port")]
    UnsafePort,
    #[error("Invalid response")]
    InvalidResponse,
    #[error("Invalid chunked encoding")]
    InvalidChunkedEncoding,
    #[error("Method not supported")]
    MethodNotSupported,
    #[error("Unexpected proxy auth")]
    UnexpectedProxyAuth,
    #[error("Empty response")]
    EmptyResponse,
    #[error("Response headers too big")]
    ResponseHeadersTooBig,
    #[error("PAC script failed")]
    PacScriptFailed,
    #[error("Request range not satisfiable")]
    RequestRangeNotSatisfiable,
    #[error("Malformed identity")]
    MalformedIdentity,
    #[error("Content decoding failed")]
    ContentDecodingFailed,
    #[error("Network IO suspended")]
    NetworkIoSuspended,
    #[error("No supported proxies")]
    NoSupportedProxies,
    #[error("HTTP/2 protocol error")]
    Http2ProtocolError,
    #[error("Invalid auth credentials")]
    InvalidAuthCredentials,
    #[error("Unsupported auth scheme")]
    UnsupportedAuthScheme,
    #[error("Encoding detection failed")]
    EncodingDetectionFailed,
    #[error("Missing auth credentials")]
    MissingAuthCredentials,
    #[error("Unexpected security library status")]
    UnexpectedSecurityLibraryStatus,
    #[error("Misconfigured auth environment")]
    MisconfiguredAuthEnvironment,
    #[error("Undocumented security library status")]
    UndocumentedSecurityLibraryStatus,
    #[error("Response body too big to drain")]
    ResponseBodyTooBigToDrain,
    #[error("Response headers multiple Content-Length")]
    ResponseHeadersMultipleContentLength,
    #[error("Incomplete HTTP/2 headers")]
    IncompleteHttp2Headers,
    #[error("PAC not in DHCP")]
    PacNotInDhcp,
    #[error("Response headers multiple Content-Disposition")]
    ResponseHeadersMultipleContentDisposition,
    #[error("Response headers multiple Location")]
    ResponseHeadersMultipleLocation,
    #[error("HTTP/2 server refused stream")]
    Http2ServerRefusedStream,
    #[error("HTTP/2 PING failed")]
    Http2PingFailed,
    #[error("Content-Length mismatch")]
    ContentLengthMismatch,
    #[error("Incomplete chunked encoding")]
    IncompleteChunkedEncoding,
    #[error("QUIC protocol error")]
    QuicProtocolError,
    #[error("Response headers truncated")]
    ResponseHeadersTruncated,
    #[error("QUIC handshake failed")]
    QuicHandshakeFailed,
    #[error("HTTP/2 inadequate transport security")]
    Http2InadequateTransportSecurity,
    #[error("HTTP/2 flow control error")]
    Http2FlowControlError,
    #[error("HTTP/2 frame size error")]
    Http2FrameSizeError,
    #[error("HTTP/2 compression error")]
    Http2CompressionError,
    #[error("Proxy auth requested with no connection")]
    ProxyAuthRequestedWithNoConnection,
    #[error("HTTP/1.1 required")]
    Http11Required,
    #[error("Proxy HTTP/1.1 required")]
    ProxyHttp11Required,
    #[error("PAC script terminated")]
    PacScriptTerminated,
    #[error("Proxy required")]
    ProxyRequired,
    #[error("Invalid HTTP response")]
    InvalidHttpResponse,
    #[error("Content decoding init failed")]
    ContentDecodingInitFailed,
    #[error("HTTP/2 RST_STREAM NO_ERROR received")]
    Http2RstStreamNoErrorReceived,
    #[error("HTTP/2 pushed stream not available")]
    Http2PushedStreamNotAvailable,
    #[error("HTTP/2 claimed pushed stream reset by server")]
    Http2ClaimedPushedStreamResetByServer,
    #[error("Too many retries")]
    TooManyRetries,
    #[error("HTTP/2 stream closed")]
    Http2StreamClosed,
    #[error("HTTP/2 client refused stream")]
    Http2ClientRefusedStream,
    #[error("HTTP/2 pushed response does not match")]
    Http2PushedResponseDoesNotMatch,

    #[error("Unknown error: {0}")]
    Unknown(i32),
}

impl HttpError {
    pub fn as_i32(&self) -> i32 {
        match self {
            HttpError::ConnectionClosed => -100,
            HttpError::ConnectionReset => -101,
            HttpError::ConnectionRefused => -102,
            HttpError::ConnectionAborted => -103,
            HttpError::ConnectionFailed => -104,
            HttpError::NameNotResolved => -105,
            HttpError::InternetDisconnected => -106,
            HttpError::SslProtocolError => -107,
            HttpError::AddressInvalid => -108,
            HttpError::AddressUnreachable => -109,
            HttpError::SslClientAuthCertNeeded => -110,
            HttpError::TunnelConnectionFailed => -111,
            HttpError::SslVersionOrCipherMismatch => -113,
            HttpError::SslRenegotiationRequested => -114,
            HttpError::ProxyAuthUnsupported => -115,
            HttpError::BadSslClientAuthCert => -117,
            HttpError::ConnectionTimedOut => -118,
            HttpError::HostResolverQueueTooLarge => -119,
            HttpError::SocksConnectionFailed => -120,
            HttpError::SocksConnectionHostUnreachable => -121,
            HttpError::AlpnNegotiationFailed => -122,
            HttpError::SslNoRenegotiation => -123,
            HttpError::WinsockUnexpectedWrittenBytes => -124,
            HttpError::SslDecompressionFailureAlert => -125,
            HttpError::SslBadRecordMacAlert => -126,
            HttpError::ProxyAuthRequested => -127,
            HttpError::ProxyConnectionFailed => -130,
            HttpError::MandatoryProxyConfigurationFailed => -131,
            HttpError::PreconnectMaxSocketLimit => -133,
            HttpError::SslClientAuthPrivateKeyAccessDenied => -134,
            HttpError::SslClientAuthCertNoPrivateKey => -135,
            HttpError::ProxyCertificateInvalid => -136,
            HttpError::NameResolutionFailed => -137,
            HttpError::NetworkAccessDenied => -138,
            HttpError::TemporarilyThrottled => -139,
            HttpError::SslClientAuthSignatureFailed => -141,
            HttpError::MsgTooBig => -142,
            HttpError::WsProtocolError => -145,
            HttpError::AddressInUse => -147,
            HttpError::SslPinnedKeyNotInCertChain => -150,
            HttpError::ClientAuthCertTypeUnsupported => -151,
            HttpError::SslDecryptErrorAlert => -153,
            HttpError::WsThrottleQueueTooLarge => -154,
            HttpError::SslServerCertChanged => -156,
            HttpError::SslUnrecognizedNameAlert => -159,
            HttpError::SocketSetReceiveBufferSizeError => -160,
            HttpError::SocketSetSendBufferSizeError => -161,
            HttpError::SocketReceiveBufferSizeUnchangeable => -162,
            HttpError::SocketSendBufferSizeUnchangeable => -163,
            HttpError::SslClientAuthCertBadFormat => -164,
            HttpError::IcannNameCollision => -166,
            HttpError::SslServerCertBadFormat => -167,
            HttpError::CtSthParsingFailed => -168,
            HttpError::CtSthIncomplete => -169,
            HttpError::UnableToReuseConnectionForProxyAuth => -170,
            HttpError::CtConsistencyProofParsingFailed => -171,
            HttpError::SslObsoleteCipher => -172,
            HttpError::WsUpgrade => -173,
            HttpError::ReadIfReadyNotImplemented => -174,
            HttpError::NoBufferSpace => -176,
            HttpError::SslClientAuthNoCommonAlgorithms => -177,
            HttpError::EarlyDataRejected => -178,
            HttpError::WrongVersionOnEarlyData => -179,
            HttpError::Tls13DowngradeDetected => -180,
            HttpError::SslKeyUsageIncompatible => -181,
            HttpError::InvalidEchConfigList => -182,
            HttpError::EchNotNegotiated => -183,
            HttpError::EchFallbackCertificateInvalid => -184,
            HttpError::ProxyUnableToConnectToDestination => -186,
            HttpError::ProxyDelegateCanceledConnectRequest => -187,
            HttpError::ProxyDelegateCanceledConnectResponse => -188,

            HttpError::InvalidUrl => -300,
            HttpError::DisallowedUrlScheme => -301,
            HttpError::UnknownUrlScheme => -302,
            HttpError::InvalidRedirect => -303,
            HttpError::TooManyRedirects => -310,
            HttpError::UnsafeRedirect => -311,
            HttpError::UnsafePort => -312,
            HttpError::InvalidResponse => -320,
            HttpError::InvalidChunkedEncoding => -321,
            HttpError::MethodNotSupported => -322,
            HttpError::UnexpectedProxyAuth => -323,
            HttpError::EmptyResponse => -324,
            HttpError::ResponseHeadersTooBig => -325,
            HttpError::PacScriptFailed => -327,
            HttpError::RequestRangeNotSatisfiable => -328,
            HttpError::MalformedIdentity => -329,
            HttpError::ContentDecodingFailed => -330,
            HttpError::NetworkIoSuspended => -331,
            HttpError::NoSupportedProxies => -336,
            HttpError::Http2ProtocolError => -337,
            HttpError::InvalidAuthCredentials => -338,
            HttpError::UnsupportedAuthScheme => -339,
            HttpError::EncodingDetectionFailed => -340,
            HttpError::MissingAuthCredentials => -341,
            HttpError::UnexpectedSecurityLibraryStatus => -342,
            HttpError::MisconfiguredAuthEnvironment => -343,
            HttpError::UndocumentedSecurityLibraryStatus => -344,
            HttpError::ResponseBodyTooBigToDrain => -345,
            HttpError::ResponseHeadersMultipleContentLength => -346,
            HttpError::IncompleteHttp2Headers => -347,
            HttpError::PacNotInDhcp => -348,
            HttpError::ResponseHeadersMultipleContentDisposition => -349,
            HttpError::ResponseHeadersMultipleLocation => -350,
            HttpError::Http2ServerRefusedStream => -351,
            HttpError::Http2PingFailed => -352,
            HttpError::ContentLengthMismatch => -354,
            HttpError::IncompleteChunkedEncoding => -355,
            HttpError::QuicProtocolError => -356,
            HttpError::ResponseHeadersTruncated => -357,
            HttpError::QuicHandshakeFailed => -358,
            HttpError::Http2InadequateTransportSecurity => -360,
            HttpError::Http2FlowControlError => -361,
            HttpError::Http2FrameSizeError => -362,
            HttpError::Http2CompressionError => -363,
            HttpError::ProxyAuthRequestedWithNoConnection => -364,
            HttpError::Http11Required => -365,
            HttpError::ProxyHttp11Required => -366,
            HttpError::PacScriptTerminated => -367,
            HttpError::ProxyRequired => -368,
            HttpError::InvalidHttpResponse => -370,
            HttpError::ContentDecodingInitFailed => -371,
            HttpError::Http2RstStreamNoErrorReceived => -372,
            HttpError::Http2PushedStreamNotAvailable => -373,
            HttpError::Http2ClaimedPushedStreamResetByServer => -374,
            HttpError::TooManyRetries => -375,
            HttpError::Http2StreamClosed => -376,
            HttpError::Http2ClientRefusedStream => -377,
            HttpError::Http2PushedResponseDoesNotMatch => -378,
            HttpError::Unknown(code) => *code,
        }
    }
}

impl From<i32> for HttpError {
    fn from(code: i32) -> Self {
        match code {
             -100 => HttpError::ConnectionClosed,
             -101 => HttpError::ConnectionReset,
             -102 => HttpError::ConnectionRefused,
             -103 => HttpError::ConnectionAborted,
             -104 => HttpError::ConnectionFailed,
             -105 => HttpError::NameNotResolved,
             -106 => HttpError::InternetDisconnected,
             -107 => HttpError::SslProtocolError,
             -108 => HttpError::AddressInvalid,
             -109 => HttpError::AddressUnreachable,
             -110 => HttpError::SslClientAuthCertNeeded,
             -111 => HttpError::TunnelConnectionFailed,
             -113 => HttpError::SslVersionOrCipherMismatch,
             -114 => HttpError::SslRenegotiationRequested,
             -115 => HttpError::ProxyAuthUnsupported,
             -117 => HttpError::BadSslClientAuthCert,
             -118 => HttpError::ConnectionTimedOut,
             -119 => HttpError::HostResolverQueueTooLarge,
             -120 => HttpError::SocksConnectionFailed,
             -121 => HttpError::SocksConnectionHostUnreachable,
             -122 => HttpError::AlpnNegotiationFailed,
             -123 => HttpError::SslNoRenegotiation,
             -124 => HttpError::WinsockUnexpectedWrittenBytes,
             -125 => HttpError::SslDecompressionFailureAlert,
             -126 => HttpError::SslBadRecordMacAlert,
             -127 => HttpError::ProxyAuthRequested,
             -130 => HttpError::ProxyConnectionFailed,
             -131 => HttpError::MandatoryProxyConfigurationFailed,
             -133 => HttpError::PreconnectMaxSocketLimit,
             -134 => HttpError::SslClientAuthPrivateKeyAccessDenied,
             -135 => HttpError::SslClientAuthCertNoPrivateKey,
             -136 => HttpError::ProxyCertificateInvalid,
             -137 => HttpError::NameResolutionFailed,
             -138 => HttpError::NetworkAccessDenied,
             -139 => HttpError::TemporarilyThrottled,
             -141 => HttpError::SslClientAuthSignatureFailed,
             -142 => HttpError::MsgTooBig,
             -145 => HttpError::WsProtocolError,
             -147 => HttpError::AddressInUse,
             -150 => HttpError::SslPinnedKeyNotInCertChain,
             -151 => HttpError::ClientAuthCertTypeUnsupported,
             -153 => HttpError::SslDecryptErrorAlert,
             -154 => HttpError::WsThrottleQueueTooLarge,
             -156 => HttpError::SslServerCertChanged,
             -159 => HttpError::SslUnrecognizedNameAlert,
             -160 => HttpError::SocketSetReceiveBufferSizeError,
             -161 => HttpError::SocketSetSendBufferSizeError,
             -162 => HttpError::SocketReceiveBufferSizeUnchangeable,
             -163 => HttpError::SocketSendBufferSizeUnchangeable,
             -164 => HttpError::SslClientAuthCertBadFormat,
             -166 => HttpError::IcannNameCollision,
             -167 => HttpError::SslServerCertBadFormat,
             -168 => HttpError::CtSthParsingFailed,
             -169 => HttpError::CtSthIncomplete,
             -170 => HttpError::UnableToReuseConnectionForProxyAuth,
             -171 => HttpError::CtConsistencyProofParsingFailed,
             -172 => HttpError::SslObsoleteCipher,
             -173 => HttpError::WsUpgrade,
             -174 => HttpError::ReadIfReadyNotImplemented,
             -176 => HttpError::NoBufferSpace,
             -177 => HttpError::SslClientAuthNoCommonAlgorithms,
             -178 => HttpError::EarlyDataRejected,
             -179 => HttpError::WrongVersionOnEarlyData,
             -180 => HttpError::Tls13DowngradeDetected,
             -181 => HttpError::SslKeyUsageIncompatible,
             -182 => HttpError::InvalidEchConfigList,
             -183 => HttpError::EchNotNegotiated,
             -184 => HttpError::EchFallbackCertificateInvalid,
             -186 => HttpError::ProxyUnableToConnectToDestination,
             -187 => HttpError::ProxyDelegateCanceledConnectRequest,
             -188 => HttpError::ProxyDelegateCanceledConnectResponse,

             -300 => HttpError::InvalidUrl,
             -301 => HttpError::DisallowedUrlScheme,
             -302 => HttpError::UnknownUrlScheme,
             -303 => HttpError::InvalidRedirect,
             -310 => HttpError::TooManyRedirects,
             -311 => HttpError::UnsafeRedirect,
             -312 => HttpError::UnsafePort,
             -320 => HttpError::InvalidResponse,
             -321 => HttpError::InvalidChunkedEncoding,
             -322 => HttpError::MethodNotSupported,
             -323 => HttpError::UnexpectedProxyAuth,
             -324 => HttpError::EmptyResponse,
             -325 => HttpError::ResponseHeadersTooBig,
             -327 => HttpError::PacScriptFailed,
             -328 => HttpError::RequestRangeNotSatisfiable,
             -329 => HttpError::MalformedIdentity,
             -330 => HttpError::ContentDecodingFailed,
             -331 => HttpError::NetworkIoSuspended,
             -336 => HttpError::NoSupportedProxies,
             -337 => HttpError::Http2ProtocolError,
             -338 => HttpError::InvalidAuthCredentials,
             -339 => HttpError::UnsupportedAuthScheme,
             -340 => HttpError::EncodingDetectionFailed,
             -341 => HttpError::MissingAuthCredentials,
             -342 => HttpError::UnexpectedSecurityLibraryStatus,
             -343 => HttpError::MisconfiguredAuthEnvironment,
             -344 => HttpError::UndocumentedSecurityLibraryStatus,
             -345 => HttpError::ResponseBodyTooBigToDrain,
             -346 => HttpError::ResponseHeadersMultipleContentLength,
             -347 => HttpError::IncompleteHttp2Headers,
             -348 => HttpError::PacNotInDhcp,
             -349 => HttpError::ResponseHeadersMultipleContentDisposition,
             -350 => HttpError::ResponseHeadersMultipleLocation,
             -351 => HttpError::Http2ServerRefusedStream,
             -352 => HttpError::Http2PingFailed,
             -354 => HttpError::ContentLengthMismatch,
             -355 => HttpError::IncompleteChunkedEncoding,
             -356 => HttpError::QuicProtocolError,
             -357 => HttpError::ResponseHeadersTruncated,
             -358 => HttpError::QuicHandshakeFailed,
             -360 => HttpError::Http2InadequateTransportSecurity,
             -361 => HttpError::Http2FlowControlError,
             -362 => HttpError::Http2FrameSizeError,
             -363 => HttpError::Http2CompressionError,
             -364 => HttpError::ProxyAuthRequestedWithNoConnection,
             -365 => HttpError::Http11Required,
             -366 => HttpError::ProxyHttp11Required,
             -367 => HttpError::PacScriptTerminated,
             -368 => HttpError::ProxyRequired,
             -370 => HttpError::InvalidHttpResponse,
             -371 => HttpError::ContentDecodingInitFailed,
             -372 => HttpError::Http2RstStreamNoErrorReceived,
             -373 => HttpError::Http2PushedStreamNotAvailable,
             -374 => HttpError::Http2ClaimedPushedStreamResetByServer,
             -375 => HttpError::TooManyRetries,
             -376 => HttpError::Http2StreamClosed,
             -377 => HttpError::Http2ClientRefusedStream,
             -378 => HttpError::Http2PushedResponseDoesNotMatch,
             _ => HttpError::Unknown(code),
        }
    }
}
