use axum::{
    extract::{Host, Query},
    handler::Handler,
    http::{StatusCode, Uri},
    response::{IntoResponse, Redirect},
    routing::get,
    BoxError, Json, Router,
};
use axum_server::tls_rustls::RustlsConfig;
use ferio::{get_holidays, Holiday, HolidayDate};
use serde_json::json;
use std::{collections::HashMap, env, net::SocketAddr};
use tower_http::cors::CorsLayer;

enum Config {
    Dev {
        port: u16,
    },
    Prod {
        http_port: u16,
        https_port: u16,
        cert_path: String,
        key_path: String,
    },
}

impl Config {
    fn load_env() -> Self {
        match env::var("ENV") {
            Ok(env) => match env.as_str() {
                "dev" => Config::Dev {
                    port: get_http_port(),
                },
                "prod" => Config::Prod {
                    http_port: get_http_port(),
                    https_port: get_https_port(),
                    cert_path: get_cert_path(),
                    key_path: get_key_path(),
                },
                _ => panic!("Invalid env"),
            },
            Err(_) => Config::Dev {
                port: get_http_port(),
            },
        }
    }
}

fn get_http_port() -> u16 {
    env::var("PORT")
        .map(|p| p.parse::<_>().expect("Failed to parse port"))
        .unwrap_or(3000)
}

fn get_https_port() -> u16 {
    env::var("HTTPS_PORT")
        .map(|p| p.parse::<_>().expect("Failed to parse port"))
        .expect("Failed to get https port")
}

fn get_cert_path() -> String {
    env::var("CERT_PATH").expect("Failed to get https cert path")
}

fn get_key_path() -> String {
    env::var("KEY_PATH").expect("Failed to get https key path")
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(holidays_service))
        .layer(CorsLayer::permissive());

    let config = Config::load_env();

    match config {
        Config::Dev { port } => {
            let addr = SocketAddr::from(([0, 0, 0, 0], port));
            println!("Listening on http://{}", &addr);

            axum_server::bind(addr)
                .serve(app.into_make_service())
                .await
                .expect("Failed to start server");
        }
        Config::Prod {
            http_port,
            https_port,
            cert_path,
            key_path,
        } => {
            let https_config = RustlsConfig::from_pem_file(cert_path, key_path)
                .await
                .expect("Failed to load cert");

            let addr = SocketAddr::from(([0, 0, 0, 0], https_port));
            println!("Listening on http://{}", &addr);

            tokio::spawn(redirect_http_to_https(http_port, https_port));

            axum_server::bind_rustls(addr, https_config)
                .serve(app.into_make_service())
                .await
                .expect("Failed to start server");
        }
    };
}

async fn redirect_http_to_https(http_port: u16, https_port: u16) {
    fn make_https(
        host: String,
        uri: Uri,
        http_port: u16,
        https_port: u16,
    ) -> Result<Uri, BoxError> {
        let mut parts = uri.into_parts();
        parts.scheme = Some(axum::http::uri::Scheme::HTTPS);

        if parts.path_and_query.is_none() {
            parts.path_and_query = Some("/".parse().unwrap());
        }

        let https_host = host.replace(&http_port.to_string(), &https_port.to_string());
        parts.authority = Some(https_host.parse()?);

        Ok(Uri::from_parts(parts)?)
    }

    let redirect = move |Host(host): Host, uri: Uri| async move {
        match make_https(host, uri, http_port, https_port) {
            Ok(uri) => Ok(Redirect::permanent(&uri.to_string())),
            Err(_) => Err(StatusCode::BAD_REQUEST),
        }
    };

    let addr = SocketAddr::from(([127, 0, 0, 1], http_port));

    axum_server::bind(addr)
        .serve(redirect.into_make_service())
        .await
        .expect("Failed to start http to https redirect server");
}

async fn holidays_service(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    let date = params
        .get("date")
        .map_or(Ok(HolidayDate::Today), |d| d.parse());

    if date.is_err() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json! {
                {
                    "error": "Invalid date"

                }
            }),
        );
    }
    let date = date.unwrap();

    match get_holidays(&date).await {
        Ok(holidays) => {
            let date = date.get_date();

            (
                StatusCode::OK,
                Json(json!({
                    "date": date,
                    "data": holidays_to_json(holidays)
                })),
            )
        }
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json! {
                {
                    "error": "Failed to get holidays"
                }
            }),
        ),
    }
}

fn holidays_to_json(holidays: Vec<Holiday>) -> Vec<serde_json::Value> {
    holidays
        .into_iter()
        .map(|h| {
            json!({
                "name": h.name,
                "greeting": h.get_greeting(),
                "wikipedia_url": h.wikipedia_url,
            })
        })
        .collect()
}
