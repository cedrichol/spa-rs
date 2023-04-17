#[derive(Clone, Default, PartialEq, Eq, Debug)]
pub struct SymbolicScalar();

impl std::ops::Add<SymbolicScalar> for SymbolicScalar {
    type Output = Self;
    fn add(self, _other: Self) -> Self {
        Self {}
    }
}
