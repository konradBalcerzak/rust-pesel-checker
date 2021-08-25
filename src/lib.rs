use std::fmt;
struct DigitVec(Vec<u32>);

impl DigitVec {
    fn new(candidate: &str) -> Result<DigitVec, String> {
        let mut pesel_digits: Vec<u32> = Vec::new();
        
        let candidate_iter = candidate
            .chars()
            .map(|char| char.to_digit(10));

        for digit_opt in candidate_iter {
            if let Some(digit) = digit_opt {
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
    fn month_prefixes() -> [u32; 5] {
        [8, 0, 2, 4, 6]
    }
    
    pub fn new(input: &str) -> Result<Pesel, String> {
        let digits = DigitVec::new(input)?;
        Pesel::check_correctness(&digits.0)?;
        Ok(Pesel {
            year: Pesel::slice_to_year(&digits.0[0..2], &digits.0[2]),
            month: Pesel::slice_to_month(&digits.0[2..4]),
            day: slice_to_num(&digits.0[4..6]) as u32,
            value: digits.0
        })
    }

    pub fn get_birthday(&self) -> String {
        format!("{}-{}-{}", self.year, prepend_zero(self.month), prepend_zero(self.day))
    }

    pub fn get_sex(&self) -> PersonSex {
        if self.value[9] % 2 == 0 {
            PersonSex::Female
        } else {
            PersonSex::Male
        }
    }
    
    fn check_correctness(candidate: &[u32]) -> Result<(), String> {
        let digit_count = candidate.len();
        match digit_count {
            x if x < 11 => return Err(String::from("Za mało znaków")),
            x if x > 11 => return Err(String::from("Za dużo znaków")),
            _ => ()
        };

        if !Pesel::correct_month(&candidate[2..4]) {
            return Err(String::from("Podany miesiąc jest nieprawidłowy"));
        }
        if !Pesel::correct_day(&candidate[0..2], &candidate[2..4], &candidate[4..6]) {
            return Err(String::from("Podany dzień jest nieprawidłowy"));
        }

        const MULTIPLIERS: [u32; 11] = [1, 3, 7, 9, 1, 3, 7, 9, 1, 3, 1];
        let mut sum = 0;

        for (iteration, (pesel_digit, multiplier)) in candidate.iter().zip(MULTIPLIERS.iter()).enumerate() {
            if iteration == 10 {
                break;
            }
            sum += pesel_digit * multiplier;
        }
        sum %= 10;
        if let Some(control_digit) = candidate.last() {
            if *control_digit == 10 - sum || *control_digit == sum {
                Ok(())
            } else {
                Err(String::from("Suma kontrolna niepoprawna"))
            }
        } else {
            Err(String::from("Nieznany błąd"))
        }
        
    }

    fn correct_month(month_digits: &[u32]) -> bool {
        let mut first_month_digit = 0;

        for prefix in Pesel::month_prefixes().iter() {
            if month_digits[0] == *prefix || month_digits[0] == *prefix+1 {
                first_month_digit = month_digits[0] - prefix;
            }
        }
        if first_month_digit == 1 && month_digits[1] > 2 {
            return false;
        };
        if month_digits[0] == 0 && month_digits[1] == 0 {
            return false;
        };
        true
    }

    fn correct_day(year_digits: &[u32],month_digits: &[u32], day_digits: &[u32]) -> bool {
        let month = Pesel::slice_to_month(month_digits);
        let day = day_digits[0] * 10 + day_digits[1];
        if !(1..=31).contains(&day) {
            return false;
        }
        let mut february_days = 28;
        if (year_digits[0] * 10 + year_digits[1]) % 4 == 0 {
            february_days = 29;
        }
        let days_in_month: [u32; 12] = [31, february_days, 29, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        day <= days_in_month[month as usize -1]
    }

    fn slice_to_month(month_digits: &[u32]) -> u32 {
        for prefix in Pesel::month_prefixes().iter() {
            let result = month_digits[0] as i32 - *prefix as i32;
            if (0..=1).contains(&result) {
                return result as u32 * 10 + month_digits[1];
            }
        };
        0
    }

    fn slice_to_year(year_digits: &[u32], month_digit: &u32) -> u32 {
        let mut year = 0;
        for prefix in Pesel::month_prefixes().iter() {
            if *month_digit == *prefix || *month_digit == *prefix+1 {
                break
            }
            year += 100;
        }
        1800 + year + year_digits[0] * 10 + year_digits[1]
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

pub enum PersonSex {
    Male,
    Female
}

pub fn header() -> Pesel {
    use std::io::stdin;

    let mut input_pesel = String::new();
    println!("================================");
    loop {
        println!("\x1b[1;34mPodaj numer PESEL:\x1b[0m");
        if stdin().read_line(&mut input_pesel).is_ok() {
            match Pesel::new(input_pesel.trim()) {
                Err(error_message) => {
                    println!("\x1b[0;31mBŁĄD: {}\x1b[0m", error_message);
                }
                Ok(pesel) => {
                    return pesel;
                }
            }
        }
        println!("--------------------------------");
        input_pesel = String::new();
    }
}

fn slice_to_num(slice: &[u32]) -> u32 {
    let digit_amount = slice.len();
    let mut number = 0;
    let mut power: u32;
    for (index, digit) in slice.iter().enumerate() {
        power = digit_amount as u32 -1 - index as u32;
        number += 10u32.pow(power) * *digit;
    };
    number
}

fn prepend_zero(num: u32) -> String {
    if num > 10 {
        num.to_string()
    } else {
        format!("0{}", num)
    }
}