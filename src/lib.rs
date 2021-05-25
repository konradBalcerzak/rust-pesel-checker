// use std::env;

struct PeselVec(Vec<u32>);

impl PeselVec {
    fn new (candidate: &str) -> Result<PeselVec, PeselErr> {
        let mut pesel_digits: Vec<u32> = Vec::new();
        
        let candidate_iter = candidate
            .chars()
            .map(|char| char.to_digit(10));

        for digit_opt in candidate_iter {
            if let Some (digit) = digit_opt {
                pesel_digits.push(digit);
            } else {
                return Err(PeselErr::Message(String::from("Podany pesel zawiera inne znaki niż cyfry")));
            }
        }
        Ok(PeselVec(pesel_digits))
    }
}

#[derive(Debug)]
pub enum PeselErr {
    Generic,
    Message(String)
}

pub struct Pesel {
    // value: Vec<u32>
}

impl Pesel {
    fn new (input: &str) -> Result<Pesel, PeselErr> {
        let digits = PeselVec::new(input)?;
        Pesel::check_correctness(&digits.0)?;
        return Ok(Pesel {
            // value: digits.0
        });
    }
    
    fn check_correctness (candidate: &Vec<u32>) -> Result<(), PeselErr> {
        let digit_count = candidate.len();
        if digit_count < 11 {
            return Err(PeselErr::Message(String::from("Za mało znaków")));
        }
        else if digit_count > 11 {
            return Err(PeselErr::Message(String::from("Za dużo znaków")));
        }

        let weights: [u32; 11] = [1, 3, 7, 9, 1, 3, 7, 9, 1, 3, 1];
        let mut sum = 0;
        let mut iteration = 0;

        for (pesel_digit, multiplier) in candidate.iter().zip(weights.iter()) {
            if iteration == 10 {
                break;
            }
            iteration += 1;
            sum += pesel_digit * multiplier;
        }
        sum = sum % 10;
        
        if let Some(control_digit) = candidate.last() {
            if *control_digit == 10 - sum || *control_digit == sum {
                return Ok(());
            } else {
                return Err(PeselErr::Message(String::from("Suma kontrolna niepoprawna")))
            }
        }
        Err(PeselErr::Message(String::from("Nieznany błąd")))
    }
}

pub fn header () -> Pesel {
    use std::io::stdin;

    let mut input_pesel = String::new();

    loop {
        println!("================================");
        println!("Podaj numer PESEL:");
        if let Ok(_) =  stdin().read_line(&mut input_pesel) {
            match Pesel::new(input_pesel.trim()) {
                Err(error) => {
                    println!("--------------------------------");
                    if let PeselErr::Message(error_msg) = error {
                        println!("BŁĄD: {}", error_msg);
                    }
                }
                Ok (pesel) => {
                    return pesel;
                }
            }
        }
        input_pesel = String::new();
        continue;
    }
}