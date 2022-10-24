#![deny(warnings)]
#![warn(unused_extern_crates)]
#![deny(clippy::todo)]
#![deny(clippy::unimplemented)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::unreachable)]
#![deny(clippy::await_holding_lock)]
#![deny(clippy::needless_pass_by_value)]
#![deny(clippy::trivially_copy_pass_by_ref)]

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use time::OffsetDateTime;
use tracing::debug;
use url::Url;
use uuid::Uuid;

use serde_json::value::Number;
use serde_json::Map as JsonMap;
use serde_json::Value;

#[macro_use]
pub mod macros;
pub mod constants;
pub mod error;
pub mod group;
pub mod user;

pub mod prelude {
    pub use crate::constants::*;
    pub use crate::error::*;
}

use crate::error::*;

/*
enum Characteristc {
    required,
    canonicalValue,
    caseExact,
    mutability
    returned,
    uniqueness,
    referenceTypes
}
*/

/*
#[derive(Debug)]
enum ScimSimpleAttr {
    String(String),
    Bool(bool),
    Decimal(f64),
    Integer(i64),
    DateTime(OffsetDateTime),
    Binary(Base64UrlSafeData),
    Reference(Url)
}
*/

#[derive(Serialize, Debug, Clone)]
enum ScimSimpleAttr {
    String(String),
    Bool(bool),
    Number(Number),
}

impl TryFrom<Value> for ScimSimpleAttr {
    type Error = ScimError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::String(s) => Ok(ScimSimpleAttr::String(s)),
            Value::Bool(b) => Ok(ScimSimpleAttr::Bool(b)),
            Value::Number(n) => Ok(ScimSimpleAttr::Number(n)),
            // These are error cases
            Value::Null | Value::Object(_) | Value::Array(_) => Err(ScimError::InvalidSingleValue),
        }
    }
}

impl Into<Value> for ScimSimpleAttr {
    fn into(self) -> Value {
        match self {
            ScimSimpleAttr::String(s) => Value::String(s),
            ScimSimpleAttr::Bool(b) => Value::Bool(b),
            ScimSimpleAttr::Number(n) => Value::Number(n),
        }
    }
}

#[derive(Serialize, Debug, Clone)]
struct ScimComplexAttr {
    // I don't think this needs to be multivalue in the simpleAttr part.
    attrs: BTreeMap<String, ScimSimpleAttr>,
}

impl TryFrom<JsonMap<String, Value>> for ScimComplexAttr {
    type Error = ScimError;

    fn try_from(map: JsonMap<String, Value>) -> Result<Self, Self::Error> {
        let attrs = map
            .into_iter()
            .map(|(k, v)| ScimSimpleAttr::try_from(v).map(|sv| (k, sv)))
            .collect::<Result<BTreeMap<_, _>, _>>()?;

        Ok(ScimComplexAttr { attrs })
    }
}

impl TryFrom<Value> for ScimComplexAttr {
    type Error = ScimError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Object(m) => Self::try_from(m),
            _ => Err(ScimError::InconsistentMultiValue),
        }
    }
}

impl Into<Value> for ScimComplexAttr {
    fn into(self) -> Value {
        Value::Object(self.attrs.into_iter().map(|(k, v)| (k, v.into())).collect())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(try_from = "Value", into = "Value")]
enum ScimAttr {
    SingleSimple(ScimSimpleAttr),
    SingleComplex(ScimComplexAttr),
    MultiSimple(Vec<ScimSimpleAttr>),
    MultiComplex(Vec<ScimComplexAttr>),
}

impl TryFrom<Value> for ScimAttr {
    type Error = ScimError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            // Could be either simple or complex
            Value::Array(v) => {
                // We need to peek the array.
                match v.get(0) {
                    None => Err(ScimError::EmptyMultiValue),
                    Some(Value::Array(_)) => Err(ScimError::NestedMultiValue),
                    Some(Value::Object(_)) => {
                        let a = v
                            .into_iter()
                            .map(ScimComplexAttr::try_from)
                            .collect::<Result<Vec<_>, _>>()?;
                        Ok(ScimAttr::MultiComplex(a))
                    }
                    Some(_) => {
                        let a = v
                            .into_iter()
                            .map(ScimSimpleAttr::try_from)
                            .collect::<Result<Vec<_>, _>>()?;
                        Ok(ScimAttr::MultiSimple(a))
                    }
                }
            }
            Value::Object(m) => Ok(ScimAttr::SingleComplex(ScimComplexAttr::try_from(m)?)),
            v => Ok(ScimAttr::SingleSimple(ScimSimpleAttr::try_from(v)?)),
        }
    }
}

impl Into<Value> for ScimAttr {
    fn into(self) -> Value {
        match self {
            ScimAttr::SingleSimple(ssa) => {
                // Into::Value
                ssa.into()
            }
            ScimAttr::SingleComplex(sca) => sca.into(),
            ScimAttr::MultiSimple(msa) => {
                Value::Array(msa.into_iter().map(|ssa| ssa.into()).collect::<Vec<_>>())
            }
            ScimAttr::MultiComplex(mca) => {
                Value::Array(mca.into_iter().map(|sca| sca.into()).collect::<Vec<_>>())
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct ScimMetaRaw {
    resource_type: String,
    created: String,
    last_modified: String,
    location: Url,
    version: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(try_from = "ScimMetaRaw", into = "ScimMetaRaw")]
struct ScimMeta {
    resource_type: String,
    created: OffsetDateTime,
    last_modified: OffsetDateTime,
    location: Url,
    version: String,
}

impl TryFrom<ScimMetaRaw> for ScimMeta {
    type Error = ScimError;

    fn try_from(value: ScimMetaRaw) -> Result<Self, Self::Error> {
        let ScimMetaRaw {
            resource_type,
            created,
            last_modified,
            location,
            version,
        } = value;

        let last_modified =
            OffsetDateTime::parse(&last_modified, time::Format::Rfc3339).map_err(|e| {
                debug!(?e);
                ScimError::InvalidAttribute
            })?;

        let created = OffsetDateTime::parse(&created, time::Format::Rfc3339).map_err(|e| {
            debug!(?e);
            ScimError::InvalidAttribute
        })?;

        Ok(ScimMeta {
            resource_type,
            created,
            last_modified,
            location,
            version,
        })
    }
}

impl Into<ScimMetaRaw> for ScimMeta {
    fn into(self) -> ScimMetaRaw {
        let ScimMeta {
            resource_type,
            created,
            last_modified,
            location,
            version,
        } = self;

        let last_modified = last_modified.format(time::Format::Rfc3339);
        let created = created.format(time::Format::Rfc3339);

        ScimMetaRaw {
            resource_type,
            created,
            last_modified,
            location,
            version,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ScimEntry {
    schemas: Vec<String>,
    id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    external_id: Option<String>,
    meta: ScimMeta,
    #[serde(flatten)]
    attrs: BTreeMap<String, ScimAttr>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::RFC7643_USER;

    #[test]
    fn parse_scim_entry() {
        let _ = tracing_subscriber::fmt::try_init();

        let u: ScimEntry =
            serde_json::from_str(RFC7643_USER).expect("Failed to parse RFC7643_USER");

        tracing::trace!(?u);

        let s = serde_json::to_string_pretty(&u).expect("Failed to serialise RFC7643_USER");
        eprintln!("{}", s);
    }
}
