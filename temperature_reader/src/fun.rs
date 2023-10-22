use crate::models::{Bounds, Leds, Temperature};
use rppal::hal::Delay;
use rppal_dht11::Dht11;
use std::sync::{Arc, Mutex};
use tokio::time::{sleep, Duration};
use warp::reply::Json;

pub async fn read_temperature_loop(
    temp_store: Arc<Mutex<Temperature>>,
    bounds_store: Arc<Mutex<Bounds>>,
    leds: Arc<Mutex<Leds>>,
    temp_pin: Arc<Mutex<Dht11>>,
    interval_in_s: u64,
) {
    loop {
        // .perform_measurement_with_retries returns a Measurement type that also has the humidity, but for this
        // excercise it is ignored.
        let mut delay = Delay::new();
        match temp_pin
            .lock()
            .unwrap()
            .perform_measurement_with_retries(&mut delay, 5)
        {
            Ok(measurement) => {
                // Reset LEDs
                leds.lock().unwrap().low.set_low();
                leds.lock().unwrap().high.set_low();
                leds.lock().unwrap().ok.set_low();

                // Assign temperature and light the correct LED
                let temp = measurement.temperature;
                temp_store.lock().unwrap().temperature = temp;
                if temp < bounds_store.lock().unwrap().lower {
                    // println!("Too cold: {}", temp.to_string());
                    leds.lock().unwrap().low.set_high();
                } else if temp > bounds_store.lock().unwrap().upper {
                    // println!("Too hot: {}", temp.to_string());
                    leds.lock().unwrap().high.set_high();
                } else {
                    // println!("Just perfect: {}", temp.to_string());
                    leds.lock().unwrap().ok.set_high();
                }
                // TODO: Implement display module
            }
            Err(_) => {
                // Handle errors in reading. The module seems to give a lot of CrcMismatch errors
                println!("Error reading temperature");
            }
        }
        sleep(Duration::from_secs(interval_in_s)).await;
    }
}

pub async fn update_bounds(
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
