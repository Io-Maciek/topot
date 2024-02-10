use std::error::Error;
use std::fs::File;
use std::io;
use std::io::BufRead;
#[derive(Debug, Clone)]
pub struct ApiConfig{
    licznikIP : String,
    solarAuth : String,
    solarIP : String,
}

impl ApiConfig{
    pub fn read(file_path : &std::path::Path)-> Result<ApiConfig, Box<dyn Error>>{
        let file = File::open(file_path)?;
        let reader = io::BufReader::new(file);
        let mut lines = Vec::new();

        for line in reader.lines(){
            let line = line?;
            lines.push(line);
        }

        Ok(ApiConfig{
            licznikIP: lines[0].clone(),
            solarAuth: lines[1].clone(),
            solarIP: lines[2].clone(),
        })
    }
}

impl ApiConfig{
    pub fn licznikIP(&self)->String{
        self.licznikIP.clone()
    }
    pub fn solarAuth(&self)->String{
        self.solarAuth.clone()
    }
    pub fn solarIP(&self)->String{
        self.solarIP.clone()
    }
}