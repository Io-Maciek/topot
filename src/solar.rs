use std::error::Error;
use curlc::{curl, CurlErrors};
use crate::config::ApiConfig;
use crate::helper::API;

#[derive(Debug)]
pub struct SolarPanel{
    value: i32,
}

impl API for SolarPanel{
    fn GetValue(config: &ApiConfig)-> Result<SolarPanel, Box<dyn Error>>{
        let output = curl(&[
            &config.solarIP(),
            "-H", &format!("Authorization: Basic {}",config.solarAuth())
        ])?;

        let searching = "var webdata_now_p = ";
        let index = output.find(searching);
        let index = index.unwrap();

        let mut text = output[index + searching.len()..].to_string();

        text = text[text.find('"').unwrap() + 1..].to_string();
        let solar_value = text[..text.find('"').unwrap()].parse::<i32>()?;
        Ok(SolarPanel{value: solar_value})
    }
}

impl SolarPanel{
    pub fn getProduction(&self)->i32{
        self.value
    }
}