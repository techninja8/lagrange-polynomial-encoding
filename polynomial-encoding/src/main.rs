use std::fmt;

struct Polynomial {
    coefficients: Vec<f64>
}

impl fmt::Display for Polynomial {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Polynomial: ")?;

        let polynomial = &self.coefficients;

        for (i, coefficient) in polynomial.iter().enumerate() {
            if i > 0 {
                write!(f, " + ")?;
            }
            write!(f, "{}x^{}", coefficient, i)?;
        }

        Ok(())
    }
}

impl Polynomial {
    fn from_message(message: &str) -> Self {
        let bytes = message.as_bytes();
        let mut coefficients = Vec::new();

        for (i, bytes) in bytes.iter().enumerate() {
            coefficients.push(*bytes as f64);
        }

        Self {
            coefficients
        }
    }
}

fn main() {
    let message = "Hello, World!";

    let mut poly = Polynomial::from_message(message);

    println!("{}", poly);

}