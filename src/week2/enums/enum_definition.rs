use weblab::weblab;

#[weblab(programming_assignment)]
/// A rust enum allows you to store data that is one of many declared variants. Each variant can store its own data. //TODO improve this explanation
/// For this assignment, define two enums, `LoginState` and `IpAddress`.
///
/// Login state is one of the following:
/// - `LoggedOut`, which stores no additional data
/// - `LoggedIn`, which stores the userid of the logged in person as `u64`
///
/// IpAddress is one of the following:
/// - `Ipv4`, which stores an address of type `[u8; 4]`
/// - `Ipv6`, which stores an address of type `[u8; 16]`
///
/// Make sure the names of your definition match the names above exactly.
#[weblab(title = "Enum Definition")]
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
