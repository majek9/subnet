#![warn(missing_docs)]

use std::net::Ipv4Addr;

use crate::{Cidr, Network, NetworkError};

///Represents a network address by storing it's network address and CIDR value.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Subnet {
    base_network: Network,
    subnetworks: Vec<Network>,
}

impl Subnet {
    /// Create and initialise a new Subnet struct. Takes a network and a new CIDR value. Return a Vector of the subnet networks.
    pub(crate) fn new(base_network: Network, cidr: Cidr) -> Result<Self, NetworkError> {
        if base_network.cidr() > cidr {
            return Err(NetworkError::InvalidSubnetCidr);
        }

        let subnetworks = Self::calculate_subnetworks(&base_network, cidr);

        let subnet = Self {
            base_network,
            subnetworks,
        };

        Ok(subnet)
    }

    /// Return the base network from the Subnet.
    pub fn base_network(&self) -> &Network {
        &self.base_network
    }

    /// Return the subnets in the network.
    pub fn subnets(&self) -> &Vec<Network> {
        &self.subnetworks
    }

    /// Calculate and return the subnet networks.
    fn calculate_subnetworks(base_network: &Network, cidr: Cidr) -> Vec<Network> {
        let number_of_subnet_bits = *cidr - *base_network.cidr();
        let number_of_subnets = 2u32.pow(number_of_subnet_bits as u32);

        let mut subnetworks = Vec::with_capacity(number_of_subnets as usize);

        let network_u32 = u32::from(base_network.network_id());

        for subnet_network in 0..number_of_subnets {
            let subnet_network_u32 = network_u32
                + (subnet_network << (32 - *base_network.cidr() - number_of_subnet_bits));

            let network = Ipv4Addr::from(subnet_network_u32);
            subnetworks.push(Network::new(network, cidr).unwrap());
        }

        subnetworks
    }
}
