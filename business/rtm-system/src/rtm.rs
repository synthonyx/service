use foundation::{dispatching::DispatchResult, traits::Dispatchable};

use crate::{config::Config, traits::HealthChecker};

/// Abstraction that holds the business logic for this runtime module.
pub struct RTM<T: Config>(std::marker::PhantomData<T>);
impl<T: Config> RTM<T> {
    // Implement business logic here as private functions.

    /// Returns and Ok result to indicate that the system is healthy.
    ///
    /// RTM functions are reached via dispatched calls, therefore returning a DispatchResult
    /// makes it easy to prolongate results up the call chain.
    fn health_check(_origin: T::Origin) -> DispatchResult<()> {
        // TODO: Implement health checks of dependencies.
        Ok(())
    }
}

// The HealthChecker trait is an adaptor to make functionality available from the outside.
// This is needed to allow interaction between runtime modules without exposing all functionality from RTM,
// but only those that have to be exposed. For this reason all functions of RTM are private.
impl<T: Config> HealthChecker<T> for RTM<T> {
    fn health_check(origin: T::Origin) -> DispatchResult<()> {
        RTM::<T>::health_check(origin)
    }
}

#[derive(Debug, PartialEq)]
/// Holds all parameters for health_check calls and implements the Dispatch trait.
///
/// No types from T are used, but T is provided to keep a consistent dispatch API.
pub struct HealthCheckCall<T: Config>(std::marker::PhantomData<T>);
impl<T: Config> HealthCheckCall<T> {
    /// Constructor for easy instantiation.
    pub fn new() -> HealthCheckCall<T> {
        HealthCheckCall(std::marker::PhantomData)
    }
}

// Implementing Dispatchable makes GreetCall dispatchable. Dispatching is a
// convenient way to move calls between the layers of the service.
//
// We will simply do: `GreetCall::new(String::from("Arjan")).dispatch(origin);`
// to make the request.
impl<T: Config> Dispatchable for HealthCheckCall<T> {
    /// Type for call origins.
    type Origin = T::Origin;

    /// Type for call responses.
    type Response = ();

    fn dispatch(self, origin: Self::Origin) -> DispatchResult<Self::Response> {
        // Execute the business logic.
        RTM::<T>::health_check(origin)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct App;

    impl foundation::Config for App {
        type Origin = ();
    }

    impl Config for App {
        const RTM_ID: &'static str = "SYSTEM";
    }

    #[test]
    fn health_check_call_dispatch_works() {
        assert_eq!(HealthCheckCall::<App>::new().dispatch(()), Ok(()));
    }
}
