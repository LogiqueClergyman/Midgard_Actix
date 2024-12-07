use bigdecimal::{num_traits::ToPrimitive, BigDecimal, FromPrimitive};
use serde::{
    de::{self, Deserializer, Visitor},
    Serializer,
};
use std::fmt;
use std::str::FromStr;

pub fn parse_i64<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    struct I64Visitor;

    impl<'de> Visitor<'de> for I64Visitor {
        type Value = i64;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string or integer representing an i64")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            v.parse::<i64>().map_err(de::Error::custom)
        }

        fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(v)
        }
    }

    deserializer.deserialize_any(I64Visitor)
}

pub fn parse_f64<'de, D>(deserializer: D) -> Result<BigDecimal, D::Error>
where
    D: de::Deserializer<'de>,
{
    struct F64Visitor;

    impl<'de> Visitor<'de> for F64Visitor {
        type Value = BigDecimal;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string or number representing a BigDecimal")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            BigDecimal::from_str(v).map_err(de::Error::custom)
        }

        fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            BigDecimal::from_f64(v).ok_or_else(|| de::Error::custom("Invalid f64"))
        }
    }

    deserializer.deserialize_any(F64Visitor)
}
pub fn serialize_bigdecimal<S>(value: &BigDecimal, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_f64(value.to_f64().unwrap_or(f64::NAN))
}
