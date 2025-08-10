use std::cmp::Ordering;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Antigen {
    A,
    AB,
    B,
    O,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
enum RhFactor {
    Positive,
    Negative,
}

#[derive(PartialEq, Eq, PartialOrd)]
pub struct BloodType {
    pub antigen: Antigen,
    pub rh_factor: RhFactor,
}

impl FromStr for Antigen {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Antigen::A),
            "B" => Ok(Antigen::B),
            "AB" => Ok(Antigen::AB),
            "O" => Ok(Antigen::O),
            _ => Err(format!("Invalid antigen: {}", s)),
        }
    }
}

impl FromStr for RhFactor {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(RhFactor::Positive),
            "-" => Ok(RhFactor::Negative),
            _ => Err(format!("Invalid Rh factor: {}", s)),
        }
    }
}

impl Ord for BloodType {
    fn cmp(&self, other: &Self) -> Ordering {
        self.antigen
            .cmp(&other.antigen)
            .then(self.rh_factor.cmp(&other.rh_factor))
    }
}

impl FromStr for BloodType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (antigen_str, rh_str) = if s.ends_with('+') {
            (&s[..s.len() - 1], "+")
        } else if s.ends_with('-') {
            (&s[..s.len() - 1], "-")
        } else {
            return Err(format!("Invalid blood type: {}", s));
        };

        Ok(BloodType {
            antigen: antigen_str.parse()?,
            rh_factor: rh_str.parse()?,
        })
    }
}

impl fmt::Debug for BloodType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let antigen_str = match self.antigen {
            Antigen::A => "A",
            Antigen::B => "B",
            Antigen::AB => "AB",
            Antigen::O => "O",
        };
        let rh_str = match self.rh_factor {
            RhFactor::Positive => "+",
            RhFactor::Negative => "-",
        };
        write!(f, "{}{}", antigen_str, rh_str)
    }
}

impl BloodType {
    pub fn can_receive_from(&self, other: &Self) -> bool {
        let antigen_ok = match self.antigen {
            Antigen::A => matches!(other.antigen, Antigen::A | Antigen::O),
            Antigen::B => matches!(other.antigen, Antigen::B | Antigen::O),
            Antigen::AB => true,
            Antigen::O => matches!(other.antigen, Antigen::O),
        };

        let rh_ok = match self.rh_factor {
            RhFactor::Positive => true,
            RhFactor::Negative => matches!(other.rh_factor, RhFactor::Negative),
        };

        antigen_ok && rh_ok
    }

    pub fn donors(&self) -> Vec<Self> {
        Self::all_types()
            .into_iter()
            .filter(|t| self.can_receive_from(t))
            .collect()
    }

    pub fn recipients(&self) -> Vec<BloodType> {
        Self::all_types()
            .into_iter()
            .filter(|t| t.can_receive_from(self))
            .collect()
    }

    fn all_types() -> Vec<Self> {
        let antigens = vec![Antigen::A, Antigen::B, Antigen::AB, Antigen::O];
        let rh_factors = vec![RhFactor::Positive, RhFactor::Negative];
        let mut result = Vec::new();
        for a in &antigens {
            for r in &rh_factors {
                result.push(BloodType {
                    antigen: a.clone(),
                    rh_factor: r.clone(),
                });
            }
        }
        result
    }
}
