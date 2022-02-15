

use crate::MoneyError;

#[derive(Debug)]
#[warn(dead_code)]
pub struct Money {
    amount: f32, 
    currency: Currency
}

impl Money {
    fn new(amount: f32, currency: Currency) -> Self {
        Money {amount, currency}
    }
}

impl std::str::FromStr for Money {
    type Err = MoneyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        match parts[..] {
            [amount, currency] => Ok(Money::new(amount.parse()?, currency.parse()?)),
            _ => Err(MoneyError::ParseFormatting(
                "Expected amount and currency".into(),
            )),
        }
    }
}

#[derive(Debug)]
pub enum Currency {
    Dollar, 
    Euro
}

impl std::str::FromStr for Currency {
    type Err = MoneyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "dollar" | "$" => Ok(Currency::Dollar),
            "euro" | "eur" | "€" => Ok(Currency::Euro),
            _ => Err(MoneyError::ParseCurrency("Unknown currency".into()))
        }
    }
}