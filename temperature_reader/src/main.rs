// TODO: Maybe implement real http status codes... Found in warp::http::StatusCode
mod env_loader;
mod models;

use crate::env_loader::Config;
use crate::models::{Bounds, Leds, Temperature, TemperatureAndBounds};
use rppal::gpio::Gpio;
use std::error::Error;
use std::sync::{Arc, Mutex};
use tokio::time::{sleep, Duration};
use warp::reply::Json;
use warp::Filter;

use rand::Rng;

const LOW_LED: u8 = 16;
const OK_LED: u8 = 20;
const HIGH_LED: u8 = 21;

fn rand_f32(min: f32, max: f32) -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}

// TODO: Implement real temperature reading here
fn read_temperature() -> f32 {
    if cfg!(feature = "dev") {
        rand_f32(-5.0, 24.0)
    } else {
        12.34
    }
}

async fn read_temperature_loop(
    temp_store: Arc<Mutex<Temperature>>,
    bounds_store: Arc<Mutex<Bounds>>,
    leds: Arc<Mutex<Leds>>,
    interval_in_s: u64,
) {
    loop {
        leds.lock().unwrap().low.set_low();
        leds.lock().unwrap().high.set_low();
        leds.lock().unwrap().ok.set_low();

        let temp = read_temperature();
        temp_store.lock().unwrap().temperature = temp; // This would need better error handling. Panics if .lock() fails for any reason.

        // Would match statement be better? Nah, this fine.
        if temp < bounds_store.lock().unwrap().lower {
            println!("Too cold: {}", temp.to_string());
            leds.lock().unwrap().low.set_high();
        } else if temp > bounds_store.lock().unwrap().upper {
            println!("Too hot: {}", temp.to_string());
            leds.lock().unwrap().high.set_high();
        } else {
            println!("Just perfect: {}", temp.to_string());
            leds.lock().unwrap().ok.set_high();
        }

        sleep(Duration::from_secs(interval_in_s)).await;
    }
}

async fn update_bounds(
    new_bounds: Bounds,
    bounds_storage: Arc<Mutex<Bounds>>,
) -> Result<Json, warp::Rejection> {
    let mut bounds = bounds_storage.lock().unwrap();
    bounds.lower = new_bounds.lower;
    bounds.upper = new_bounds.upper;
    if cfg!(feature = "dev") {
        println!(
            "Bounds updated. Lower: {}, upper: {}",
            bounds.lower, bounds.upper
        );
    }
    Ok(warp::reply::json(
        &"Updated bounds successfully".to_string(),
    ))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load .env configs
    let conf: Config = env_loader::load();

    // Initialize storage variables
    let temperature_storage = Arc::new(Mutex::new(Temperature { temperature: 0.0 }));
    let bounds_storage = Arc::new(Mutex::new(Bounds {
        lower: conf.lower_bound,
        upper: conf.upper_bound,
    }));
    let temp_storage_clone = temperature_storage.clone(); // Create a new pointer to the same data for the temp reading loop
    let bounds_storage_clone = bounds_storage.clone(); // Goes to the update_bounds() function
    let bounds_storage_check = bounds_storage.clone(); // Goes to the read_temperature_loop() function

    let leds = Arc::new(Mutex::new(Leds {
        low: Gpio::new()?.get(LOW_LED)?.into_output(),
        high: Gpio::new()?.get(HIGH_LED)?.into_output(),
        ok: Gpio::new()?.get(OK_LED)?.into_output(),
    }));
    let leds_clone = leds.clone();

    // Spawn temperature reading loop
    tokio::spawn(async move {
        read_temperature_loop(
            temp_storage_clone,
            bounds_storage_check,
            leds_clone,
            conf.interval,
        )
        .await;
    });

    // API endpoints
    let cors = warp::reply::with::header("Access-Control-Allow-Origin", "*");
    let temperature_route = warp::path!("temperature")
        .map(move || {
            let temp = temperature_storage.lock().unwrap().clone(); // Easier to clone than to borrow
            let bounds = bounds_storage.lock().unwrap().clone();
            let res = TemperatureAndBounds { t: temp, b: bounds };
            warp::reply::json(&res)
        })
        .with(&cors);

    let bounds_route = warp::path("bounds") // Match path /bounds
        .and(warp::post()) // Match POST requests
        .and(warp::query::<Bounds>()) // Extract query parameters as an instance of Bounds
        .and(with_bounds_storage(bounds_storage_clone)) // Inject bounds_storage_clone pointer into the handler
        .and_then(update_bounds) // Call update_bounds
        .with(&cors);

    // Create filter to clone bounds_storage pointer and so it can be handed to the handler
    fn with_bounds_storage(
        bounds_storage: Arc<Mutex<Bounds>>,
    ) -> impl Filter<Extract = (Arc<Mutex<Bounds>>,), Error = std::convert::Infallible> + Clone
    {
        warp::any().map(move || bounds_storage.clone())
    }

    #[cfg(feature = "dev")]
    {
        println!("Listening on port: {}", conf.port);
        println!("Starting with lower bound: {}", conf.lower_bound);
        println!("Starting with upper bound: {}", conf.upper_bound);
    }
    // TODO: Do I need cors?
    let routes = temperature_route.or(bounds_route);
    warp::serve(routes).run(([0, 0, 0, 0], conf.port)).await;

    Ok(())
}
