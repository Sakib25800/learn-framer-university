pub use app::TestApp;
use axum_test::{TestRequest, TestServer};
pub use user::{MockAdmin, MockAnonymous, MockUser};

mod app;
mod user;

pub trait RequestHelper {
    fn server(&self) -> &TestServer;
    fn app(&self) -> &TestApp;

    /// Apply default configuration to every request.
    fn apply_defaults(&self, request: TestRequest) -> TestRequest {
        request
    }

    fn get(&self, path: &str) -> TestRequest {
        let request = self.server().get(path);
        self.apply_defaults(request)
    }

    fn post(&self, path: &str) -> TestRequest {
        let request = self.server().post(path);
        self.apply_defaults(request)
    }

    fn put(&self, path: &str) -> TestRequest {
        let request = self.server().put(path);
        self.apply_defaults(request)
    }

    fn delete(&self, path: &str) -> TestRequest {
        let request = self.server().delete(path);
        self.apply_defaults(request)
    }
}
