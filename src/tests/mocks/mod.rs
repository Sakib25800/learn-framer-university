pub use app::TestApp;
use axum_test::{TestRequest, TestServer};
use lfu_database::models::user::UserModel;

use crate::auth::Tokens;

mod app;

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

    #[allow(dead_code)]
    fn put(&self, path: &str) -> TestRequest {
        let request = self.server().put(path);
        self.apply_defaults(request)
    }

    #[allow(dead_code)]
    fn delete(&self, path: &str) -> TestRequest {
        let request = self.server().delete(path);
        self.apply_defaults(request)
    }
}

pub struct MockUser {
    app: TestApp,
    user: UserModel,
    tokens: Tokens,
}

impl MockUser {
    pub fn as_model(&self) -> &UserModel {
        &self.user
    }
}

impl RequestHelper for MockUser {
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

pub struct MockAdmin {
    app: TestApp,
    user: UserModel,
    tokens: Tokens,
}

impl MockAdmin {
    pub fn as_model(&self) -> &UserModel {
        &self.user
    }
}

impl RequestHelper for MockAdmin {
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

pub struct MockAnonymous {
    app: TestApp,
}

impl RequestHelper for MockAnonymous {
    fn server(&self) -> &TestServer {
        self.app().server()
    }

    fn app(&self) -> &TestApp {
        &self.app
    }
}
