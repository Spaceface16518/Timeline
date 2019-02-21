use serde::{Deserialize, Serialize};
use std::{
    fmt,
    ops::{Add, AddAssign, Sub, SubAssign},
};

#[derive(
    Serialize,
    Deserialize,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
pub struct Date(i32);

impl Date {
    /// Construct a new `Date`. Takes anything that can be cast into an `i32`.
    #[inline]
    pub fn new<T: Into<i32>>(year: T) -> Self { Date(year.into()) }

    /// Returns true if this date is in the common era, false if not.
    #[inline]
    pub fn commmon_era(self) -> bool { self.0 >= 0 }

    /// Returns the "era text" for this `Date`. This means CE for a common era
    /// date and BCE for a date before common era.
    #[inline]
    pub fn era_text(self) -> &'static str {
        if self.commmon_era() {
            "CE"
        } else {
            "BCE"
        }
    }

    /// Returns the year, without regards to relative positivity from 0 CE.
    ///
    /// WARNING: Using this method could result in corrupted dates if not used
    /// properly.
    ///
    /// The type parameter allows you to create another `Date` directly from
    /// this function, or just extract the year into whatever type of integer
    /// you want.
    #[inline]
    pub fn abs_year<T: From<i32>>(self) -> T { self.0.abs().into() }

    /// Returns the raw year.
    ///
    /// Be warned, it could be negative.
    #[inline]
    pub fn year(self) -> i32 { self.0 }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.abs_year::<i32>(), self.era_text())
    }
}

impl Add for Date {
    type Output = Self;

    fn add(self, lhs: Self) -> Self::Output { Date(self.year() + lhs.year()) }
}

impl AddAssign for Date {
    fn add_assign(&mut self, lhs: Self) { self.0 += lhs.year(); }
}

impl Sub for Date {
    type Output = Self;

    fn sub(self, lhs: Self) -> Self::Output { Date(self.year() - lhs.year()) }
}

impl SubAssign for Date {
    fn sub_assign(&mut self, lhs: Self) { self.0 -= lhs.year() }
}

impl<T: Into<i32>> From<T> for Date {
    fn from(x: T) -> Date { Date(x.into()) }
}
