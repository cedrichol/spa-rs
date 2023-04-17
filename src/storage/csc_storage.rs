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
pub struct SLaneView<'a, Scalar: ScalarT, Size: SizeT> {
    pub(crate) m: usize,
    pub(crate) i: IdxStorage<&'a [Size]>,
    pub(crate) x: &'a [Scalar],
}

impl<'a, Scalar: ScalarT, Size: SizeT> SLaneView<'a, Scalar, Size> {
    pub fn from_raw(m: usize, i: &'a [Size], x: &'a [Scalar]) -> Self {
        Self {
            m,
            i: IdxStorage::from(i),
            x,
        }
    }
    pub fn from_matrix_lane(a: &'a SMatrix<Scalar, Size>, j: usize) -> Self {
        let range = a.p.get(j)..a.p.get(j + 1);
        let i = IdxStorage::from(&a.i.values[range.clone()]);
        let x = &a.x[range];
        Self {
            m: a.get_shape().0,
            i,
            x,
        }
    }
    pub fn get_shape(&self) -> usize {
        self.m
    }
}
