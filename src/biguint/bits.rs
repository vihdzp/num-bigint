use super::{BigUint, IntDigits};

use core::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign};

forward_val_val_binop!(impl BitAnd for BigUint, bitand);
forward_ref_val_binop!(impl BitAnd for BigUint, bitand);

// do not use forward_ref_ref_binop_commutative! for bitand so that we can
// clone the smaller value rather than the larger, avoiding over-allocation
impl<'a, 'b> BitAnd<&'b BigUint> for &'a BigUint {
    type Output = BigUint;

    #[inline]
    fn bitand(self, other: &BigUint) -> BigUint {
        // forward to val-ref, choosing the smaller to clone
        if self.data.len() <= other.data.len() {
            self.clone() & other
        } else {
            other.clone() & self
        }
    }
}

forward_val_assign!(impl BitAndAssign for BigUint, bitand_assign);

impl<'a> BitAnd<&'a BigUint> for BigUint {
    type Output = BigUint;

    #[inline]
    fn bitand(mut self, other: &BigUint) -> BigUint {
        self &= other;
        self
    }
}
impl<'a> BitAndAssign<&'a BigUint> for BigUint {
    #[inline]
    fn bitand_assign(&mut self, other: &BigUint) {
        self.data.and_trim(|vec| {
            for (ai, &bi) in vec.iter_mut().zip(other.data.iter()) {
                *ai &= bi;
            }
            vec.truncate(other.data.len());
        });
    }
}

forward_all_binop_to_val_ref_commutative!(impl BitOr for BigUint, bitor);
forward_val_assign!(impl BitOrAssign for BigUint, bitor_assign);

impl<'a> BitOr<&'a BigUint> for BigUint {
    type Output = BigUint;

    fn bitor(mut self, other: &BigUint) -> BigUint {
        self |= other;
        self
    }
}
impl<'a> BitOrAssign<&'a BigUint> for BigUint {
    #[inline]
    fn bitor_assign(&mut self, other: &BigUint) {
        self.data.and_trim(|vec| {
            for (ai, &bi) in vec.iter_mut().zip(other.data.iter()) {
                *ai |= bi;
            }
            if other.data.len() > vec.len() {
                let extra = &other.data[vec.len()..];
                vec.extend(extra.iter().cloned());
            }
        });
    }
}

forward_all_binop_to_val_ref_commutative!(impl BitXor for BigUint, bitxor);
forward_val_assign!(impl BitXorAssign for BigUint, bitxor_assign);

impl<'a> BitXor<&'a BigUint> for BigUint {
    type Output = BigUint;

    fn bitxor(mut self, other: &BigUint) -> BigUint {
        self ^= other;
        self
    }
}
impl<'a> BitXorAssign<&'a BigUint> for BigUint {
    #[inline]
    fn bitxor_assign(&mut self, other: &BigUint) {
        self.data.and_trim(|vec| {
            for (ai, &bi) in vec.iter_mut().zip(other.data.iter()) {
                *ai ^= bi;
            }
            if other.data.len() > vec.len() {
                let extra = &other.data[vec.len()..];
                vec.extend(extra.iter().cloned());
            }
        });
    }
}
