#![warn(missing_docs)]

/// Utility for working calculating network subnets.
use std::net::Ipv4Addr;

use crate::Cidr;
use crate::NetworkError;

/// Represents a IPv4 Network by storing it's Network address and CIDR value.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Network {
    network_id: Ipv4Addr,
    cidr: Cidr,
}

impl Network {
    /// Create and initialise a new Network struct. Takes a std::net::Ipv4Addr and a Cidr.  Convert Ipv4Addr into the network address before storing and returning the struct.
    pub fn new(ip_address: Ipv4Addr, cidr: Cidr) -> Result<Self, NetworkError> {
        let bitmask = cidr.to_bitmask();
        let id = u32::from(ip_address) & bitmask;
        Ok(Self {
            network_id: Ipv4Addr::from(id),
            cidr,
        })
    }

    /// Return the broadcast address of the subnet
    pub fn broadcast_address(&self) -> Option<Ipv4Addr> {
        if *self.cidr == 32 {
            return None;
        }

        let bitmask = !self.cidr.to_bitmask();
        let broadcast = u32::from(self.network_id) | bitmask;
        Some(Ipv4Addr::from(broadcast))
    }

    /// Return the first host address in the subnet
    pub fn first_host_address(&self) -> Option<Ipv4Addr> {
        if *self.cidr == 32 {
            return None;
        }

        if *self.cidr == 31 {
            return Some(self.network_id());
        }

        let mut octets = self.network_id.octets();
        octets[3] += 1;
        Some(Ipv4Addr::from(octets))
    }

    /// Return the last host address in the subnet
    pub fn last_host_address(&self) -> Option<Ipv4Addr> {
        if *self.cidr == 32 {
            return None;
        }

        if *self.cidr == 31 {
            return self.broadcast_address();
        }

        let mut octets = self.broadcast_address()?.octets();
        octets[3] -= 1;
        Some(Ipv4Addr::from(octets))
    }

    /// Return the number of hosts available in the subnet
    pub fn number_of_hosts(&self) -> u32 {
        if *self.cidr == 32 {
            return 0;
        }

        if *self.cidr == 31 {
            return 2;
        }
        2_u32.pow(32 - *self.cidr as u32) - 2
    }
}

impl TryFrom<&str> for Network {
    type Error = NetworkError;

    fn try_from(s: &str) -> Result<Network, NetworkError> {
        let s = s.trim();

        //Test for CIDR format
        if let Some(pos) = s.find('/') {
            let ip_addr = Network::try_str_to_ipv4addr(&s[0..pos])?;
            if let Ok(num) = s[(pos + 1)..].parse::<u8>() {
                return Network::new(ip_addr, Cidr::new(num)?);
            }
        }

        //Test for ip_address + netmask format
        if let Some(pos) = s.find(' ') {
            let ip_addr = Network::try_str_to_ipv4addr(&s[0..pos])?;
            let bitmask = u32::from(Network::try_str_to_ipv4addr(&s[(pos + 1)..])?);
            let leading_ones = bitmask.leading_ones();
            let trailing_zeros = bitmask.trailing_zeros();
            if leading_ones + trailing_zeros == 32 {
                return Network::new(ip_addr, Cidr::new(leading_ones as u8)?);
            }
        }
        Err(NetworkError::ParsingError)
    }
}

impl Network {
    /// Return the Network Id.
    pub fn network_id(&self) -> Ipv4Addr {
        self.network_id
    }

    /// Return the CIDR value.
    pub fn cidr(&self) -> Cidr {
        self.cidr
    }
}

impl Network {
    /// Parse str and convert to IPv4 address
    /// Expects str to contain 4 decimal octets seperated by dots
    fn try_str_to_ipv4addr(s: &str) -> Result<Ipv4Addr, NetworkError> {
        let mut octets = Vec::with_capacity(4);
        for octet in s.split('.') {
            match octet.parse::<u8>() {
                Ok(num) => octets.push(num),
                Err(..) => {
                    return Err(NetworkError::IPv4AddressError);
                }
            }
        }
        let octets: Result<[u8; 4], _> = octets.try_into();
        if let Ok(arr) = octets {
            Ok(Ipv4Addr::from(arr))
        } else {
            Err(NetworkError::IPv4AddressError)
        }
    }
}
