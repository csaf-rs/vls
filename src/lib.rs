mod comparator;
mod constraint;
mod error;
mod utils;
mod vls;

// public api
pub use comparator::{Comparator, EqualComparatorKind};
pub use constraint::VersionConstraint;
pub use error::{VersionConstraintError, VlsError};
pub use vls::Vls;
