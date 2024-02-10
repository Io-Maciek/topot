use std::error::Error;
use crate::config::ApiConfig;

pub fn mapr(x: i32, in_min: i32, in_max: i32, out_min: i32, out_max: i32) -> i32 {
    (x - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
}


pub trait API{
    fn GetValue(config: &ApiConfig)-> Result<Self, Box<dyn Error>>
        where Self: Sized;
}


pub fn get_send_val(solar_value: f64) -> i32 {
    let mut send_value = 0;
    if solar_value >= 3600.0 {
        send_value = 15;
    } else if solar_value >= 3300.0 {
        send_value = 14;
    } else if solar_value >= 3000.0 {
        send_value = 13;
    } else if solar_value >= 2700.0 {
        send_value = 12;
    } else if solar_value >= 2400.0 {
        send_value = 11;
    } else if solar_value >= 2100.0 {
        send_value = 10;
    } else if solar_value >= 1800.0 {
        send_value = 9;
    } else if solar_value >= 1500.0 {
        send_value = 8;
    } else if solar_value >= 1200.0 {
        send_value = 7;
    }
    mapr(send_value, 0, 15, 0, 100)
}