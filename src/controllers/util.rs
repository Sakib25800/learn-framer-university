use http::request::Parts;
use http::{Extensions, Request};

pub trait RequestPartsExt {
    fn extensions(&self) -> &Extensions;
}

impl RequestPartsExt for Parts {
    fn extensions(&self) -> &Extensions {
        &self.extensions
    }
}

impl<B> RequestPartsExt for Request<B> {
    fn extensions(&self) -> &Extensions {
        self.extensions()
    }
}
