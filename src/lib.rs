// use std::env;

#[derive(Debug)]
pub enum PeselErr {
    Generic
}

pub struct Pesel {
    // value: String
}

impl Pesel {
    fn new (input: &str) -> Result<Pesel, PeselErr> {
        if Pesel::check_correctness(input) {
            return Ok(Pesel {
                // value: String::from(input)
            });
        }
        else {
            return Err(PeselErr::Generic);
        }
    }
    
    fn check_correctness (candidate: &str) -> bool {
        if candidate.bytes().count() == 11 {
            let weights: [u32; 11] = [1, 3, 7, 9, 1, 3, 7, 9, 1, 3, 1];
            let mut digit_amount = 0;
            
            let pesel_digits = candidate
            .chars()
            .map(|char| {
                let digit_opt = char.to_digit(10);
                if let Some(_) = digit_opt {
                    digit_amount += 1;
                }
                digit_opt
            });
            for digit in pesel_digits {
                println!("{:?}", digit);
            }
            let mut sum = 0;

            for (offset, char) in candidate[..10].char_indices() {
                if let Some(digit) = char.to_digit(10) {
                    sum += digit * weights[offset];
                }
                else {
                    return false;
                };
            }
            sum = sum % 10;
            let control_num = 10 - sum;
            
            if let Some(char) = candidate.chars().last() {
                if let Some(digit) = char.to_digit(10) {
                    return digit == control_num || digit == sum;
                }
            }
        }
        false
    }
}

pub fn header () -> Result<Pesel, PeselErr> {
    use std::io::stdin;

    let mut input_pesel = String::new();

    loop {
        println!("\n================================\n");
        println!("Podaj numer PESEL:");
        if let Ok(_) =  stdin().read_line(&mut input_pesel) {
            input_pesel = String::from(input_pesel.trim());
            if let Ok(pesel) = Pesel::new(&input_pesel) {
                return Ok(pesel);
            };
        }
        input_pesel = String::new();
        continue;
    }
    println!("\n================================\n");
    return Err(PeselErr::Generic);
}