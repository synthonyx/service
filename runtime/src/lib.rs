pub use foundation::primitives::ConstUsize;
pub use foundation::traits::Dispatchable;
pub use rtm_greeter::{GreetCall, GreetResponse};

#[derive(Debug, PartialEq)]
/// Composite element for runtime modules.
pub struct Runtime;

// Every Config of runtime modules use foundation::Config as its base implementation,
// therefore Runtime should implement it, as it will be used as the value for generics
// of calls.
impl foundation::Config for Runtime {
    type Origin = ();
}

// Implement the system runtime module for health check support.
impl rtm_system::Config for Runtime {
    const RTM_ID: &'static str = "SYSTEM";
}

// Implement the Config trait of each runtime module that will be enabled for the runtime.

// Enable rtm_greeter by implementing the Config trait.
impl rtm_greeter::Config for Runtime {
    const RTM_ID: &'static str = "UPPER";
    type Times = ConstUsize<3>;
}

/// Aliases for easier composing of calls.
pub type SystemHealthCheckCall = rtm_system::HealthCheckCall<Runtime>;
pub type GreeterGreetCall = GreetCall<Runtime>;

/// Aliases for easier compising of responses.
pub type GreeterGreetResponse = GreetResponse<Runtime>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runtime_works() {
        assert!(SystemHealthCheckCall::new().dispatch(()).is_ok());

        let result = GreeterGreetCall::new("Luna".into()).dispatch(()).unwrap();
        assert_eq!(
            result,
            GreeterGreetResponse::new("Hello, hello, hello, Luna!".into())
        );
    }
}
