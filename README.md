# subnet

Subnet is a personal project that I am writing to cement my networking knowledge and further develop and progress my software development skills, in particular with Rust.

It currently only works with IPv4 addresses.  While there are no current plans to expand this to IPv6, it is a possibility.

It currently has the ability to:
- Calculate the network ID from a given IP address and CIDR.
- Calculate the broadcast ID form a given IP address and CIDR.
- Return the first and last host addresses in a Network.
- Calculate the number of available hosts in a Network (It is hardcoded to return a host count of 2 for /31 networks).
- Convert strings in the format "ip_address/cidr" and "ip_address netmask" into a Network object.
- Given a Network and a CIDR, generate and return an iterator over the resulting SLSM subnets.
- Given a Network and a list of required hosts in subnets, generate and return an iterator over the resulting VLVM subnets.
    
Future development plans:
- Test the public interface provided by the library, to ensure it meets my requirements.
- Currently, when generating VLSM subnets, it we run out of IP address to allocate to subnets, then the iterator just stops.  Introduce a test at the start to see if there is space to make the required subnets before returning the iterator.

Additional related projects:
- If I have time, write another program to use this library to expose a REST API, mirroring the abilities of the library.
- Introduce a subnet Trait to cover both SLSM and VSLM subnets.