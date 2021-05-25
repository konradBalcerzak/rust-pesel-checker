use pesel_lib;

fn main() {
    let pesel = pesel_lib::header();

    println!("PESEL: {:?}", pesel);

    println!("================================");
}