pub trait SizeT: Copy + TryFrom<usize> + TryInto<usize> {
    fn to_underlying(x: usize) -> Self {
        store(x)
    }
    fn from_underlying(self) -> usize {
        load(self)
    }
}

impl<T> SizeT for T where T: Copy + TryFrom<usize> + TryInto<usize> {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdxStorage<
    Storage,
    Contained = <<Storage as std::ops::Deref>::Target as std::ops::Index<usize>>::Output,
> where
    Storage: std::ops::Deref,
    <Storage as std::ops::Deref>::Target: std::ops::Index<usize, Output = Contained>,
    <<Storage as std::ops::Deref>::Target as std::ops::Index<usize>>::Output: SizeT,
{
    pub values: Storage,
}

impl<Storage> IdxStorage<Storage>
where
    Storage: std::ops::Deref,
    <Storage as std::ops::Deref>::Target: std::ops::Index<usize>,
    <<Storage as std::ops::Deref>::Target as std::ops::Index<usize>>::Output: SizeT,
{
    pub fn from(vec: Storage) -> Self {
        Self { values: vec }
    }
    pub fn get(&self, i: usize) -> usize {
        self.values[i].from_underlying()
    }
}

impl<Storage> IdxStorage<Storage>
where
    Storage: std::ops::DerefMut,
    <Storage as std::ops::Deref>::Target: std::ops::IndexMut<usize>,
    <<Storage as std::ops::Deref>::Target as std::ops::Index<usize>>::Output: SizeT,
{
    pub fn set(&mut self, i: usize, val: usize) {
        self.values[i] = <<<Storage as std::ops::Deref>::Target as std::ops::Index<usize>>::Output as SizeT>::to_underlying(val);
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn compiles() {
        let a = vec![1, 2, 3];
        let _store_vec = IdxStorage::from(a);

        let a = vec![1, 2, 3, 4];
        let s = &a[..];
        let _store_slice = IdxStorage::from(s);
    }
}
