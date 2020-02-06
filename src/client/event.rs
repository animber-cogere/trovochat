use crate::AsOwned;
use std::fmt::Debug;

/// A marker trait for Event subscription
pub trait Event<'a>: crate::internal::private::event_marker::Sealed {
    /// Event message parsing
    type Parsed: crate::Parse<&'a crate::decode::Message<'a>> + AsOwned;
}

/// A trait to convert an Event::Parsed to a 'static type
pub trait EventMapped<'a, T>: crate::internal::private::mapped_marker::Sealed<T>
where
    T: Event<'a>,
{
    /// Event message mapping
    type Owned: Clone + Debug + Send + Sync + 'static;
    /// Converts this to the owned representation
    fn into_owned(data: T::Parsed) -> Self::Owned;
}

impl<'a, T> EventMapped<'a, T> for T
where
    T: Event<'a>,
    <T::Parsed as AsOwned>::Owned: Clone + Debug + Send + Sync + 'static,
{
    type Owned = <T::Parsed as AsOwned>::Owned;
    fn into_owned(data: T::Parsed) -> Self::Owned {
        <T::Parsed as AsOwned>::as_owned(&data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn event_mapped() {
        fn e<'a, T>(msg: &'a crate::decode::Message<'a>) -> T::Owned
        where
            T: Event<'a> + 'static,
            T: EventMapped<'a, T>,
        {
            use crate::Parse as _;
            T::into_owned(T::Parsed::parse(msg).unwrap())
        }

        let msg = crate::decode("PING :1234567890\r\n")
            .next()
            .unwrap()
            .unwrap();

        let msg: crate::messages::Ping<'static> = e::<crate::events::Ping>(&msg);
        assert_eq!(msg.token, "1234567890")
    }
}
