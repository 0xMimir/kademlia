use crate::KEY_SIZE;
use sha2::{Digest, Sha256};
use std::fmt::{Debug, Display, Error, Formatter};

use super::distance::Distance;

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Copy)]
pub struct Key(pub(crate) [u8; KEY_SIZE]);

impl Key {
    pub fn new(input: String) -> Self {
        let result = Sha256::digest(input.as_bytes());
        let hash = result.into();
        Self(hash)
    }

    pub fn distance(&self, key: &Key) -> Distance {
        Distance::new(self, key)
    }
}

impl Display for Key {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        self.0.iter().map(|x| write!(f, "{x:X}")).try_collect()?;
        Ok(())
    }
}

impl Debug for Key {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self, f)
    }
}
