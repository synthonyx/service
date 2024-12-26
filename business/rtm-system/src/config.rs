/// Contains all configurable elements of the runtime module.
///
/// All Config traits should implement the core Config trait, which
/// contains types that should always be accessible anywhere.
pub trait Config: foundation::Config {
    // The identifier of this element.
    const RTM_ID: &'static str;
}
