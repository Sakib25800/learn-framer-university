pub mod test_app;

use crate::{auth::Tokens, models::user::User};
use axum_test::{TestRequest, TestServer};
use test_app::TestApp;

/// A collection of helper methods for the two authentication types
/// - Anonymous
/// - User
///
/// Helper methods should not modify the database directly
#[allow(async_fn_in_trait)]
pub trait RequestHelper {
    fn server(&self) -> &TestServer;
    fn app(&self) -> &TestApp;

    /// Apply default configuration to every request
    fn apply_defaults(&self, request: TestRequest) -> TestRequest {
        request
    }

    fn get(&self, path: &str) -> TestRequest {
        let request = self.server().get(path);
        self.apply_defaults(request)
    }

    fn post(&self, path: &str, body: String) -> TestRequest {
        let request = self.server().post(path).json(&body);
        self.apply_defaults(request)
    }

    fn put(&self, path: &str, body: String) -> TestRequest {
        let request = self.server().put(path).json(&body);
        self.apply_defaults(request)
    }

    fn delete(&self, path: &str) -> TestRequest {
        let request = self.server().delete(path);
        self.apply_defaults(request)
    }
}

/// A type that that can generate unauthenticated requests
pub struct MockAnonymousUser {
    app: TestApp,
}

impl RequestHelper for MockAnonymousUser {
    fn server(&self) -> &TestServer {
        self.app().server()
    }

    fn app(&self) -> &TestApp {
        &self.app
    }
}

/// A type that can generate authenticated requests
pub struct MockAuthUser {
    app: TestApp,
    user: User,
    tokens: Tokens,
}

impl RequestHelper for MockAuthUser {
    fn server(&self) -> &TestServer {
        self.app().server()
    }

    fn app(&self) -> &TestApp {
        &self.app
    }

    fn apply_defaults(&self, request: TestRequest) -> TestRequest {
        request.authorization_bearer(&self.tokens.access_token)
    }
}

impl MockAuthUser {
    /// Returns a reference to the database `User`
    pub fn as_model(&self) -> &User {
        &self.user
    }
}
