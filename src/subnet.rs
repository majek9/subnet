#![warn(missing_docs)]

use std::net::Ipv4Addr;

use crate::{Cidr, Network, NetworkError};

///Represents a network address by storing it's network address and CIDR value.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Subnet {
    base_network: Network,
    cidr: Cidr,
    number_of_subnet_bits: u8,
    length: u32,
    current_subnet: u32,
}

impl Subnet {
    /// Create and initialise a new Subnet struct. Takes a network and a new CIDR value. Return a Vector of the subnet networks.
    pub fn new(base_network: Network, cidr: Cidr) -> Result<Self, NetworkError> {
        if base_network.cidr() > cidr {
            return Err(NetworkError::InvalidSubnetCidr);
        }

        let number_of_subnet_bits = *cidr - *base_network.cidr();
        let length = 2u32.pow(number_of_subnet_bits as u32);

        Ok(Self {
            base_network,
            cidr,
            number_of_subnet_bits,
            length,
            current_subnet: 0,
        })
    }

    /// Return the base network from the Subnet.
    pub fn base_network(&self) -> &Network {
        &self.base_network
    }

    /// Return the cidr being applied to the base network.
    pub fn cidr(&self) -> Cidr {
        self.cidr
    }
}

impl Iterator for Subnet {
    type Item = Network;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_subnet >= self.length {
            return None;
        }

        let subnet_network_u32 = u32::from(self.base_network.network_id())
            + (self.current_subnet
                << (32 - *self.base_network.cidr() - self.number_of_subnet_bits));

        self.current_subnet += 1;

        Network::new(Ipv4Addr::from(subnet_network_u32), self.cidr).ok()
    }
}
