//! Comparator type for the csaf-rs/vls library.
//!
//! The `Comparator` enum represents the different types of comparators that can be used
//! in version constraints, such as = (implicit or explicit), !=, <, <=, >, >=, and *.

use std::fmt;

/// String representation of the Equal comparator.
pub const EQUAL: &str = "=";
/// String representation of the NotEqual comparator.
pub const NOT_EQUAL: &str = "!=";
/// String representation of the LessThan comparator.
pub const LESS_THAN: &str = "<";
/// String representation of the LessThanOrEqual comparator.
pub const LESS_THAN_OR_EQUAL: &str = "<=";
/// String representation of the GreaterThan comparator.
pub const GREATER_THAN: &str = ">";
/// String representation of the GreaterThanOrEqual comparator.
pub const GREATER_THAN_OR_EQUAL: &str = ">=";
/// String representation of the Any comparator.
pub const ANY: &str = "*";

/// Comparator for version constraints.
///
/// This enum represents the different types of comparators that can be used
/// in version constraints. Each comparator defines how a version is compared
/// to the constraint version.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Comparator {
    /// Equal (=) - The version must be exactly equal to the constraint version.
    Equal(EqualComparatorKind),
    /// Not equal (!=) - The version must not be equal to the constraint version.
    NotEqual,
    /// Less than (<) - The version must be less than the constraint version.
    LessThan,
    /// Less than or equal (<=) - The version must be less than or equal to the constraint version.
    LessThanOrEqual,
    /// Greater than (>) - The version must be greater than the constraint version.
    GreaterThan,
    /// Greater than or equal (>=) - The version must be greater than or equal to the constraint version.
    GreaterThanOrEqual,
    /// Any version (*) - Matches any version. Must be used alone.
    Any,
}

#[derive(Debug, Clone, Copy)]
pub enum EqualComparatorKind {
    Implicit,
    Explicit,
}

impl PartialEq for EqualComparatorKind {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl Eq for EqualComparatorKind {}



impl fmt::Display for EqualComparatorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EqualComparatorKind::Implicit => write!(f, ""),
            EqualComparatorKind::Explicit => write!(f, "="),
        }
    }
}

impl fmt::Display for Comparator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Comparator::Equal(kind) => write!(f, "{kind}"),
            Comparator::NotEqual => write!(f, "{NOT_EQUAL}"),
            Comparator::LessThan => write!(f, "{LESS_THAN}"),
            Comparator::LessThanOrEqual => write!(f, "{LESS_THAN_OR_EQUAL}"),
            Comparator::GreaterThan => write!(f, "{GREATER_THAN}"),
            Comparator::GreaterThanOrEqual => write!(f, "{GREATER_THAN_OR_EQUAL}"),
            Comparator::Any => write!(f, "{ANY}"),
        }
    }
}
