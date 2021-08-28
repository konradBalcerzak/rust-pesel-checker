#[derive(Debug)]
pub struct DigitVec(
    Vec<u32>
);

impl DigitVec {
    pub fn new(candidate: &str) -> Result<DigitVec, &str> {
        let mut pesel_digits = DigitVec(Vec::new());

        for digit_opt in candidate.chars().map(|character: char| character.to_digit(10)) {
            let digit = digit_opt
                .ok_or("Podany pesel zawiera inne znaki niż cyfry")?;
            pesel_digits.0.push(digit);
        }
        Ok(pesel_digits)
    }

    pub fn get(&self) -> &[u32] {
        &self.0
    }
}