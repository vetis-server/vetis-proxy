# vetis-proxy

[![Crates.io downloads](https://img.shields.io/crates/d/vetis-proxy)](https://crates.io/crates/vetis-proxy) [![crates.io](https://img.shields.io/crates/v/vetis-proxy?style=flat-square)](https://crates.io/crates/vetis-proxy) [![Build Status](https://github.com/vetis-server/vetis-proxy/actions/workflows/rust.yml/badge.svg?event=push)](https://github.com/vetis-server/vetis-proxy/actions/workflows/rust.yml) ![Crates.io MSRV](https://img.shields.io/crates/msrv/vetis-proxy) [![Documentation](https://docs.rs/vetis-proxy/badge.svg)](https://docs.rs/vetis-proxy/latest/vetis-proxy) [![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/vetis-server/vetis-proxy/blob/main/LICENSE.md)  [![codecov](https://codecov.io/gh/vetis-server/vetis-proxy/graph/badge.svg?token=T0HSBAPVSI)](https://codecov.io/gh/vetis-server/vetis-proxy)

Proxy support for the vetis HTTP server framework. This module provides HTTP proxy functionality to forward requests to backend servers.

## Features

- Forward HTTP requests to backend servers
- Configurable URI prefix and target URL
- Support for Tokio async runtime
- Integration with vetis virtual host configuration
- TLS support via rustls

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
vetis-proxy = "0.1.0"
```

### Runtime Features

Currently only Tokio runtime is supported:

- `runtime-tokio` (default): Use with Tokio runtime

## Configuration

### Using API

Configure reverse proxy using the builder pattern:

```rust,ignore
use vetis_proxy::ProxyPathConfig;

let config = ProxyPathConfig::builder()
    .uri("/api")
    .target("http://localhost:8080")
    .build()?;
```

### vetis.yaml

```yaml
type: proxy
uri: /api
target: http://localhost:8080
```

### Configuration Options

#### ProxyPathConfig

- **uri**: The URI prefix for the proxy path (required)
- **target**: The target URL to forward requests to (required)

## Runtime-Specific Modules

Import the Tokio runtime module:

```rust,ignore
// For Tokio runtime
#[cfg(feature = "runtime-tokio")]
use vetis_proxy::tokio;
```

## License

Licensed under either of

- Apache License, Version 2.0
  (LICENSE-APACHE or <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license
  (LICENSE-MIT or <https://opensource.org/licenses/MIT>)

at your option.

## Author

Rogerio Pereira Araujo <rogerio.araujo@gmail.com>
