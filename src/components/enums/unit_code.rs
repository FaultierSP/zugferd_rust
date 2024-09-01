use serde::{Serialize,Serializer};

#[derive(Debug,Clone,Copy)]
pub enum UnitCode {
    Piece,
}

impl UnitCode {
    pub fn as_str(&self) -> &str {
        match self {
            UnitCode::Piece => "H87",
        }
    }
}

impl Serialize for UnitCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}