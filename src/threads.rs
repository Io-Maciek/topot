use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::Sender;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use crate::config::ApiConfig;
use crate::helper::API;
use crate::licznik::Licznik;
use crate::solar::SolarPanel;

pub fn start_panel_solarny_thread(config: ApiConfig) -> (JoinHandle<()>, Arc<Mutex<Option<i32>>>, Sender<()>) {
    let (solar_tx, solar_rx) = mpsc::channel::<()>();
    let result_panel_solarny = Arc::new(Mutex::new(None));

    let result_panel_solarny_clone = result_panel_solarny.clone();

    (thread::spawn(move || {
        while solar_rx.try_recv().is_err() {
            let panel_solarny = match SolarPanel::GetValue(&config) {
                Ok(o) => o.getProduction(),
                Err(e) => {
                    //println!("Solar err: {}", e);
                    0
                }
            };
            let mut result = result_panel_solarny_clone.lock().unwrap();
            *result = Some(panel_solarny);
            //println!("{}");
        }
        //println!("Kończę wątek panelu solarnego...");
        let mut result = result_panel_solarny_clone.lock().unwrap();
        *result = None;
    }),
     result_panel_solarny,
     solar_tx
    )
}

pub fn start_licznik_thread(config: ApiConfig) -> (JoinHandle<()>, Arc<Mutex<Option<Licznik>>>, Sender<()>) {
    let result_licznik = Arc::new(Mutex::new(None));
    let (licznik_tx, licznik_rx) = mpsc::channel::<()>();

    let result_licznik_clone = result_licznik.clone();

    (thread::spawn(move || {
        while licznik_rx.try_recv().is_err() {
            let licznik = Licznik::GetValue(&config).ok();
            let mut result = result_licznik_clone.lock().unwrap();
            *result = licznik;
            thread::sleep(Duration::from_secs(6));
        }
        //println!("Kończę wątek licznika...");
        let mut result = result_licznik_clone.lock().unwrap();
        *result = None;

    }),
     result_licznik,
     licznik_tx
    )
}