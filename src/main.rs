struct Temperature {
    value: f32,
    unit: char
}

fn main() {
    let input = std::env::args().nth(1).expect("Please enter a temperature in F, C or K");
    let inputTemp = Temperature {
        value: input[..(input.chars().count() - 1)].parse::<f32>().unwrap(), unit: input.chars().nth(input.chars().count() - 1).unwrap().to_uppercase().collect::<Vec<_>>()[0]
    };
    match inputTemp.unit{
        'K'=>println!("{}{}\n{}{}\n{}{}", inputTemp.value, 'K', toCels(inputTemp.value, 'K'), 'C', toFahr(inputTemp.value, 'K'), 'F'),
        'C'=>println!("{}{}\n{}{}\n{}{}", toKelv(inputTemp.value, 'C'), 'K', inputTemp.value, 'C', toFahr(inputTemp.value, 'C'), 'F'),
        'F'=>println!("{}{}\n{}{}\n{}{}", toKelv(inputTemp.value, 'F'), 'K', toCels(inputTemp.value, 'F'), 'C', inputTemp.value, 'F'),
        _=>println!("Invalid Entry")
    }
}

fn toCels(value: f32, unit: char) -> f32 {
    match unit {
        'K'=> return (value - 273.15),
        'F'=> return ((value - 32.0) * (5.0/9.0)),
        _=> return (0.0)
    }
}

fn toFahr(value: f32, unit: char) -> f32 {
    match unit {
        'K'=> return ((value - 273.15) * 9.0/5.0 + 32.0),
        'C'=> return (value * 9.0/5.0 + 32.0),
        _=> return (0.0)
    }
}

fn toKelv(value: f32, unit: char) -> f32 {
    match unit {
        'F'=> return ((value - 32.0) * 5.0/9.0 + 273.15),
        'C'=> return (value + 273.15),
        _=> return (0.0)
    }
}
