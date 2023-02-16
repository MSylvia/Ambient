/// An identifier for an entity in the world.
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct EntityId {
    #[doc(hidden)]
    pub id0: u64,
    #[doc(hidden)]
    pub id1: u64,
}
impl EntityId {
    #[doc(hidden)]
    pub const fn new(id0: u64, id1: u64) -> Self {
        Self { id0, id1 }
    }

    #[doc(hidden)]
    pub fn from_base64(encoded: &str) -> Self {
        let len = data_encoding::BASE64URL_NOPAD
            .decode_len(encoded.len())
            .unwrap();

        if len >= 16 {
            panic!("base64 entityid is more than 16 bytes");
        }

        let mut bytes = [0u8; 16];
        data_encoding::BASE64URL_NOPAD
            .decode_mut(encoded.as_bytes(), &mut bytes)
            .unwrap();

        Self {
            id0: u64::from_le_bytes(bytes[0..8].try_into().unwrap()),
            id1: u64::from_le_bytes(bytes[8..].try_into().unwrap()),
        }
    }
}
impl std::fmt::Display for EntityId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", ((self.id0 as u128) << 64) + self.id1 as u128)
    }
}
impl std::fmt::Debug for EntityId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}
