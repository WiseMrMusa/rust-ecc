use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, BitXor, Mul};
use super::traits::{ IsPrime, is_prime};

#[derive(Debug, Copy, Clone)]
pub struct Field {
    pub a: u64,
    p: u64
}

pub trait FieldTrait {
    fn new(a: u64, p: u64) -> Self;
    fn inv(self) -> Self;
}

pub trait Pow<RHS> {
    type Output;

    // Required method
    fn pow(self, rhs: RHS) -> Self::Output;
}

impl FieldTrait for Field {
    fn new(a: u64, p: u64) -> Field {
        assert!(a < p, "Not an element of the prime field");
        Field {a,p}
    }

    fn inv(self) -> Self{
        self ^ (self.p as usize - 2)
    }
}



impl Display for Field {
    fn fmt(&self, f:&mut Formatter<'_>) -> Result {
        write!(f,"{} F[{}]", self.a, self.p)
    }
}

impl IsPrime for Field {
    fn is_prime(&self) -> bool {
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

impl PartialEq for Field{
    fn eq(&self, other: &Self) -> bool {
        assert_eq!(self.p,other.p, "You can only compare between same field");
        self.a == other.a
    }
}

impl Eq for Field {}

impl BitXor<usize> for Field{
    type Output = Field;
    fn bitxor(self, rhs: usize) -> Field {
        let n = if rhs > self.p as usize {
            rhs % self.p as usize
        } else {
            rhs
        };
        let mut res =self;
        for _ in 2..=n{
            res = res * self;
        } 
        res
    }
}


pub fn main() {
    println!("A main function from the finite field village");
    let a = Field::new(3, 5);
    let b = 5;
    // let b = Field::new(2, 7);
    let c = a^b;
    println!("{a}^{b} = {c}");

    println!("INVERSE ELEMENT");
    let a = Field::new(436,563);
    let b = a.inv();
    println!("The inverse of {a} is {b}");
    // let zero = Field::new(0,13);
    // let any_number = Field::new(2,7);
    // let identity = any_number + zero == any_number;
    // println!("Zero is an identity: {identity}");
}