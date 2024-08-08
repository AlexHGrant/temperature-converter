#[derive(Debug)]
enum Scale {
    Kelvin,
    Celsius,
    Fahrenheit,
}

fn main() {
    let input = std::env::args().nth(1).expect("Please enter a temperature in F, C or K");
    match calculate(input) {
        Ok(v) => println! ("{:?}: {}\n{:?}: {}\n{:?}: {}", v.0.0, v.0.1, v.1.0, v.1.1, v.2.0, v.2.1),
        Err(e) => println!("{}", e)
    }
}

fn convert(scale: &Scale, value: f32) -> ((Scale, f32), (Scale, f32)) {
    match scale {
        Scale::Kelvin => return ((Scale::Celsius, to_cels(&scale, value)), (Scale::Fahrenheit, to_fahr(&scale, value))),
        Scale::Celsius => return ((Scale::Kelvin, to_kelv(&scale, value)), (Scale::Fahrenheit, to_fahr(&scale, value))),
        Scale::Fahrenheit => return ((Scale::Kelvin, to_kelv(&scale, value)), (Scale::Celsius, to_cels(&scale, value)))
    }
}

fn calculate(input: String) -> Result<((Scale, f32), (Scale, f32), (Scale, f32)), String> {
    let mut conversions: ((Scale, f32), (Scale, f32)) = ((Scale::Kelvin, 0.0), (Scale::Kelvin, 0.0));
    let mut r: String = "".to_string();
    match parse_temp_input(input.as_str()) {
        Ok(parsed_input) => {
            conversions = convert(&parsed_input.0, parsed_input.1);
            return Ok(((parsed_input.0, parsed_input.1), (conversions.0.0, conversions.0.1), (conversions.1.0, conversions.1.1)));
        },
        Err(str) => return Err(format!("{}", str))
    }
}

fn to_cels(scale: &Scale, value: f32) -> f32 {
    match scale {
        Scale::Kelvin => return value - 273.15,
        Scale::Fahrenheit => return (value - 32.0) * 5.0/9.0,
        _=> return 0.0
    }
}

fn to_fahr(scale: &Scale, value: f32) -> f32 {
    match scale {
        Scale::Kelvin => return (value - 273.15) * 9.0/5.0 + 32.0,
        Scale::Celsius => return value * 9.0/5.0 + 32.0,
        _=> return 0.0
    }
}

fn to_kelv(scale: &Scale, value: f32) -> f32 {
    match scale {
        Scale::Fahrenheit => return (value - 32.0) * 5.0/9.0 + 273.15,
        Scale::Celsius => return value + 273.15,
        _=> return 0.0
    }
}

fn parse_temp_input(input: &str) -> Result<(Scale, f32), String> {
    // get all but last character
    let temp_str = input.chars().take(input.len() - 1).collect::<String>();
    let temp = match temp_str.parse::<f32>() {
        Ok(val) => val,
        Err(_) => {
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
            Scale::Celsius
        }
        Some(c) if c == 'f' || c == 'F' => {
            Scale::Fahrenheit
        }
        Some(c) if c == 'k' || c == 'K' => {
            Scale::Kelvin
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
                    Ok(v) => {
                        //testing for accuracy to the ten-thousandth of a degree instead of to the full figure due to rounding differences
                        let ex0 : f32 = $expected.0;
                        assert!(v.0.1.abs() - ex0.abs() < 0.0001);
                        let ex1 : f32 = $expected.1;
                        assert!(v.1.1.abs() - ex1.abs() < 0.0001);
                        let ex2 : f32 = $expected.2;
                        assert!(v.2.1.abs() - ex2.abs() < 0.0001);
                    },
                    Err(s) => assert!(false)
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
                    Ok(v) => assert!(false),
                    Err(s) => assert_eq!(s, $expected)
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
                    Ok(v) => {
                        let scale = $expected.0;
                        assert!(matches!(v.0, scale) && v.1 == $expected.1);
                    },
                    Err(s) => assert!(false)
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
                    Ok(v) => assert!(false),
                    Err(s) => assert_eq!(s, $expected)
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
                let v = convert(&$in.0, $in.1);
                let ex0 : f32 = $expected.0;
                assert!(v.0.1.abs() - ex0.abs() < 0.0001);
                let ex1 : f32 = $expected.1;
                assert!(v.1.1.abs() - ex1.abs() < 0.0001);
            }
        )+
    };
}

test_convert_round![
    test_convert_round_0: (Scale::Celsius, 10.0) => (283.15, 50.0)
    test_convert_round_2: (Scale::Fahrenheit, 10.0) => (260.9278, -12.22222)
    test_convert_round_4: (Scale::Kelvin, 10.0) => (-263.15, -441.67)
    test_convert_round_6: (Scale::Celsius, 0.0) => (273.15, 32.0)
    test_convert_round_8: (Scale::Fahrenheit, 0.0) => (255.3722, -17.77778)
    test_convert_round_10: (Scale::Kelvin, 0.0) => (-273.15, -459.67)
    test_convert_round_12: (Scale::Celsius, 1234.0) => (1507.15, 2253.2)
    test_convert_round_15: (Scale::Fahrenheit, 1234.0) => (940.9278, 667.7778)
    test_convert_round_17: (Scale::Kelvin, 1234.0) => (960.85, 1761.53)
    test_convert_round_19: (Scale::Celsius, -10.0) => (263.15,14.0)
    test_convert_round_21: (Scale::Fahrenheit, -10.0) => (249.8167, -23.33333)
    test_convert_round_22: (Scale::Kelvin, -10.0) => (-283.15, -477.67)
    test_convert_round_24: (Scale::Celsius, -0.0) => (273.15, 32.0)
    test_convert_round_26: (Scale::Fahrenheit, -0.0) => (255.3722, -17.77778)
    test_convert_round_28: (Scale::Kelvin, -0.0) => (-273.15, -459.67)
    test_convert_round_30: (Scale::Celsius, -1234.0) => (-960.85, -2189.2)
    test_convert_round_32: (Scale::Fahrenheit, -1234.0) => (-430.1833, -703.3333)
    test_convert_round_34: (Scale::Kelvin, -1234.0) => (-1507.15, -2680.87)
];