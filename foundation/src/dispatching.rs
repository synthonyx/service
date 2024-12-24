#[derive(Clone, Debug, PartialEq)]
pub enum DispatchError {
    RTM {
        rtm_id: &'static str,
        reason: String,
    },
    /// All other errors, with some explanatory string.
    Other(&'static str),
}

impl std::fmt::Display for DispatchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::RTM { rtm_id: _, reason } => reason.to_owned(),
            Self::Other(err) => err.to_string(),
        };
        write!(f, "{:?}", msg)
    }
}

impl std::error::Error for DispatchError {}

pub type DispatchResult<T> = Result<T, DispatchError>;
