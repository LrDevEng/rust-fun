pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}

pub fn kelvin_to_celsius(k: f64) -> f64 {
    k - 273.15
}

pub fn celsius_to_kelvin(c: f64) -> f64 {
    c + 273.15
}

pub fn kelvin_to_fahrenheit(k: f64) -> f64 {
    k * 9.0 / 5.0 - 459.67
}

pub fn fahrenheit_to_kelvin(f: f64) -> f64 {
    (f + 459.67) * 5.0 / 9.0
}
