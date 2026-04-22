//! Error types for the `vls` crate.

use std::collections::HashSet;
use thiserror::Error;

/// Errors specific to a single constraint within a VLS string.
#[derive(Error, Debug, PartialEq, Eq)]
pub enum VersionConstraintError {
    /// A constraint segment was empty (e.g. from `||`, a leading `|`, or a trailing `|`).
    #[error("Empty constraint")]
    EmptyConstraint,

    /// The version part of a constraint was empty (e.g. `>=` without a version).
    #[error("Empty version in constraint")]
    EmptyVersion,

    /// The version string contains characters outside the allowed grammar.
    /// See vls::Vls for more details on the grammar.
    #[error("Invalid character(s) in version string: {}", .0.iter().map(|c| format!("'{}'", c.escape_default())).collect::<Vec<_>>().join(", "))]
    InvalidVersionCharacters(Vec<char>),
}

/// Errors that can occur when parsing a vls string.
#[derive(Error, Debug, PartialEq, Eq)]
pub enum VlsError {
    /// The input string was empty.
    #[error("Empty vls input")]
    EmptyInput,

    /// The input contains characters not allowed by the VLS grammar.
    /// See vls::Vls for more details on the grammar.
    #[error("Invalid character(s) in VLS: {}", .0.iter().map(|c| format!("'{}'", c.escape_default())).collect::<Vec<_>>().join(", "))]
    InvalidCharacters(Vec<char>),

    /// The input contains a `vers:` URI prefix, which is not allowed in a VLS string.
    #[error("VLS must not contain a 'vers:' URI prefix")]
    ContainsVersPrefix,

    /// The input most likely contains a `vers` versioning-scheme
    /// component (e.g. `gem/>=2.2.0`), indicated by the presence of the scheme delimiter `/`.
    #[error("VLS must not contain a versioning-scheme component")]
    ContainsVersioningScheme,

    /// The wildcard `*` constraint was combined with other constraints.
    ///
    /// `*` matches all versions and must be the sole constraint in a VLS string.
    #[error("Any ('*') must be the only constraint")]
    AnyWithOtherConstraints,

    /// One or more version strings contain characters outside the allowed grammar.
    #[error("Invalid constraint(s): {}", .0.iter().map(|e| e.to_string()).collect::<Vec<_>>().join(", "))]
    InvalidConstraintError(Vec<VersionConstraintError>),

    /// The input contains duplicate constraint version, irrespective of their comparators.
    #[error("Duplicate constraint(s): {}", .0.iter().map(|s| format!("'{}'", s)).collect::<Vec<_>>().join(", "))]
    DuplicateConstraintVersions(HashSet<String>),
}
