#![allow(warnings)]

use std::str::FromStr;
use serde_json::Value;

lalrpop_mod!(pub filter1);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttrPath {
    // Uri: Option<String>,
    a: String,
    s: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScimFilter {
    Present(AttrPath),
    Equal(AttrPath, Value),

}

#[cfg(test)]
mod test {
    use crate::filter::ScimFilter;
    use crate::filter::filter1;
    use crate::filter::AttrPath;
    use serde_json::Value;

    #[test]
    fn test_scimfilter_attrname() {
        assert!(filter1::AnameParser::new().parse("abcd-_").is_ok());
        assert!(filter1::AnameParser::new().parse("aB-_CD").is_ok());
        assert!(filter1::AnameParser::new().parse("a1-_23").is_ok());
        assert!(filter1::AnameParser::new().parse("-bcd").is_err());
        assert!(filter1::AnameParser::new().parse("_bcd").is_err());
        assert!(filter1::AnameParser::new().parse("0bcd").is_err());
    }

    #[test]
    fn test_scimfilter_attrpath() {
        assert!(filter1::AttrPathParser::new().parse("abcd") == Ok(AttrPath {
            a: "abcd".to_string(),
            s: None
        }));

        assert!(filter1::AttrPathParser::new().parse("abcd.abcd") == Ok(AttrPath {
            a: "abcd".to_string(),
            s: Some("abcd".to_string())
        }));

        assert!(filter1::AttrPathParser::new().parse("abcd.0").is_err());
        assert!(filter1::AttrPathParser::new().parse("abcd._").is_err());
        assert!(filter1::AttrPathParser::new().parse("abcd,0").is_err());
        assert!(filter1::AttrPathParser::new().parse(".abcd").is_err());
    }

    #[test]
    fn test_scimfilter_pres() {
        assert!(filter1::AttrExpParser::new().parse("abcd pr") == Ok(ScimFilter::Present(
            AttrPath {
                a: "abcd".to_string(),
                s: None
            }
        )));
    }

    #[test]
    fn test_scimfilter_eq() {
        let r = filter1::AttrExpParser::new().parse("abcd eq dcba");
        eprintln!("{:?}", r);
        assert!(filter1::AttrExpParser::new().parse("abcd eq dcba") == Ok(ScimFilter::Equal(
            AttrPath {
                a: "abcd".to_string(),
                s: None
            },
            Value::String(
                "dcba".to_string()
            )
        )));
    }
}


