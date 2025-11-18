use std::time::Instant;

use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Data, Orbit, Build, Request, Response, Rocket};
use tracing::{info, Span};
use tracing_subscriber::{fmt, layer::SubscriberExt, EnvFilter, Registry};
use tracing_appender::rolling;
use once_cell::sync::OnceCell;
use std::fs;

static APPENDER_GUARD: OnceCell<tracing_appender::non_blocking::WorkerGuard> = OnceCell::new();

pub fn init_tracing() {
    let env_filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info,sqlx=warn,rocket=info"))
        .unwrap();

    // Rolling daily file writer to logs/app.log
    let _ = fs::create_dir_all("logs");
    let file_appender = rolling::daily("logs", "app.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
    let _ = APPENDER_GUARD.set(guard);

    // stdout JSON logs
    let stdout_layer = fmt::layer()
        .with_target(true)
        .with_thread_ids(false)
        .with_ansi(false)
        .json();

    // file JSON logs
    let file_layer = fmt::layer()
        .with_target(true)
        .with_thread_ids(false)
        .with_ansi(false)
        .json()
        .with_writer(non_blocking);

    let subscriber = Registry::default()
        .with(env_filter)
        .with(stdout_layer)
        .with(file_layer);

    tracing::subscriber::set_global_default(subscriber).ok();
}

pub struct RequestTracingFairing;

#[rocket::async_trait]
impl Fairing for RequestTracingFairing {
    fn info(&self) -> Info {
        Info {
            name: "request_tracing",
            kind: Kind::Request | Kind::Response | Kind::Ignite | Kind::Liftoff,
        }
    }

    async fn on_ignite(&self, rocket: Rocket<Build>) -> rocket::fairing::Result {
        info!(event = "ignite", "Rocket is igniting");
        Ok(rocket)
    }

    async fn on_liftoff(&self, _rocket: &Rocket<Orbit>) {
        info!(event = "liftoff", "Rocket has launched");
    }

    async fn on_request(&self, request: &mut Request<'_>, _data: &mut Data<'_>) {
        let start = Instant::now();
        request.local_cache(|| start);

        let request_id = uuid::Uuid::new_v4().to_string();
        request.add_header(rocket::http::Header::new("x-request-id", request_id.clone()));

        let method = request.method().as_str().to_string();
        let uri = request.uri().to_string();
        Span::current();
        info!(
            request_id = %request_id,
            method = %method,
            uri = %uri,
            event = "request_start"
        );
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        let latency_ms = request
            .local_cache::<Instant, _>(|| Instant::now())
            .elapsed()
            .as_millis();

        let request_id = request
            .headers()
            .get_one("x-request-id")
            .map(|s| s.to_string())
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

        response.set_raw_header("x-request-id", request_id.clone());

        info!(
            request_id = %request_id,
            method = %request.method(),
            uri = %request.uri(),
            status = %response.status(),
            latency_ms = latency_ms as u64,
            event = "request_end"
        );
    }
}


