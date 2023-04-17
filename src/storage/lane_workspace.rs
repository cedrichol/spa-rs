use crate::prelude::*;

// https://research.swtch.com/sparse

// invariant : does not read before a write
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DLaneWorkspace<Scalar, Size: SizeT = usize> {
    pub sparse_i: IdxStorage<Vec<Size>>,
    pub dense_exist: BoolWorkspace,
    pub dense_x: Vec<Scalar>,
}

impl<Scalar, Size: SizeT> DLaneWorkspace<Scalar, Size> {
	pub fn m(&self) -> usize {
		self.dense_x.len()
	}
    pub fn new(size: usize) -> Self {
    	let mut dense_x = Vec::with_capacity(size);
    	let dense_exist = BoolWorkspace {values: vec![false; size]};
    	let sparse_i = IdxStorage::from(Vec::new());
    	unsafe { dense_x.set_len(size) }

        Self {
            sparse_i,
            dense_exist,
            dense_x,
        }
    }
}
