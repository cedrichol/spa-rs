
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Storage<Scalar> {
	pub values: Vec<Scalar>
}

impl<Scalar, Idx> Index<Idx> for Balance {
    type Output = Scalar;

    fn index(&self, index: Idx) -> &Self::Output {
    	&self.values[load(index)]
    }
}

impl<Scalar, Idx> IndexMut<Idx> for Balance {
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
    	&mut self.values[load(index)]
    }
}






fn load<SizeT>(x: SizeT) -> usize
where
    SizeT: TryFrom<usize> + TryInto<usize>,
{
    let handle = |_| {
        if cfg!(debug_assertions) {
            panic!("Your usize is too small to fit the data in SizeT")
        } else {
            unsafe { std::hint::unreachable_unchecked() }
        }
    };
    x.try_into().unwrap_or_else(handle)
}

fn store<SizeT>(x: usize) -> SizeT
where
    SizeT: TryFrom<usize> + TryInto<usize>,
{
    let handle = |_| {
        if cfg!(debug_assertions) {
            panic!("Your SizeT is too small to fit the data in usize: x={x}")
        } else {
            unsafe { std::hint::unreachable_unchecked() }
        }
    };
    x.try_into().unwrap_or_else(handle)
}
