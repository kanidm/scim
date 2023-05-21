#[macro_export]
macro_rules! get_string {
    ($value_attrs:expr, $key:expr) => {
        $value_attrs
            .remove($key)
            .ok_or(ScimError::MissingRequiredAttribute)
            .and_then(|v| match v {
                ScimSimpleAttr::String(s) => Ok(s),
                _ => Err(ScimError::InvalidAttribute),
            })
    };
}

#[macro_export]
macro_rules! get_uuid {
    ($value_attrs:expr, $key:expr) => {
        $value_attrs
            .remove($key)
            .ok_or(ScimError::MissingRequiredAttribute)
            .and_then(|v| match v {
                ScimSimpleAttr::String(u) => Uuid::parse_str(&u).map_err(|e| {
                    debug!(?e);
                    ScimError::InvalidAttribute
                }),
                _ => Err(ScimError::InvalidAttribute),
            })
    };
}

#[macro_export]
macro_rules! get_url {
    ($value_attrs:expr, $key:expr) => {
        $value_attrs
            .remove($key)
            .ok_or(ScimError::MissingRequiredAttribute)
            .and_then(|v| match v {
                ScimSimpleAttr::String(u) => Url::parse(&u).map_err(|e| {
                    debug!(?e);
                    ScimError::InvalidAttribute
                }),
                _ => Err(ScimError::InvalidAttribute),
            })
    };
}

#[macro_export]
macro_rules! get_option_string {
    ($value_attrs:expr, $key:expr) => {
        $value_attrs
            .remove($key)
            .map(|v| match v {
                ScimSimpleAttr::String(s) => Ok(s),
                _ => Err(ScimError::InvalidAttribute),
            })
            .transpose()
    };
}

#[macro_export]
macro_rules! get_option_url {
    ($value_attrs:expr, $key:expr) => {
        $value_attrs
            .remove($key)
            .map(|v| match v {
                ScimSimpleAttr::String(u) => Url::parse(&u).map_err(|e| {
                    debug!(?e);
                    ScimError::InvalidAttribute
                }),
                _ => Err(ScimError::InvalidAttribute),
            })
            .transpose()
    };
}

#[macro_export]
macro_rules! get_option_bool {
    ($value_attrs:expr, $key:expr) => {
        $value_attrs
            .remove($key)
            .map(|v| match v {
                ScimSimpleAttr::Bool(b) => Ok(b),
                _ => Err(ScimError::InvalidAttribute),
            })
            .transpose()
    };
}

#[macro_export]
macro_rules! get_option_single_string {
    ($value_attrs:expr, $key:expr) => {
        $value_attrs
            .remove($key)
            .map(|v| match v {
                ScimAttr::SingleSimple(ScimSimpleAttr::String(s)) => Ok(s),
                _ => Err(ScimError::InvalidAttribute),
            })
            .transpose()
    };
}

/*
macro_rules! get_option_single_datetime {
    ($value_attrs:expr, $key:expr) => {
        $value_attrs
            .remove($key)
            .map(|v| match v {
                ScimAttr::SingleSimple(ScimSimpleAttr::String(t)) => {
                    OffsetDateTime::parse(&t, &time::format_description::well_known::Rfc3339).map_err(|e| {
                        debug!(?e);
                        ScimError::InvalidAttribute
                    })
                }
                _ => Err(ScimError::InvalidAttribute),
            })
            .transpose()
    };
}
*/

#[macro_export]
macro_rules! get_option_single_url {
    ($value_attrs:expr, $key:expr) => {
        $value_attrs
            .remove($key)
            .map(|v| match v {
                ScimAttr::SingleSimple(ScimSimpleAttr::String(u)) => Url::parse(&u).map_err(|e| {
                    debug!(?e);
                    ScimError::InvalidAttribute
                }),
                _ => Err(ScimError::InvalidAttribute),
            })
            .transpose()
    };
}

#[macro_export]
macro_rules! try_from_option_single_string {
    ($value_attrs:expr, $key:expr, $ty:ident) => {
        $value_attrs
            .remove($key)
            .map(|v| match v {
                ScimAttr::SingleSimple(ScimSimpleAttr::String(s)) => $ty::try_from(s),
                _ => Err(ScimError::InvalidAttribute),
            })
            .transpose()
    };
}

#[macro_export]
macro_rules! get_option_single_complex {
    ($value_attrs:expr, $key:expr, $ty:ident) => {
        $value_attrs
            .remove($key)
            .map(|v| match v {
                ScimAttr::SingleComplex(sca) => $ty::try_from(sca),
                _ => Err(ScimError::InvalidAttribute),
            })
            .transpose()
    };
}

#[macro_export]
macro_rules! get_single_string {
    ($value_attrs:expr, $key:expr) => {
        $value_attrs
            .remove($key)
            .ok_or(ScimError::MissingRequiredAttribute)
            .and_then(|v| match v {
                ScimAttr::SingleSimple(ScimSimpleAttr::String(s)) => Ok(s),
                _ => Err(ScimError::InvalidAttribute),
            })
    };
}

#[macro_export]
macro_rules! get_single_bool {
    ($value_attrs:expr, $key:expr) => {
        $value_attrs
            .remove($key)
            .ok_or(ScimError::MissingRequiredAttribute)
            .and_then(|v| match v {
                ScimAttr::SingleSimple(ScimSimpleAttr::Bool(b)) => Ok(b),
                _ => Err(ScimError::InvalidAttribute),
            })
    };
}

#[macro_export]
macro_rules! get_option_multi_complex {
    ($value_attrs:expr, $key:expr, $ty:ident) => {
        $value_attrs
            .remove($key)
            .map(|v| match v {
                ScimAttr::MultiComplex(mca) => mca
                    .into_iter()
                    .map($ty::try_from)
                    .collect::<Result<Vec<_>, _>>(),
                _ => Err(ScimError::InvalidAttribute),
            })
            .transpose()?
            .unwrap_or_default()
    };
}

#[macro_export]
macro_rules! set_bool {
    ($value_attrs:expr, $key:expr, $val:expr) => {
        $value_attrs.insert(
            $key.to_string(),
            ScimAttr::SingleSimple(ScimSimpleAttr::Bool($val)),
        );
    };
}

#[macro_export]
macro_rules! set_string {
    ($value_attrs:expr, $key:expr, $val:expr) => {
        $value_attrs.insert(
            $key.to_string(),
            ScimAttr::SingleSimple(ScimSimpleAttr::String($val)),
        );
    };
}

#[macro_export]
macro_rules! set_option_string {
    ($value_attrs:expr, $key:expr, $val:expr) => {
        if let Some(val) = $val {
            $value_attrs.insert(
                $key.to_string(),
                ScimAttr::SingleSimple(ScimSimpleAttr::String(val)),
            );
        }
    };
}

#[macro_export]
macro_rules! set_option_to_string {
    ($value_attrs:expr, $key:expr, $val:expr) => {
        if let Some(val) = $val {
            $value_attrs.insert(
                $key.to_string(),
                ScimAttr::SingleSimple(ScimSimpleAttr::String(val.to_string())),
            );
        }
    };
}

#[macro_export]
macro_rules! set_option_u32 {
    ($value_attrs:expr, $key:expr, $val:expr) => {
        if let Some(val) = $val {
            $value_attrs.insert(
                $key.to_string(),
                ScimAttr::SingleSimple(ScimSimpleAttr::Number(val.into())),
            );
        }
    };
}

#[macro_export]
macro_rules! set_option_complex {
    ($value_attrs:expr, $key:expr, $val:expr) => {
        if let Some(val) = $val {
            $value_attrs.insert($key.to_string(), ScimAttr::SingleComplex(val.into()));
        }
    };
}

#[macro_export]
macro_rules! set_multi_complex {
    ($value_attrs:expr, $key:expr, $val:expr) => {
        if !$val.is_empty() {
            $value_attrs.insert(
                $key.to_string(),
                ScimAttr::MultiComplex($val.into_iter().map(|x| x.into()).collect()),
            );
        }
    };
}
