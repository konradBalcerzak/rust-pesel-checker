use pesel_lib::PersonSex;

fn main() {

    let pesel = pesel_lib::header();

    println!("\x1b[1;34mData urodzenia:\x1b[0m \x1b[1;33m{}\x1b[0m", pesel.get_birthday());

    println!("\x1b[1;34mPłeć:\x1b[0m \x1b[1;33m{}\x1b[0m", match pesel.get_sex() {
        PersonSex::Female => 'K',
        PersonSex::Male   => 'M'
    });

    println!("================================");
}