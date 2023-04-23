use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SMatrix<Scalar, Size = usize>
where
    Scalar: ScalarT,
    Size: SizeT,
{
    pub(crate) m: usize,
    pub(crate) n: usize,
    pub(crate) p: IdxStorage<Vec<Size>>,
    pub(crate) i: IdxStorage<Vec<Size>>,
    pub(crate) x: Vec<Scalar>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoordsMatrix<Scalar, Size = usize> {
    pub i: Vec<Size>,
    pub j: Vec<Size>,
    pub x: Vec<Scalar>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct SLaneAccessor<'a, Scalar: ScalarT, Size: SizeT> {
    pub matrix: &'a SMatrix<Scalar, Size>,
    pub j: usize,
}

impl<'a, Scalar: ScalarT, Size: SizeT> SLaneAccessor<'a, Scalar, Size> {
    pub fn from_matrix_lane(matrix: &'a SMatrix<Scalar, Size>, j: usize) -> Self {
        Self { matrix, j }
    }
    pub fn value_range(&self) -> std::ops::Range<usize> {
        self.matrix.p.get(self.j)..self.matrix.p.get(self.j + 1)
    }
}
