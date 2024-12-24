/// Contains functionality all origins should provide.
// TODO: Implement, as it's just a dummy implementation for now.
pub trait Origin {
    fn is_origin() -> bool {
        true
    }
}

// Temporaily implement Origin for ().
// TODO: Remove.
impl Origin for () {}
