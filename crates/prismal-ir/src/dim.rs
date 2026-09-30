//! Dimensions: products of rational powers of the seven SI base dimensions (MK-3.1).

use serde::de::{self, Deserializer, MapAccess, Visitor};
use serde::ser::{SerializeMap, Serializer};
use serde::{Deserialize, Serialize};
use std::fmt;

/// A rational exponent, kept normalized (denominator positive, lowest terms).
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Ratio {
    pub n: i32,
    pub d: i32,
}

impl Ratio {
    pub const ZERO: Ratio = Ratio { n: 0, d: 1 };
    pub const ONE: Ratio = Ratio { n: 1, d: 1 };

    pub fn new(n: i32, d: i32) -> Ratio {
        assert!(d != 0, "zero denominator");
        let g = gcd(n.abs(), d.abs()).max(1);
        let s = if d < 0 { -1 } else { 1 };
        Ratio { n: s * n / g, d: s * d / g }
    }
    pub fn int(n: i32) -> Ratio {
        Ratio { n, d: 1 }
    }
    pub fn is_zero(self) -> bool {
        self.n == 0
    }
    pub fn add(self, o: Ratio) -> Ratio {
        Ratio::new(self.n * o.d + o.n * self.d, self.d * o.d)
    }
    pub fn neg(self) -> Ratio {
        Ratio { n: -self.n, d: self.d }
    }
    pub fn mul(self, o: Ratio) -> Ratio {
        Ratio::new(self.n * o.n, self.d * o.d)
    }
    pub fn to_f64(self) -> f64 {
        self.n as f64 / self.d as f64
    }
}

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

/// Base dimension symbols in fixed order: length, mass, time, current, temperature, amount, luminosity.
pub const BASE: [&str; 7] = ["L", "M", "T", "I", "Th", "N", "J"];

/// A dimension (MK-3.1). The all-zero dimension is dimensionless.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Dim(pub [Ratio; 7]);

impl Default for Dim {
    fn default() -> Self {
        Dim::NONE
    }
}

impl Dim {
    pub const NONE: Dim = Dim([Ratio::ZERO; 7]);

    pub fn base(i: usize) -> Dim {
        let mut d = Dim::NONE;
        d.0[i] = Ratio::ONE;
        d
    }
    pub fn length() -> Dim {
        Dim::base(0)
    }
    pub fn mass() -> Dim {
        Dim::base(1)
    }
    pub fn time() -> Dim {
        Dim::base(2)
    }
    pub fn is_none(&self) -> bool {
        self.0.iter().all(|r| r.is_zero())
    }
    pub fn mul(&self, o: &Dim) -> Dim {
        let mut r = *self;
        for i in 0..7 {
            r.0[i] = r.0[i].add(o.0[i]);
        }
        r
    }
    pub fn div(&self, o: &Dim) -> Dim {
        self.mul(&o.inv())
    }
    pub fn inv(&self) -> Dim {
        let mut r = *self;
        for x in r.0.iter_mut() {
            *x = x.neg();
        }
        r
    }
    pub fn pow(&self, p: Ratio) -> Dim {
        let mut r = *self;
        for x in r.0.iter_mut() {
            *x = x.mul(p);
        }
        r
    }

    /// Parses a dimension expression such as `L/T^2`, `M L^2 T^-2`, `1/L`, `1`.
    pub fn parse(s: &str) -> Result<Dim, String> {
        let mut dim = Dim::NONE;
        let mut sign = 1;
        let cleaned = s.replace('*', " ").replace('/', " / ");
        for tok in cleaned.split_whitespace() {
            if tok == "/" {
                sign = -1;
                continue;
            }
            if tok == "1" {
                continue;
            }
            let (sym, exp) = match tok.split_once('^') {
                Some((a, b)) => (a, parse_ratio(b)?),
                None => (tok, Ratio::ONE),
            };
            let i = BASE
                .iter()
                .position(|b| *b == sym)
                .ok_or_else(|| format!("unknown base dimension `{sym}`"))?;
            let e = if sign < 0 { exp.neg() } else { exp };
            dim.0[i] = dim.0[i].add(e);
        }
        Ok(dim)
    }
}

fn parse_ratio(s: &str) -> Result<Ratio, String> {
    let s = s.trim_matches(|c| c == '(' || c == ')');
    if let Some((a, b)) = s.split_once('/') {
        Ok(Ratio::new(
            a.parse().map_err(|_| format!("bad exponent `{s}`"))?,
            b.parse().map_err(|_| format!("bad exponent `{s}`"))?,
        ))
    } else {
        Ok(Ratio::int(s.parse().map_err(|_| format!("bad exponent `{s}`"))?))
    }
}

impl fmt::Display for Dim {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_none() {
            return write!(f, "1");
        }
        let mut first = true;
        for (i, r) in self.0.iter().enumerate() {
            if r.is_zero() {
                continue;
            }
            if !first {
                write!(f, " ")?;
            }
            first = false;
            write!(f, "{}", BASE[i])?;
            if *r != Ratio::ONE {
                if r.d == 1 {
                    write!(f, "^{}", r.n)?;
                } else {
                    write!(f, "^({}/{})", r.n, r.d)?;
                }
            }
        }
        Ok(())
    }
}

impl Serialize for Dim {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let count = self.0.iter().filter(|r| !r.is_zero()).count();
        let mut m = s.serialize_map(Some(count))?;
        for (i, r) in self.0.iter().enumerate() {
            if r.is_zero() {
                continue;
            }
            if r.d == 1 {
                m.serialize_entry(BASE[i], &r.n)?;
            } else {
                m.serialize_entry(BASE[i], &[r.n, r.d])?;
            }
        }
        m.end()
    }
}

impl<'de> Deserialize<'de> for Dim {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Dim, D::Error> {
        struct V;
        impl<'de> Visitor<'de> for V {
            type Value = Dim;
            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "a map of base dimensions to exponents")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Dim, A::Error> {
                let mut dim = Dim::NONE;
                while let Some(k) = map.next_key::<String>()? {
                    let i = BASE
                        .iter()
                        .position(|b| *b == k)
                        .ok_or_else(|| de::Error::custom(format!("unknown base dimension `{k}`")))?;
                    let v: serde_json::Value = map.next_value()?;
                    let r = match v {
                        serde_json::Value::Number(n) => Ratio::int(
                            n.as_i64().ok_or_else(|| de::Error::custom("exponent must be an integer"))?
                                as i32,
                        ),
                        serde_json::Value::Array(a) if a.len() == 2 => Ratio::new(
                            a[0].as_i64().unwrap_or(0) as i32,
                            a[1].as_i64().unwrap_or(1) as i32,
                        ),
                        _ => return Err(de::Error::custom("bad exponent")),
                    };
                    dim.0[i] = r;
                }
                Ok(dim)
            }
        }
        d.deserialize_map(V)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_and_display() {
        let a = Dim::parse("L/T^2").unwrap();
        assert_eq!(a.to_string(), "L T^-2");
        assert_eq!(Dim::parse("1/L").unwrap(), Dim::length().inv());
        assert_eq!(Dim::parse("M L^2 T^-2").unwrap().to_string(), "L^2 M T^-2");
        assert!(Dim::parse("1").unwrap().is_none());
        assert_eq!(a.pow(Ratio::new(1, 2)).to_string(), "L^(1/2) T^-1");
    }

    #[test]
    fn json_round_trip() {
        let a = Dim::parse("L T^-2").unwrap().pow(Ratio::new(1, 2));
        let s = serde_json::to_string(&a).unwrap();
        assert_eq!(s, r#"{"L":[1,2],"T":-1}"#);
        let b: Dim = serde_json::from_str(&s).unwrap();
        assert_eq!(a, b);
    }
}
