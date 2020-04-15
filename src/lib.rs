//! # CNPJ util
//!
//! `cnpj util` is a library focused on solving a common problems
//! that we face daily in the development of applications using 
//! CNPJ (Brazil companies ID number).

use std::cmp;
use std::cmp::Ordering;

const CNPJ_LENGTH: usize = 14;

fn get_separator(x: usize) -> &'static str {
    match x {
        2 | 5 => ".",
        8 => "/",
        12 => "-",
        _ => ""
    }
}

/// Format string with CNPJ mask.
///
/// # Examples
///
/// ```
/// let cnpj_without_mask = "46843485000186";
/// let cnpj_with_mask = cnpj_util::format(cnpj_without_mask);
///
/// assert_eq!("46.843.485/0001-86", cnpj_with_mask);
/// ```
///
/// ```
/// let cnpj_without_mask = "468434850001860000000000";
/// let cnpj_with_mask = cnpj_util::format(cnpj_without_mask);
///
/// assert_eq!("46.843.485/0001-86", cnpj_with_mask);
/// ```
///
/// ```
/// let cnpj_without_mask = "46.?ABC843.485/0001-86abc";
/// let cnpj_with_mask = cnpj_util::format(cnpj_without_mask);
///
/// assert_eq!("46.843.485/0001-86", cnpj_with_mask);
/// ```
pub fn format(cnpj: &str) -> String {
    let cnpj = cnpj.matches(char::is_numeric).collect::<Vec<_>>();

    let mut cnpj_with_mask: String = String::from("");

    for x in 0..cmp::min(cnpj.len(), 14) {
        cnpj_with_mask.push_str(get_separator(x));
        cnpj_with_mask.push_str(cnpj[x]);
    }

    cnpj_with_mask
}

pub fn reserved_numbers() -> Vec<String> {
    vec![
        String::from("00000000000000"),
        String::from("11111111111111"),
        String::from("22222222222222"),
        String::from("33333333333333"),
        String::from("44444444444444"),
        String::from("55555555555555"),
        String::from("66666666666666"),
        String::from("77777777777777"),
        String::from("88888888888888"),
        String::from("99999999999999"),
    ]
}


fn check_sum(cnpj: &Vec<&str>, factors: Vec<u32>) -> u32 {
    let mut sum: u32 = 0;
    for x in 0..factors.len() {
        sum = sum + cnpj[x].parse::<u32>().unwrap() * factors[x];
    }

    let mod_11 = sum % 11;
    
    match mod_11.cmp(&2) {
        Ordering::Less => 0,
        _ => 11 - mod_11
    }
}

fn validate(cnpj: String) -> bool {
    let cnpj = cnpj.matches(char::is_numeric).collect::<Vec<_>>();
    
    let factors = vec![5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
    let digito_1 = check_sum(&cnpj, factors);
    
    let factors = vec![6, 5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
    let digito_2 = check_sum(&cnpj, factors);

    digito_1 == cnpj[CNPJ_LENGTH - 2].parse::<u32>().unwrap()
        && digito_2 == cnpj[CNPJ_LENGTH - 1].parse::<u32>().unwrap()
}

pub fn is_valid(cnpj: &str) -> bool {
    if cnpj.len() != CNPJ_LENGTH {
        return false;
    }

    let cnpj = cnpj.matches(char::is_numeric).collect::<Vec<_>>().concat();
    
    !reserved_numbers().contains(&cnpj)
        && !cnpj.is_empty()
        && validate(cnpj)
}

#[cfg(test)]
mod test_is_valid {
    use super::*;

    #[test]
    fn should_return_false_when_it_is_on_reserved_numbers() {
        for reserved_number in reserved_numbers() {
            assert_eq!(is_valid(&reserved_number), false);
        }
    }

    #[test]
    fn should_return_false_when_is_a_empty_string() {
        assert_eq!(is_valid(""), false);
    }

    #[test]
    fn should_return_false_when_dont_match_with_cnpj_length() {
        assert_eq!(is_valid("12312312312"), false);
    }

    #[test]
    fn should_return_false_when_contains_only_letters_or_special_characters() {
        assert_eq!(is_valid("ababcabcabcdab"), false);
    }

    #[test]
    fn should_return_false_when_is_a_cnpj_invalid_test_numbers_with_letters() {
        assert_eq!(is_valid("6ad0.t391.9asd47/0ad001-00"), false);
    }

    #[test]
    fn should_return_false_when_is_a_cnpj_invalid() {
        assert_eq!(is_valid("11257245286531"), false);
    }
}

#[cfg(test)]
mod test_format {
    use super::*;

    #[test]
    fn should_format_cnpj_with_mask() {
        assert_eq!(format(""), "");
        assert_eq!(format("4"), "4");
        assert_eq!(format("46"), "46");
        assert_eq!(format("468"), "46.8");
        assert_eq!(format("4684"), "46.84");
        assert_eq!(format("46843"), "46.843");
        assert_eq!(format("468434"), "46.843.4");
        assert_eq!(format("4684348"), "46.843.48");
        assert_eq!(format("46843485"), "46.843.485");
        assert_eq!(format("468434850"), "46.843.485/0");
        assert_eq!(format("4684348500"), "46.843.485/00");
        assert_eq!(format("46843485000"), "46.843.485/000");
        assert_eq!(format("468434850001"), "46.843.485/0001");
        assert_eq!(format("4684348500018"), "46.843.485/0001-8");
        assert_eq!(format("46843485000186"), "46.843.485/0001-86");
    }

    #[test]
    fn should_not_add_digits_after_the_cnpj_length() {
        assert_eq!(format("468434850001860000000000"), "46.843.485/0001-86");
    }

    #[test]
    fn should_remove_all_non_numeric_characters() {
        assert_eq!(format("46.?ABC843.485/0001-86abc"), "46.843.485/0001-86");
    }
}
