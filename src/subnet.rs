#![warn(missing_docs)]

use std::cmp::Reverse;
use std::net::Ipv4Addr;

use crate::{Cidr, Network, NetworkError};

///Partitions a Network into subnets by using a Single Length Subnet Mask
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Slsm {
    base_network: Network,
    cidr: Cidr,
    number_of_subnet_bits: u8,
    length: u32,
    current_subnet: u32,
}

impl Slsm {
    /// Create and initialise a new Slsm struct. Takes a network and a new CIDR value.
    /// Return an iterator over the resulting slsm subnets.
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

impl Iterator for Slsm {
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

///Partitions a Network into subnets by using a Variable Length Subnet Mask.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Vlsm {
    base_network: Network,
    required_hosts: Vec<u32>,
    current_subnet: usize,
    next_network_id: Ipv4Addr,
}

impl Vlsm {
    /// Create and initialise a new Vlsm struct. Takes a network and list of required hosts numbers per subnet.
    /// Return an iterator over the resulting subnets. The iterator will return the subnets in the order of greatest number of hosts to the smallest.
    /// It will stop returning Networks when it reaches the end of the required number of hosts, or it does not have enough ip addresses to allocate the next subnet.
    pub fn new(base_network: Network, required_hosts: Vec<u32>) -> Result<Self, NetworkError> {
        let mut required_hosts = required_hosts;
        required_hosts.sort_by_key(|c| Reverse(*c));

        let next_network_id = base_network.network_id();

        Ok(Self {
            base_network,
            required_hosts,
            current_subnet: 0,
            next_network_id,
        })
    }

    /// Return the base network from the Subnet.
    pub fn base_network(&self) -> &Network {
        &self.base_network
    }

    /// Returns the required Cidr to accommodate the required number of hosts.
    fn required_cidr_for_host_count(hosts: u32) -> Result<Cidr, NetworkError> {
        let required_cidr = 32 - (((hosts+2) as f32).log2().ceil()) as u8;
        Cidr::new(required_cidr)
    }
}

impl Iterator for Vlsm {
    type Item = Network;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_subnet >= self.required_hosts.len() {
            return None;
        }

        // If we have left the range of the base network, stop iterating.
        if u32::from(self.next_network_id) >= u32::from(self.base_network.broadcast_address()?) {
            return None;
        }

        let required_cidr =
            Vlsm::required_cidr_for_host_count(self.required_hosts[self.current_subnet as usize])
                .ok()?;

        // If this overflows then the required number of hosts will not fit in the available space.
        let number_of_subnet_bits = required_cidr.checked_sub( *self.base_network.cidr())?;

        //The next network to bbe returned from the iterator
        let result = Network::new(self.next_network_id, required_cidr).ok();


        //Calculate the next network id.
        let subnet_network_u32 = u32::from(self.next_network_id)
            + (1 << (32 - *self.base_network.cidr() - number_of_subnet_bits ));
        self.next_network_id = Ipv4Addr::from(subnet_network_u32);

        self.current_subnet += 1;

        result
    }
}
