
use std::io;

fn f2c(temp: f64) -> f64 {
    (temp - 32.0) * 5.0 / 9.0
}

fn c2f(temp: f64) -> f64 {
    temp * (9.0 / 5.0) + 32.0
}

fn main() {

    println!("1 - F2C, 2 - C2F:");
    let mut convert_type = String::new();
    let convert_type = loop {

        io::stdin()
            .read_line(&mut convert_type)
            .expect("Failed to read line");

        let convert_type: u32 = match convert_type.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if convert_type == 1 || convert_type == 2 {
            break convert_type
        }
    };

    println!("Temperature:");
    let mut temp = String::new();
    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line");
    let temp: f64 = temp.trim().parse().expect("Must input float.");

    let converted_temp = if convert_type == 1 {f2c(temp)} else {c2f(temp)};
    println!("Converted temp: {converted_temp}");


}
