use foundation::dispatching::DispatchResult;

use crate::config::Config;

/// Contains the parts of RTM that should be exposed to the outside world.
///
/// In this case there's just one function, therefore one trait, but in more complex
/// scenarios there will be several traits representing logical groups of functionality,
/// for example: reading, writing, etc. That way other runtime modules can select just
/// the functionality they need when interacting with a runtime module.
pub trait HealthChecker<T: Config> {
    fn health_check(_origin: T::Origin) -> DispatchResult<()>;
}
