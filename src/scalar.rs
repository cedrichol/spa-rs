// try to get rid of Default some day

pub trait ScalarT: Clone {}
impl<T> ScalarT for T where T: Clone {}
