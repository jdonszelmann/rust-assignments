use weblab::weblab;

#[weblab(programming_assignment)]
/// A rust enum also allows to store data in one of many declared variants. Each variant can have a different data type.
/// For this assignment, define two enums, `LoginState` and `IpAddress`.
///
/// Login state is one of the following:
/// - `LoggedOut`, which stores no additional data
/// - `LoggedIn`, which stores the userid of the logged in person as `u64`
///
/// IP addresses are used in the internet to identify computers. There are two different types of IP addresses,
/// IPv4 which is a 32-bit number, and the improved and larger version, IPv6 which is a 128-bit number.
/// In this assigment, an `IpAddress` is represented as one of the following:
/// - `Ipv4`, which stores an address of type `[u8; 4]`
/// - `Ipv6`, which stores an address of type `[u8; 16]`
///
/// Make sure the names of your definition exactly match the names above.
#[weblab(title = "Complex Enum Definitions")]
mod assignment {
    #[weblab(solution)]
    mod solution {
        pub enum LoginState {
            LoggedOut,
            LoggedIn(u64),
        }

        pub enum IpAddress {
            Ipv4([u8; 4]),
            Ipv6([u8; 16]),
        }
    }

    #[allow(unused_variables)]
    #[weblab(solution_template)]
    mod solution_template {
        /// TODO Add variants
        pub enum LoginState {}

        /// TODO Add variants
        pub enum IpAddress {}
    }

    #[weblab(test)]
    mod test {
        use super::solution::*;
        use weblab::{solution_only, template_only};

        //Both tests are public, since otherwise I can imagine debugging this to be a hell
        //No point in keeping these secret

        #[test]
        fn test_login_state() {
            let _ = LoginState::LoggedOut;
            let _ = LoginState::LoggedIn(0u64);
        }

        #[test]
        fn ip_address() {
            let _ = IpAddress::Ipv4([0u8; 4]);
            let _ = IpAddress::Ipv6([0u8; 16]);
        }
    }
}
