// TODO: Maybe implement real http status codes... Found in warp::http::StatusCode
// TODO: When app closes reset the LED pins
mod env_loader;
mod fun;
mod models;

use crate::env_loader::Config;
use crate::fun::{read_temperature_loop, update_bounds};
use crate::models::{Bounds, Leds, Temperature, TemperatureAndBounds};
use rppal::gpio::{Gpio, Mode};
use rppal_dht11::Dht11;
use std::error::Error;
use std::sync::{Arc, Mutex};
use warp::Filter;

const LOW_LED: u8 = 16;
const OK_LED: u8 = 20;
const HIGH_LED: u8 = 21;
const TEMP_PIN: u8 = 17;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load .env configs
    let conf: Config = env_loader::load();

    // Initialize storage variables
    let temperature_storage = Arc::new(Mutex::new(Temperature { temperature: 0 }));
    let bounds_storage = Arc::new(Mutex::new(Bounds {
        lower: conf.lower_bound,
        upper: conf.upper_bound,
    }));
    let leds = Arc::new(Mutex::new(Leds {
        low: Gpio::new()?.get(LOW_LED)?.into_output(),
        high: Gpio::new()?.get(HIGH_LED)?.into_output(),
        ok: Gpio::new()?.get(OK_LED)?.into_output(),
    }));
    let temp_pin = Gpio::new()?.get(TEMP_PIN)?.into_io(Mode::Output);
    let dht11 = Arc::new(Mutex::new(Dht11::new(temp_pin)));
    let dht11_clone = dht11.clone();

    // TODO: Instead of cloning here, maybe just clone in the function calls.
    // This would create borrow checker problems that would need solving.
    let temp_storage_clone = temperature_storage.clone(); // Create a new pointer to the same data for the temp reading loop
    let bounds_storage_clone = bounds_storage.clone(); // Goes to the update_bounds() function
    let bounds_storage_check = bounds_storage.clone(); // Goes to the read_temperature_loop() function
    let leds_clone = leds.clone();

    // Spawn temperature reading loop
    tokio::spawn(async move {
        read_temperature_loop(
            temp_storage_clone,
            bounds_storage_check,
            leds_clone,
            dht11_clone,
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
