use std::env;

use getopts::Options;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error>{
    let mut to_print: String = "".to_string();
    let mut to_file: String = "".to_string();
    
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();

    opts.optopt("t", "temp", "input temperature and scale", "TEMP");
    opts.optopt("z", "zip", "input zip code", "ZIP");
    opts.optflag("h", "help", "print help");
    opts.optflag("r", "read", "print use history");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!("{}", f.to_string()) }
    };

    if matches.opt_present("help") {
        to_print = 
            "-= temperature-converter =-\n    -t  --temp  :  Enter a temperature and scale (ex: 12C) to convert\n    -z  --zip   :  Enter a zip code to get the current temperature\n    -r  --read  :  Print out app use history\n All entries are recorded."
            .to_string();
        to_file = "Help requested".to_string();
    } else if matches.opt_present("temp") {
        let input = match matches.opt_str("temp") {
            Some(str) => str,
            None => "".to_string()
        };
        match parse_temp_input(&input) {
            Ok(t) => to_print = {
                let r = temperatureconverter::calculate(t);
                format!(
                    "-= Convert input temperature =-\n    {:?}: {}\n    {:?}: {}\n    {:?}: {}", 
                    r.0.0, r.0.1, r.1.0, r.1.1, r.2.0, r.2.1)
            },
            Err(e) => to_print = e.to_string()
        }
        to_file = format!("Temperature converted (\n{}\n)", to_print).to_string();
    } else if matches.opt_present("zip") {
        match matches.opt_str("zip") {
            Some(str) => {
                match temperatureconverter::get_current_temp(str).await {
                    Ok(t) => {
                        match parse_temp_input(&format!("{}C", t.2)) {
                            Ok(x) => to_print = {
                                    let r = temperatureconverter::calculate(x);
                                    format!(
                                    "-= Retrieve temperature in {}, {} =-\n    {:?}: {}\n    {:?}: {}\n    {:?}: {}", 
                                    t.0, t.1, r.0.0, r.0.1, r.1.0, r.1.1, r.2.0, r.2.1)
                            },
                            Err(e) => to_print = e.to_string()
                        }
                    },
                    Err(e) => to_print = e.to_string()
                }
            },
            None => to_print = "".to_string()
        };
        to_file = format!("Temperature retrieved by ZIP code (\n{}\n)", to_print).to_string();
    } else if matches.opt_present("read") {
        to_print = match temperatureconverter::read_from_file() {
            Ok(t) => format!("-= Print use history =-\n{}", t),
            Err(_) => "File read error".to_string()
        };
        to_file = "History accessed".to_string();
    } else {
        to_print = "Enter -h or --help to see a list of commands".to_string();
        to_file = "Invalid entry".to_string();
    }

    to_file = format!("{} on {}", to_file, chrono::offset::Local::now());

    temperatureconverter::write_to_file(&to_file);

    println!("{}", to_print);

    Ok(())
}

fn parse_temp_input(input: &str) -> Result<(temperatureconverter::Scale, f32), String> {
    // get all but last character
    let temp_str = input.chars().take(input.len() - 1).collect::<String>();
    let temp = match temp_str.parse::<f32>() {
        Ok(t)=> t,
        Err(e) => {
            if (temp_str.contains(' ')) {
                return Err("invalid entry: contains space".to_string());
            } else {
                return Err(format!("invalid number {}", temp_str));
            }
        }
    };

    // get last character
    let scale = match input.chars().last() {
        Some(c) if c == 'c' || c == 'C' => {
            temperatureconverter::Scale::Celsius
        }
        Some(c) if c == 'f' || c == 'F' => {
            temperatureconverter::Scale::Fahrenheit
        }
        Some(c) if c == 'k' || c == 'K' => {
            temperatureconverter::Scale::Kelvin
        }
        Some(c) => {
            return Err(format!("unknown scale {}", c));
        }
        None => {
            return Err("empty input".to_string());
        }
    };

    return Ok((scale, temp));
}


// MUST RETOOL TESTS AFTER CHANGES MADE TO CALCULATE FUNCTION
macro_rules! test_calculate_round_success {
    (
        $(
            $test_name:ident : $in:expr => $expected:expr
        )+
    ) => {
        $(
            #[test]
            fn $test_name() {
                match calculate($in) {
                    Ok(t) => {
                        //testing for accuracy to the ten-thousandth of a degree instead of to the full figure due to rounding differences
                        let ex0 : f32 = $expected.0;
                        assert!(t.0.1.abs() - ex0.abs() < 0.0001);
                        let ex1 : f32 = $expected.1;
                        assert!(t.1.1.abs() - ex1.abs() < 0.0001);
                        let ex2 : f32 = $expected.2;
                        assert!(t.2.1.abs() - ex2.abs() < 0.0001);
                    },
                    Err(e) => assert!(false)
                }
            }
        )+
    };
}

test_calculate_round_success![
    test_calculate_round_success_0: "10c".to_string() => (10.0, 283.15, 50.0)
    test_calculate_round_success_1: "10C".to_string() => (10.0, 283.15, 50.0)
    test_calculate_round_success_2: "10f".to_string() => (10.0, 260.9278, -12.22222)
    test_calculate_round_success_3: "10F".to_string() => (10.0, 260.9278, -12.22222)
    test_calculate_round_success_4: "10k".to_string() => (10.0, -263.15, -441.67)
    test_calculate_round_success_5: "10K".to_string() => (10.0, -263.15, -441.67)
    test_calculate_round_success_6: "0c".to_string() => (0.0, 273.15, 32.0)
    test_calculate_round_success_7: "0C".to_string() => (0.0, 273.15, 32.0)
    test_calculate_round_success_8: "0f".to_string() => (0.0, 255.3722, -17.77778)
    test_calculate_round_success_9: "0F".to_string() => (0.0, 255.3722, -17.77778)
    test_calculate_round_success_10: "0k".to_string() => (0.0, -273.15, -459.67)
    test_calculate_round_success_11: "0K".to_string() => (0.0, -273.15, -459.67)
    test_calculate_round_success_12: "1234c".to_string() => (1234.0, 1507.15, 2253.2)
    test_calculate_round_success_13: "1234C".to_string() => (1234.0, 1507.15, 2253.2)
    test_calculate_round_success_14: "1234f".to_string() => (1234.0, 940.9278, 667.7778)
    test_calculate_round_success_15: "1234F".to_string() => (1234.0, 940.9278, 667.7778)
    test_calculate_round_success_16: "1234k".to_string() => (1234.0, 960.85, 1761.53)
    test_calculate_round_success_17: "1234K".to_string() => (1234.0, 960.85, 1761.53)
    test_calculate_round_success_18: "-10c".to_string() => (-10.0, 263.15,14.0)
    test_calculate_round_success_19: "-10C".to_string() => (-10.0, 263.15,14.0)
    test_calculate_round_success_20: "-10f".to_string() => (-10.0, 249.8167, -23.33333)
    test_calculate_round_success_21: "-10F".to_string() => (-10.0, 249.8167, -23.33333)
    test_calculate_round_success_22: "-10k".to_string() => (-10.0, -283.15, -477.67)
    test_calculate_round_success_23: "-10K".to_string() => (-10.0, -283.15, -477.67)
    test_calculate_round_success_24: "-0c".to_string() => (0.0, 273.15, 32.0)
    test_calculate_round_success_25: "-0C".to_string() => (0.0, 273.15, 32.0)
    test_calculate_round_success_26: "-0f".to_string() => (0.0, 255.3722, -17.77778)
    test_calculate_round_success_27: "-0F".to_string() => (0.0, 255.3722, -17.77778)
    test_calculate_round_success_28: "-0k".to_string() => (0.0, -273.15, -459.67)
    test_calculate_round_success_29: "-0K".to_string() => (0.0, -273.15, -459.67)
    test_calculate_round_success_30: "-1234c".to_string() => (-1234.0, -960.85, -2189.2)
    test_calculate_round_success_31: "-1234C".to_string() => (-1234.0, -960.85, -2189.2)
    test_calculate_round_success_32: "-1234f".to_string() => (-1234.0, -430.1833, -703.3333)
    test_calculate_round_success_33: "-1234F".to_string() => (-1234.0, -430.1833, -703.3333)
    test_calculate_round_success_34: "-1234k".to_string() => (-1234.0, -1507.15, -2680.87)
    test_calculate_round_success_35: "-1234K".to_string() => (-1234.0, -1507.15, -2680.87)
];

macro_rules! test_calculate_fail {
    (
        $(
            $test_name:ident : $in:expr => $expected:expr
        )+
    ) => {
        $(
            #[test]
            fn $test_name() {
                match calculate($in) {
                    Ok(t) => assert!(false),
                    Err(e) => assert_eq!(e, $expected)
                }
            }
        )+
    };
}

test_calculate_fail![
    test_input_calculate_fail_0: "10t".to_string() => "unknown scale t".to_string()
    test_input_calculate_fail_1: "10".to_string() => "unknown scale 0".to_string()
    test_input_calculate_fail_2: "10 k".to_string() => "invalid entry: contains space".to_string()
    test_input_calculate_fail_3: "10qwes".to_string() => "invalid number 10qwe".to_string()
    test_input_calculate_fail_4: "AWDS".to_string() => "invalid number AWD".to_string()
];

macro_rules! test_input_parse_succeed {
    (
        $(
            //failed expr identifies if the test is for a purposeful failed state
            $test_name:ident : $in:expr => $expected:expr
        )+
    ) => {
        $(
            #[test]
            fn $test_name() {
                match parse_temp_input($in) {
                    Ok(t) => {
                        let scale = $expected.0;
                        assert!(matches!(t.0, scale) && t.1 == $expected.1);
                    },
                    Err(e) => assert!(false)
                }
            }
        )+
    };
}

test_input_parse_succeed![
    test_input_parse_succeed_0: "10c" => (Scale::Celsius, 10.0)
    test_input_parse_succeed_1: "10C" => (Scale::Celsius, 10.0)
    test_input_parse_succeed_2: "10f" => (Scale::Fahrenheit, 10.0)
    test_input_parse_succeed_3: "10F" => (Scale::Fahrenheit, 10.0)
    test_input_parse_succeed_4: "10k" => (Scale::Kelvin, 10.0)
    test_input_parse_succeed_5: "10K" => (Scale::Kelvin, 10.0)
    test_input_parse_succeed_6: "0c" => (Scale::Celsius, 0.0)
    test_input_parse_succeed_7: "0C" => (Scale::Celsius, 0.0)
    test_input_parse_succeed_8: "0f" => (Scale::Fahrenheit, 0.0)
    test_input_parse_succeed_9: "0F" => (Scale::Fahrenheit, 0.0)
    test_input_parse_succeed_10: "0k" => (Scale::Kelvin, 0.0)
    test_input_parse_succeed_11: "0K" => (Scale::Kelvin, 0.0)
    test_input_parse_succeed_12: "1234c" => (Scale::Celsius, 1234.0)
    test_input_parse_succeed_13: "1234C" => (Scale::Celsius, 1234.0)
    test_input_parse_succeed_14: "1234f" => (Scale::Fahrenheit, 1234.0)
    test_input_parse_succeed_15: "1234F" => (Scale::Fahrenheit, 1234.0)
    test_input_parse_succeed_16: "1234k" => (Scale::Kelvin, 1234.0)
    test_input_parse_succeed_17: "1234K" => (Scale::Kelvin, 1234.0)
    test_input_parse_succeed_18: "-10c" => (Scale::Celsius, -10.0)
    test_input_parse_succeed_19: "-10C" => (Scale::Celsius, -10.0)
    test_input_parse_succeed_20: "-10f" => (Scale::Fahrenheit, -10.0)
    test_input_parse_succeed_21: "-10F" => (Scale::Fahrenheit, -10.0)
    test_input_parse_succeed_22: "-10k" => (Scale::Kelvin, -10.0)
    test_input_parse_succeed_23: "-10K" => (Scale::Kelvin, -10.0)
    test_input_parse_succeed_24: "-0c" => (Scale::Celsius, 0.0)
    test_input_parse_succeed_25: "-0C" => (Scale::Celsius, 0.0)
    test_input_parse_succeed_26: "-0f" => (Scale::Fahrenheit, 0.0)
    test_input_parse_succeed_27: "-0F" => (Scale::Fahrenheit, 0.0)
    test_input_parse_succeed_28: "-0k" => (Scale::Kelvin, 0.0)
    test_input_parse_succeed_29: "-0K" => (Scale::Kelvin, 0.0)
    test_input_parse_succeed_30: "-1234c" => (Scale::Celsius, -1234.0)
    test_input_parse_succeed_31: "-1234C" => (Scale::Celsius, -1234.0)
    test_input_parse_succeed_32: "-1234f" => (Scale::Fahrenheit, -1234.0)
    test_input_parse_succeed_33: "-1234F" => (Scale::Fahrenheit, -1234.0)
    test_input_parse_succeed_34: "-1234k" => (Scale::Kelvin, -1234.0)
    test_input_parse_succeed_35: "-1234K" => (Scale::Kelvin, -1234.0)
];

macro_rules! test_input_parse_fail {
    (
        $(
            //failed expr identifies if the test is for a purposeful failed state
            $test_name:ident : $in:expr => $expected:expr
        )+
    ) => {
        $(
            #[test]
            fn $test_name() {
                match parse_temp_input($in) {
                    Ok(t) => assert!(false),
                    Err(e) => assert_eq!(e, $expected)
                }
            }
        )+
    };
}

test_input_parse_fail![
    test_input_parse_fail_0: "10t" => "unknown scale t".to_string()
    test_input_parse_fail_1: "10" => "unknown scale 0".to_string()
    test_input_parse_fail_2: "10 k" => "invalid entry: contains space".to_string()
    test_input_parse_fail_3: "10qwes" => "invalid number 10qwe".to_string()
    test_input_parse_fail_4: "AWDS" => "invalid number AWD".to_string()
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