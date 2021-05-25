// use std::env;

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
                return Err(String::from(""));
            }
        }
        Ok(DigitVec(pesel_digits))
    }
}

#[derive(Debug)]
pub enum PeselErr {
    Generic
}

pub struct Pesel {
    value: Vec<u32>
}

impl Pesel {
    fn new (input: &str) -> Result<Pesel, PeselErr> {
        if let Ok(digits) = DigitVec::new(input) {
            if Pesel::check_correctness(&digits.0) {
                return Ok(Pesel {
                    value: digits.0
                });
            }
        }
        return Err(PeselErr::Generic);
    }
    
    fn check_correctness (candidate: &Vec<u32>) -> bool {
        let weights: [u32; 11] = [1, 3, 7, 9, 1, 3, 7, 9, 1, 3, 1];

        if candidate.len() == 11 {
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
            let control_num = 10 - sum;
            
            if let Some(control_digit) = candidate.last() {
                return *control_digit == control_num || *control_digit == sum;
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
            if let Ok(pesel) = Pesel::new(input_pesel.trim()) {
                return Ok(pesel);
            };
        }
        input_pesel = String::new();
        continue;
    }
    return Err(PeselErr::Generic);
}