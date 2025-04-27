use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tokio::signal::unix::{signal, SignalKind};

use framer_university::{app::App, build_handler, email::Emails};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _sentry = framer_university::sentry::init();

    framer_university::util::tracing::init();

    tracing::info_span!("server.run");

    let config = framer_university::config::Server::from_environment()?;

    let emails = Emails::from_environment(&config);

    let app = Arc::new(App::build(config, emails, None).await);

    // Start the background thread periodically logging instance metrics.
    log_instance_metrics_thread(app.clone());

    let axum_router = build_handler(app.clone());

    let make_service = axum_router.into_make_service_with_connect_info::<SocketAddr>();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    tracing::info!("Listening at 0.0.0.0:8080");

    axum::serve(listener, make_service)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();

    tracing::info!("Server shutdown");

    Ok(())
}

async fn shutdown_signal() {
    let interrupt = async {
        signal(SignalKind::interrupt())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    let terminate = async {
        signal(SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    tokio::select! {
        _ = interrupt => {},
        _ = terminate => {},
    }
}

fn log_instance_metrics_thread(app: Arc<App>) {
    // Only run the thread if the configuration is provided
    let interval = match app.config.instance_metrics_log_every_seconds {
        Some(secs) => Duration::from_secs(secs),
        None => return,
    };

    std::thread::spawn(move || loop {
        if let Err(err) = log_instance_metrics_inner(&app) {
            tracing::error!(?err, "log_instance_metrics error");
        }
        std::thread::sleep(interval);
    });
}

fn log_instance_metrics_inner(app: &App) -> anyhow::Result<()> {
    let metrics = app.instance_metrics.gather(app)?;

    // Log metrics directly to stdout
    tracing::info!(
        metrics = ?metrics,
        "Instance metrics gathered"
    );

    Ok(())
}
