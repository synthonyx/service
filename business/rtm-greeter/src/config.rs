use foundation::traits::Get;

/// Contains all configurable elements of the runtime module.
///
/// All Config traits should implement the core Config trait, which
/// contains types that should always be accessible anywhere.
pub trait Config: foundation::Config {
    /// The identifier of this element.
    const RTM_ID: &'static str;

    /// Type holding the number of times "hello" shall be repeated in the response.
    type Times: Get<usize>;
}
