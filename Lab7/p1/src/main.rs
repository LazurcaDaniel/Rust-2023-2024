#[derive(Debug, PartialEq, Clone, Copy)]
struct Complex {
    real: f64,
    imag: f64,
}

use std::{fmt::Debug, fmt::Display, ops::Add, ops::AddAssign, ops::Mul, ops::Neg, ops::Sub, ops::SubAssign , ops :: MulAssign};

impl Complex {
    fn new<S, T>(x: S, y: T) -> Complex
    where
        S: Into<f64>,
        T: Into<f64>,
    {
        Complex {
            real: x.into(),
            imag: y.into(),
        }
    }

    fn conjugate(mut self) -> Self {
        self.imag *= -1.0;
        self
    }
}

impl From<i32> for Complex {
    fn from(value: i32) -> Self {
        Complex::new(value, 0.0)
    }
}
impl From<f64> for Complex {
    fn from(value: f64) -> Self {
        Complex::new(value, 0.0)
    }
}

impl Neg for Complex {
    type Output = Complex;

    fn neg(self) -> Self::Output {
        Complex {
            real: self.real * -1.0,
            imag: self.imag * -1.0,
        }
    }
}

impl<T> Add<T> for Complex
where
    T: Into<Complex>,
{
    type Output = Complex;

    fn add(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        Complex {
            real: self.real + rhs.real,
            imag: self.imag + rhs.imag,
        }
    }
}

impl<T> Sub<T> for Complex
where
    T: Into<Complex>,
{
    type Output = Complex;

    fn sub(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        Complex {
            real: self.real - rhs.real,
            imag: self.imag - rhs.imag,
        }
    }
}

impl<T> Mul<T> for Complex
where
    T: Into<Complex>,
{
    type Output = Complex;

    fn mul(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        Complex {
            real: self.real * rhs.real - self.imag * rhs.imag,
            imag: self.imag * rhs.real + self.real * rhs.imag,
        }
    }
}

impl<T> AddAssign<T> for Complex
where
    T: Into<Complex>,
{
    fn add_assign(&mut self, rhs: T) {
        let other = rhs.into();
        *self = Complex::new(self.real + other.real, self.imag + other.imag);
    }
}

impl<T> SubAssign<T> for Complex
where
    T: Into<Complex>,
{
    fn sub_assign(&mut self, rhs: T) {
        let other = rhs.into();
        *self = Complex::new(self.real - other.real, self.imag - other.imag);
    }
}

impl<T> MulAssign<T> for Complex
where
    T: Into<Complex>,
{
    fn mul_assign(&mut self, rhs: T) {
        let other = rhs.into();
        *self = Complex::new(self.real * other.real - self.imag * other.imag, self.imag * other.real + self.real * other.imag);
    }
}



impl Display for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.real == 0.0 {
            if self.imag == 0.0 {
                write!(f, "0")?
            } else {
                write!(f, "{}i", self.imag)?
            }
        } else {
            if self.imag == 0.0 {
                write!(f, "{}", self.real)?
            } else if self.imag > 0.0 {
                write!(f, "{}{:+}i", self.real, self.imag)?
            } else {
                write!(f, "{}{}i", self.real, self.imag)?
            }
        }
        Ok(())
    }
}

fn eq_rel(x: f64, y: f64) -> bool {
    (x - y).abs() < 0.001
}
// This is a macro that panics if 2 floats are not equal using an epsilon.
// You are not required to understand it yet, just to use it.
macro_rules! assert_eq_rel {
    ($x:expr, $y: expr) => {
        let x = $x as f64;
        let y = $y as f64;
        let r = eq_rel(x, y);
        assert!(r, "{} != {}", x, y);
    };
}

fn main() {
    let a = Complex::new(1.0, 2.0);
    assert_eq_rel!(a.real, 1);
    assert_eq_rel!(a.imag, 2);

    let b = Complex::new(2.0, 3);
    let c = a + b;
    assert_eq_rel!(c.real, 3);
    assert_eq_rel!(c.imag, 5);

    let d = c - a;
    assert_eq!(b, d);

    let e = (a * d).conjugate();
    assert_eq_rel!(e.imag, -7);

    let f = (a + b - d) * c;
    assert_eq!(f, Complex::new(-7, 11));

    // Note: .to_string() uses Display to format the type
    assert_eq!(Complex::new(1, 2).to_string(), "1+2i");
    assert_eq!(Complex::new(1, -2).to_string(), "1-2i");
    assert_eq!(Complex::new(0, 5).to_string(), "5i");
    assert_eq!(Complex::new(7, 0).to_string(), "7");
    assert_eq!(Complex::new(0, 0).to_string(), "0");

    let h = Complex::new(-4, -5);
    let i = h - (h + 5) * 2.0;
    assert_eq_rel!(i.real, -6);

    let mut j = -i + i;
    assert_eq_rel!(j.real, 0);
    assert_eq_rel!(j.imag, 0);

    j += h; 
    assert_eq!(j, h); // j = -4 -5i

    j += 4;
    assert_eq_rel!(j.real,0); // j = -5i

    j -= 4;
    assert_eq!(j, h); // j = -4 -5i

    j -= h;
    assert_eq_rel!(j.real, 0);
    assert_eq_rel!(j.imag, 0);

    let mut k = Complex :: new(1,1);
    k *= 4;
    assert_eq!(k, Complex::new(4,4));
    
    k*=Complex::new(4,-5.0);
    assert_eq!(k, Complex::new(36,-4));

    println!("ok!");
}
