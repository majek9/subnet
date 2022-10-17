pub use cidr::Cidr;
pub use network::Network;
pub use network_error::NetworkError;
pub use subnet::Subnet;

mod cidr;
mod network;
mod network_error;
mod subnet;

#[cfg(test)]
mod tests {
    use std::net::Ipv4Addr;

    use crate::cidr::Cidr;
    use crate::network::Network;
    use crate::network_error::NetworkError;

    #[test]
    fn create_network() {
        let ip_address = Ipv4Addr::new(0, 0, 0, 0);
        let cidr = Cidr::new(0).unwrap();
        let network = Network::new(ip_address, cidr).unwrap();
        let got_network_id = network.network_id();
        let want_network_id = Ipv4Addr::new(0, 0, 0, 0);
        assert_eq!(got_network_id, want_network_id,);
        assert_eq!(*network.cidr(), *cidr,);

        let ip_address = Ipv4Addr::new(10, 1, 5, 32);
        let cidr = Cidr::new(0).unwrap();
        let network = Network::new(ip_address, cidr).unwrap();
        let got_network_id = network.network_id();
        let want_network_id = Ipv4Addr::new(0, 0, 0, 0);
        assert_eq!(got_network_id, want_network_id,);
        assert_eq!(*network.cidr(), *cidr,);

        let ip_address = Ipv4Addr::new(192, 52, 224, 255);
        let cidr = Cidr::new(1).unwrap();
        let network = Network::new(ip_address, cidr).unwrap();
        let got_network_id = network.network_id();
        let want_network_id = Ipv4Addr::new(128, 0, 0, 0);
        assert_eq!(got_network_id, want_network_id,);
        assert_eq!(*network.cidr(), *cidr,);

        let ip_address = Ipv4Addr::new(180, 44, 125, 57);
        let cidr = Cidr::new(4).unwrap();
        let network = Network::new(ip_address, cidr).unwrap();
        let got_network_id = network.network_id();
        let want_network_id = Ipv4Addr::new(176, 0, 0, 0);
        assert_eq!(got_network_id, want_network_id,);
        assert_eq!(*network.cidr(), *cidr,);

        let ip_address = Ipv4Addr::new(1, 37, 12, 54);
        let cidr = Cidr::new(12).unwrap();
        let network = Network::new(ip_address, cidr).unwrap();
        let got_network_id = network.network_id();
        let want_network_id = Ipv4Addr::new(1, 32, 0, 0);
        assert_eq!(got_network_id, want_network_id,);
        assert_eq!(*network.cidr(), *cidr,);

        let ip_address = Ipv4Addr::new(152, 207, 23, 5);
        let cidr = Cidr::new(15).unwrap();
        let network = Network::new(ip_address, cidr).unwrap();
        let got_network_id = network.network_id();
        let want_network_id = Ipv4Addr::new(152, 206, 0, 0);
        assert_eq!(got_network_id, want_network_id,);
        assert_eq!(*network.cidr(), *cidr,);

        let ip_address = Ipv4Addr::new(152, 206, 23, 5);
        let cidr = Cidr::new(15).unwrap();
        let network = Network::new(ip_address, cidr).unwrap();
        let got_network_id = network.network_id();
        let want_network_id = Ipv4Addr::new(152, 206, 0, 0);
        assert_eq!(got_network_id, want_network_id,);
        assert_eq!(*network.cidr(), *cidr,);

        let ip_address = Ipv4Addr::new(1, 65, 157, 51);
        let cidr = Cidr::new(16).unwrap();
        let network = Network::new(ip_address, cidr).unwrap();
        let got_network_id = network.network_id();
        let want_network_id = Ipv4Addr::new(1, 65, 0, 0);
        assert_eq!(got_network_id, want_network_id,);
        assert_eq!(*network.cidr(), *cidr,);

        let ip_address = Ipv4Addr::new(8, 55, 156, 87);
        let cidr = Cidr::new(17).unwrap();
        let network = Network::new(ip_address, cidr).unwrap();
        let got_network_id = network.network_id();
        let want_network_id = Ipv4Addr::new(8, 55, 128, 0);
        assert_eq!(got_network_id, want_network_id,);
        assert_eq!(*network.cidr(), *cidr,);

        let ip_address = Ipv4Addr::new(8, 55, 56, 87);
        let cidr = Cidr::new(17).unwrap();
        let network = Network::new(ip_address, cidr).unwrap();
        let got_network_id = network.network_id();
        let want_network_id = Ipv4Addr::new(8, 55, 0, 0);
        assert_eq!(got_network_id, want_network_id,);
        assert_eq!(*network.cidr(), *cidr,);

        let ip_address = Ipv4Addr::new(125, 25, 18, 95);
        let cidr = Cidr::new(29).unwrap();
        let network = Network::new(ip_address, cidr).unwrap();
        let got_network_id = network.network_id();
        let want_network_id = Ipv4Addr::new(125, 25, 18, 88);
        assert_eq!(got_network_id, want_network_id,);
        assert_eq!(*network.cidr(), *cidr,);

        let ip_address = Ipv4Addr::new(33, 5, 87, 23);
        let cidr = Cidr::new(30).unwrap();
        let network = Network::new(ip_address, cidr).unwrap();
        let got_network_id = network.network_id();
        let want_network_id = Ipv4Addr::new(33, 5, 87, 20);
        assert_eq!(got_network_id, want_network_id,);
        assert_eq!(*network.cidr(), *cidr,);

        let ip_address = Ipv4Addr::new(45, 187, 24, 123);
        let cidr = Cidr::new(31).unwrap();
        let network = Network::new(ip_address, cidr).unwrap();
        let got_network_id = network.network_id();
        let want_network_id = Ipv4Addr::new(45, 187, 24, 122);
        assert_eq!(got_network_id, want_network_id,);
        assert_eq!(*network.cidr(), *cidr,);

        let ip_address = Ipv4Addr::new(45, 187, 24, 124);
        let cidr = Cidr::new(31).unwrap();
        let network = Network::new(ip_address, cidr).unwrap();
        let got_network_id = network.network_id();
        let want_network_id = Ipv4Addr::new(45, 187, 24, 124);
        assert_eq!(got_network_id, want_network_id,);
        assert_eq!(*network.cidr(), *cidr,);

        let ip_address = Ipv4Addr::new(45, 24, 11, 56);
        let cidr = Cidr::new(32).unwrap();
        let network = Network::new(ip_address, cidr).unwrap();
        let got_network_id = network.network_id();
        let want_network_id = Ipv4Addr::new(45, 24, 11, 56);
        assert_eq!(got_network_id, want_network_id,);
        assert_eq!(*network.cidr(), *cidr,);

        let ip_address = Ipv4Addr::new(255, 255, 255, 255);
        let cidr = Cidr::new(32).unwrap();
        let network = Network::new(ip_address, cidr).unwrap();
        let got_network_id = network.network_id();
        let want_network_id = Ipv4Addr::new(255, 255, 255, 255);
        assert_eq!(got_network_id, want_network_id,);
        assert_eq!(*network.cidr(), *cidr,);
    }

    #[test]
    fn attempt_to_create_invalid_cidr() {
        let cidr = Cidr::new(33);
        assert_eq!(cidr, Err(NetworkError::CidrOutOfRangeError),);

        let cidr = Cidr::new(134);
        assert_eq!(cidr, Err(NetworkError::CidrOutOfRangeError),);

        let cidr = Cidr::new(255);
        assert_eq!(cidr, Err(NetworkError::CidrOutOfRangeError),);
    }

    #[test]
    fn attempt_create_network_from_invalid_str() {
        let s = "";
        let network = Network::try_from(s);
        assert_eq!(network, Err(NetworkError::ParsingError));

        let s = "45.25.247.15/34";
        let network = Network::try_from(s);
        assert_eq!(network, Err(NetworkError::CidrOutOfRangeError));

        let s = "154.251.15.84/";
        let network = Network::try_from(s);
        assert_eq!(network, Err(NetworkError::ParsingError));

        let s = "154.251.15.84";
        let network = Network::try_from(s);
        assert_eq!(network, Err(NetworkError::ParsingError));

        let s = "/12";
        let network = Network::try_from(s);
        assert_eq!(network, Err(NetworkError::IPv4AddressError));

        let s = "55.54.15.36.154/1";
        let network = Network::try_from(s);
        assert_eq!(network, Err(NetworkError::IPv4AddressError));

        let s = "155.5.15/1";
        let network = Network::try_from(s);
        assert_eq!(network, Err(NetworkError::IPv4AddressError));

        let s = "154.554.12.55/1";
        let network = Network::try_from(s);
        assert_eq!(network, Err(NetworkError::IPv4AddressError));

        let s = "4e5.57.5.1/23";
        let network = Network::try_from(s);
        assert_eq!(network, Err(NetworkError::IPv4AddressError));

        let s = "45.57.5.1/2d3";
        let network = Network::try_from(s);
        assert_eq!(network, Err(NetworkError::ParsingError));

        let s = "12.554.12.55 255.255.0.0";
        let network = Network::try_from(s);
        assert_eq!(network, Err(NetworkError::IPv4AddressError));

        let s = "155.66.12.3.5 255.255.255.0";
        let network = Network::try_from(s);
        assert_eq!(network, Err(NetworkError::IPv4AddressError));

        let s = "10.18.235 255.255.255.0";
        let network = Network::try_from(s);
        assert_eq!(network, Err(NetworkError::IPv4AddressError));

        let s = "127.1x8.25.9 255.255.128.21";
        let network = Network::try_from(s);
        assert_eq!(network, Err(NetworkError::IPv4AddressError));

        let s = "123.45.67.89 ";
        let network = Network::try_from(s);
        assert_eq!(network, Err(NetworkError::ParsingError));

        let s = "10.31.44.98 255.255.255.128.0";
        let network = Network::try_from(s);
        assert_eq!(network, Err(NetworkError::IPv4AddressError));

        let s = "54.41.135.254 128.255.255.0";
        let network = Network::try_from(s);
        assert_eq!(network, Err(NetworkError::ParsingError));

        let s = "127.18.25.9 255.255.128";
        let network = Network::try_from(s);
        assert_eq!(network, Err(NetworkError::IPv4AddressError));

        let s = "127.1x8.25.9 255.255.128.2e1";
        let network = Network::try_from(s);
        assert_eq!(network, Err(NetworkError::IPv4AddressError));
    }

    #[test]
    fn create_network_from_str_with_cidr() {
        let s = "0.0.0.0/0";
        let network = Network::try_from(s);
        assert_eq!(
            network,
            Network::new(Ipv4Addr::new(0, 0, 0, 0), Cidr::new(0).unwrap())
        );
        let s = "10.1.5.21/0";
        let network = Network::try_from(s);
        assert_eq!(
            network,
            Network::new(Ipv4Addr::new(0, 0, 0, 0), Cidr::new(0).unwrap())
        );
        let s = "192.52.224.255/1";
        let network = Network::try_from(s);
        assert_eq!(
            network,
            Network::new(Ipv4Addr::new(128, 0, 0, 0), Cidr::new(1).unwrap())
        );
        let s = "180.44.125.57/4";
        let network = Network::try_from(s);
        assert_eq!(
            network,
            Network::new(Ipv4Addr::new(176, 0, 0, 0), Cidr::new(4).unwrap())
        );
        let s = "1.37.12.54/12";
        let network = Network::try_from(s);
        assert_eq!(
            network,
            Network::new(Ipv4Addr::new(1, 32, 0, 0), Cidr::new(12).unwrap())
        );
        let s = "152.207.23.5/15";
        let network = Network::try_from(s);
        assert_eq!(
            network,
            Network::new(Ipv4Addr::new(152, 206, 0, 0), Cidr::new(15).unwrap())
        );
        let s = "152.206.23.5/15";
        let network = Network::try_from(s);
        assert_eq!(
            network,
            Network::new(Ipv4Addr::new(152, 206, 0, 0), Cidr::new(15).unwrap())
        );
        let s = "1.65.157.51/16";
        let network = Network::try_from(s);
        assert_eq!(
            network,
            Network::new(Ipv4Addr::new(1, 65, 0, 0), Cidr::new(16).unwrap())
        );
        let s = "8.55.156.87/17";
        let network = Network::try_from(s);
        assert_eq!(
            network,
            Network::new(Ipv4Addr::new(8, 55, 158, 0), Cidr::new(17).unwrap())
        );
        let s = "8.55.56.87/17";
        let network = Network::try_from(s);
        assert_eq!(
            network,
            Network::new(Ipv4Addr::new(8, 55, 0, 0), Cidr::new(17).unwrap())
        );
        let s = "125.25.18.95/29";
        let network = Network::try_from(s);
        assert_eq!(
            network,
            Network::new(Ipv4Addr::new(125, 25, 18, 88), Cidr::new(29).unwrap())
        );
        let s = "33.5.87.23/30";
        let network = Network::try_from(s);
        assert_eq!(
            network,
            Network::new(Ipv4Addr::new(33, 5, 87, 20), Cidr::new(30).unwrap())
        );
        let s = "45.187.24.123/31";
        let network = Network::try_from(s);
        assert_eq!(
            network,
            Network::new(Ipv4Addr::new(45, 187, 24, 122), Cidr::new(31).unwrap())
        );
        let s = "45.187.24.124/31";
        let network = Network::try_from(s);
        assert_eq!(
            network,
            Network::new(Ipv4Addr::new(45, 187, 24, 124), Cidr::new(31).unwrap())
        );
        let s = "45.24.11.56/32";
        let network = Network::try_from(s);
        assert_eq!(
            network,
            Network::new(Ipv4Addr::new(45, 24, 11, 56), Cidr::new(32).unwrap())
        );
        let s = "255.255.255.255/32";
        let network = Network::try_from(s);
        assert_eq!(
            network,
            Network::new(Ipv4Addr::new(255, 255, 255, 255), Cidr::new(32).unwrap())
        );
    }

    #[test]
    fn create_network_from_str_with_netmask() {
        let s = "0.0.0.0 0.0.0.0";
        let network = Network::try_from(s);
        assert_eq!(
            network,
            Network::new(Ipv4Addr::new(0, 0, 0, 0), Cidr::new(0).unwrap())
        );
        let s = "10.1.5.32 0.0.0.0";
        let network = Network::try_from(s);
        assert_eq!(
            network,
            Network::new(Ipv4Addr::new(128, 0, 0, 0), Cidr::new(0).unwrap())
        );
        let s = "192.52.224.255 128.0.0.0";
        let network = Network::try_from(s);
        assert_eq!(
            network,
            Network::new(Ipv4Addr::new(128, 0, 0, 0), Cidr::new(1).unwrap())
        );
        let s = "180.44.125.57 240.0.0.0";
        let network = Network::try_from(s);
        assert_eq!(
            network,
            Network::new(Ipv4Addr::new(176, 0, 0, 0), Cidr::new(4).unwrap())
        );
        let s = "1.37.12.54 255.240.0.0";
        let network = Network::try_from(s);
        assert_eq!(
            network,
            Network::new(Ipv4Addr::new(1, 32, 0, 0), Cidr::new(12).unwrap())
        );
        let s = "152.207.23.5 255.254.0.0";
        let network = Network::try_from(s);
        assert_eq!(
            network,
            Network::new(Ipv4Addr::new(152, 206, 0, 0), Cidr::new(15).unwrap())
        );
        let s = "152.206.23.5 255.254.0.0";
        let network = Network::try_from(s);
        assert_eq!(
            network,
            Network::new(Ipv4Addr::new(152, 206, 0, 0), Cidr::new(15).unwrap())
        );
        let s = "1.65.157.51 255.255.0.0";
        let network = Network::try_from(s);
        assert_eq!(
            network,
            Network::new(Ipv4Addr::new(1, 65, 0, 0), Cidr::new(16).unwrap())
        );
        let s = "8.55.156.87 255.255.128.0";
        let network = Network::try_from(s);
        assert_eq!(
            network,
            Network::new(Ipv4Addr::new(8, 55, 158, 0), Cidr::new(17).unwrap())
        );
        let s = "8.55.56.87 255.255.128.0";
        let network = Network::try_from(s);
        assert_eq!(
            network,
            Network::new(Ipv4Addr::new(8, 55, 0, 0), Cidr::new(17).unwrap())
        );
        let s = "125.25.18.95 255.255.255.248";
        let network = Network::try_from(s);
        assert_eq!(
            network,
            Network::new(Ipv4Addr::new(125, 25, 18, 88), Cidr::new(29).unwrap())
        );
        let s = "33.5.87.23 255.255.255.252";
        let network = Network::try_from(s);
        assert_eq!(
            network,
            Network::new(Ipv4Addr::new(33, 5, 87, 20), Cidr::new(30).unwrap())
        );
        let s = "45.187.24.123 255.255.255.254";
        let network = Network::try_from(s);
        assert_eq!(
            network,
            Network::new(Ipv4Addr::new(45, 187, 24, 122), Cidr::new(31).unwrap())
        );
        let s = "45.187.24.124 255.255.255.254";
        let network = Network::try_from(s);
        assert_eq!(
            network,
            Network::new(Ipv4Addr::new(45, 187, 24, 124), Cidr::new(31).unwrap())
        );
        let s = "45.24.11.56 255.255.255.255";
        let network = Network::try_from(s);
        assert_eq!(
            network,
            Network::new(Ipv4Addr::new(45, 24, 11, 56), Cidr::new(32).unwrap())
        );
        let s = "255.255.255.255 255.255.255.255";
        let network = Network::try_from(s);
        assert_eq!(
            network,
            Network::new(Ipv4Addr::new(255, 255, 255, 255), Cidr::new(32).unwrap())
        );
    }

    #[test]
    fn convert_cidr_to_bitmask() {
        assert_eq!(Cidr::new(0).unwrap().to_bitmask(), 0);
        assert_eq!(Cidr::new(1).unwrap().to_bitmask(), 2_147_483_648);
        assert_eq!(Cidr::new(2).unwrap().to_bitmask(), 3_221_225_472);
        assert_eq!(Cidr::new(16).unwrap().to_bitmask(), 4_294_901_760);
        assert_eq!(Cidr::new(30).unwrap().to_bitmask(), 4_294_967_292);
        assert_eq!(Cidr::new(31).unwrap().to_bitmask(), 4_294_967_294);
        assert_eq!(Cidr::new(32).unwrap().to_bitmask(), 4_294_967_295);
    }

    #[test]
    fn get_first_host_address() {
        let network = Network::new(Ipv4Addr::new(0, 0, 0, 0), Cidr::new(0).unwrap()).unwrap();
        assert_eq!(
            network.first_host_address(),
            Some(Ipv4Addr::new(0, 0, 0, 1))
        );

        let network = Network::new(Ipv4Addr::new(0, 0, 0, 0), Cidr::new(16).unwrap()).unwrap();
        assert_eq!(
            network.first_host_address(),
            Some(Ipv4Addr::new(0, 0, 0, 1))
        );

        let network = Network::new(Ipv4Addr::new(0, 0, 0, 0), Cidr::new(32).unwrap()).unwrap();
        assert_eq!(network.first_host_address(), None);

        let network =
            Network::new(Ipv4Addr::new(255, 255, 255, 255), Cidr::new(0).unwrap()).unwrap();
        assert_eq!(
            network.first_host_address(),
            Some(Ipv4Addr::new(0, 0, 0, 1))
        );

        let network =
            Network::new(Ipv4Addr::new(255, 255, 255, 255), Cidr::new(19).unwrap()).unwrap();
        assert_eq!(
            network.first_host_address(),
            Some(Ipv4Addr::new(255, 255, 224, 1))
        );

        let network =
            Network::new(Ipv4Addr::new(255, 255, 255, 255), Cidr::new(32).unwrap()).unwrap();
        assert_eq!(network.first_host_address(), None);

        let network = Network::new(Ipv4Addr::new(129, 5, 1, 33), Cidr::new(1).unwrap()).unwrap();
        assert_eq!(
            network.first_host_address(),
            Some(Ipv4Addr::new(128, 0, 0, 1))
        );

        let network = Network::new(Ipv4Addr::new(25, 255, 254, 1), Cidr::new(23).unwrap()).unwrap();
        assert_eq!(
            network.first_host_address(),
            Some(Ipv4Addr::new(25, 255, 254, 1))
        );

        let network = Network::new(Ipv4Addr::new(172, 18, 25, 9), Cidr::new(24).unwrap()).unwrap();
        assert_eq!(
            network.first_host_address(),
            Some(Ipv4Addr::new(172, 18, 25, 1))
        );

        let network =
            Network::new(Ipv4Addr::new(172, 18, 25, 137), Cidr::new(25).unwrap()).unwrap();
        assert_eq!(
            network.first_host_address(),
            Some(Ipv4Addr::new(172, 18, 25, 129))
        );

        let network = Network::new(Ipv4Addr::new(10, 55, 8, 13), Cidr::new(30).unwrap()).unwrap();
        assert_eq!(
            network.first_host_address(),
            Some(Ipv4Addr::new(10, 55, 8, 13))
        );

        let network = Network::new(Ipv4Addr::new(10, 55, 8, 12), Cidr::new(31).unwrap()).unwrap();
        assert_eq!(
            network.first_host_address(),
            Some(Ipv4Addr::new(10, 55, 8, 12))
        );

        let network = Network::new(Ipv4Addr::new(10, 55, 8, 13), Cidr::new(31).unwrap()).unwrap();
        assert_eq!(
            network.first_host_address(),
            Some(Ipv4Addr::new(10, 55, 8, 12))
        );
    }

    #[test]
    fn get_last_host_address() {
        let network = Network::new(Ipv4Addr::new(0, 0, 0, 0), Cidr::new(0).unwrap()).unwrap();
        assert_eq!(
            network.last_host_address(),
            Some(Ipv4Addr::new(255, 255, 255, 254))
        );

        let network = Network::new(Ipv4Addr::new(0, 0, 0, 0), Cidr::new(19).unwrap()).unwrap();
        assert_eq!(
            network.last_host_address(),
            Some(Ipv4Addr::new(0, 0, 31, 254))
        );

        let network = Network::new(Ipv4Addr::new(0, 0, 0, 0), Cidr::new(32).unwrap()).unwrap();
        assert_eq!(network.last_host_address(), None);

        let network =
            Network::new(Ipv4Addr::new(255, 255, 255, 255), Cidr::new(0).unwrap()).unwrap();
        assert_eq!(
            network.last_host_address(),
            Some(Ipv4Addr::new(255, 255, 255, 254))
        );

        let network =
            Network::new(Ipv4Addr::new(255, 255, 255, 255), Cidr::new(19).unwrap()).unwrap();
        assert_eq!(
            network.last_host_address(),
            Some(Ipv4Addr::new(255, 255, 255, 254))
        );

        let network =
            Network::new(Ipv4Addr::new(255, 255, 255, 255), Cidr::new(32).unwrap()).unwrap();
        assert_eq!(network.last_host_address(), None);

        let network = Network::new(Ipv4Addr::new(129, 5, 1, 33), Cidr::new(1).unwrap()).unwrap();
        assert_eq!(
            network.last_host_address(),
            Some(Ipv4Addr::new(255, 255, 255, 254))
        );

        let network = Network::new(Ipv4Addr::new(25, 255, 254, 1), Cidr::new(23).unwrap()).unwrap();
        assert_eq!(
            network.last_host_address(),
            Some(Ipv4Addr::new(25, 255, 255, 254))
        );

        let network = Network::new(Ipv4Addr::new(172, 18, 25, 9), Cidr::new(24).unwrap()).unwrap();
        assert_eq!(
            network.last_host_address(),
            Some(Ipv4Addr::new(172, 18, 25, 254))
        );

        let network =
            Network::new(Ipv4Addr::new(172, 18, 25, 137), Cidr::new(25).unwrap()).unwrap();
        assert_eq!(
            network.last_host_address(),
            Some(Ipv4Addr::new(172, 18, 25, 254))
        );

        let network = Network::new(Ipv4Addr::new(10, 55, 8, 13), Cidr::new(30).unwrap()).unwrap();
        assert_eq!(
            network.last_host_address(),
            Some(Ipv4Addr::new(10, 55, 8, 14))
        );

        let network = Network::new(Ipv4Addr::new(10, 55, 8, 12), Cidr::new(31).unwrap()).unwrap();
        assert_eq!(
            network.last_host_address(),
            Some(Ipv4Addr::new(10, 55, 8, 13))
        );

        let network = Network::new(Ipv4Addr::new(10, 55, 8, 13), Cidr::new(31).unwrap()).unwrap();
        assert_eq!(
            network.last_host_address(),
            Some(Ipv4Addr::new(10, 55, 8, 13))
        );
    }

    #[test]
    fn get_broadcast_address() {
        let network = Network::new(Ipv4Addr::new(0, 0, 0, 0), Cidr::new(0).unwrap()).unwrap();
        assert_eq!(
            network.broadcast_address(),
            Some(Ipv4Addr::new(255, 255, 255, 255))
        );

        let network = Network::new(Ipv4Addr::new(0, 0, 0, 0), Cidr::new(19).unwrap()).unwrap();
        assert_eq!(
            network.broadcast_address(),
            Some(Ipv4Addr::new(0, 0, 31, 255))
        );

        let network = Network::new(Ipv4Addr::new(0, 0, 0, 0), Cidr::new(32).unwrap()).unwrap();
        assert_eq!(network.broadcast_address(), None);

        let network =
            Network::new(Ipv4Addr::new(255, 255, 255, 255), Cidr::new(0).unwrap()).unwrap();
        assert_eq!(
            network.broadcast_address(),
            Some(Ipv4Addr::new(255, 255, 255, 255))
        );

        let network =
            Network::new(Ipv4Addr::new(255, 255, 255, 255), Cidr::new(19).unwrap()).unwrap();
        assert_eq!(
            network.broadcast_address(),
            Some(Ipv4Addr::new(255, 255, 255, 255))
        );

        let network =
            Network::new(Ipv4Addr::new(255, 255, 255, 255), Cidr::new(32).unwrap()).unwrap();
        assert_eq!(network.broadcast_address(), None);

        let network = Network::new(Ipv4Addr::new(129, 5, 1, 33), Cidr::new(2).unwrap()).unwrap();
        assert_eq!(
            network.broadcast_address(),
            Some(Ipv4Addr::new(191, 255, 255, 255))
        );

        let network = Network::new(Ipv4Addr::new(25, 255, 254, 1), Cidr::new(23).unwrap()).unwrap();
        assert_eq!(
            network.broadcast_address(),
            Some(Ipv4Addr::new(25, 255, 255, 255))
        );

        let network = Network::new(Ipv4Addr::new(172, 18, 25, 9), Cidr::new(24).unwrap()).unwrap();
        assert_eq!(
            network.broadcast_address(),
            Some(Ipv4Addr::new(172, 18, 25, 255))
        );

        let network =
            Network::new(Ipv4Addr::new(172, 18, 25, 137), Cidr::new(25).unwrap()).unwrap();
        assert_eq!(
            network.broadcast_address(),
            Some(Ipv4Addr::new(172, 18, 25, 255))
        );

        let network =
            Network::new(Ipv4Addr::new(172, 18, 25, 167), Cidr::new(27).unwrap()).unwrap();
        assert_eq!(
            network.broadcast_address(),
            Some(Ipv4Addr::new(172, 18, 25, 191))
        );

        let network = Network::new(Ipv4Addr::new(10, 55, 8, 13), Cidr::new(30).unwrap()).unwrap();
        assert_eq!(
            network.broadcast_address(),
            Some(Ipv4Addr::new(10, 55, 8, 15))
        );

        let network = Network::new(Ipv4Addr::new(10, 55, 8, 12), Cidr::new(31).unwrap()).unwrap();
        assert_eq!(
            network.broadcast_address(),
            Some(Ipv4Addr::new(10, 55, 8, 13))
        );

        let network = Network::new(Ipv4Addr::new(10, 55, 8, 13), Cidr::new(31).unwrap()).unwrap();
        assert_eq!(
            network.broadcast_address(),
            Some(Ipv4Addr::new(10, 55, 8, 13))
        );
    }
    #[test]
    fn create_subnet() {
        let ip_address = Ipv4Addr::new(145, 52, 12, 8);
        let cidr = Cidr::new(24).unwrap();
        let network = Network::new(ip_address, cidr).unwrap();
        let subnet_cidr = Cidr::new(26).unwrap();
        let subnet = network.create_subnet(subnet_cidr).unwrap();
        let got_base_network_id = subnet.base_network().network_id();
        let want_base_network_id = Ipv4Addr::new(145, 52, 12, 0);
        let want_subnets = vec![
            Network::new(Ipv4Addr::new(145, 52, 12, 0), Cidr::new(26).unwrap()).unwrap(),
            Network::new(Ipv4Addr::new(145, 52, 12, 64), Cidr::new(26).unwrap()).unwrap(),
            Network::new(Ipv4Addr::new(145, 52, 12, 128), Cidr::new(26).unwrap()).unwrap(),
            Network::new(Ipv4Addr::new(145, 52, 12, 192), Cidr::new(26).unwrap()).unwrap(),
        ];
        let got_subnets = subnet.subnets();
        assert_eq!(got_base_network_id, want_base_network_id);
        assert_eq!(*got_subnets, want_subnets);

        let ip_address = Ipv4Addr::new(253, 65, 181, 18);
        let cidr = Cidr::new(25).unwrap();
        let network = Network::new(ip_address, cidr).unwrap();
        let subnet_cidr = Cidr::new(25).unwrap();
        let subnet = network.create_subnet(subnet_cidr).unwrap();
        let got_base_network_id = subnet.base_network().network_id();
        let want_base_network_id = Ipv4Addr::new(253, 65, 181, 0);
        let want_subnets =
            vec![Network::new(Ipv4Addr::new(253, 65, 181, 18), Cidr::new(25).unwrap()).unwrap()];
        let got_subnets = subnet.subnets();
        assert_eq!(got_base_network_id, want_base_network_id);
        assert_eq!(*got_subnets, want_subnets);

        let ip_address = Ipv4Addr::new(1, 255, 255, 8);
        let cidr = Cidr::new(14).unwrap();
        let network = Network::new(ip_address, cidr).unwrap();
        let subnet_cidr = Cidr::new(18).unwrap();
        let subnet = network.create_subnet(subnet_cidr).unwrap();
        let got_base_network_id = subnet.base_network().network_id();
        let want_base_network_id = Ipv4Addr::new(1, 252, 0, 0);
        let want_subnets = vec![
            Network::new(Ipv4Addr::new(1, 252, 0, 0), Cidr::new(18).unwrap()).unwrap(),
            Network::new(Ipv4Addr::new(1, 252, 64, 0), Cidr::new(18).unwrap()).unwrap(),
            Network::new(Ipv4Addr::new(1, 252, 128, 0), Cidr::new(18).unwrap()).unwrap(),
            Network::new(Ipv4Addr::new(1, 252, 192, 0), Cidr::new(18).unwrap()).unwrap(),
            Network::new(Ipv4Addr::new(1, 253, 0, 0), Cidr::new(18).unwrap()).unwrap(),
            Network::new(Ipv4Addr::new(1, 253, 64, 0), Cidr::new(18).unwrap()).unwrap(),
            Network::new(Ipv4Addr::new(1, 253, 128, 0), Cidr::new(18).unwrap()).unwrap(),
            Network::new(Ipv4Addr::new(1, 253, 192, 0), Cidr::new(18).unwrap()).unwrap(),
            Network::new(Ipv4Addr::new(1, 254, 0, 0), Cidr::new(18).unwrap()).unwrap(),
            Network::new(Ipv4Addr::new(1, 254, 64, 0), Cidr::new(18).unwrap()).unwrap(),
            Network::new(Ipv4Addr::new(1, 254, 128, 0), Cidr::new(18).unwrap()).unwrap(),
            Network::new(Ipv4Addr::new(1, 254, 192, 0), Cidr::new(18).unwrap()).unwrap(),
            Network::new(Ipv4Addr::new(1, 255, 0, 0), Cidr::new(18).unwrap()).unwrap(),
            Network::new(Ipv4Addr::new(1, 255, 64, 0), Cidr::new(18).unwrap()).unwrap(),
            Network::new(Ipv4Addr::new(1, 255, 128, 0), Cidr::new(18).unwrap()).unwrap(),
            Network::new(Ipv4Addr::new(1, 255, 192, 0), Cidr::new(18).unwrap()).unwrap(),
        ];
        let got_subnets = subnet.subnets();
        assert_eq!(got_base_network_id, want_base_network_id);
        assert_eq!(*got_subnets, want_subnets);

        let ip_address = Ipv4Addr::new(145, 52, 12, 8);
        let cidr = Cidr::new(16).unwrap();
        let network = Network::new(ip_address, cidr).unwrap();
        let subnet_cidr = Cidr::new(15).unwrap();
        let subnet = network.create_subnet(subnet_cidr);
        let subnet_error = subnet.unwrap_err();
        assert_eq!(subnet_error, NetworkError::InvalidSubnetCidr);

        let ip_address = Ipv4Addr::new(145, 52, 12, 8);
        let cidr = Cidr::new(32).unwrap();
        let network = Network::new(ip_address, cidr).unwrap();
        let subnet_cidr = Cidr::new(0).unwrap();
        let subnet = network.create_subnet(subnet_cidr);
        let subnet_error = subnet.unwrap_err();
        assert_eq!(subnet_error, NetworkError::InvalidSubnetCidr);
    }
}
