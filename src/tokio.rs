use crate::ProxyPathConfig;
use vetis::{
    Request, Response, VetisFutureResult,
    errors::VetisError,
    host::{HostContext, path::Path},
};

/// Proxy path
pub struct ProxyPath {
    config: ProxyPathConfig,
}

impl ProxyPath {
    /// Create a new proxy path with provided configuration
    ///
    /// # Arguments
    ///
    /// * `config` - The proxy path configuration
    ///
    /// # Returns
    ///
    /// * `ProxyPath` - The proxy path
    pub fn new(config: ProxyPathConfig) -> ProxyPath {
        ProxyPath { config }
    }
}

impl Path for ProxyPath {
    /// Get the URI of the proxy path
    ///
    /// # Returns
    ///
    /// * `&str` - The URI of the proxy path
    fn uri(&self) -> &str {
        &self.config.uri
    }

    /// Handle proxy request
    ///
    /// # Arguments
    ///
    /// * `request` - The request to handle
    /// * `uri` - The URI of the request
    ///
    /// # Returns
    ///
    /// * `Pin<Box<dyn Future<Output = Result<Response, VetisError>> + Send + Sync + '_>>` - The future that will resolve to the response
    fn handle<'a>(
        &'a self,
        request: Request,
        host_context: HostContext,
    ) -> VetisFutureResult<'a, Response> {
        Box::pin(async move { Ok::<Response, VetisError>(Response::builder().empty()) })
    }
}
