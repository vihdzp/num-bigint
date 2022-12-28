use xmath_traits::*;

use crate::{BigInt, BigUint};

impl Zero for BigUint {
    fn zero() -> Self {
        num_traits::Zero::zero()
    }

    fn is_zero(&self) -> bool {
        num_traits::Zero::is_zero(self)
    }
}

impl One for BigUint {
    fn one() -> Self {
        num_traits::One::one()
    }

    fn is_one(&self) -> bool {
        num_traits::One::is_one(self)
    }
}

impl ZeroNeOne for BigUint {}

impl Add for BigUint {
    fn add(&self, rhs: &Self) -> Self {
        self + rhs
    }

    fn add_mut(&mut self, rhs: &Self) {
        *self += rhs;
    }

    fn add_rhs_mut(&self, rhs: &mut Self) {
        *rhs += self;
    }

    fn double(&self) -> Self {
        self << 1
    }

    fn double_mut(&mut self) {
        *self <<= 1;
    }
}

impl AddMonoid for BigUint {}
impl CommAdd for BigUint {}

impl Mul for BigUint {
    fn mul(&self, rhs: &Self) -> Self {
        self * rhs
    }

    fn mul_mut(&mut self, rhs: &Self) {
        *self *= rhs;
    }

    fn mul_rhs_mut(&self, rhs: &mut Self) {
        *rhs *= self;
    }
}

impl MulMonoid for BigUint {}
impl CommMul for BigUint {}

impl Zero for BigInt {
    fn zero() -> Self {
        num_traits::Zero::zero()
    }

    fn is_zero(&self) -> bool {
        num_traits::Zero::is_zero(self)
    }
}

impl One for BigInt {
    fn one() -> Self {
        num_traits::One::one()
    }

    fn is_one(&self) -> bool {
        num_traits::One::is_one(self)
    }
}

impl ZeroNeOne for BigInt {}

impl Neg for BigInt {
    fn neg(&self) -> Self {
        -self
    }

    fn neg_mut(&mut self) {
        self.neg_mut();
    }
}

impl Add for BigInt {
    fn add(&self, rhs: &Self) -> Self {
        self + rhs
    }

    fn add_mut(&mut self, rhs: &Self) {
        *self += rhs;
    }

    fn add_rhs_mut(&self, rhs: &mut Self) {
        *rhs += self;
    }
}

impl Sub for BigInt {
    fn sub(&self, rhs: &Self) -> Self {
        self - rhs
    }

    fn sub_mut(&mut self, rhs: &Self) {
        *self -= rhs;
    }

    fn sub_rhs_mut(&self, rhs: &mut Self) {
        rhs.neg_mut();
        *rhs += self;
    }
}

impl Mul for BigInt {
    fn mul(&self, x: &Self) -> Self {
        self * x
    }

    fn mul_mut(&mut self, rhs: &Self) {
        *self *= rhs;
    }

    fn mul_rhs_mut(&self, rhs: &mut Self) {
        *rhs *= self;
    }
}
