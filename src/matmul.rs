/*
use crate::prelude::*;

impl<ScalarT, SizeT> std::ops::Mul<&Self> for SMatrix<ScalarT, SizeT>
where
    ScalarT: Clone
        + Default
        + std::ops::Add<ScalarT, Output = ScalarT>
        + std::ops::Mul<ScalarT, Output = ScalarT>,
    SizeT: Copy + TryFrom<usize> + TryInto<usize>,
{
    type Output = Self;
    fn mul(mut self, rhs: &Self) -> Self {
        /*
        for colright in ??? {
            add_dot(&self, colright)
        }*/
        self
    }
}
/*
// x += Ax
impl<ScalarT, SizeT> SMatrix<ScalarT, SizeT>
where
    ScalarT: Clone
        + Default
        + std::ops::Add<ScalarT, Output = ScalarT>
        + std::ops::Mul<ScalarT, Output = ScalarT>,
    SizeT: Copy + TryFrom<usize> + TryInto<usize>,
{
    pub fn add_dot(&self, x: (&mut [SizeT], &mut [ScalarT])) {
        /*
        for j, x in nonzeros(column) {
            for i in nonzeros(matrix, col j) {
                x[i] += A[i, j] * x[i]
            }
        }*/
        for kx in 0..x.0.len() {
            let j = Self::load(x.0[kx]);
            let xj = x.1[kx];
            for ka in self::load(self.p[j])..self::load(self.p[j+1]) {
                let i = Self::load(self.i[ka]);
                let aij = self.x[ka];
                // ouch
            }
        }
    }
}
*/
*/
