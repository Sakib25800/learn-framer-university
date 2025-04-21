use axum_test::{TestRequest, TestServer};
use lfu_database::models::user::UserModel;

use crate::auth::Tokens;

use super::{app::TestApp, RequestHelper};

pub struct MockUser {
    pub app: TestApp,
    pub user: UserModel,
    pub tokens: Tokens,
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
    pub app: TestApp,
    pub user: UserModel,
    pub tokens: Tokens,
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
    pub app: TestApp,
}

impl RequestHelper for MockAnonymous {
    fn server(&self) -> &TestServer {
        self.app().server()
    }

    fn app(&self) -> &TestApp {
        &self.app
    }
}
