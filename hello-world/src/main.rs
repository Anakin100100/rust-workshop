use lazy_static::lazy_static;
use std::{collections::HashMap, f64::consts::PI};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

static INITIAL_VELOCITY: f64 = 3 as f64 / 44 as f64;

fn z_1(t: f64) -> f64 {
    return INITIAL_VELOCITY * t - 0.5 * 9.8 * t * t;
}

#[derive(EnumIter, Debug, Hash, Eq, PartialEq, Display)]
enum Planet {
    Mercury,
    Venus,
    Earth,
    Mars,
    Jupiter,
    Saturn,
    Uranus,
    Neptune,
}

lazy_static! {
    static ref PLANET_GRAVITY: HashMap<&'static Planet, f64> = {
        let mut map = HashMap::new();
        map.insert(&Planet::Mercury, 3.7);
        map.insert(&Planet::Venus, 8.9);
        map.insert(&Planet::Earth, 9.8);
        map.insert(&Planet::Mars, 3.7);
        map.insert(&Planet::Jupiter, 23.1);
        map.insert(&Planet::Saturn, 9.0);
        map.insert(&Planet::Uranus, 8.7);
        map.insert(&Planet::Neptune, 11.0);
        map
    };
}

fn z_2(planet: &Planet, t: f64) -> f64 {
    return INITIAL_VELOCITY * t - 0.5 * PLANET_GRAVITY.get(planet).unwrap() * t * t;
}

#[derive(EnumIter, Debug, Hash, Eq, PartialEq, Display)]
enum ImperialUnit {
    Inch,
    Foot,
    Yard,
    Mile,
}

lazy_static! {
    static ref METERS_TO_IMPERIAL_UNITS: HashMap<&'static ImperialUnit, f64> = {
        let mut map = HashMap::new();
        map.insert(&ImperialUnit::Inch, (1 as f64 / 2.54) * 100 as f64);
        map.insert(
            &ImperialUnit::Foot,
            (1 as f64 / 2.54) * (100 as f64 / 12 as f64) as f64,
        );
        map.insert(
            &ImperialUnit::Yard,
            (1 as f64 / 2.54) * (100 as f64 / (12 * 3) as f64) as f64,
        );
        map.insert(
            &ImperialUnit::Mile,
            (1 as f64 / 2.54) * (100 as f64 / (12 * 3 * 1760) as f64) as f64,
        );
        map
    };
}

fn z_3(meters: f64, imperial_unit: &ImperialUnit) -> f64 {
    return meters * METERS_TO_IMPERIAL_UNITS.get(imperial_unit).unwrap();
}

enum Temp {
    Celcius,
    Fahrenheit,
}

fn z_4(from: Temp, to: Temp, value: f64) -> f64 {
    return match (from, to) {
        (Temp::Celcius, Temp::Fahrenheit) => (9.0 / 5.0) * value + 32.0,
        (Temp::Fahrenheit, Temp::Celcius) => (value - 32.0) * (5.0 / 9.0),
        _ => value,
    };
}

// Z5
struct Circle {
    radius: f64,
}

impl Circle {
    fn area(&self) -> f64 {
        return PI * self.radius * self.radius;
    }

    fn circumference(&self) -> f64 {
        return 2.0 * PI * self.radius;
    }
}

// Z6
struct CirclePartial {
    radius: f64,
    angle: f64,
}

impl CirclePartial {
    fn area(&self) -> f64 {
        return (self.radius * self.radius) / (2.0 * self.angle);
    }

    fn circumference(&self) -> f64 {
        return 3.0 * self.radius;
    }
}

// Zad 7
fn y(x: f64, h0: f64, v0: f64, alpha: f64, g: f64) -> f64 {
    h0 + x * alpha.tan() - (g * x * x) / (2.0 * v0 * v0 * alpha.cos() * alpha.cos())
}

fn compound_interest(
    initial_ammount: f64,
    interest_rate: f64,
    capitalization_steps_per_year: i32,
    number_of_years: i32,
) -> f64 {
    initial_ammount
        * (1.0 + interest_rate / capitalization_steps_per_year as f64)
            .powi(capitalization_steps_per_year * number_of_years)
}

#[cfg(test)]
mod tests {
    use crate::y;
    use crate::z_3;
    use crate::z_4;
    use crate::ImperialUnit;
    use crate::Temp;

    #[test]
    fn z_3_test() {
        //rounding z_3 result to two decimal places
        assert_eq!(
            (z_3(640 as f64, &ImperialUnit::Inch) * 100 as f64).round() / 100 as f64,
            25196.85
        );
        assert_eq!(
            (z_3(640 as f64, &ImperialUnit::Foot) * 100 as f64).round() / 100 as f64,
            2099.74
        );
        assert_eq!(
            (z_3(640 as f64, &ImperialUnit::Yard) * 100 as f64).round() / 100 as f64,
            699.91
        );
        assert_eq!(
            (z_3(640 as f64, &ImperialUnit::Mile) * 10000 as f64).round() / 10000 as f64,
            0.3977
        );
    }

    #[test]
    fn z_4_test() {
        assert_eq!(z_4(Temp::Celcius, Temp::Fahrenheit, 40.0), 104.0);
        assert_eq!(z_4(Temp::Fahrenheit, Temp::Celcius, 50.0), 10.0);
        assert_eq!(z_4(Temp::Celcius, Temp::Celcius, 10.0), 10.0);
        assert_eq!(z_4(Temp::Fahrenheit, Temp::Fahrenheit, 10.0), 10.0);
    }

    #[test]
    fn z_7_test() {
        assert_eq!((y(2.963, 12.5, 3.0, 0.12, 9.8) * 10.0).round() / 10.0, 8.0)
    }
}

fn main() {
    // Z1
    let results_1 = vec![z_1(0.54), z_1(0.1), z_1(0.235)];
    println!("Velocity after 0.54s: {}", results_1[0]);
    println!("Velocity after 0.1s: {}", results_1[1]);
    println!("Velocity after 0.235s: {}", results_1[2]);

    // Z2
    for planet in Planet::iter() {
        for time in vec![0.54, 0.1, 0.235] {
            println!(
                "Velocity at {} after {}s = {}",
                &planet,
                time,
                z_2(&planet, time)
            )
        }
    }

    // Z8
    println!(
        "stopa procentowa wynosi 5%, mamy roczną kapitalizacją odsetek i loakata trwa 2 lata: {:+e}",
        compound_interest(1000.0, 0.05, 1, 2)
    );
    println!(
        "stopa procentowa wynosi 2.3%, mamy kwartalną kapitalizację odsetek i lokata trwa 36 miesięcy.: {:+e}",
        compound_interest(1000.0, 0.023, 4, 3)
    );

    //Zad 9 i 10
    println!("{:+e}", 12345);
    println!("{:+e}", 0.12345);
    println!("{:+e}", 0.00000123);
    println!("{:+3e}", 1020304050);
    println!("Mars circumference: {:+e}", 21344);
    println!(
        "Earth distance travelled in an hour {:+e} kilometers",
        30 * 3600
    );
    println!(
        "Earth distance travelled in a day {:+e} kilometers",
        30 * 3600 * 24
    );
    println!(
        "Earth distance travelled in an year {:+e} kilometers",
        30 * 3600 * 24 * 365
    );
}
