pub mod test_app;

use crate::{auth::Tokens, models::user::User};
use axum_test::{TestRequest, TestServer};
use test_app::TestApp;

/// A collection of helper methods for the three authentication types
/// - Anonymous
/// - User
/// - Admin
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

/// A type that can generated unauthorized requests
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

/// A type that can generated authorized requests
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

/// A type that can generated authorized requests as an admin
pub struct MockAdminUser {
    app: TestApp,
    user: User,
    tokens: Tokens,
}

impl RequestHelper for MockAdminUser {
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

impl MockAdminUser {
    /// Returns a reference to the database `User`
    pub fn as_model(&self) -> &User {
        &self.user
    }
}

pub struct MockAdmin {
    app: TestApp,
}

impl RequestHelper for MockAdmin {
    fn server(&self) -> &TestServer {
        self.app().server()
    }

    fn app(&self) -> &TestApp {
        &self.app
    }
}
