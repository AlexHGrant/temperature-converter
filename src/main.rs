fn main() {
    let input = std::env::args().nth(1).expect("Please enter a temperature in F, C or K");
    let inputValue = input[..(input.chars().count() - 1)].parse::<f32>().unwrap();
    let inputUnit = input.chars().nth(input.chars().count() - 1).unwrap().to_uppercase().collect::<Vec<_>>()[0];
    match inputUnit{
        'K'=>println!("{}{}\n{}{}\n{}{}", inputValue, 'K', toCels(inputValue, 'K'), 'C', toFahr(inputValue, 'K'), 'F'),
        'C'=>println!("{}{}\n{}{}\n{}{}", toKelv(inputValue, 'C'), 'K', inputValue, 'C', toFahr(inputValue, 'C'), 'F'),
        'F'=>println!("{}{}\n{}{}\n{}{}", toKelv(inputValue, 'F'), 'K', toCels(inputValue, 'F'), 'C', inputValue, 'F'),
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
