use dotenv::dotenv;
use std::env;

/// Contains the following parameters
/// - interval: u64
/// - port: u16
/// - lower_bound: f32
/// - upper_bound: f32
#[derive(Debug)]
pub struct Config {
    pub interval: u64,
    pub port: u16,
    pub lower_bound: f32,
    pub upper_bound: f32,
}

// Note that the first unwrap_or_else grabs errors in reading env variables
// The second one grabs errors in parsing the value

/// Loads configuration values from environment variables.
///
/// This function attempts to read the following environment variables:
/// - `INTERVAL_IN_S`: The interval in seconds.
/// - `API_PORT`: The port number for the API.
/// - `LOWER_BOUND`: The lower bound value.
/// - `UPPER_BOUND`: The upper bound value.
///
/// If any of these variables cannot be read or parsed, default values are used.
/// - Default for `INTERVAL_IN_S` is 600.
/// - Default for `API_PORT` is 8080.
/// - Default for `LOWER_BOUND` is 19.0.
/// - Default for `UPPER_BOUND` is 23.0.
///
/// # Returns
/// Returns a `Config` struct containing the loaded configuration values.
pub fn load() -> Config {
    dotenv().ok();

    let interval: u64 = env::var("INTERVAL_IN_S")
        .unwrap_or_else(|_| {
            println!("Failed to read INTERVAL_IN_S, using default value: 600");
            "600".to_string()
        })
        .parse()
        .unwrap_or_else(|_| {
            println!("Failed to parse INTERVAL_IN_S, using default value: 600");
            600
        });

    let port: u16 = env::var("API_PORT")
        .unwrap_or_else(|_| {
            println!("Failed to read API_PORT, using default value: 8080");
            "8080".to_string()
        })
        .parse()
        .unwrap_or_else(|_| {
            println!("Failed to parse API_PORT, using default value: 8080");
            8080
        });

    let lower_bound: f32 = env::var("LOWER_BOUND")
        .unwrap_or_else(|_| {
            println!("Failed to read LOWER_BOUND, using default value: 19.0");
            "19.0".to_string()
        })
        .parse()
        .unwrap_or_else(|_| {
            println!("Failed to parse LOWER_BOUND, using default value: 19.0");
            19.0
        });

    let upper_bound: f32 = env::var("UPPER_BOUND")
        .unwrap_or_else(|_| {
            println!("Failed to read UPPER_BOUND, using default value: 23.0");
            "23.0".to_string()
        })
        .parse()
        .unwrap_or_else(|_| {
            println!("Failed to parse UPPER_BOUND, using default value: 23.0");
            23.0
        });

    Config {
        interval,
        port,
        lower_bound,
        upper_bound,
    }
}
