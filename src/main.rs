use std::{num::{ParseIntError, ParseFloatError}};

mod money;

use money::*;

#[macro_use] extern crate failure;

#[derive(Debug, Fail)]
pub enum MoneyError {
    #[fail(display = "Invalid input: {}", _0)]
    ParseAmount(ParseFloatError),
    
    #[fail(display= "{}", _0)]
    ParseFormatting(String),

    #[fail(display = "{}", _0)]
    ParseCurrency(String)
}

impl From<ParseFloatError> for MoneyError {
    fn from(error: ParseFloatError) -> Self {
        MoneyError::ParseAmount(error)
    }
}

pub fn unidiomatic_parse_money(input: &str) -> (i32, String) {
    let parts: Vec<&str> = input.split_whitespace().collect();
    let maybe_amount = parts[0].parse::<i32>();

    // here we're handling the error manually
    if maybe_amount.is_err() {
        return (-1, "invalid".to_string());
    }

    let currency = parts[1].to_string();

    return (maybe_amount.unwrap(), currency);
}

pub fn more_idiomatic_parse_money(input: &str) -> (i32, String) {
    let parts: Vec<&str> = input.split_whitespace().collect();

    // we're now using the unwrap function instead of manually handling the error
    let maybe_amount = parts[0].parse::<i32>().unwrap();
    let currency = parts[1].to_string();

    return (maybe_amount, currency);
}

pub fn kind_of_idiomatic_parse_money(input :&str) -> Result<(i32, String), ParseIntError> {
    let parts: Vec<&str> = input.split_whitespace().collect();

    // we're now using question mark operator instead of using the unwrap function
    let maybe_amount = parts[0].parse::<i32>()?;
    let currency = parts[1].to_string();

    return Ok((maybe_amount, currency));
}

pub fn parse_money_with_custome_error_type(input: &str) -> Result<(f32, String), MoneyError> {
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.len() != 2 {
        Err(MoneyError::ParseFormatting(
            "Expecting amount and currency".into()
        ))
    } else {
        let (amount, currency) = (parts[0], parts[1]);
        Ok((amount.parse::<f32>()?, currency.to_string()))
    }
}

pub fn parse_money_with_slice_pattern(input: &str) -> Result<(f32, String), MoneyError> {
    let parts: Vec<&str> = input.split_whitespace().collect();
    match parts[..] {
        [amount, currency] => Ok((amount.parse::<f32>()?, currency.to_string())),
        _ => Err(MoneyError::ParseFormatting(
            "Expected amount and currency".into(),
        )),
    }
}

fn main() {
    let res = unidiomatic_parse_money("20 Euro");
    println!("{:?}", res);
    let res_2 = more_idiomatic_parse_money("45 Dollar");
    println!("{:?}", res_2);
    let res_3 = kind_of_idiomatic_parse_money("55.5 Dollar");
    println!("{:?}", res_3);
    let res_4 = parse_money_with_custome_error_type("Dollar");
    println!("{:?}", res_4);
    let res_5 = parse_money_with_slice_pattern("40 Euros");
    println!("{:?}", res_5);
    println!("{:?}", "56 €".parse::<Money>().unwrap());
    println!("{:?}", "OneMillion Bitcoin".parse::<Money>());
    println!("{:?}", "euro".parse::<Currency>());
}
