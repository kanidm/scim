use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
pub enum ScimError {
    EntryMissingSchema,
    InconsistentMultiValue,
    EmptyMultiValue,
    NestedMultiValue,
    InvalidSingleValue,
    MissingRequiredAttribute,
    InvalidAttribute,
    UnknownLocale,
    UnknownTimezone,
}

impl fmt::Display for ScimError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
