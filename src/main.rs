fn main() {
    let input_value = input[..(input.chars().count() - 1)].parse::<f64>().unwrap();
    let input_unit = input.chars().nth(input.chars().count() - 1).unwrap().to_uppercase().collect::<Vec<_>>()[0];
    match input_unit{
        'K'=>println!("{}{}\n{}{}\n{}{}", input_value, 'K', to_cels(input_value, 'K'), 'C', to_fahr(input_value, 'K'), 'F'),
        'C'=>println!("{}{}\n{}{}\n{}{}", to_kelv(input_value, 'C'), 'K', input_value, 'C', to_fahr(input_value, 'C'), 'F'),
        'F'=>println!("{}{}\n{}{}\n{}{}", to_kelv(input_value, 'F'), 'K', to_cels(input_value, 'F'), 'C', input_value, 'F'),
        _=>println!("Invalid Entry")
    }
}

fn to_cels(value: f64, unit: char) -> f64 {
    match unit {
        'K'=> return value - 273.15,
        'F'=> return (value - 32.0) * 5.0/9.0,
        _=> return 0.0
    }
}

fn to_fahr(value: f64, unit: char) -> f64 {
    match unit {
        'K'=> return (value - 273.15) * 9.0/5.0 + 32.0,
        'C'=> return value * 9.0/5.0 + 32.0,
        _=> return 0.0
    }
}

fn to_kelv(value: f64, unit: char) -> f64 {
    match unit {
        'F'=> return (value - 32.0) * 5.0/9.0 + 273.15,
        'C'=> return value + 273.15,
        _=> return 0.0
    }
}
