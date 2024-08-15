use std::{fs::File};

use serde::{Serialize, Deserialize};

use std::io::prelude::*;

use chrono;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Scale {
    Kelvin,
    Celsius,
    Fahrenheit,
}

#[derive(Debug, PartialEq)]
pub enum Application {
    CLI,
    GUI
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

pub fn write_to_file(input: &String, app: Application) -> std::io::Result<()> {
    let mut file = File::open("temperature-converter-log.txt")?;
    let mut contents= match read_from_file() {
        Ok(t) => t,
        Err(_) => "".to_string()
    };
    let mut ad = format!("on {}", chrono::offset::Local::now()).to_string();
    if app == Application::CLI {
        ad = format!(" - from CLI {}", ad);
    } else {
        ad = format!(" - from GUI {}", ad);
    }
    file = File::create("temperature-converter-log.txt")?;
    file.write_all((contents + input + &ad + "\n\n").as_bytes())?;
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

macro_rules! test_calculate_round_success {
    (
        $(
            $test_name:ident : $in:expr => $expected:expr
        )+
    ) => {
        $(
            #[test]
            fn $test_name() {
                let t = calculate($in);
                //testing for accuracy to the ten-thousandth of a degree instead of to the full figure due to rounding differences
                let ex0 : f32 = $expected.0;
                assert!(t.0.1.abs() - ex0.abs() < 0.0001);
                let ex1 : f32 = $expected.1;
                assert!(t.1.1.abs() - ex1.abs() < 0.0001);
                let ex2 : f32 = $expected.2;
                assert!(t.2.1.abs() - ex2.abs() < 0.0001);
            }
        )+
    };
}

test_calculate_round_success![
    test_calculate_round_success_0: (Scale::Celsius, 10.0) => (10.0, 283.15, 50.0)
    test_calculate_round_success_1: (Scale::Fahrenheit, 10.0) => (10.0, 260.9278, -12.22222)
    test_calculate_round_success_2: (Scale::Kelvin, 10.0) => (10.0, -263.15, -441.67)
    test_calculate_round_success_3: (Scale::Celsius, 0.0) => (0.0, 273.15, 32.0)
    test_calculate_round_success_4: (Scale::Fahrenheit, 0.0) => (0.0, 255.3722, -17.77778)
    test_calculate_round_success_5: (Scale::Kelvin, 0.0) => (0.0, -273.15, -459.67)
    test_calculate_round_success_6: (Scale::Celsius, 1234.0) => (1234.0, 1507.15, 2253.2)
    test_calculate_round_success_7: (Scale::Fahrenheit, 1234.0) => (1234.0, 940.9278, 667.7778)
    test_calculate_round_success_8: (Scale::Kelvin, 1234.0) => (1234.0, 960.85, 1761.53)
    test_calculate_round_success_9: (Scale::Celsius, -10.0) => (-10.0, 263.15,14.0)
    test_calculate_round_success_10: (Scale::Fahrenheit, -10.0) => (-10.0, 249.8167, -23.33333)
    test_calculate_round_success_11: (Scale::Kelvin, -10.0) => (-10.0, -283.15, -477.67)
    test_calculate_round_success_12: (Scale::Celsius, -0.0) => (0.0, 273.15, 32.0)
    test_calculate_round_success_13: (Scale::Fahrenheit, -0.0) => (0.0, 255.3722, -17.77778)
    test_calculate_round_success_14: (Scale::Kelvin, -0.0) => (0.0, -273.15, -459.67)
    test_calculate_round_success_15: (Scale::Celsius, -1234.0) => (-1234.0, -960.85, -2189.2)
    test_calculate_round_success_16: (Scale::Fahrenheit, -1234.0) => (-1234.0, -430.1833, -703.3333)
    test_calculate_round_success_17: (Scale::Kelvin, -1234.0) => (-1234.0, -1507.15, -2680.87)
];


macro_rules! test_convert_round {
    (
        $(
            $test_name:ident : $in:expr => $expected:expr
        )+
    ) => {
        $(
            #[test]
            fn $test_name() {
                let converted_input = convert(&$in.0, $in.1);
                let ex0 : f32 = $expected.0;
                assert!(converted_input.0.1.abs() - ex0.abs() < 0.0001);
                let ex1 : f32 = $expected.1;
                assert!(converted_input.1.1.abs() - ex1.abs() < 0.0001);
            }
        )+
    };
}

test_convert_round![
    test_convert_round_0: (Scale::Celsius, 10.0) => (283.15, 50.0)
    test_convert_round_1: (Scale::Fahrenheit, 10.0) => (260.9278, -12.22222)
    test_convert_round_2: (Scale::Kelvin, 10.0) => (-263.15, -441.67)
    test_convert_round_3: (Scale::Celsius, 0.0) => (273.15, 32.0)
    test_convert_round_4: (Scale::Fahrenheit, 0.0) => (255.3722, -17.77778)
    test_convert_round_5: (Scale::Kelvin, 0.0) => (-273.15, -459.67)
    test_convert_round_6: (Scale::Celsius, 1234.0) => (1507.15, 2253.2)
    test_convert_round_7: (Scale::Fahrenheit, 1234.0) => (940.9278, 667.7778)
    test_convert_round_8: (Scale::Kelvin, 1234.0) => (960.85, 1761.53)
    test_convert_round_9: (Scale::Celsius, -10.0) => (263.15,14.0)
    test_convert_round_10: (Scale::Fahrenheit, -10.0) => (249.8167, -23.33333)
    test_convert_round_11: (Scale::Kelvin, -10.0) => (-283.15, -477.67)
    test_convert_round_12: (Scale::Celsius, -0.0) => (273.15, 32.0)
    test_convert_round_13: (Scale::Fahrenheit, -0.0) => (255.3722, -17.77778)
    test_convert_round_14: (Scale::Kelvin, -0.0) => (-273.15, -459.67)
    test_convert_round_15: (Scale::Celsius, -1234.0) => (-960.85, -2189.2)
    test_convert_round_16: (Scale::Fahrenheit, -1234.0) => (-430.1833, -703.3333)
    test_convert_round_17: (Scale::Kelvin, -1234.0) => (-1507.15, -2680.87)
];