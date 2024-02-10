use std::error::Error;
use curlc::{curl, CurlErrors};
use rocket::serde::Serialize;
use crate::helper::API;
use serde_json::Value;
use crate::config::ApiConfig;

#[derive(Debug, Default, Clone, Serialize)]
pub struct Licznik{
    phasesPowerActive: Vec<f64>,
    powerActiveSum: f64,
}

pub struct LicznikDifference{
    phasesPowerActiveDifference: Vec<f64>,
}

impl API for Licznik{
    fn GetValue(config: &ApiConfig) -> Result<Licznik, Box<dyn Error>> {
        let output = curl(&[
            config.licznikIP()
        ])?;
        let mut api_obj = serde_json::from_str::<Value>(&output)?;
        let phases = api_obj
            .get_mut("phases")
            .unwrap()
            .as_array()
            .unwrap();

        let mut power_active_sum = 0.0;

        let mut phasesActive = Vec::<f64>::new();

        for phase in phases{
            let power_active = phase["powerActive"].as_f64().unwrap();
            phasesActive.push(power_active);
        }

        //Ok(Licznik{powerActive: (power_active_sum)})
        Ok(
            Licznik{
                powerActiveSum: phasesActive.iter().sum(),
                phasesPowerActive: phasesActive,
            }
        )
    }
}

impl Licznik{
    pub fn getPowerActiveSum(&self)->f64{
        self.powerActiveSum
    }
    pub fn getPhases(&self) -> Vec<f64>{
        self.phasesPowerActive.clone()
    }
    pub fn getDifference1(&self, other: &Licznik) -> LicznikDifference{
        let mut diff_vec = Vec::<f64>::new();

        for (index, element) in self.phasesPowerActive.iter().enumerate() {
            diff_vec.push(element - other.phasesPowerActive[index]);
        }

        LicznikDifference{phasesPowerActiveDifference: diff_vec}
    }
}
