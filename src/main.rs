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
            return Err(format!("invalid number {}", temp_str));
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

macro_rules! test_calculate_round {
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

test_calculate_round![
    test_calculate_round_0: "10c".to_string() => (10.0, 283.15, 50.0)
    test_calculate_round_1: "10C".to_string() => (10.0, 283.15, 50.0)
    test_calculate_round_2: "10f".to_string() => (10.0, 260.9278, -12.22222)
    test_calculate_round_3: "10F".to_string() => (10.0, 260.9278, -12.22222)
    test_calculate_round_4: "10k".to_string() => (10.0, -263.15, -441.67)
    test_calculate_round_5: "10K".to_string() => (10.0, -263.15, -441.67)
    test_calculate_round_6: "0c".to_string() => (0.0, 273.15, 32.0)
    test_calculate_round_7: "0C".to_string() => (0.0, 273.15, 32.0)
    test_calculate_round_8: "0f".to_string() => (0.0, 255.3722, -17.77778)
    test_calculate_round_9: "0F".to_string() => (0.0, 255.3722, -17.77778)
    test_calculate_round_10: "0k".to_string() => (0.0, -273.15, -459.67)
    test_calculate_round_11: "0K".to_string() => (0.0, -273.15, -459.67)
    test_calculate_round_12: "1234c".to_string() => (1234.0, 1507.15, 2253.2)
    test_calculate_round_13: "1234C".to_string() => (1234.0, 1507.15, 2253.2)
    test_calculate_round_14: "1234f".to_string() => (1234.0, 940.9278, 667.7778)
    test_calculate_round_15: "1234F".to_string() => (1234.0, 940.9278, 667.7778)
    test_calculate_round_16: "1234k".to_string() => (1234.0, 960.85, 1761.53)
    test_calculate_round_17: "1234K".to_string() => (1234.0, 960.85, 1761.53)
    test_calculate_round_18: "-10c".to_string() => (-10.0, 263.15,14.0)
    test_calculate_round_19: "-10C".to_string() => (-10.0, 263.15,14.0)
    test_calculate_round_20: "-10f".to_string() => (-10.0, 249.8167, -23.33333)
    test_calculate_round_21: "-10F".to_string() => (-10.0, 249.8167, -23.33333)
    test_calculate_round_22: "-10k".to_string() => (-10.0, -283.15, -477.67)
    test_calculate_round_23: "-10K".to_string() => (-10.0, -283.15, -477.67)
    test_calculate_round_24: "-0c".to_string() => (0.0, 273.15, 32.0)
    test_calculate_round_25: "-0C".to_string() => (0.0, 273.15, 32.0)
    test_calculate_round_26: "-0f".to_string() => (0.0, 255.3722, -17.77778)
    test_calculate_round_27: "-0F".to_string() => (0.0, 255.3722, -17.77778)
    test_calculate_round_28: "-0k".to_string() => (0.0, -273.15, -459.67)
    test_calculate_round_29: "-0K".to_string() => (0.0, -273.15, -459.67)
    test_calculate_round_30: "-1234c".to_string() => (-1234.0, -960.85, -2189.2)
    test_calculate_round_31: "-1234C".to_string() => (-1234.0, -960.85, -2189.2)
    test_calculate_round_32: "-1234f".to_string() => (-1234.0, -430.1833, -703.3333)
    test_calculate_round_33: "-1234F".to_string() => (-1234.0, -430.1833, -703.3333)
    test_calculate_round_34: "-1234k".to_string() => (-1234.0, -1507.15, -2680.87)
    test_calculate_round_35: "-1234K".to_string() => (-1234.0, -1507.15, -2680.87)
];

        // "10c",
        // "10C",
        // "10f",
        // "10F",
        // "10k",
        // "10K",
        // "0c",
        // "0C",
        // "0f",
        // "0F",
        // "0k",
        // "0K",
        // "1234c",
        // "1234C",
        // "1234f",
        // "1234F",
        // "1234k",
        // "1234K",
        // "-10c",
        // "-10C",
        // "-10f",
        // "-10F",
        // "-10k",
        // "-10K",
        // "-0c",
        // "-0C",
        // "-0f",
        // "-0F",
        // "-0k",
        // "-0K",
        // "-1234c",
        // "-1234C",
        // "abc",
        // "-1234G",
        // "-1234k",
        // "-1234K",