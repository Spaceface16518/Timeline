use serde::{Deserialize, Serialize};
use std::{
    fmt,
    ops::{Add, AddAssign, Sub, SubAssign},
};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Hash)]
pub struct Entry {
    label: String,
    start: Date,
    end: Option<Date>,
}

impl Entry {
    #[inline]
    pub fn new<S: ToString, I: Into<Date>>(
        label: S,
        start: I,
        end: Option<I>,
    ) -> Self {
        Entry {
            label: label.to_string(),
            start: start.into(),
            end: match end {
                Some(d) => Some(d.into()),
                None => None,
            },
        }
    }

    #[inline]
    pub fn point<S: ToString, I: Into<Date>>(label: S, point: I) -> Self {
        Entry {
            label: label.to_string(),
            start: point.into(),
            end: None,
        }
    }

    #[inline]
    pub fn range<S: ToString, I: Into<Date>>(
        label: S,
        start: I,
        end: I,
    ) -> Self {
        Entry {
            label: label.to_string(),
            start: start.into(),
            end: Some(end.into()),
        }
    }
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(end_date) = self.end {
            write!(f, "{} - {}: {}", self.start, end_date, self.label)
        } else {
            write!(f, "{}: {}", self.start, self.label)
        }
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
    #[inline]
    pub fn new<T: Into<i32>>(year: T) -> Self { Date { year: year.into() } }

    #[inline]
    pub fn commmon_era(&self) -> bool { self.year >= 0 }

    #[inline]
    pub fn era_text(&self) -> &'static str {
        if self.commmon_era() {
            "CE"
        } else {
            "BCE"
        }
    }

    #[inline]
    pub fn abs_year<T: From<i32>>(&self) -> T { self.year.abs().into() }

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
    use serde_json;

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
