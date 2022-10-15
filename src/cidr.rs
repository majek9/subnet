#![warn(missing_docs)]

use std::ops::Deref;

use crate::NetworkError;

/// Holds CIDR value for subnet
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub struct Cidr {
    inner: u8,
}

impl Cidr {
    /// Create and initialise a new Cidr struct
    pub fn new(cidr: u8) -> Result<Self, NetworkError> {
        if cidr > 32 {
            return Err(NetworkError::CidrOutOfRangeError);
        }
        Ok(Self { inner: cidr })
    }
}

impl Deref for Cidr {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl Cidr {
    /// Generates u32 from cidr.
    /// The Cidr most significant bits of returned u32 are set to one, the rest are set to zero.
    pub(crate) fn to_bitmask(self) -> u32 {
        let cidr_value = self.inner;
        if cidr_value == 0 {
            return 0;
        }

        let mut mask: u32 = 0;
        for _ in 0..cidr_value {
            mask <<= 1;
            mask += 1;
        }
        mask << (32 - cidr_value)
    }
}
