#[macro_use]
extern crate rocket;

mod helper;
mod solar;
mod licznik;
mod threads;
mod config;

use std::error::Error;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use curlc::curl;
use crate::helper::API;
use std::sync::mpsc::{self};
use crate::config::ApiConfig;
use rocket::{Build, Rocket, State};
use crate::licznik::Licznik;
use rocket::serde::json::Json;

fn send_if_needed(send_value: i32, current_value: i32) -> Result<i32, Box<dyn Error>> {
    if send_value != current_value {
        let output = curl(&[
            "-X", "POST",
            &format!("192.168.68.151/set?value={send_value}"),
        ])?;

        let new_value = output.parse::<i32>()?;
        Ok(new_value)
    } else {
        Ok(current_value)
    }
}

fn get_ep() -> Result<i32, Box<dyn Error>> {
    let output = curl(&[
        "192.168.68.151/get",
    ])?;

    let ep_value = output.parse::<i32>()?;
    Ok(ep_value)
}

#[get("/solar")]
fn solar_route(results: &State<ThreadResults>) -> String {
    let result = results.result_panel_solarny.lock().unwrap();
    let x = *result;
    format!("{:?}", x)
}

#[get("/licznik")]
fn licznik_route(results: &State<ThreadResults>) -> Json<Option<Licznik>> {
    let mut result = results.result_licznik.lock().unwrap();
    let licznik = result.clone();
    Json(licznik)
}

#[get("/test")]
fn licznik_test(results: &State<ThreadResults>) -> String {
    let mut result = results.result_licznik.lock().unwrap();
    let licznik = result.clone();
format!("{:?}", licznik)
}


struct ThreadResults {
    result_panel_solarny: Arc<Mutex<Option<i32>>>,
    result_licznik: Arc<Mutex<Option<Licznik>>>,
}

#[launch]
fn rocket() -> Rocket<Build> {
    let config = ApiConfig::read(Path::new("config.txt")).unwrap();

    let (thread_handle_panel_solarny, result_panel_solarny, solar_tx) =
        threads::start_panel_solarny_thread(config.clone());


    let (thread_handle_licznik, result_licznik, licznik_tx) =
        threads::start_licznik_thread(config.clone());


    ctrlc::set_handler(move || {
        solar_tx.send(()).unwrap();
        licznik_tx.send(()).unwrap();
    }).unwrap();


    rocket::build()
        .mount("/", routes![licznik_route, solar_route, licznik_test])
        .manage(ThreadResults {
            result_panel_solarny,
            result_licznik,
        })

    /*while main_rx.try_recv().is_err() {
        let panel_solarny = *result_panel_solarny.lock().unwrap();
        let licznik = result_licznik.lock().unwrap();

        if panel_solarny.is_none() || licznik.is_none() {
            continue;
        }

        let panel_solarny = panel_solarny.unwrap();
        let licznik = licznik.clone().unwrap();

        println!("===========================");
        println!("Panel Solarny: {panel_solarny}", );
        println!("Licznik: {:?}", licznik);


        thread::sleep(Duration::from_secs(1));
    }
//println!("Wychodz z głównej petli...");

    if thread_handle_panel_solarny.join().is_err() {
        println!("Wystąpił błąd poczas zamykania wątku panelu solarnego.")
    }
    if thread_handle_licznik.join().is_err() {
        println!("Wystąpił błąd poczas zamykania wątku licznika.");
    }*/
//println!("Wątki zostały zakończone!");
}
