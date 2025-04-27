use axum::extract::{ConnectInfo, Request};
use axum::middleware::Next;
use axum::response::IntoResponse;
use derive_more::Deref;
use std::net::{IpAddr, SocketAddr};

#[derive(Copy, Clone, Debug, Deref)]
pub struct RealIp(IpAddr);

pub async fn middleware(
    ConnectInfo(socket_addr): ConnectInfo<SocketAddr>,
    mut req: Request,
    next: Next,
) -> impl IntoResponse {
    let headers = req.headers();
    let real_ip: IpAddr;

    // Try Fly-Client-IP first
    if let Some(ip) = headers
        .get("fly-client-ip")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.parse().ok())
    {
        real_ip = ip;
    } else {
        // Fallback to X-Forwarded-For
        if let Some(forwarded) = headers
            .get("x-forwarded-for")
            .and_then(|h| h.to_str().ok())
            .and_then(|s| s.split(",").next())
            .and_then(|s| s.trim().parse().ok())
        {
            real_ip = forwarded;
        } else {
            real_ip = socket_addr.ip();
        }
    }

    req.extensions_mut().insert(RealIp(real_ip));

    next.run(req).await
}
