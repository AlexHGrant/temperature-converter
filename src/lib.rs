use std::{env, fs::File};

use getopts::Options;

use serde::{Serialize, Deserialize};

use std::io::prelude::*;

use chrono;

#[derive(Debug)]
pub enum Scale {
    Kelvin,
    Celsius,
    Fahrenheit,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Todo {
    pub location: Location,
    pub current: Current,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Location {
    pub name: String,
    pub region: String,
    pub country: String,
    pub lat: f64,
    pub lon: f64,
    pub tz_id: String,
    pub localtime_epoch: i64,
    pub localtime: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Current {
    pub last_updated_epoch: i64,
    pub last_updated: String,
    pub temp_c: f32,
    pub temp_f: f32,
    pub is_day: i64,
    pub condition: Condition,
    pub wind_mph: f64,
    pub wind_kph: f64,
    pub wind_degree: i64,
    pub wind_dir: String,
    pub pressure_mb: f64,
    pub pressure_in: f64,
    pub precip_mm: f64,
    pub precip_in: f64,
    pub humidity: i64,
    pub cloud: i64,
    pub feelslike_c: f64,
    pub feelslike_f: f64,
    pub windchill_c: f64,
    pub windchill_f: f64,
    pub heatindex_c: f64,
    pub heatindex_f: f64,
    pub dewpoint_c: f64,
    pub dewpoint_f: f64,
    pub vis_km: f64,
    pub vis_miles: f64,
    pub uv: f64,
    pub gust_mph: f64,
    pub gust_kph: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Condition {
    pub text: String,
    pub icon: String,
    pub code: i64,
}

pub fn read_from_file() -> std::io::Result<(String)> {
    let mut file = File::open("temperature-converter-log.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok((contents))
}

pub fn write_to_file(input: &String) -> std::io::Result<()> {
    let mut file = File::open("temperature-converter-log.txt")?;
    let mut contents= match read_from_file() {
        Ok(t) => t,
        Err(_) => "".to_string()
    };
    file = File::create("temperature-converter-log.txt")?;
    file.write_all((contents + input + "\n\n").as_bytes())?;
    Ok(())
}

pub async fn get_current_temp(zip: String) -> Result<(String, String, f32), reqwest::Error> {
    let resp: Todo = reqwest::Client::new().get(
        (format!("http://api.weatherapi.com/v1/current.json?key=78a83ea7b80d4c7ab46221407241502&q={}&aqi=no", zip)))
        .send().await?.json().await?;
    Ok(((resp.location.name, resp.location.region, resp.current.temp_c)))
}

pub fn calculate(input: (Scale, f32)) -> ((Scale, f32), (Scale, f32), (Scale, f32)) {
    //Changed input from unparsed string to output of parse_temp_input and made changes to main method accordingly
    let mut conversions: ((Scale, f32), (Scale, f32)) = ((Scale::Kelvin, 0.0), (Scale::Kelvin, 0.0));
    conversions = convert(&input.0, input.1);
    return ((input.0, input.1), (conversions.0.0, conversions.0.1), (conversions.1.0, conversions.1.1));
}

pub fn convert(scale: &Scale, value: f32) -> ((Scale, f32), (Scale, f32)) {
    match scale {
        Scale::Kelvin => return ((Scale::Celsius, to_cels(&scale, value)), (Scale::Fahrenheit, to_fahr(&scale, value))),
        Scale::Celsius => return ((Scale::Kelvin, to_kelv(&scale, value)), (Scale::Fahrenheit, to_fahr(&scale, value))),
        Scale::Fahrenheit => return ((Scale::Kelvin, to_kelv(&scale, value)), (Scale::Celsius, to_cels(&scale, value)))
    }
}

pub fn to_cels(scale: &Scale, value: f32) -> f32 {
    match scale {
        Scale::Kelvin => return value - 273.15,
        Scale::Fahrenheit => return (value - 32.0) * 5.0/9.0,
        _=> return 0.0
    }
}

pub fn to_fahr(scale: &Scale, value: f32) -> f32 {
    match scale {
        Scale::Kelvin => return (value - 273.15) * 9.0/5.0 + 32.0,
        Scale::Celsius => return value * 9.0/5.0 + 32.0,
        _=> return 0.0
    }
}

pub fn to_kelv(scale: &Scale, value: f32) -> f32 {
    match scale {
        Scale::Fahrenheit => return (value - 32.0) * 5.0/9.0 + 273.15,
        Scale::Celsius => return value + 273.15,
        _=> return 0.0
    }
}