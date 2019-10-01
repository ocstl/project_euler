use std::convert::TryFrom;
use std::fmt::{Display, Formatter};

pub struct Roman(String);

impl Roman {
    pub fn new(s: &str) -> Self {
        Roman(s.to_string())
    }

    fn units(num: u32) -> &'static str {
        match num % 10 {
            0 => "",
            1 => "I",
            2 => "II",
            3 => "III",
            4 => "IV",
            5 => "V",
            6 => "VI",
            7 => "VII",
            8 => "VIII",
            9 => "IX",
            _ => unreachable!(),
        }
    }

    fn tens(num: u32) -> &'static str {
        match (num % 100) / 10 {
            0 => "",
            1 => "X",
            2 => "XX",
            3 => "XXX",
            4 => "XL",
            5 => "L",
            6 => "LX",
            7 => "LXX",
            8 => "LXXX",
            9 => "XC",
            _ => unreachable!(),
        }
    }

    fn hundreds(num: u32) -> &'static str {
        match (num % 1000) / 100 {
            0 => "",
            1 => "C",
            2 => "CC",
            3 => "CCC",
            4 => "CD",
            5 => "D",
            6 => "DC",
            7 => "DCC",
            8 => "DCCC",
            9 => "CM",
            _ => unreachable!(),
        }
    }

    fn thousands(num: u32) -> String {
        std::iter::repeat("M")
            .take(num as usize / 1000)
            .collect::<String>()
    }
}

impl Display for Roman {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<u32> for Roman {
    fn from(num: u32) -> Self {
        Roman(
            [
                Roman::thousands(num).as_str(),
                Roman::hundreds(num),
                Roman::tens(num),
                Roman::units(num),
            ]
            .concat(),
        )
    }
}

impl TryFrom<&Roman> for u32 {
    type Error = &'static str;

    fn try_from(s: &Roman) -> Result<Self, Self::Error> {
        let mut previous = RomanNumerals::I;
        let mut n: u32 = 0;

        for c in s.0.chars().rev() {
            let current = RomanNumerals::try_from(c)?;
            if current < previous {
                n -= u32::from(current);
            } else {
                n += u32::from(current);
            }
            previous = current;
        }

        Ok(n)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum RomanNumerals {
    I,
    V,
    X,
    L,
    C,
    D,
    M,
}

impl TryFrom<char> for RomanNumerals {
    type Error = &'static str;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'I' => Ok(RomanNumerals::I),
            'V' => Ok(RomanNumerals::V),
            'X' => Ok(RomanNumerals::X),
            'L' => Ok(RomanNumerals::L),
            'C' => Ok(RomanNumerals::C),
            'D' => Ok(RomanNumerals::D),
            'M' => Ok(RomanNumerals::M),
            _ => Err("Unrecognized symbol."),
        }
    }
}

impl From<RomanNumerals> for u32 {
    fn from(r: RomanNumerals) -> u32 {
        match r {
            RomanNumerals::I => 1,
            RomanNumerals::V => 5,
            RomanNumerals::X => 10,
            RomanNumerals::L => 50,
            RomanNumerals::C => 100,
            RomanNumerals::D => 500,
            RomanNumerals::M => 1000,
        }
    }
}
