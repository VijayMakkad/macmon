#[macro_use]
extern crate rocket;

mod metrics;  // Import the `metrics` module
mod sources;

use rocket::serde::json::Json;
use metrics::Sampler;
use serde::Serialize;
use rocket_cors::{AllowedOrigins, CorsOptions};

// Define a structure for API responses
#[derive(Serialize)]
struct MetricsResponse {
    cpu_power: f64,
    gpu_power: f64,
    mem_usage: u64,
    mem_total: u64,
    cpu_temp_avg: f32,
    gpu_temp_avg: f32,
    ecpu_usage: (u32, f32), // freq, percent_from_max
    pcpu_usage: (u32, f32), // freq, percent_from_max
    gpu_usage: (u32, f32),  // freq, percent_from_max
    sys_power: f32,         // Watts
    all_power: f32,         // Watts
}

// Function to get the metrics and return them in the API format
fn fetch_metrics() -> Result<MetricsResponse, Box<dyn std::error::Error>> {
    // Create a new Sampler
    let mut sampler = Sampler::new()?;
    
    // Fetch the metrics
    let metrics = sampler.get_metrics(1000)?;

    // Prepare response
    Ok(MetricsResponse {
        cpu_power: metrics.cpu_power as f64,
        gpu_power: metrics.gpu_power as f64,
        mem_usage: metrics.memory.ram_usage,
        mem_total: metrics.memory.ram_total,
        cpu_temp_avg: metrics.temp.cpu_temp_avg,
        gpu_temp_avg: metrics.temp.gpu_temp_avg,
        ecpu_usage: metrics.ecpu_usage,
        pcpu_usage: metrics.pcpu_usage,
        gpu_usage: metrics.gpu_usage,
        sys_power: metrics.sys_power,
        all_power: metrics.all_power,
    })
}

// Rocket route to expose metrics as JSON
#[get("/metrics")]
fn metrics_api() -> Result<Json<MetricsResponse>, rocket::http::Status> {
    match fetch_metrics() {
        Ok(response) => Ok(Json(response)),
        Err(_) => Err(rocket::http::Status::InternalServerError),  // Return an internal server error if anything fails
    }
}

// Launch the Rocket server
#[launch]
fn rocket() -> _ {
  let cors = CorsOptions::default()
    .to_cors()
    .expect("CorsOptions::default() failed");

  rocket::build()
    .mount("/", routes![metrics_api])
    .attach(cors)
}
