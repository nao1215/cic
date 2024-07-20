use crate::calculations::Investment;
use actix_web::{web, App, HttpResponse, HttpServer, Result};
use serde::Deserialize;
use serde_json::json;

/// Starts an HTTP server that listens on the specified port.
///
/// # Arguments
///
/// * `port` - The port number on which the server will listen for incoming requests.
///
/// # Returns
///
/// Returns a `std::io::Result<()>`. On success, returns `Ok(())`, indicating that the server has started successfully.
/// On failure, returns an `std::io::Error`.
///
/// # Panics
///
/// This function may panic if the server fails to bind to the specified address or port.
pub async fn start_server(port: u16) -> std::io::Result<()> {
    println!("Starting server, port: {}", port);
    println!("POST /compound-interests");

    HttpServer::new(|| {
        App::new().route("/compound-interests", web::post().to(calculate_investment))
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}

#[derive(Debug, Deserialize)]
/// Represents the parameters required for calculating an investment.
///
/// This struct is used to deserialize JSON payloads sent to the `/compound-interests` endpoint.
///
/// # Fields
///
/// * `principal` - The initial amount of money invested (default: 0.0).
/// * `contribution` - The monthly contribution added to the investment (default: 1.0).
/// * `rate` - The annual interest rate as a percentage (default: 5.0).
/// * `years` - The number of years the money is invested for (default: 5).
pub struct InvestmentParams {
    #[serde(default = "default_principal")]
    pub principal: f64,
    #[serde(default = "default_contribution")]
    pub contribution: f64,
    #[serde(default = "default_rate")]
    pub rate: f64,
    #[serde(default = "default_years")]
    pub years: i32,
}

fn default_principal() -> f64 {
    0.0
}

fn default_contribution() -> f64 {
    1.0
}

fn default_rate() -> f64 {
    5.0
}

fn default_years() -> i32 {
    5
}

/// Handles HTTP POST requests to the `/compound-interests` endpoint.
///
/// This function extracts investment parameters from the request body, calculates the investment summary,
/// and returns the result as a JSON response.
///
/// # Arguments
///
/// * `params` - The incoming JSON payload containing the investment parameters.
///
/// # Returns
///
/// Returns a `Result<HttpResponse>`. On success, returns an `HttpResponse` with status `200 OK` and a JSON payload
/// representing the yearly summary of the investment. On failure, returns an error response with the appropriate HTTP status code.
///
/// # Errors
///
/// Returns a `BadRequest` error if the parameters are invalid or cannot be parsed, and an `InternalServerError`
/// if serialization of the summary fails.
pub async fn calculate_investment(params: web::Json<InvestmentParams>) -> Result<HttpResponse> {
    let investment = Investment::from_params(params.into_inner())
        .map_err(|e| actix_web::error::ErrorBadRequest(e))?;

    let summary = investment.yearly_summary();
    let json = json!(summary);

    Ok(HttpResponse::Ok().json(json))
}
