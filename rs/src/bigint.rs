use std::fmt;
use std::marker::PhantomData;
use num::traits::Zero;
use std::ops::Add;
use std::cmp::{ Eq, PartialEq };

#[derive(Debug, Eq, PartialEq)]
pub struct BigInt<const NUM_LIMBS: usize, const LOG_LIMB_SIZE: u32> {
    pub limbs: [u32; NUM_LIMBS],
    pub log_limb_size: u32,
    _phantom: PhantomData<u32>,
}

#[derive(Debug, Clone)]
pub struct LimbSizeError;

impl fmt::Display for LimbSizeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid limb size")
    }
}

impl<const NUM_LIMBS: usize, const LOG_LIMB_SIZE: u32> Add for BigInt<NUM_LIMBS, LOG_LIMB_SIZE> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        // NOTE: this will overflow when the inputs are large enough!
        let mask = 2u32.pow(LOG_LIMB_SIZE) - 1u32;
        let mut res = [0u32; NUM_LIMBS];
        let mut carry = 0u32;

        for i in 0..NUM_LIMBS {
            let c = self.limbs[i] + rhs.limbs[i] + carry;
            res[i] = c & mask;
            carry = c >> LOG_LIMB_SIZE;
        }

        Self {
            limbs: res,
            log_limb_size: LOG_LIMB_SIZE,
            _phantom: PhantomData,
        }
    }
}

impl<const NUM_LIMBS: usize, const LOG_LIMB_SIZE: u32> Zero for BigInt<NUM_LIMBS, LOG_LIMB_SIZE> {
    fn zero() -> Self {
        let limbs = [0u32; NUM_LIMBS];
        Self {
            limbs,
            log_limb_size: LOG_LIMB_SIZE,
            _phantom: PhantomData,
        }
    }

    fn is_zero(&self) -> bool {
        for limb in &self.limbs {
            if *limb != 0u32 {
                return false;
            }
        }
        true
    }
}

impl<const NUM_LIMBS: usize, const LOG_LIMB_SIZE: u32> BigInt<NUM_LIMBS, LOG_LIMB_SIZE> {
    pub fn from(limbs: [u32; NUM_LIMBS]) -> Result<Self, LimbSizeError> {
        let max_limb_size = 2u32.pow(LOG_LIMB_SIZE);
        for limb in &limbs {
            if *limb >= max_limb_size {
                return Err(LimbSizeError);
            }
        }

        Ok(Self {
            limbs,
            log_limb_size: LOG_LIMB_SIZE,
            _phantom: PhantomData,
        })
    }
}

#[test]
pub fn test_bigint_eq() {
    let a = BigInt::<2, 3>::from([5u32, 1u32]).unwrap();
    let b = BigInt::<2, 3>::from([5u32, 1u32]).unwrap();
    assert!(a == b);
}

#[test]
pub fn test_bigint_add() {
    let a = BigInt::<2, 3>::from([5u32, 1u32]).unwrap();
    let b = BigInt::<2, 3>::from([5u32, 1u32]).unwrap();
    let c = a + b;

    let expected = BigInt::<2, 3>::from([2u32, 3u32]).unwrap();
    assert!(c == expected);
}

#[test]
pub fn test_bigint_from() {
    let max_limb_size = 2u32.pow(3u32);
    let valid_limbs = [1u32, 2u32, 3u32, 4u32];
    let valid = BigInt::<4, 3>::from(valid_limbs);
    assert!(valid.is_ok());
    for limb in &valid.unwrap().limbs {
        assert!(*limb < max_limb_size);
    }
}

#[test]
pub fn test_bigint_is_zero() {
    let limbs = [0u32; 4];
    let a = BigInt::<4, 3>::from(limbs);
    for limb in &a.unwrap().limbs {
        assert!(*limb == 0u32);
    }
}

#[test]
pub fn test_bigint_zero() {
    let a = BigInt::<16, 16>::zero();
    assert!(a.is_zero());
}
