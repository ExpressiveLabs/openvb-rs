// Timestamp contains a time value in microseconds.

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Timestamp {
    pub value: u64
}

impl Timestamp {
    pub fn new(value: u64) -> Self {
        Timestamp {
            value
        }
    }

    pub fn from_minified(value: u64) -> Self {
        Timestamp {
            value
        }
    }

    pub fn seconds(&self) -> f64 {
        self.value as f64 / 1_000_000.0
    }
    pub fn from_seconds(seconds: f64) -> Self {
        Timestamp {
            value: (seconds * 1_000_000.0) as u64
        }
    }

    pub fn milliseconds(&self) -> f64 {
        self.value as f64 / 1_000.0
    }
    pub fn from_milliseconds(milliseconds: f64) -> Self {
        Timestamp {
            value: (milliseconds * 1_000.0) as u64
        }
    }

    pub fn samples(&self, sample_rate: f64) -> usize {
        (self.value as f64 * sample_rate / 1_000_000.0).round() as usize
    }
    pub fn from_samples(samples: usize, sample_rate: f64) -> Self {
        Timestamp {
            value: (samples as f64 * 1_000_000.0 / sample_rate) as u64
        }
    }

    pub fn zero() -> Self {
        Timestamp {
            value: 0
        }
    }
}

// Implement operators
impl std::ops::Add<Timestamp> for Timestamp {
    type Output = Timestamp;

    fn add(self, other: Timestamp) -> Timestamp {
        Timestamp {
            value: self.value + other.value
        }
    }
}

impl std::ops::Sub<Timestamp> for Timestamp {
    type Output = Timestamp;

    fn sub(self, other: Timestamp) -> Timestamp {
        Timestamp {
            value: self.value - other.value
        }
    }
}

impl std::ops::AddAssign<Timestamp> for Timestamp {
    fn add_assign(&mut self, other: Timestamp) {
        self.value += other.value;
    }
}

impl std::ops::SubAssign<Timestamp> for Timestamp {
    fn sub_assign(&mut self, other: Timestamp) {
        self.value -= other.value;
    }
}

impl std::ops::Mul<f64> for Timestamp {
    type Output = Timestamp;

    fn mul(self, other: f64) -> Timestamp {
        Timestamp {
            value: (self.value as f64 * other) as u64
        }
    }
}

impl std::ops::Div<f64> for Timestamp {
    type Output = Timestamp;

    fn div(self, other: f64) -> Timestamp {
        Timestamp {
            value: (self.value as f64 / other) as u64
        }
    }
}

impl std::ops::MulAssign<f64> for Timestamp {
    fn mul_assign(&mut self, other: f64) {
        self.value = (self.value as f64 * other) as u64;
    }
}

impl std::ops::DivAssign<f64> for Timestamp {
    fn div_assign(&mut self, other: f64) {
        self.value = (self.value as f64 / other) as u64;
    }
}

impl std::ops::Rem<Timestamp> for Timestamp {
    type Output = Timestamp;

    fn rem(self, other: Timestamp) -> Timestamp {
        Timestamp {
            value: self.value % other.value
        }
    }
}

impl std::ops::RemAssign<Timestamp> for Timestamp {
    fn rem_assign(&mut self, other: Timestamp) {
        self.value %= other.value;
    }
}

// Implement math functions
impl Timestamp {
    pub fn min(self, other: Timestamp) -> Timestamp {
        if self < other {
            self
        } else {
            other
        }
    }

    pub fn max(self, other: Timestamp) -> Timestamp {
        if self > other {
            self
        } else {
            other
        }
    }

    pub fn pow(self, exp: u32) -> Timestamp {
        Timestamp {
            value: self.value.pow(exp)
        }
    }

    pub fn sqrt(self) -> Timestamp {
        Timestamp {
            value: (self.value as f64).sqrt() as u64
        }
    }
}

// Implement from
impl From<u64> for Timestamp {
    fn from(value: u64) -> Self {
        Timestamp {
            value
        }
    }
}

// Implement Display
impl std::fmt::Display for Timestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Depending on the value, we will display the time in seconds, milliseconds or microseconds
        if self.value >= 1_000_000 {
            write!(f, "{:.3}s", self.seconds())
        } else if self.value >= 1_000 {
            write!(f, "{:.3}ms", self.milliseconds())
        } else {
            write!(f, "{}µs", self.value)
        }
    }
}
