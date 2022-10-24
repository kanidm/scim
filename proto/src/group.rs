use crate::constants::*;
use crate::error::*;

use crate::{ScimAttr, ScimComplexAttr, ScimEntry, ScimMeta, ScimSimpleAttr};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use tracing::debug;
use url::Url;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Member {
    value: Uuid,
    ref_: Url,
    display: String,
}

impl TryFrom<ScimComplexAttr> for Member {
    type Error = ScimError;

    fn try_from(mut sca: ScimComplexAttr) -> Result<Self, Self::Error> {
        // let type_ = get_option_string!(sca.attrs, "type")?;
        let display = get_string!(sca.attrs, "display")?;
        let value = get_uuid!(sca.attrs, "value")?;
        let ref_ = get_url!(sca.attrs, "$ref")?;

        debug_assert!(sca.attrs.is_empty());

        Ok(Member {
            display,
            value,
            ref_,
        })
    }
}

impl Into<ScimComplexAttr> for Member {
    fn into(self) -> ScimComplexAttr {
        let Member {
            value,
            ref_,
            display,
        } = self;

        let mut attrs = BTreeMap::default();

        attrs.insert("display".to_string(), ScimSimpleAttr::String(display));

        attrs.insert("$ref".to_string(), ScimSimpleAttr::String(ref_.to_string()));

        attrs.insert(
            "value".to_string(),
            ScimSimpleAttr::String(value.to_string()),
        );

        ScimComplexAttr { attrs }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(try_from = "ScimEntry", into = "ScimEntry")]
struct Group {
    id: Uuid,
    external_id: Option<String>,
    meta: ScimMeta,

    display_name: String,
    members: Vec<Member>,
}

impl TryFrom<ScimEntry> for Group {
    type Error = ScimError;

    fn try_from(mut value: ScimEntry) -> Result<Self, Self::Error> {
        // Does it contain our correct schema?
        if !value.schemas.iter().any(|i| i == SCIM_SCHEMA_GROUP) {
            return Err(ScimError::EntryMissingSchema);
        }

        let display_name = get_single_string!(value.attrs, "displayName")?;
        let members = get_option_multi_complex!(value.attrs, "members", Member);

        debug_assert!(value.attrs.is_empty());

        Ok(Group {
            display_name,
            members,
            id: value.id,
            external_id: value.external_id,
            meta: value.meta,
        })
    }
}

impl Into<ScimEntry> for Group {
    fn into(self) -> ScimEntry {
        let Group {
            id,
            external_id,
            meta,
            display_name,
            members,
        } = self;

        let schemas = vec![SCIM_SCHEMA_GROUP.to_string()];

        let mut attrs = BTreeMap::default();

        set_string!(attrs, "displayName", display_name);
        set_multi_complex!(attrs, "members", members);

        ScimEntry {
            schemas,
            id,
            external_id,
            meta,
            attrs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::RFC7643_GROUP;

    #[test]
    fn parse_group() {
        let _ = tracing_subscriber::fmt::try_init();

        let g: Group = serde_json::from_str(RFC7643_GROUP).expect("Failed to parse RFC7643_GROUP");

        tracing::trace!(?g);

        let s = serde_json::to_string_pretty(&g).expect("Failed to serialise RFC7643_USER");
        eprintln!("{}", s);
    }
}
