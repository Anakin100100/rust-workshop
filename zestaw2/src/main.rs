use core::panic;
use std::error::Error;

use clap::Parser;


fn heaveside(num: f64) -> f64 {
    if num >= 0.0 {
        return 1.0;
    }
    return 0.0;
}

fn absolute_value(num: f64) -> f64 {
    if num < 0.0 {
        return -num;
    }
    return num;
}

fn sgn(num: f64) -> f64 {
    if num >= 0.0 {
        return 1.0;
    }
    return -1.0;
}

fn entier(num: f64, entier: f64) -> f64 {
    if num > entier {
        entier;
    }
    return num;
}

fn mantysa(num: f64, entier_val: f64) -> f64 {
    return (num - entier(num, entier_val));
}

// Zad 2
fn area_and_circumference(a: f64, b: f64, c: f64) -> Result<(f64, f64), String> {
    if a <= 0.0 || b <= 0.0 || c <= 0.0 {
        return Err("All sides have to be positive".to_string());
    }
    if a + b < c || a + c < b || c + b < a {
        return Err("Sum of any two sides must be greater than the third one".to_string());
    }
    let circumference = a + b + c;
    let p = 0.5 * (a + b + c);
    let area = (p * (p - a) * (p - b) * (p - c)).sqrt();

    return Ok((circumference, area));

    
}

enum Quarter {
    I, 
    II,
    III,
    IV
}

// Zad 4
fn get_quarter(x: f64, y: f64) -> Quarter {
    if x == 0.0 || y == 0.0 {
        panic!("Cannot assign a quarter when y = 0.0 or x = 0.0")
    }
    return match (x > 0.0, y > 0.0) {
        (true, true) => Quarter::I,
        (true, false) => Quarter::II,
        (false, false) => Quarter::III,
        (false, true) => Quarter::IV
    }
}

// Zad 5
fn bit_shift() {
    let x = 5;
    println!("{}", x << 3);
    let y = 122;
    println!("{}", y >> 1);
}

// Zad 6
fn split_string() {
    let s = "Ala ma kota".to_string(); 
    let split_s: Vec<String> = s.split_whitespace().map(str::to_string).collect();
    println!("Ala ma kota split: {:?}", split_s)
}

// Zad 8
fn test_bin_hex_oct() {
    let num = 21;
    println!("num: {}, bin: {:b}, hex: {:x}, oct: {:o}", num, num, num, num)
}


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Pesel belonging to a person we want to analyze
    #[arg(short, long)]
    pesel: String,
}


fn main() {
    let args = Args::parse();

    let pesel = args.pesel;
    
    if pesel.len() != 11 {
        panic!("Pesel must be 11 characters long")
    }

    let mut nums: Vec<i32> = vec![];
    for char in pesel.chars() {
        if char.is_digit(10) == false {
            panic!("Every char in pesel must be a number {} is not a number", {char})
        }
        nums.push(char.to_digit(10).unwrap() as i32)
    }

    if nums[10] % 2 == 0 {
        println!("Płeć: mężczyzna")
    } else {
        println!("Płeć: kobieta")
    }

    if nums[2] * 10 + nums[3] > 20 {
        println!("Miesiąc urodzenie: {}{}", nums[2] * 10 - 20, nums[3])
    } else {
        println!("Miesiąc urodzenie: {}{}", nums[2], nums[3])
    }

    if nums[0] * 10 + nums[1] < 22 {
        println!("Wiek: 20{}{}", nums[0], nums[1])
    } else {
        println!("Wiek: 19{}{}", nums[0], nums[1])
    }

    test_bin_hex_oct()
}
