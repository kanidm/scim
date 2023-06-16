use crate::constants::*;
use crate::error::*;
use crate::{ScimAttr, ScimComplexAttr, ScimEntry, ScimMeta, ScimSimpleAttr};
use base64urlsafedata::Base64UrlSafeData;
use std::collections::BTreeMap;
use std::fmt;
use url::Url;
use uuid::Uuid;

use tracing::debug;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Name {
    // The full name including all middle names and titles
    formatted: Option<String>,
    family_name: Option<String>,
    given_name: Option<String>,
    middle_name: Option<String>,
    honorific_prefix: Option<String>,
    honorific_suffix: Option<String>,
}

impl TryFrom<ScimComplexAttr> for Name {
    type Error = ScimError;

    fn try_from(mut value: ScimComplexAttr) -> Result<Self, Self::Error> {
        let formatted = get_option_string!(value.attrs, "formatted")?;
        let family_name = get_option_string!(value.attrs, "familyName")?;
        let given_name = get_option_string!(value.attrs, "givenName")?;
        let middle_name = get_option_string!(value.attrs, "middleName")?;
        let honorific_prefix = get_option_string!(value.attrs, "honorificPrefix")?;
        let honorific_suffix = get_option_string!(value.attrs, "honorificSuffix")?;
        // Fully consumed.
        debug_assert!(value.attrs.is_empty());

        Ok(Name {
            formatted,
            family_name,
            given_name,
            middle_name,
            honorific_prefix,
            honorific_suffix,
        })
    }
}

impl Into<ScimComplexAttr> for Name {
    fn into(self) -> ScimComplexAttr {
        let Name {
            formatted,
            family_name,
            given_name,
            middle_name,
            honorific_prefix,
            honorific_suffix,
        } = self;

        let mut attrs = BTreeMap::default();

        if let Some(formatted) = formatted {
            attrs.insert("formatted".to_string(), ScimSimpleAttr::String(formatted));
        };

        if let Some(family_name) = family_name {
            attrs.insert(
                "familyName".to_string(),
                ScimSimpleAttr::String(family_name),
            );
        };

        if let Some(given_name) = given_name {
            attrs.insert("givenName".to_string(), ScimSimpleAttr::String(given_name));
        };

        if let Some(middle_name) = middle_name {
            attrs.insert(
                "middleName".to_string(),
                ScimSimpleAttr::String(middle_name),
            );
        };

        if let Some(honorific_prefix) = honorific_prefix {
            attrs.insert(
                "honorificPrefix".to_string(),
                ScimSimpleAttr::String(honorific_prefix),
            );
        };

        if let Some(honorific_suffix) = honorific_suffix {
            attrs.insert(
                "honorificSuffix".to_string(),
                ScimSimpleAttr::String(honorific_suffix),
            );
        };

        ScimComplexAttr { attrs }
    }
}

/*
// https://datatracker.ietf.org/doc/html/rfc7231#section-5.3.5
//
// https://www.iana.org/assignments/language-subtag-registry/language-subtag-registry
// Same as locale?
#[derive(Serialize, Deserialize, Debug, Clone)]
enum Language {
    en,
}
*/

// https://datatracker.ietf.org/doc/html/rfc5646
#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Clone)]
enum Locale {
    en,
    en_AU,
    en_US,
    de,
    de_DE,
}

impl TryFrom<String> for Locale {
    type Error = ScimError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "en" => Ok(Locale::en),
            "en-AU" => Ok(Locale::en_AU),
            "en-US" => Ok(Locale::en_US),
            "de" => Ok(Locale::de),
            "de-DE" => Ok(Locale::de_DE),
            l => {
                debug!(?l, "invalid locale");
                Err(ScimError::UnknownLocale)
            }
        }
    }
}

impl fmt::Display for Locale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Locale::en => write!(f, "en"),
            Locale::en_AU => write!(f, "en-AU"),
            Locale::en_US => write!(f, "en-US"),
            Locale::de => write!(f, "de"),
            Locale::de_DE => write!(f, "de-DE"),
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Clone)]
enum Timezone {
    Australia_Brisbane,
    America_Los_Angeles,
}

impl TryFrom<String> for Timezone {
    type Error = ScimError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "Australia/Brisbane" => Ok(Timezone::Australia_Brisbane),
            "America/Los_Angeles" => Ok(Timezone::America_Los_Angeles),
            l => {
                debug!(?l, "invalid locale");
                Err(ScimError::UnknownLocale)
            }
        }
    }
}

impl fmt::Display for Timezone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Timezone::Australia_Brisbane => write!(f, "Australia/Brisbane"),
            Timezone::America_Los_Angeles => write!(f, "America/Los_Angeles"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MultiValueAttr {
    pub type_: Option<String>,
    pub primary: Option<bool>,
    pub display: Option<String>,
    pub ref_: Option<Url>,
    pub value: String,
}

impl TryFrom<ScimComplexAttr> for MultiValueAttr {
    type Error = ScimError;

    fn try_from(mut sca: ScimComplexAttr) -> Result<Self, Self::Error> {
        let type_ = get_option_string!(sca.attrs, "type")?;
        let primary = get_option_bool!(sca.attrs, "primary")?;
        let display = get_option_string!(sca.attrs, "display")?;
        let value = get_string!(sca.attrs, "value")?;
        let ref_ = get_option_url!(sca.attrs, "$ref")?;

        debug_assert!(sca.attrs.is_empty());

        Ok(MultiValueAttr {
            type_,
            primary,
            display,
            value,
            ref_,
        })
    }
}

impl Into<ScimComplexAttr> for MultiValueAttr {
    fn into(self) -> ScimComplexAttr {
        let MultiValueAttr {
            type_,
            primary,
            display,
            ref_,
            value,
        } = self;

        let mut attrs = BTreeMap::default();

        if let Some(type_) = type_ {
            attrs.insert("type".to_string(), ScimSimpleAttr::String(type_));
        }

        if let Some(primary) = primary {
            attrs.insert("primary".to_string(), ScimSimpleAttr::Bool(primary));
        }

        if let Some(display) = display {
            attrs.insert("display".to_string(), ScimSimpleAttr::String(display));
        }

        if let Some(ref_) = ref_ {
            attrs.insert("$ref".to_string(), ScimSimpleAttr::String(ref_.to_string()));
        }

        attrs.insert("value".to_string(), ScimSimpleAttr::String(value));

        ScimComplexAttr { attrs }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Photo {
    type_: Option<String>,
    primary: Option<bool>,
    display: Option<String>,
    ref_: Option<Url>,
    value: Url,
}

impl TryFrom<ScimComplexAttr> for Photo {
    type Error = ScimError;

    fn try_from(sca: ScimComplexAttr) -> Result<Self, Self::Error> {
        let MultiValueAttr {
            type_,
            primary,
            display,
            ref_,
            value,
        } = MultiValueAttr::try_from(sca)?;

        let value = Url::parse(&value).map_err(|e| {
            debug!(?e);
            ScimError::InvalidAttribute
        })?;

        Ok(Photo {
            type_,
            primary,
            display,
            value,
            ref_,
        })
    }
}

impl Into<ScimComplexAttr> for Photo {
    fn into(self) -> ScimComplexAttr {
        let Photo {
            type_,
            primary,
            display,
            ref_,
            value,
        } = self;

        let mut attrs = BTreeMap::default();

        if let Some(type_) = type_ {
            attrs.insert("type".to_string(), ScimSimpleAttr::String(type_));
        }

        if let Some(primary) = primary {
            attrs.insert("primary".to_string(), ScimSimpleAttr::Bool(primary));
        }

        if let Some(display) = display {
            attrs.insert("display".to_string(), ScimSimpleAttr::String(display));
        }

        if let Some(ref_) = ref_ {
            attrs.insert("$ref".to_string(), ScimSimpleAttr::String(ref_.to_string()));
        }

        attrs.insert(
            "value".to_string(),
            ScimSimpleAttr::String(value.to_string()),
        );

        ScimComplexAttr { attrs }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Binary {
    type_: Option<String>,
    primary: Option<bool>,
    display: Option<String>,
    ref_: Option<Url>,
    value: Base64UrlSafeData,
}

impl TryFrom<ScimComplexAttr> for Binary {
    type Error = ScimError;

    fn try_from(sca: ScimComplexAttr) -> Result<Self, Self::Error> {
        let MultiValueAttr {
            type_,
            primary,
            display,
            ref_,
            value,
        } = MultiValueAttr::try_from(sca)?;

        let value = Base64UrlSafeData::try_from(value.as_str()).map_err(|e| {
            debug!(?e);
            ScimError::InvalidAttribute
        })?;

        Ok(Binary {
            type_,
            primary,
            display,
            value,
            ref_,
        })
    }
}

impl Into<ScimComplexAttr> for Binary {
    fn into(self) -> ScimComplexAttr {
        let Binary {
            type_,
            primary,
            display,
            ref_,
            value,
        } = self;

        let mut attrs = BTreeMap::default();

        if let Some(type_) = type_ {
            attrs.insert("type".to_string(), ScimSimpleAttr::String(type_));
        }

        if let Some(primary) = primary {
            attrs.insert("primary".to_string(), ScimSimpleAttr::Bool(primary));
        }

        if let Some(display) = display {
            attrs.insert("display".to_string(), ScimSimpleAttr::String(display));
        }

        if let Some(ref_) = ref_ {
            attrs.insert("$ref".to_string(), ScimSimpleAttr::String(ref_.to_string()));
        }

        attrs.insert(
            "value".to_string(),
            ScimSimpleAttr::String(value.to_string()),
        );

        ScimComplexAttr { attrs }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Address {
    type_: Option<String>,
    primary: Option<bool>,
    formatted: Option<String>,
    street_address: Option<String>,
    locality: Option<String>,
    region: Option<String>,
    postal_code: Option<String>,
    country: Option<String>,
}

impl TryFrom<ScimComplexAttr> for Address {
    type Error = ScimError;

    fn try_from(mut sca: ScimComplexAttr) -> Result<Self, Self::Error> {
        let type_ = get_option_string!(sca.attrs, "type")?;
        let primary = get_option_bool!(sca.attrs, "primary")?;

        let formatted = get_option_string!(sca.attrs, "formatted")?;
        let street_address = get_option_string!(sca.attrs, "streetAddress")?;
        let locality = get_option_string!(sca.attrs, "locality")?;
        let region = get_option_string!(sca.attrs, "region")?;
        let postal_code = get_option_string!(sca.attrs, "postalCode")?;
        let country = get_option_string!(sca.attrs, "country")?;

        debug_assert!(sca.attrs.is_empty());

        Ok(Address {
            type_,
            primary,
            formatted,
            street_address,
            locality,
            region,
            postal_code,
            country,
        })
    }
}

impl Into<ScimComplexAttr> for Address {
    fn into(self) -> ScimComplexAttr {
        let Address {
            type_,
            primary,
            formatted,
            street_address,
            locality,
            region,
            postal_code,
            country,
        } = self;

        let mut attrs = BTreeMap::default();

        if let Some(type_) = type_ {
            attrs.insert("type".to_string(), ScimSimpleAttr::String(type_));
        }

        if let Some(primary) = primary {
            attrs.insert("primary".to_string(), ScimSimpleAttr::Bool(primary));
        }

        if let Some(formatted) = formatted {
            attrs.insert("formatted".to_string(), ScimSimpleAttr::String(formatted));
        }

        if let Some(street_address) = street_address {
            attrs.insert(
                "streetAddress".to_string(),
                ScimSimpleAttr::String(street_address),
            );
        }

        if let Some(locality) = locality {
            attrs.insert("locality".to_string(), ScimSimpleAttr::String(locality));
        }

        if let Some(region) = region {
            attrs.insert("region".to_string(), ScimSimpleAttr::String(region));
        }

        if let Some(postal_code) = postal_code {
            attrs.insert(
                "postalCode".to_string(),
                ScimSimpleAttr::String(postal_code),
            );
        }

        if let Some(country) = country {
            attrs.insert("country".to_string(), ScimSimpleAttr::String(country));
        }

        ScimComplexAttr { attrs }
    }
}

/*
#[derive(Serialize, Deserialize, Debug, Clone)]
enum Membership {
    Direct,
    Indirect,
}
*/

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Group {
    value: Uuid,
    ref_: Url,
    display: String,
    type_: Option<String>,
}

impl TryFrom<ScimComplexAttr> for Group {
    type Error = ScimError;

    fn try_from(mut sca: ScimComplexAttr) -> Result<Self, Self::Error> {
        let type_ = get_option_string!(sca.attrs, "type")?;
        let display = get_string!(sca.attrs, "display")?;
        let value = get_uuid!(sca.attrs, "value")?;
        let ref_ = get_url!(sca.attrs, "$ref")?;

        debug_assert!(sca.attrs.is_empty());

        Ok(Group {
            type_,
            display,
            value,
            ref_,
        })
    }
}

impl Into<ScimComplexAttr> for Group {
    fn into(self) -> ScimComplexAttr {
        let Group {
            type_,
            value,
            ref_,
            display,
        } = self;

        let mut attrs = BTreeMap::default();

        if let Some(type_) = type_ {
            attrs.insert("type".to_string(), ScimSimpleAttr::String(type_));
        }

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
struct User {
    id: Uuid,
    external_id: Option<String>,
    meta: Option<ScimMeta>,
    // required, must be unique, string.
    user_name: String,
    // Components of the users name.
    name: Option<Name>,
    // required, must be unique, string.
    display_name: Option<String>,
    nick_name: Option<String>,
    profile_url: Option<Url>,
    title: Option<String>,
    user_type: Option<String>,
    preferred_language: Option<Locale>,
    locale: Option<Locale>,
    // https://datatracker.ietf.org/doc/html/rfc6557
    // How can we validate this? https://docs.rs/iana-time-zone/0.1.51/iana_time_zone/fn.get_timezone.html
    timezone: Option<Timezone>,
    active: bool,
    password: Option<String>,
    emails: Vec<MultiValueAttr>,
    phone_numbers: Vec<MultiValueAttr>,
    ims: Vec<MultiValueAttr>,
    photos: Vec<Photo>,
    addresses: Vec<Address>,
    groups: Vec<Group>,
    entitlements: Vec<MultiValueAttr>,
    roles: Vec<MultiValueAttr>,
    x509certificates: Vec<Binary>,
}

impl TryFrom<ScimEntry> for User {
    type Error = ScimError;

    fn try_from(mut value: ScimEntry) -> Result<Self, Self::Error> {
        // Does it contain our correct schema?
        if !value.schemas.iter().any(|i| i == SCIM_SCHEMA_USER) {
            return Err(ScimError::EntryMissingSchema);
        }

        // Build up each value.
        let user_name = get_single_string!(value.attrs, "userName")?;

        let name = get_option_single_complex!(value.attrs, "name", Name)?;
        let display_name = get_option_single_string!(value.attrs, "displayName")?;
        let nick_name = get_option_single_string!(value.attrs, "nickName")?;
        let profile_url = get_option_single_url!(value.attrs, "profileUrl")?;
        let title = get_option_single_string!(value.attrs, "title")?;
        let user_type = get_option_single_string!(value.attrs, "userType")?;
        let preferred_language =
            try_from_option_single_string!(value.attrs, "preferredLanguage", Locale)?;
        let locale = try_from_option_single_string!(value.attrs, "locale", Locale)?;
        let timezone = try_from_option_single_string!(value.attrs, "timezone", Timezone)?;
        let active = get_single_bool!(value.attrs, "active")?;
        let password = get_option_single_string!(value.attrs, "password")?;

        let emails = get_option_multi_complex!(value.attrs, "emails", MultiValueAttr);
        let ims = get_option_multi_complex!(value.attrs, "ims", MultiValueAttr);
        let phone_numbers = get_option_multi_complex!(value.attrs, "phoneNumbers", MultiValueAttr);
        let photos = get_option_multi_complex!(value.attrs, "photos", Photo);

        let addresses = get_option_multi_complex!(value.attrs, "photos", Address);

        let groups = get_option_multi_complex!(value.attrs, "groups", Group);
        let entitlements = get_option_multi_complex!(value.attrs, "entitlements", MultiValueAttr);
        let roles = get_option_multi_complex!(value.attrs, "roles", MultiValueAttr);
        let x509certificates = get_option_multi_complex!(value.attrs, "x509Certificates", Binary);

        Ok(User {
            id: value.id,
            external_id: value.external_id,
            meta: value.meta,
            user_name,
            name,
            display_name,
            nick_name,
            profile_url,
            title,
            user_type,
            preferred_language,
            locale,
            timezone,
            active,
            password,
            emails,
            ims,
            phone_numbers,
            photos,
            addresses,
            groups,
            entitlements,
            roles,
            x509certificates,
        })
    }
}

impl Into<ScimEntry> for User {
    fn into(self) -> ScimEntry {
        let User {
            id,
            external_id,
            meta,
            user_name,
            name,
            display_name,
            nick_name,
            profile_url,
            title,
            user_type,
            preferred_language,
            locale,
            timezone,
            active,
            password,
            emails,
            ims,
            phone_numbers,
            photos,
            addresses,
            groups,
            entitlements,
            roles,
            x509certificates,
        } = self;

        let schemas = vec![SCIM_SCHEMA_USER.to_string()];

        let mut attrs = BTreeMap::default();

        set_string!(attrs, "userName", user_name);
        set_option_complex!(attrs, "name", name);
        set_option_string!(attrs, "displayName", display_name);
        set_option_string!(attrs, "nickName", nick_name);
        set_option_to_string!(attrs, "profileUrl", profile_url);
        set_option_string!(attrs, "title", title);
        set_option_string!(attrs, "userType", user_type);
        set_option_to_string!(attrs, "preferredLanguage", preferred_language);
        set_option_to_string!(attrs, "locale", locale);
        set_option_to_string!(attrs, "timezone", timezone);
        set_bool!(attrs, "active", active);
        set_option_to_string!(attrs, "password", password);

        set_multi_complex!(attrs, "emails", emails);
        set_multi_complex!(attrs, "phoneNumbers", phone_numbers);
        set_multi_complex!(attrs, "ims", ims);
        set_multi_complex!(attrs, "photos", photos);
        set_multi_complex!(attrs, "addresses", addresses);
        set_multi_complex!(attrs, "groups", groups);
        set_multi_complex!(attrs, "entitlements", entitlements);
        set_multi_complex!(attrs, "roles", roles);
        set_multi_complex!(attrs, "x509certificates", x509certificates);

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
    use crate::constants::RFC7643_USER;

    #[test]
    fn parse_user() {
        let _ = tracing_subscriber::fmt::try_init();

        let u: User = serde_json::from_str(RFC7643_USER).expect("Failed to parse RFC7643_USER");

        tracing::trace!(?u);

        let s = serde_json::to_string_pretty(&u).expect("Failed to serialise RFC7643_USER");
        eprintln!("{}", s);
    }
}
