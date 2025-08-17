use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy)]
pub struct Language {
    pub name: &'static str,
    pub exts: &'static [&'static str],
}

impl Serialize for Language {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("Language", 2)?;
        state.serialize_field("name", self.name)?;
        state.serialize_field("exts", self.exts)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for Language {
    fn deserialize<D>(_: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        unimplemented!("Language deserialization not supported");
    }
}
