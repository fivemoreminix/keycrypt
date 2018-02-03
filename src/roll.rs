use rayon::prelude::*;

#[inline(always)]
pub fn add_one(c: char) -> char {
    match c {
        '0' => '1',
        '1' => '2',
        '2' => '3',
        '3' => '4',
        '4' => '5',
        '5' => '6',
        '6' => '7',
        '7' => '8',
        '8' => '9',
        '9' => '0',
        _ => panic!("Read an invalid character while rolling."),
    }
}

#[inline(always)]
pub fn sub_one(c: char) -> char {
    match c {
        '0' => '9',
        '1' => '0',
        '2' => '1',
        '3' => '2',
        '4' => '3',
        '5' => '4',
        '6' => '5',
        '7' => '6',
        '8' => '7',
        '9' => '8',
        _ => panic!("Read an invalid character while rolling."),
    }
}

#[inline(always)]
pub fn roll_digit(digit: char, roll: u32) -> char {
    assert!(roll != 0);
    let mut output = digit;
    for _ in 0..roll {
        output = add_one(output);
    }
    output
}

#[inline(always)]
pub fn unroll_digit(digit: char, roll: u32) -> char {
    assert!(roll != 0);
    let mut output = digit;
    for _ in 0..roll {
        output = sub_one(output);
    }
    output
}

/*
pub fn roll_crypt(crypt: String, roll: u32) -> String {
    crypt.par_chars().map(|c| roll_digit(c, roll)).collect()
}
*/

pub fn unroll_crypt(crypt: String, roll: u32) -> String {
    crypt.par_chars().map(|c| unroll_digit(c, roll)).collect()
}

#[inline(always)]
pub fn generate_key() -> u32 {
    ::rand::random::<u32>()
}
