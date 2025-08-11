use crate::RomanDigit::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RomanDigit {
    Nulla,
    I,
    V,
    X,
    L,
    C,
    D,
    M,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RomanNumber(pub Vec<RomanDigit>);

impl From<u32> for RomanDigit {
    fn from(value: u32) -> Self {
        match value {
            0 => Nulla,
            1 => I,
            5 => V,
            10 => X,
            50 => L,
            100 => C,
            500 => D,
            1000 => M,
            _ => panic!("Invalid value for RomanDigit"),
        }
    }
}

impl From<u32> for RomanNumber {
    fn from(mut num: u32) -> Self {
        if num == 0 {
            return RomanNumber(vec![Nulla]);
        }

        let values = [
            (1000, M),
            (900, C),
            (500, D),
            (400, C),
            (100, C),
            (90, X),
            (50, L),
            (40, X),
            (10, X),
            (9, I),
            (5, V),
            (4, I),
            (1, I),
        ];

        let mut digits = Vec::new();

        for &(val, digit) in &values {
            while num >= val {
                if val == 900 {
                    digits.push(C);
                    digits.push(M);
                } else if val == 400 {
                    digits.push(C);
                    digits.push(D);
                } else if val == 90 {
                    digits.push(X);
                    digits.push(C);
                } else if val == 40 {
                    digits.push(X);
                    digits.push(L);
                } else if val == 9 {
                    digits.push(I);
                    digits.push(X);
                } else if val == 4 {
                    digits.push(I);
                    digits.push(V);
                } else {
                    digits.push(digit);
                }
                num -= val;
            }
        }

        RomanNumber(digits)
    }
}
