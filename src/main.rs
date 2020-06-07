extern crate actix_web;
extern crate chrono;
#[macro_use] extern crate serde;
extern crate rand;
extern crate serde_json;
extern crate uuid;
#[macro_use] extern crate log;
#[macro_use] extern crate sentry;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate prometheus;

#[macro_use] mod macros;

mod api;
mod models;
mod store;

use actix_cors::Cors;
use actix_web::{middleware, App, HttpServer};
use actix_web_prom::PrometheusMetrics;
use actix_web_httpauth::middleware::HttpAuthentication;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let raven = sentry::init((
        "https://b7ca8a41e8e84fef889e4f428071dab2@sentry.io/1415519",
        sentry::ClientOptions {
            release: release_name!(),
            ..Default::default()
        },
    ));

    if raven.is_enabled() {
        sentry::integrations::panic::register_panic_handler();
    }

    let state = models::GlobalState::new();
    let metrics = PrometheusMetrics::new_with_registry(prometheus::default_registry().clone(), "rex", Some("/api/v1/metrics"), None).unwrap();

    HttpServer::new(move || {
        App::new()
            .data(state.clone())
            .wrap(metrics.clone())
            .wrap(middleware::Logger::default())
            .wrap(HttpAuthentication::bearer(api::auth_validator))
            .wrap(api::Auth{})
            .wrap(Cors::new().send_wildcard().finish())
            .configure(api::configure)
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
