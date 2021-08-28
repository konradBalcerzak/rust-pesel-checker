use pesel_lib::pesel::{Pesel, PersonGender::*};

fn main() {
    cli_header();
}

fn cli_header() {

    println!("================================");
    
    let pesel = read_pesel();
    let [year, month, day] = pesel.get_birthday();
    let gender = match pesel.get_gender() { Male => 'M', Female => 'K', };

    println!("\x1b[1;34mData urodzenia:\x1b[0m \x1b[1;33m{}-{:02}-{:02}\x1b[0m", year, month, day);
    println!("\x1b[1;34mPłeć:\x1b[0m \x1b[1;33m{}\x1b[0m", gender);

    println!("================================");
}

fn read_pesel () -> Pesel {
    use std::io::stdin;

    let mut input_pesel = String::new();
    loop {
        println!("\x1b[1;34mPodaj numer PESEL:\x1b[0m");

        if stdin().read_line(&mut input_pesel).is_ok() {
            match Pesel::new(input_pesel.trim()) {
                Ok(pesel)          => return pesel,
                Err(error_message) => println!("\x1b[0;31mBŁĄD: {}\x1b[0m", error_message),
            };
        }
        println!("--------------------------------");
        input_pesel = String::new();
    }
}