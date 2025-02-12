use crate::{app::AppState, auth::AuthCheck, util::errors::AppResult};
use axum::{extract::Request, middleware::Next, response::Response};

pub async fn auth(state: AppState, req: Request, next: Next) -> AppResult<Response> {
    let mut conn = state.database.get().await?;

    let (parts, body) = req.into_parts();

    crate::controllers::util::verify_origin(&parts)?;

    let user = AuthCheck::check(&state.config.jwt_secret, &parts, &mut conn).await?;

    let mut req = Request::from_parts(parts, body);

    req.extensions_mut().insert(user);

    Ok(next.run(req).await)
}
