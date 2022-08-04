use std::iter::successors;

const ONES: [&str; 20] = [
    "zero",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "foureen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
];

const TENS: [&str; 10] = [
    "zero", "ten", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
];
const ORDERS: [&str; 7] = [
    "zero",
    "thousand",
    "million",
    "billion",
    "trillion",
    "quadrillion",
    "quintillion",
];

/// Converts a [`u64`] to a word representation,
/// e.g 1,234 -> one thousand two hundred thiry four
///
/// Can handle up to [`u64::MAX`] because why not?
pub fn num_to_words(val: u64) -> String {
    match val {
        0..=19 => ONES[val as usize].to_string(),
        20..=99 => {
            let tens = (val / 10) as usize;
            match val % 10 {
                0 => TENS[tens].to_string(),
                ones => format!("{} {}", TENS[tens], num_to_words(ones)),
            }
        }
        100..=999 => format_num(val, 100, "hundred"),
        _ => {
            let (div, order) = successors(Some(1u64), |v| v.checked_mul(1000))
                .zip(ORDERS.iter())
                .find(|&(e, _)| e > val / 1000)
                .unwrap();

            format_num(val, div, order)
        }
    }
}

/// determines if the word string should be just
/// "one hundred" or "one hundred one" depending on the dividing value
fn format_num(num: u64, div: u64, order: &str) -> String {
    match (num / div, num % div) {
        (upper, 0) => format!("{} {}", num_to_words(upper), order),
        (upper, lower) => {
            format!("{} {} {}", num_to_words(upper), order, num_to_words(lower))
        }
    }
}

fn main() {
    for i in 0..=100 {
        if i % 10 == 0 {
            println!("{}", num_to_words(i));
        } else {
            println!("{}", i);
        }
    }
}
