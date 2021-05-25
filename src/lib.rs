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

#[derive(Debug)]
pub struct Pesel {
    value: Vec<u32>,
    year: u32,
    month: u32,
    day: u32
}

impl Pesel {
    fn new (input: &str) -> Result<Pesel, String> {
        let digits = DigitVec::new(input)?;
        Pesel::check_correctness(&digits.0)?;
        return Ok(Pesel {
            year: Pesel::slice_to_year(&digits.0[0..2], &digits.0[2]),
            month: Pesel::slice_to_month(&digits.0[2..4]),
            day: slice_to_num(&digits.0[4..6]) as u32,
            value: digits.0
        });
    }

    fn slice_to_month(month_digits: &[u32]) -> u32 {
        const MONTH_PREFIXES: [u32; 5] = [8, 0, 2, 4, 6];
        
        for prefix in MONTH_PREFIXES.iter() {
            let result = month_digits[0] as i32 - *prefix as i32;
            if  result >= 0 && result <= 1  {
                return result as u32 * 10 + month_digits[1];
            }
        };
        0
    }

    fn slice_to_year(year_digits: &[u32], month_digit: &u32) -> u32 {
        const MONTH_PREFIXES: [u32; 5] = [8, 0, 2, 4, 6];

        let mut year = 1800;
        for prefix in MONTH_PREFIXES.iter() {
            let result = *month_digit as i32 - *prefix as i32;
            if  result >= 0 && result <= 1 {
                break;
            }
            year += 100;
        }
        year += year_digits[0] * 10 + year_digits[1];
        year
    }
    
    fn check_correctness (candidate: &Vec<u32>) -> Result<(), String> {
        let digit_count = candidate.len();
        if digit_count < 11 {
            return Err(String::from("Za mało znaków"));
        }
        else if digit_count > 11 {
            return Err(String::from("Za dużo znaków"));
        }

        if !Pesel::correct_month(&candidate[2..4]) {
            return Err(String::from("Podany miesiąc jest nieprawidłowy"));
        }
        if !Pesel::correct_day(&candidate[0..2], &candidate[2..4], &candidate[4..6]) {
            return Err(String::from("Podany dzień jest nieprawidłowy"));
        }

        const MULTIPLIERS: [u32; 11] = [1, 3, 7, 9, 1, 3, 7, 9, 1, 3, 1];
        let mut sum = 0;
        let mut iteration = 0;

        for (pesel_digit, multiplier) in candidate.iter().zip(MULTIPLIERS.iter()) {
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

    fn correct_month(month_digits: &[u32]) -> bool {
        const MONTH_PREFIXES: [u32; 5] = [8, 0, 2, 4, 6];
        let mut first_month_digit = 0;

        for prefix in MONTH_PREFIXES.iter() {
            let result = month_digits[0] as i32 - *prefix as i32;
            if  result >= 0 && result <= 1  {
                first_month_digit = result;
            }
        }
        if first_month_digit == 1 && month_digits[1] > 2 {
            return false;
        }
        else if month_digits[0] == 0 && month_digits[1] == 0 {
            return false;
        };
        return true;
    }

    fn correct_day(year_digits: &[u32],month_digits: &[u32], day_digits: &[u32]) -> bool {
        let month = Pesel::slice_to_month(month_digits);
        let day = day_digits[0] * 10 + day_digits[1];
        if day < 1 || day > 31 {
            return false;
        }
        let mut february_days = 28;
        if (year_digits[0] * 10 + year_digits[1]) % 4 == 0 {
            february_days = 29;
        }
        let days_in_month: [u32; 12] = [31, february_days, 29, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        day <= days_in_month[month as usize -1]
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

fn slice_to_num (slice: &[u32]) -> i32 {
    let mut number = 0;
    let digit_amount = slice.len();
    for (index, digit) in slice.iter().enumerate() {
        number += 10i32.pow(digit_amount as u32-1 - index as u32) * *digit as i32;
    };
    println!("{:?}: {}",slice , number);
    number as i32
}