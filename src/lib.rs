use std::fmt;

struct DigitVec(Vec<u32>);

impl DigitVec {
    fn new (candidate: &str) -> Result<DigitVec, String> {
        let mut pesel_digits: Vec<u32> = Vec::new();
        
        let candidate_iter = candidate
            .chars()
            .map(|char| char.to_digit(10));

        for digit_opt in candidate_iter {
            if let Some (digit) = digit_opt {
                pesel_digits.push(digit);
            } else {
                return Err(String::from("Podany pesel zawiera inne znaki niż cyfry"));
            }
        }
        Ok(DigitVec(pesel_digits))
    }
}

pub struct Pesel {
    value: Vec<u32>
}

impl Pesel {
    fn new (input: &str) -> Result<Pesel, String> {
        let digits = DigitVec::new(input)?;
        Pesel::check_correctness(&digits.0)?;
        return Ok(Pesel {
            value: digits.0
        });
    }
    
    fn check_correctness (candidate: &Vec<u32>) -> Result<(), String> {
        let digit_count = candidate.len();
        if digit_count < 11 {
            return Err(String::from("Za mało znaków"));
        }
        else if digit_count > 11 {
            return Err(String::from("Za dużo znaków"));
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
                return Err(String::from("Suma kontrolna niepoprawna"))
            }
        }
        Err(String::from("Nieznany błąd"))
    }
}

impl fmt::Display for Pesel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>)->fmt::Result {
        let mut pesel_fmt = String::new();
        for digit in &self.value {
            if let Some(char) = char::from_digit(*digit, 10) {
                pesel_fmt.push(char);
            }
        }

        write!(f, "{}", pesel_fmt)
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
                Err(error_message) => {
                    println!("BŁĄD: {}", error_message);
                }
                Ok (pesel) => {
                    return pesel;
                }
            }
        }
        println!("--------------------------------");
        input_pesel = String::new();
        continue;
    }
}