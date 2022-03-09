use std::{convert, ops, error, fmt};

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Probability(f64);

impl TryFrom<f64> for Probability {
    type Error = ProbabilityError;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if (0.0..=1.0).contains(&value) {
            Ok(Probability(value))
        } else {
            Err(ProbabilityError::OutOfBounds)
        }
    }
}

impl From<Probability> for f64 {
    fn from(value: Probability) -> Self {
        value.0
    }
}

macro_rules! impl_op {
    ($trait:ty, $func:expr) => {
        impl $trait for Probability {
            paste::paste! {
                type Output = Result<Self, ProbabilityError>;
                fn [<$func>](self, rhs: Self) -> Self::Output {
                    Probability::try_from(self.0.$func(rhs.0))
                }
            }
        }
    }
}

impl_op!(ops::Add, add);
impl_op!(ops::Sub, sub);
impl_op!(ops::Mul, mul);
impl_op!(ops::Div, div);

impl convert::AsRef<f64> for Probability {
    fn as_ref(&self) -> &f64 {
        &self.0
    }
}

/// Probability type errors
#[derive(Debug)]
pub enum ProbabilityError {
    OutOfBounds,
}

impl fmt::Display for ProbabilityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            ProbabilityError::OutOfBounds => write!(f, "Number is not in the 0.0..=1.0 range")
        }
    }
}

impl error::Error for ProbabilityError {}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;

    #[test]
    fn in_bound() {
        let p = Probability::try_from(0.5).unwrap();
        assert_eq!(p, Probability(0.5));
    }

    #[test]
    fn randomized_in_bound() {
        let mut rng = thread_rng();

        for _ in 0..2048 {
            let inner: f64 = rng.gen_range(0.0..=1.0);
            let p = Probability::try_from(inner).unwrap();
            assert_eq!(p, Probability(inner));
        }
    }

    #[test]
    fn under_bounds() {
        match Probability::try_from(-0.1).unwrap_err() {
            ProbabilityError::OutOfBounds => (),
            // _ => assert!(false),
        }
    }

    #[test]
    fn randomized_under_bounds() {
        let mut rng = thread_rng();

        for _ in 0..2048 {
            let inner: f64 = rng.gen_range(f64::MIN..0.0);
            match Probability::try_from(inner).unwrap_err() {
                ProbabilityError::OutOfBounds => (),
                // _ => assert!(false),
            }
        }
    }

    #[test]
    fn over_bounds() {
        match Probability::try_from(1.1).unwrap_err() {
            ProbabilityError::OutOfBounds => (),
            // _ => assert!(false),
        }
    }

    #[test]
    fn randomized_over_bounds() {
        let mut rng = thread_rng();

        for _ in 0..2048 {
            let inner: f64 = rng.gen_range(1.1..f64::MAX);
            match Probability::try_from(inner).unwrap_err() {
                ProbabilityError::OutOfBounds => (),
                // _ => assert!(false),
            }
        }
    }

    #[test]
    fn add() {
        let p1 = Probability::try_from(0.1).unwrap();
        let p2 = Probability::try_from(0.3).unwrap();

        let expected = Probability::try_from(0.4).unwrap();

        assert_eq!((p1 + p2).unwrap(), expected);
    }

    #[test]
    fn add_err() {
        let p1 = Probability::try_from(0.3).unwrap();
        let p2 = Probability::try_from(0.9).unwrap();

        match (p1 + p2).unwrap_err() {
            ProbabilityError::OutOfBounds => (),
            // _ => assert!(false),
        }
    }

    #[test]
    fn into() {
        let p = Probability::try_from(0.1).unwrap();
        let v: f64 = p.into();
        assert_eq!(v, 0.1);
    }
}