use serde::{Deserialize, Serialize};
use std::{
    fmt,
    ops::{Add, AddAssign, Sub, SubAssign},
};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Hash)]
pub struct Entry {
    label: String,
    start: Date,
    end: Date,
}

impl Entry {
    /// Create a new Entry. The end parameter is optional. If `None` is
    /// provided, the Entry is treated as a range.
    #[inline]
    pub fn new<S: ToString, I: Into<Date>>(
        label: S,
        start: I,
        end: I,
    ) -> Self {
        Entry {
            label: label.to_string(),
            start: start.into(),
            end: end.into(),
        }
    }

    /// Convience function for when you know you are entering a point, not a
    /// range.
    #[inline]
    pub fn point<S: ToString, I: Into<Date>>(label: S, point: I) -> Self {
        let point = point.into();
        Entry {
            label: label.to_string(),
            start: point.clone(),
            end: point,
        }
    }

    pub fn label(&self) -> String {
        self.label.clone()
    }

    pub fn start(&self) -> Date {
        self.start
    }

    pub fn end(self) -> Date {
        self.end
    }
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} - {}: {}", self.start, self.end, self.label)
    }
}

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
pub struct Date {
    year: i32,
}

impl Date {
    /// Construct a new `Date`. Takes anything that can be cast into an `i32`.
    #[inline]
    pub fn new<T: Into<i32>>(year: T) -> Self { Date { year: year.into() } }

    /// Returns true if this date is in the common era, false if not.
    #[inline]
    pub fn commmon_era(&self) -> bool { self.year >= 0 }

    /// Returns the "era text" for this `Date`. This means CE for a common era
    /// date and BCE for a date before common era.
    #[inline]
    pub fn era_text(&self) -> &'static str {
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
    pub fn abs_year<T: From<i32>>(&self) -> T { self.year.abs().into() }

    /// Returns the raw year.
    ///
    /// Be warned, it could be negative.
    #[inline]
    fn year(&self) -> i32 { self.year }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.abs_year::<i32>(), self.era_text())
    }
}

impl Add for Date {
    type Output = Self;

    fn add(self, lhs: Self) -> Self::Output {
        Date {
            year: self.year + lhs.year(),
        }
    }
}

impl AddAssign for Date {
    fn add_assign(&mut self, lhs: Self) { self.year += lhs.year(); }
}

impl Sub for Date {
    type Output = Self;

    fn sub(self, lhs: Self) -> Self::Output {
        Date {
            year: self.year - lhs.year(),
        }
    }
}

impl SubAssign for Date {
    fn sub_assign(&mut self, lhs: Self) { self.year -= lhs.year() }
}

impl<T: Into<i32>> From<T> for Date {
    fn from(x: T) -> Date { Date { year: x.into() } }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_entry_with_end() {
        let entry = Entry::new(
            "test".to_string(),
            Date { year: 0 },
            Some(Date { year: 1 }),
        );
        assert_eq!(
            entry,
            Entry {
                label: "test".to_string(),
                start: Date { year: 0 },
                end: Some(Date { year: 1 })
            }
        )
    }

    #[test]
    fn test_entry_no_end() {
        let entry = Entry::new("test".to_string(), Date { year: 0 }, None);
        assert_eq!(
            entry,
            Entry {
                label: "test".to_string(),
                start: Date { year: 0 },
                end: None
            }
        )
    }

    #[test]
    fn test_entry_traits() {
        let entry = Entry::new("test", 0, Some(1));
        assert_eq!(
            entry,
            Entry {
                label: "test".to_string(),
                start: Date { year: 0 },
                end: Some(Date { year: 1 })
            }
        )
    }

    #[test]
    fn test_entry_point() {
        let entry = Entry::point("test".to_string(), Date { year: 0 });
        assert_eq!(
            entry,
            Entry {
                label: "test".to_string(),
                start: Date { year: 0 },
                end: None
            }
        )
    }

    #[test]
    fn test_entry_point_traits() {
        let entry = Entry::point("test", 0);
        assert_eq!(
            entry,
            Entry {
                label: "test".to_string(),
                start: Date { year: 0 },
                end: None
            }
        )
    }

    #[test]
    fn test_entry_range() {
        let entry = Entry::range(
            "test".to_string(),
            Date { year: 0 },
            Date { year: 1 },
        );
        assert_eq!(
            entry,
            Entry {
                label: "test".to_string(),
                start: Date { year: 0 },
                end: Some(Date { year: 1 })
            }
        )
    }

    #[test]
    fn test_entry_range_traits() {
        let entry = Entry::range("test", 0, 1);
        assert_eq!(
            entry,
            Entry {
                label: "test".to_string(),
                start: Date { year: 0 },
                end: Some(Date { year: 1 })
            }
        )
    }

    #[test]
    fn test_entry_display_point() {
        let entry = Entry {
            label: "test".to_string(),
            start: Date { year: 0 },
            end: None,
        };
        assert_eq!(entry.to_string(), "0 CE: test".to_string())
    }

    #[test]
    fn test_entry_display_range() {
        let entry = Entry {
            label: "test".to_string(),
            start: Date { year: 0 },
            end: Some(Date { year: 1 }),
        };
        assert_eq!(entry.to_string(), "0 CE - 1 CE: test".to_string());
    }
}
