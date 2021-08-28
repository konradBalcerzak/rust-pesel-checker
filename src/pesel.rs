use std::fmt;
use chrono::NaiveDate;
use crate::digit_vec::DigitVec;

#[derive(Debug)]
pub enum PersonGender {
    Male,
    Female
}

#[derive(Debug)]
pub struct Pesel {
    value: DigitVec,
    date: NaiveDate,
    gender: PersonGender
}

impl Pesel {
    fn month_prefixes() -> [u32; 5] {
        [8, 0, 2, 4, 6]
    }
    
    pub fn new(input: &str) -> Result<Pesel, String> {
        let value = DigitVec::new(input)?;
        let digits = value.get();
        Pesel::check_correctness(digits)?;
        let (year, month, day) = Pesel::extract_ymd(value.get());

        let date = NaiveDate::from_ymd(year, month, day);
        let gender = match digits[9] {
            x if x % 2 == 0 => PersonGender::Female,
            _ => PersonGender::Male, 
        };

        Ok(Pesel {
            value,
            date,
            gender,
        })
    }

    pub fn get_birthday(&self) -> NaiveDate {
        self.date
    }

    pub fn get_gender(&self) -> PersonGender {
        let digits = self.value.get();
        if digits[9] % 2 == 0 {
            PersonGender::Female
        } else {
            PersonGender::Male
        }
    }
    
    fn check_correctness(candidate_pesel: &[u32]) -> Result<(), String> {
        match candidate_pesel.len() {
            x if x < 11 => return Err(String::from("Za mało znaków")),
            x if x > 11 => return Err(String::from("Za dużo znaków")),
            _ => ()
        };
        
        if !Pesel::correct_month(&candidate_pesel[2..4]) {
            return Err(String::from("Podany miesiąc jest nieprawidłowy"));
        }
        if !Pesel::correct_day(candidate_pesel) {
            return Err(String::from("Podany dzień jest nieprawidłowy"));
        }

        const MULTIPLIERS: [u32; 11] = [1, 3, 7, 9, 1, 3, 7, 9, 1, 3, 1];
        let mut sum = 0;

        for (iteration, (pesel_digit, multiplier)) in candidate_pesel.iter().zip(MULTIPLIERS).enumerate() {
            if iteration == 10 {
                break;
            }
            sum += pesel_digit * multiplier;
        }
        sum %= 10;
        if let Some(control_digit) = candidate_pesel.last() {
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
        if (first_month_digit == 1 && month_digits[1] > 2) || (month_digits[0] == 0 && month_digits[1] == 0) {
            return false;
        };
        true
    }
    fn correct_day(candidate_pesel: &[u32]) -> bool {
        let month = Pesel::extract_month(candidate_pesel);
        let day = candidate_pesel[0] * 10 + candidate_pesel[1];

        let mut february_days = 28;
        if (candidate_pesel[0] * 10 + candidate_pesel[1]) % 4 == 0 {
            february_days = 29;
        }
        let days_in_month: [u32; 12] = [31, february_days, 29, 30, 31, 30, 31, 31, 30, 31, 30, 31];

        day <= days_in_month[month as i32 as usize -1]
    }

    fn extract_ymd(pesel_digits: &[u32]) -> (i32, u32, u32) {
        (Pesel::extract_year(pesel_digits), Pesel::extract_month(pesel_digits), Pesel::extract_day(pesel_digits))
    }
    fn extract_year(pesel_digits: &[u32]) -> i32 {
        let mut year = 0;
        for prefix in Pesel::month_prefixes() {
            if pesel_digits[2] == prefix || pesel_digits[2] == prefix+1 {
                break
            }
            year += 100;
        }
        (1800 + year + pesel_digits[0] * 10 + pesel_digits[1]) as i32
    }
    fn extract_month(pesel_digits: &[u32]) -> u32 {
        for prefix in Pesel::month_prefixes().iter() {
            let result = pesel_digits[2] as i32 - *prefix as i32;
            if (0..2).contains(&result) {
                return result as u32 * 10 + pesel_digits[3];
            }
        };
        0
    }
    fn extract_day(pesel_digits: &[u32]) -> u32 {
        pesel_digits[4] * 10 + pesel_digits[5]
    }
}

impl fmt::Display for Pesel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>)->fmt::Result {
        let mut pesel_fmt = String::new();
        for digit in self.value.get() {
            if let Some(char) = char::from_digit(*digit, 10) {
                pesel_fmt.push(char);
            }
        }
        write!(f, "{}", pesel_fmt)
    }
}