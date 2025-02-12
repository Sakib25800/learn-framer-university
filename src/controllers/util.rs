use crate::middleware::app::RequestApp;
use crate::util::errors::{forbidden, AppResult};
use http::request::Parts;
use http::{header, Extensions, HeaderMap, HeaderValue, Request};

/// We don't want to accept authenticated requests that originated from other sites, so this
/// function returns an error if the Origin header doesn't match what we expect "this site" to
/// be: <https://learn.framer.university> in production, or <http://localhost:port/> in development.
pub fn verify_origin(parts: &Parts) -> AppResult<()> {
    let headers = parts.headers();
    let allowed_origins = &parts.app().config.allowed_origins;

    let bad_origin = headers
        .get_all(header::ORIGIN)
        .iter()
        .find(|value| !allowed_origins.contains(value));

    if bad_origin.is_some() {
        return Err(forbidden("Invalid origin header"));
    }

    Ok(())
}

pub trait RequestPartsExt {
    fn headers(&self) -> &HeaderMap<HeaderValue>;
    fn extensions(&self) -> &Extensions;
}

impl RequestPartsExt for Parts {
    fn headers(&self) -> &HeaderMap<HeaderValue> {
        &self.headers
    }
    fn extensions(&self) -> &Extensions {
        &self.extensions
    }
}

impl<B> RequestPartsExt for Request<B> {
    fn headers(&self) -> &HeaderMap<HeaderValue> {
        self.headers()
    }
    fn extensions(&self) -> &Extensions {
        self.extensions()
    }
}
