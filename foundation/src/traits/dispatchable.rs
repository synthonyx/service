use crate::dispatching::DispatchResult;

pub trait Dispatchable {
    type Origin;
    type Response;

    fn dispatch(self, origin: Self::Origin) -> DispatchResult<Self::Response>;
}
