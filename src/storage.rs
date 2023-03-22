#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdxStorage<Contained>
where
    Contained: Copy + TryFrom<usize> + TryInto<usize>,
{
    pub values: Vec<Contained>,
}

impl<Contained> IdxStorage<Contained>
where
    Contained: Copy + TryFrom<usize> + TryInto<usize>,
{
    pub fn from(vec: Vec<Contained>) -> Self {
        Self { values: vec }
    }
    pub fn len(&self) -> usize {
    	self.values.len()
    }
    pub fn is_empty(&self) -> bool {
    	self.len() == 0
    }
    pub fn get(&self, i: usize) -> usize {
        load(self.values[i])
    }
    pub fn set(&mut self, i: usize, val: usize) {
        self.values[i] = Self::to_contained(val);
    }
    pub fn to_contained(val: usize) -> Contained {
    	store(val)
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
