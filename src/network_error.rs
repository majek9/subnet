#![warn(missing_docs)]
/// NetworkError enumerates the errors returned by the subnet library.
#[derive(Debug, PartialEq, Eq)]
pub enum NetworkError {
    /// The value given as a CIDR is greater than 32
    CidrOutOfRangeError,
    /// There was an error parsing the str into an IPv4 address
    IPv4AddressError,
    /// Malformed string when parsing str
    ParsingError,
    /// The subnet cidr is less than the network cidr
    InvalidSubnetCidr,
}
