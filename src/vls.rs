//! The core [`Vls`] type.

use crate::comparator::Comparator;
use crate::constraint::VersionConstraint;
use crate::error::{VersionConstraintError, VlsError};
use crate::valid_chars::{VlsSpecialCharSet, collect_invalid_characters};
use std::collections::HashSet;
use std::fmt;
use std::fmt::Display;
use std::str::FromStr;

/// A **Vers-like Specifier** (VLS).
///
/// VLS is the `<version-constraint>` part of a [vers](https://github.com/package-url/vers-spec)
/// URL *without* the `vers:<scheme>/` prefix.  It is an ordered, `|`-separated list of
/// [`VersionConstraint`] values.
///
/// In VLS, versions are stored as plain [`String`]s. Due to the unspecified
/// format of the versions, only exact matching is possible and containment checks are not supported.
///
/// # Syntax
///
/// Derived from the [vers specification](https://www.packageurl.org/docs/vers/how-to-parse).
/// There currently is no "official" grammar for vers-like specifier / the <version-constraint> part of
/// vers. This is a best-effort attempt used for this library.
///
/// TODO: Revisit this once vers has been ratified through ECMA, which might include an official grammar.
///
/// ```text
/// vls            = constraint *( "|" constraint )
/// constraint     = comparator version-string / version-string / "*"
/// comparator     = "!=" / "<=" / ">=" / "=" / "<" / ">"
/// version-string = 1*( ALPHA / DIGIT / "-" / "." / "_" / "+" / "~" )
/// ```
///
/// For validation, this leads to two sets of characters allowed in the context of the grammar.
///
/// For `vls`: `ALPHA / DIGIT / "-" / "." / "_" / "+" / "~" / "=" / "!" / "<" / ">" / "|" / "*"`
///
/// For `version-string`: `ALPHA / DIGIT / "-" / "." / "_" / "+" / "~"`
///
/// # Examples
///
/// ```
/// use vls::Vls;
///
/// let vls: Vls = "<=2".parse().unwrap();
/// assert_eq!(vls.len(), 1);
///
/// let vls: Vls = ">10.9a|!=10.9c|!=10.9f|<=10.9k".parse().unwrap();
/// assert_eq!(vls.len(), 4);
/// assert_eq!(vls.to_string(), ">10.9a|!=10.9c|!=10.9f|<=10.9k");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vls {
    constraints: Vec<VersionConstraint>,
}

impl Vls {
    /// Return a slice of all constraints in declaration order.
    pub fn constraints(&self) -> &[VersionConstraint] {
        &self.constraints
    }

    /// Return the number of constraints.
    pub fn len(&self) -> usize {
        self.constraints.len()
    }

    /// Return `true` if there are no constraints.
    pub fn is_empty(&self) -> bool {
        self.constraints.is_empty()
    }

    /// Return `true` if this specifier pins exactly one version,
    /// i.e. it contains a single [Comparator::Equal] constraint.
    pub fn is_single_version(&self) -> bool {
        matches!(
            self.constraints.as_slice(),
            [VersionConstraint {
                comparator: Comparator::Equal(_),
                ..
            }]
        )
    }
}

impl FromStr for Vls {
    type Err = VlsError;

    /// Try to parse the provided string as [Vls].
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // If the string is empty, return an error
        if s.is_empty() {
            return Err(VlsError::EmptyInput);
        }

        // Early return for Comparator::Any
        if s == "*" {
            return Ok(Self {
                constraints: vec![VersionConstraint::new(Comparator::Any, String::default())],
            });
        }

        // The next two checks are not strictly necessary, as we would try to parse
        // a string containing the vers URI prefix and / or a scheme component as part of
        // the first constraint, which would fail the parsing.
        // As this library is tightly coupled to csaf-rs, we still include them for easier /
        // more informative error handling there, as both indicate this might be a vers string.

        // If the string contains the vers URI prefix, return an error
        if s.starts_with("vers:") {
            return Err(VlsError::ContainsVersPrefix);
        }

        // `/` is not a valid character in the vls grammar, but is used as the scheme delimiter in vers.
        // Its presence indicates the string contains a "<scheme>/" component
        if s.contains('/') {
            return Err(VlsError::ContainsVersioningScheme);
        }

        // Reject any character that is not part of the vls grammar.
        if let Some(invalid) = collect_invalid_characters(s, VlsSpecialCharSet::VlsString) {
            return Err(VlsError::InvalidCharacters(invalid));
        }

        // Split the constraints
        let parts: Vec<&str> = s.split('|').collect();

        // Parse the constraints, generating parsed VersionConstraint or VersionConstraintErrors for each
        let mut constraints = Vec::with_capacity(parts.len());
        let mut constraint_errors: Option<Vec<VersionConstraintError>> = None;

        for part in parts {
            match VersionConstraint::parse(part) {
                Ok(constraint) => constraints.push(constraint),
                Err(errors) => constraint_errors.get_or_insert_default().extend(errors),
            }
        }

        // Report constraint errors before parse errors
        if let Some(constraint_errors) = constraint_errors {
            return Err(VlsError::InvalidConstraintError(constraint_errors));
        }

        // The Comparator::Any represents any version, it is standalone by definition.
        // If Comparator::Any is not the only constraint, return an error
        let has_any = constraints.iter().any(|c| c.comparator == Comparator::Any);
        if has_any && constraints.len() > 1 {
            return Err(VlsError::AnyWithOtherConstraints);
        }

        // Check for duplicate constraints
        let mut seen_versions: HashSet<&str> = HashSet::new();
        let mut duplicate_versions: Option<HashSet<String>> = None;
        for c in &constraints {
            if !seen_versions.insert(&c.version) {
                duplicate_versions
                    .get_or_insert_default()
                    .insert(c.version.clone());
            }
        }
        if let Some(duplicate_versions) = duplicate_versions {
            return Err(VlsError::DuplicateConstraintVersions(duplicate_versions));
        }

        Ok(Self { constraints })
    }
}

impl Display for Vls {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut first = true;
        for c in &self.constraints {
            if !first {
                f.write_str("|")?;
            }
            first = false;
            c.fmt(f)?;
        }
        Ok(())
    }
}
