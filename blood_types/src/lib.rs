use std::cmp::{Ord, Ordering};
use std::fmt::{self, Debug};
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

impl FromStr for Antigen {}

impl FromStr for RhFactor {}

impl Ord for BloodType {}

impl FromStr for BloodType {}

impl Debug for BloodType {
    
}

impl BloodType {
    pub fn can_receive_from(&self, other: &Self) -> bool {}

    pub fn donors(&self) -> Vec<Self> {}

    pub fn recipients(&self) -> Vec<BloodType> {}
}
