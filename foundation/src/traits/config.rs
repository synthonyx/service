use super::Origin;

pub trait Config {
    type Origin: Origin;
}
