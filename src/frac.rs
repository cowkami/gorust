use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Frac {
    num: usize,
    denom: usize,
}

impl Frac {
    pub fn new(num: usize, denom: usize) -> Result<Self, String> {
        if denom == 0 {
            Err("denominator must not be zero.".to_string())
        } else {
            Ok(Frac { num, denom })
        }
    }
}

impl Add for Frac {
    type Output = Frac;
    fn add(self, rhs: Self) -> Self::Output {
        let new_denom = self.denom * rhs.denom;
        let new_lhs_num = self.num * rhs.denom;
        let new_rhs_num = rhs.num * self.denom;
        let new_frac = Frac {
            num: new_lhs_num + new_rhs_num,
            denom: new_denom,
        };
        for i in (2..=new_frac.denom).rev() {
            if new_frac.num % i == 0 && new_frac.denom % i == 0 {
                return Frac {
                    num: new_frac.num / i,
                    denom: new_frac.denom / i,
                };
            }
        }
        new_frac
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frac() {
        let a = Frac::new(1, 0);
        assert!(a.is_err());

        let a = Frac::new(1, 1).unwrap();
        let b = Frac::new(1, 1).unwrap();
        let c = a + b;
        assert_eq!(c, Frac::new(2, 1).unwrap());

        let a = Frac::new(1, 2).unwrap();
        let b = Frac::new(1, 3).unwrap();
        let c = a + b;
        assert_eq!(c, Frac::new(5, 6).unwrap());

        let a = Frac::new(1, 2).unwrap();
        let b = Frac::new(1, 4).unwrap();
        let c = a + b;
        assert_eq!(c, Frac::new(3, 4).unwrap());

        let a = Frac::new(3, 4).unwrap();
        let b = Frac::new(1, 4).unwrap();
        let c = a + b;
        assert_eq!(c, Frac::new(1, 1).unwrap());
    }
}
