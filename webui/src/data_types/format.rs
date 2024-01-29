use crate::prelude::*;

/// Format Decimal with commas and digit limit
///
/// Digit limit will perform rounding based ono Decimal::round_dp rounding (Bankers Rounding).
///
/// Example
/// ```rust
/// use webui::prelude::*;
///
/// let number = Decimal::from_u32(1000).unwrap();
/// assert_eq!("1,000.00", format_decimal(number,2));
/// let number = Decimal::from_f64_retain(10000000.23984).unwrap();
/// assert_eq!("10,000,000.24", format_decimal(number,2));
/// assert_eq!("10,000,000", format_decimal(number,0));
/// assert_eq!("10,000,000.23984", format_decimal(number,5));
/// assert_eq!("10,000,000.2398400", format_decimal(number,7));
/// let number = Decimal::from_f32(200000.502f32).unwrap();
/// assert_eq!("200,000.50", format_decimal(number,2));
/// ```
pub fn format_decimal(number: Decimal, digits: u32) -> String {
    let value = &mut format!("{}", number.round_dp(digits));
    let split = &mut value.split('.');
    let left = &mut String::new();
    if let Some(value) = split.next() {
        let count = &mut (value.len() % 3);
        let comma_next = &mut false;
        for ch in value.chars() {
            if *comma_next {
                left.push(',');
                *comma_next = false;
            }
            left.push(ch);
            if *count == 1 {
                *comma_next = true;
                *count = 0;
            } else if *count == 0 {
                *count = 2;
            } else {
                *count -= 1;
            }
        }
    };
    if left.is_empty() {
        left.push('0');
    }
    if digits == 0 {
        return left.to_string();
    }
    let right = &mut String::new();
    if let Some(value) = split.next() {
        right.push_str(value);
    }
    while right.len() < digits.try_into().unwrap() {
        right.push('0');
    }
    format!("{}.{}", left, right)
}

/// Format number with commas and digit limit
///
/// Digit limit will trim off excess digits without any rounding
///
/// Example
/// ```rust
/// use webui::prelude::*;
///
/// let number = 1000f32;
/// assert_eq!("1,000.00", format_number(number,2));
/// let number = 10000000.23984f64;
/// assert_eq!("10,000,000.23", format_number(number,2));
/// assert_eq!("10,000,000", format_number(number,0));
/// assert_eq!("10,000,000.23984", format_number(number,5));
/// assert_eq!("10,000,000.2398400", format_number(number,7));
/// let number = 200000.502f32;
/// assert_eq!("200,000.50", format_number(number,2));
/// ```
pub fn format_number<T>(number: T, digits: u32) -> String
where
    T: std::fmt::Display,
{
    let value = &mut format!("{}", number);
    println!("{}  {}", value, digits);
    let split = &mut value.split('.');
    let left = &mut String::new();
    if let Some(value) = split.next() {
        let count = &mut (value.len() % 3);
        let comma_next = &mut false;
        for ch in value.chars() {
            if *comma_next {
                left.push(',');
                *comma_next = false;
            }
            left.push(ch);
            if *count == 1 {
                *comma_next = true;
                *count = 0;
            } else if *count == 0 {
                *count = 2;
            } else {
                *count -= 1;
            }
        }
    };
    if left.is_empty() {
        left.push('0');
    }
    if digits == 0 {
        return left.to_string();
    }
    let right = &mut String::new();
    let udigits: usize = digits.try_into().unwrap();
    if let Some(value) = split.next() {
        for ch in value.chars() {
            if right.len() == udigits {
                break;
            }
            right.push(ch);
        }
    };
    while right.len() < udigits {
        right.push('0');
    }
    format!("{}.{}", left, right)
}
