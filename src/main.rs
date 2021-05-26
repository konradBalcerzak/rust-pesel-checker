use pesel_lib;
use pesel_lib::PersonSex;

fn main() {
    let pesel = pesel_lib::header();

    println!("PESEL: {}", pesel);

    println!("Data urodzenia: {}", pesel.get_birthday());

    println!("Płeć: {}", match pesel.get_sex() {
        PersonSex::Female => 'K',
        PersonSex::Male   => 'M'
    });

    println!("================================");
}