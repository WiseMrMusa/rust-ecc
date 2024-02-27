use std::fmt::{Display,Formatter};

use super::ecc_def::EllipticCurve;
use crate::{elliptic_curve::short_weierstrass::ecc_def::EllipticCurveTraits, finite_field::field::Field};


#[derive(Debug, Clone, Copy)]
pub struct EllipticPoint{
    pub x: Field,
    pub y: Field,
    curve: EllipticCurve
}

impl EllipticPoint {
    pub fn new(x: Field, y: Field, curve: EllipticCurve) -> Self {
        assert!(curve.satisfy_curve(x, y),"Not a point");
        EllipticPoint { x, y, curve }
    }
}

impl Display for EllipticPoint{
    fn fmt(&self,f: &mut Formatter) -> Result<(), std::fmt::Error>{
        write!(f,"({},{}) is a point in {}",self.x.a, self.y.a , self.curve)
    }
}
