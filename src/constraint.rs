use crate::VersionConstraintError;
use crate::comparator::{self, Comparator, EqualComparatorKind};
use crate::valid_chars::{collect_invalid_characters, VlsSpecialCharSet};
use std::fmt::{Display, Formatter};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct VersionConstraint {
    pub comparator: Comparator,
    pub version: String,
}

impl VersionConstraint {
    pub fn new(comparator: Comparator, version: String) -> VersionConstraint {
        VersionConstraint {
            comparator,
            version,
        }
    }

    pub fn parse(constraint_str: &str) -> Result<VersionConstraint, Vec<VersionConstraintError>> {
        // Check if the constraint is empty
        if constraint_str.is_empty() {
            return Err(vec![VersionConstraintError::EmptyConstraint]);
        }

        // Check if the constraint is Comparator::Any
        if constraint_str == comparator::ANY {
            return Ok(Self {
                comparator: Comparator::Any,
                version: String::default(),
            });
        }

        // Match the comparators
        // Order must be kept, as the two-char comparators take precedence over the one-char comparators)
        // TODO: Add regression test for this
        let (comparator, version) = if let Some(stripped) =
            constraint_str.strip_prefix(comparator::GREATER_THAN_OR_EQUAL)
        {
            (Comparator::GreaterThanOrEqual, stripped)
        } else if let Some(stripped) = constraint_str.strip_prefix(comparator::LESS_THAN_OR_EQUAL) {
            (Comparator::LessThanOrEqual, stripped)
        } else if let Some(stripped) = constraint_str.strip_prefix(comparator::NOT_EQUAL) {
            (Comparator::NotEqual, stripped)
        } else if let Some(stripped) = constraint_str.strip_prefix(comparator::GREATER_THAN) {
            (Comparator::GreaterThan, stripped)
        } else if let Some(stripped) = constraint_str.strip_prefix(comparator::LESS_THAN) {
            (Comparator::LessThan, stripped)
        } else if let Some(stripped) = constraint_str.strip_prefix(comparator::EQUAL) {
            (Comparator::Equal(EqualComparatorKind::Explicit), stripped)
        } else {
            (
                Comparator::Equal(EqualComparatorKind::Implicit),
                constraint_str,
            )
        };

        // Check if the string after the comparator is empty
        if version.is_empty() {
            return Err(vec![VersionConstraintError::EmptyVersion]);
        }

        // Reject any character that is not part of the version-string grammar.
        // See vls::Vls for more details on the grammar.
        let invalid_version_chars = collect_invalid_characters(version, VlsSpecialCharSet::VersionString);
        if let Some(invalid_version_chars) = invalid_version_chars {
            return Err(vec![VersionConstraintError::InvalidVersionCharacters(
                invalid_version_chars,
            )]);
        }

        // This is a valid vls constraint
        Ok(VersionConstraint::new(comparator, version.to_string()))
    }
}

impl Display for VersionConstraint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.comparator {
            Comparator::Equal(kind) => write!(f, "{kind}{}", self.version),
            Comparator::Any => write!(f, "*"),
            _ => write!(f, "{}{}", self.comparator, self.version),
        }
    }
}
