use super::{FromIrcMessage, IrcMessage, Str};

use serde::{
    de::{Error, MapAccess, Visitor},
    Deserialize, Deserializer,
};

use std::marker::PhantomData;

impl<'de, 't> Deserialize<'de> for Str<'t> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        <Box<str>>::deserialize(deserializer).map(Str::Owned)
    }
}

pub struct RawVisitor<'a, T>(PhantomData<&'a T>);

impl<'a, T> Default for RawVisitor<'a, T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<'de, 't, T> Visitor<'de> for RawVisitor<'t, T>
where
    T: FromIrcMessage<'t>,
    T::Error: std::error::Error,
{
    type Value = T;
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            formatter,
            "expected to parse '{}'",
            std::any::type_name::<T>()
        )
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        const RAW: &str = "raw";

        let mut raw = None;
        while let Some(key) = map.next_key::<&str>()? {
            if key != RAW {
                map.next_value::<serde::de::IgnoredAny>()?;
                continue;
            }
            let val = map.next_value::<Str<'t>>()?;
            if raw.replace(val).is_some() {
                return Err(A::Error::duplicate_field(RAW));
            }
        }

        let raw = raw.ok_or_else(|| A::Error::missing_field(RAW))?;
        let irc = IrcMessage::parse(raw);

        T::from_irc(irc).map_err(|err| {
            A::Error::custom(format!(
                "cannot parse '{}' because: {}",
                std::any::type_name::<T>(),
                err
            ))
        })
    }
}

#[cfg(test)]
pub(crate) fn round_trip_json<'a, T>(input: &'a str)
where
    T: FromIrcMessage<'a> + PartialEq + std::fmt::Debug,
    T::Error: std::fmt::Debug,
    for<'de> T: ::serde::Serialize + ::serde::Deserialize<'de>,
{
    let (_, msg) = crate::ng::irc::parse_one(input).unwrap();
    let left = T::from_irc(msg).unwrap();
    let json = serde_json::to_string(&left).unwrap();
    let right = serde_json::from_str::<T>(&json).unwrap();
    assert_eq!(left, right)
}
