use axum::{extract::Query, http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use ferio::{get_holidays, Holiday, HolidayDate};
use serde_json::json;
use std::{collections::HashMap, env, net::SocketAddr};

fn get_port() -> u16 {
    env::var("PORT")
        .map(|p| p.parse::<_>().expect("Failed to parse port"))
        .unwrap_or(3000)
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(holidays_service));
    let addr = SocketAddr::from(([0, 0, 0, 0], get_port()));
    println!("Listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
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
