use chrono::prelude::*;
use chrono_tz::Tz;
use serde::Deserialize;
use std::convert::Infallible;
use std::sync::Arc;
use tokio::sync::RwLock;
use warp::{http::StatusCode, reply, Filter, Rejection, Reply};

type Result<T> = std::result::Result<T, Rejection>;
type Dates = Arc<RwLock<Vec<DateTime<Utc>>>>;

#[derive(Deserialize)]
struct DateTimeRequest {
    date_time: String,
}

#[tokio::main]
async fn main() {
    let dates: Dates = Arc::new(RwLock::new(Vec::new()));

    let create_route = warp::path("create")
        .and(warp::post())
        .and(with_dates(dates.clone()))
        .and(warp::body::json())
        .and_then(create_handler);
    let fetch_route = warp::path("fetch")
        .and(warp::get())
        .and(warp::path::param())
        .and(with_dates(dates.clone()))
        .and_then(fetch_handler);

    println!("Server started at localhost:8080");
    warp::serve(create_route.or(fetch_route))
        .run(([0, 0, 0, 0], 8080))
        .await;
}

fn with_dates(dates: Dates) -> impl Filter<Extract = (Dates,), Error = Infallible> + Clone {
    warp::any().map(move || dates.clone())
}

async fn create_handler(dates: Dates, body: DateTimeRequest) -> Result<impl Reply> {
    let dt: DateTime<FixedOffset> = match DateTime::parse_from_rfc3339(&body.date_time) {
        Ok(v) => v,
        Err(e) => {
            return Ok(reply::with_status(
                format!("could not parse date: {}", e),
                StatusCode::BAD_REQUEST,
            ))
        }
    };

    dates.write().await.push(dt.with_timezone(&Utc));

    Ok(reply::with_status(
        format!("Added date with timezone: {} as UTC", dt.timezone()),
        StatusCode::OK,
    ))
}

async fn fetch_handler(time_zone: String, dates: Dates) -> Result<impl Reply> {
    let parsed_time_zone = time_zone.replace("%2F", "/"); // minimal url-encoding fix for parsing time zones like Africa/Algiers etc.
    let tz: Tz = match parsed_time_zone.parse() {
        Ok(v) => v,
        Err(e) => {
            return Ok(reply::with_status(
                format!("could not parse timezone: {}", e),
                StatusCode::BAD_REQUEST,
            ))
        }
    };

    Ok(
        match serde_json::to_string(
            &dates
                .read()
                .await
                .iter()
                .map(|t: &DateTime<Utc>| t.with_timezone(&tz).to_rfc3339())
                .collect::<Vec<_>>(),
        ) {
            Ok(v) => reply::with_status(v, StatusCode::OK),
            Err(e) => {
                return Ok(reply::with_status(
                    format!("could not serialize json: {}", e),
                    StatusCode::INTERNAL_SERVER_ERROR,
                ))
            }
        },
    )
}
