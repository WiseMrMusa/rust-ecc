use std::fmt::{Display, Formatter, Result};
use std::ops::{Add,Mul, Div, Sub};
use super::traits::{isPrime, is_prime};


#[derive(Debug, Copy, Clone)]
pub struct Field {
    pub a: u64,
    p: u64
}

pub trait FieldTrait {
    fn new(a: u64, p: u64) -> Self;
}

impl FieldTrait for Field {
    fn new(a: u64, p: u64) -> Field {
        assert!(a < p, "Not an element of the prime field");
        Field {a,p}
    }
}

impl Default for Field {
    fn default() -> Self{
        Field{a:5,p:8}
    }
}

impl Display for Field {
    fn fmt(&self, f:&mut Formatter<'_>) -> Result {
        write!(f,"{} F[{}]", self.a, self.p)
    }
}

impl isPrime for Field {
    fn isPrime(&self) -> bool {
        is_prime(self.p, 12)
    }
}

impl Add for Field {
    type Output = Self;
    fn add(self, other: Field) -> Self {
        if other.p != self.p{
            panic!("Add Operation should be within the field");
        }
        Field{
            a: (self.a + other.a) % self.p,
            p: self.p
        }
    }
}

impl Mul for Field {
    type Output = Self;
    fn mul(self, other: Field) -> Self {
        if other.p != self.p {
            panic!("Mul Operation should be within the field");
        }
        Field{
            a: (self.a * other.a) % self.p,
            p: self.p
        }
    }
}