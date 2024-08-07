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
        Scale::Fahrenheit => return ((Scale::Celsius, to_cels(&scale, value)), (Scale::Kelvin, to_kelv(&scale, value)))
    }
}

fn calculate(input: String) -> Result<((Scale, f32), (Scale, f32), (Scale, f32)), String> {
    let mut conversions: ((Scale, f32), (Scale, f32)) = ((Scale::Kelvin, 0.0), (Scale::Kelvin, 0.0));
    let mut r: String = "".to_string();
    match parse_temp_input(input.as_str()) {
        Ok(parsedInput) => {
            conversions = convert(&parsedInput.0, parsedInput.1);
            return Ok(((parsedInput.0, parsedInput.1), (conversions.0.0, conversions.0.1), (conversions.1.0, conversions.1.1)));
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

#[test]
fn test_calculate() {
    let inputs = vec![
        "10c",
        "10C",
        "10f",
        "10F",
        "10k",
        "10K",
        "0c",
        "0C",
        "0f",
        "0F",
        "0k",
        "0K",
        "1234c",
        "1234C",
        "1234f",
        "1234F",
        "1234k",
        "1234K",
        "-10c",
        "-10C",
        "-10f",
        "-10F",
        "-10k",
        "-10K",
        "-0c",
        "-0C",
        "-0f",
        "-0F",
        "-0k",
        "-0K",
        "-1234c",
        "-1234C",
        "abc",
        "-1234G",
        "-1234k",
        "-1234K",
    ];

    let outputs = vec![
        "10c",
        "10C",
        "10f",
        "10F",
        "10k",
        "10K",
        "0c",
        "0C",
        "0f",
        "0F",
        "0k",
        "0K",
        "1234c",
        "1234C",
        "1234f",
        "1234F",
        "1234k",
        "1234K",
        "-10c",
        "-10C",
        "-10f",
        "-10F",
        "-10k",
        "-10K",
        "-0c",
        "-0C",
        "-0f",
        "-0F",
        "-0k",
        "-0K",
        "-1234c",
        "-1234C",
        "abc",
        "-1234G",
        "-1234k",
        "-1234K",
    ];

    for i in 0..inputs.len() {
        assert_eq!(calculate(inputs[i].to_string()), outputs[i].to_string());
    }
}