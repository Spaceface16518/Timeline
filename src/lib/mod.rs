#![deny(clippy::all)]
pub use date::Date;
use serde::{Deserialize, Serialize};
use std::{cmp::Ordering, fmt};

mod date;

#[derive(
    Serialize,
    Deserialize,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Ord,
    PartialOrd,
    Hash,
)]
#[serde(untagged)]
enum EDate {
    Point(Date),
    Range { start: Date, end: Date },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Hash, Ord, Eq)]
pub struct Entry {
    label: String,
    tag: Option<String>,
    date: EDate,
}

impl Entry {
    /// Create a new Entry. The end parameter is optional. If `None` is
    /// provided, the Entry is treated as a range.
    #[inline]
    pub fn range<S: ToString, I: Into<Date>>(
        label: S,
        tag: Option<S>,
        start: I,
        end: I,
    ) -> Self {
        Entry {
            label: label.to_string(),
            tag: if let Some(s) = tag {
                Some(s.to_string())
            } else {
                None
            },
            date: EDate::Range {
                start: start.into(),
                end: end.into(),
            },
        }
    }

    /// Convience function for when you know you are entering a point, not a
    /// range.
    #[inline]
    pub fn point<S: ToString, I: Into<Date>>(
        label: S,
        tag: Option<S>,
        point: I,
    ) -> Self {
        let point = point.into();
        Entry {
            label: label.to_string(),
            tag: if let Some(s) = tag {
                Some(s.to_string())
            } else {
                None
            },
            date: EDate::Point(point),
        }
    }

    pub fn label(&self) -> String { self.label.clone() }

    pub fn tag(&self) -> Option<String> {
        if let Some(s) = &self.tag {
            Some(s.clone())
        } else {
            None
        }
    }

    pub fn start(&self) -> i32 {
        match self.date {
            EDate::Point(n) => n.year(),
            EDate::Range { start, .. } => start.year(),
        }
    }

    pub fn end(&self) -> i32 {
        match self.date {
            EDate::Point(n) => n.year(),
            EDate::Range { end, .. } => end.year(),
        }
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.date {
            EDate::Point(year) => {
                match other.date {
                    EDate::Point(other_year) => year.partial_cmp(&other_year),
                    EDate::Range { start, .. } => year.partial_cmp(&start),
                }
            },
            EDate::Range { start, .. } => {
                let self_start = start;
                match other.date {
                    EDate::Point(other_year) => {
                        self_start.partial_cmp(&other_year)
                    },
                    EDate::Range { start, .. } => {
                        self_start.partial_cmp(&start)
                    },
                }
            },
        }
    }
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(s) = self.tag() {
            write!(f, "({}) ", s)?;
        }
        match self.date {
            EDate::Range { start, end } => write!(f, "{} - {}: ", start, end)?,
            EDate::Point(year) => write!(f, "{}: ", year)?,
        };
        write!(f, "{}", self.label())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_entry_with_end() {
        let entry = Entry::range(
            "test".to_string(),
            Some("test".to_string()),
            Date::new(0),
            Date::new(1),
        );
        assert_eq!(
            entry,
            Entry {
                label: "test".to_string(),
                tag: Some("test".to_string()),
                date: EDate::Range {
                    start: Date::new(0),
                    end: Date::new(1)
                }
            }
        )
    }

    #[test]
    fn test_entry_no_end() {
        let entry = Entry::range(
            "test".to_string(),
            Some("test".to_string()),
            Date::new(0),
            Date::new(0),
        );
        assert_eq!(
            entry,
            Entry {
                label: "test".to_string(),
                tag: Some("test".to_string()),
                date: EDate::Range {
                    start: Date::new(0),
                    end: Date::new(0)
                }
            }
        )
    }

    #[test]
    fn test_entry_traits() {
        let entry = Entry::range("test", Some("test"), 0, 1);
        assert_eq!(
            entry,
            Entry {
                label: "test".to_string(),
                tag: Some("test".to_string()),
                date: EDate::Range {
                    start: Date::new(0),
                    end: Date::new(1)
                }
            }
        )
    }

    #[test]
    fn test_entry_point() {
        let entry = Entry::point(
            "test".to_string(),
            Some("test".to_string()),
            Date::new(0),
        );
        assert_eq!(
            entry,
            Entry {
                label: "test".to_string(),
                tag: Some("test".to_string()),
                date: EDate::Point(Date::new(0))
            }
        )
    }

    #[test]
    fn test_entry_point_traits() {
        let entry = Entry::point("test", Some("test"), 0);
        assert_eq!(
            entry,
            Entry {
                label: "test".to_string(),
                tag: Some("test".to_string()),
                date: EDate::Point(Date::new(0))
            }
        )
    }

    #[test]
    fn test_entry_display_range() {
        let entry = Entry {
            label: "test".to_string(),
            tag: Some("test".to_string()),
            date: EDate::Range {
                start: Date::new(0),
                end: Date::new(1),
            },
        };
        assert_eq!(entry.to_string(), "(test) 0 CE - 1 CE: test".to_string());
    }

    #[test]
    fn test_entry_display_range_no_tag() {
        let entry = Entry {
            label: "test".to_string(),
            tag: None,
            date: EDate::Range {
                start: Date::new(0),
                end: Date::new(1),
            },
        };
        assert_eq!(entry.to_string(), "0 CE - 1 CE: test".to_string());
    }

    #[test]
    fn test_entry_display_point() {
        let entry = Entry {
            label: "test".to_string(),
            tag: Some("test".to_string()),
            date: EDate::Point(Date::new(0)),
        };
        assert_eq!(entry.to_string(), "(test) 0 CE: test".to_string());
    }

    #[test]
    fn test_entry_display_point_no_tag() {
        let entry = Entry {
            label: "test".to_string(),
            tag: None,
            date: EDate::Point(Date::new(0)),
        };
        assert_eq!(entry.to_string(), "0 CE: test".to_string());
    }
}
