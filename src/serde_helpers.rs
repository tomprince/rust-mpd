use serde::{Serialize, Serializer};
use time::{Duration, Tm};

pub trait ExternalSerialize {
    type Target: Serialize;
    fn to_serialize(&self) -> Self::Target;
}

pub fn serialize_external<T: ExternalSerialize, S: Serializer>(t: &T, s: S) -> Result<S::Ok, S::Error> {
    t.to_serialize().serialize(s)
}

impl<T: ExternalSerialize> ExternalSerialize for Option<T> {
    type Target = Option<T::Target>;
    fn to_serialize(&self) -> Self::Target {
        self.as_ref().map(|t| t.to_serialize())
    }
}

impl<T: ExternalSerialize, U: ExternalSerialize> ExternalSerialize for (T, U) {
    type Target = (T::Target, U::Target);
    fn to_serialize(&self) -> Self::Target {
        (self.0.to_serialize(), self.1.to_serialize())
    }
}

impl ExternalSerialize for Tm {
    type Target = i64;
    fn to_serialize(&self) -> Self::Target {
        self.to_timespec().sec
    }
}

impl ExternalSerialize for Duration {
    type Target = i64;
    fn to_serialize(&self) -> Self::Target {
        self.num_seconds()
    }
}
