#[derive(Default, Eq, PartialEq, Debug)]
pub enum Game {
    #[default] None,
    UmaPD
}

impl Game {
    pub fn key(&self) -> Option<MD5Key> {
        match self {
            Self::None => None,
            Self::UmaPD => Some(MD5Key::UmaPD)
        }
    }
}

#[derive(Default)]
pub enum MD5Key {
    #[default] UmaPD
}

impl MD5Key {
    pub fn data(self) -> &'static [u8] {
        match self {
            Self::UmaPD => include_bytes!("../assets/umapd.key")
        }
    }
}