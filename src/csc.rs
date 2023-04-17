#![allow(non_snake_case)]

use crate::prelude::*;

// Size must be big enough to either hold N, M, or nnz.
// For instance a 200*200 matrix with 260 entries cannot be represented with u8

impl<Scalar: ScalarT, Size: SizeT> SMatrix<Scalar, Size> {
    pub fn get_shape(&self) -> (usize, usize) {
        (self.m, self.n)
    }

    pub fn get_x(&self) -> &[Scalar] {
        &self.x
    }

    pub fn get_nnz(&self) -> usize {
        debug_assert!(self.i.values.len() == self.x.len());
        self.i.values.len()
    }

    pub fn from_coords_no_dedup(shape: (usize, usize), A: &CoordsMatrix<Scalar, Size>) -> Self {
        let Ai = &A.i;
        let Aj = &A.j;
        let Ax = &A.x;

        assert!(
            Ai.len() == Aj.len() && Aj.len() == Ax.len(),
            "Ai, Aj and Ax must be the same lenght"
        );

        let nnz = Ai.len();
        let (M, N) = shape;
        // Bi and Bx could be uninit. Not Bp
        let mut Bp = IdxStorage::from(vec![Size::to_underlying(0); N + 1]);
        let mut Bi = IdxStorage::from(vec![Size::to_underlying(0); nnz]);
        // let mut Bx = vec![Scalar::default(); nnz]; // would require default
        let mut Bx = unsafe { vec![std::mem::MaybeUninit::uninit().assume_init(); nnz] };
        // let mut Bx = unsafe { let mut v = Vec::with_capacity(nnz); v.set_len(nnz); v }; // might allocate too much

        // make p: the cumulative sum of indices of each column
        for j in Aj {
            let j = (*j)
                .try_into()
                .unwrap_or_else(|_| panic!("Aj[k] doesn't fit in Size"));
            Bp.set(j, 1 + Bp.get(j));
        }

        let mut cumsum = 0;
        #[allow(clippy::needless_range_loop)]
        for j in 0..N {
            let temp = Bp.get(j);
            Bp.set(j, cumsum);
            cumsum += temp;
        }

        // make	new i,x from the given COOs
        // uses p as as the global index
        for k in 0..nnz {
            let j = Aj[k]
                .try_into()
                .unwrap_or_else(|_| panic!("Aj[k] doesn't fit in Size"));
            let global_i = Bp.get(j);
            Bp.set(j, 1 + Bp.get(j));
            Bi.values[global_i] = Ai[k];
            Bx[global_i] = Ax[k].clone();
        }

        // at the end p has to be shifted to be restored
        Bp.values
            .pop()
            .unwrap_or_else(|| panic!("That should be impossible : N + 1 > 0"));
        Bp.values.insert(0, Size::to_underlying(0));

        // CSR representation (possible duplicates, unsorted column)
        Self {
            m: M,
            n: N,
            p: Bp,
            i: Bi,
            x: Bx,
        }
    }

    pub fn dedup_by(mut self, reduce: impl Fn(Scalar, Scalar) -> Scalar) -> Self {
        let mut last_seen_at = IdxStorage::from(vec![Size::to_underlying(0); self.get_shape().0]);
        let mut writeidx = 0;
        for col in 0..self.n {
            let readrange = self.p.get(col)..self.p.get(col + 1);
            self.p.set(col, writeidx);

            for readidx in readrange {
                let i = self.i.get(readidx);
                let is_duplicate = last_seen_at.get(i) > self.p.get(col);
                if is_duplicate {
                    let to_add_to = last_seen_at.get(i) - 1;
                    self.x[to_add_to] = reduce(self.x[to_add_to].clone(), self.x[readidx].clone());
                } else {
                    self.i.set(writeidx, self.i.get(readidx));
                    self.x[writeidx] = self.x[readidx].clone();
                    writeidx += 1;
                    last_seen_at.set(i, writeidx);
                }
            }
        }
        let last = self.n;
        self.p.set(last, writeidx);
        self.i.values.truncate(writeidx);
        self.x.truncate(writeidx);
        self
    }

    pub fn to_coords(self) -> CoordsMatrix<Scalar, Size> {
        let mut j = Vec::<Size>::with_capacity(self.get_nnz());
        for k in 0..self.n {
            j.extend(
                std::iter::repeat(Size::to_underlying(k)).take(self.p.get(k + 1) - self.p.get(k)),
            )
        }
        CoordsMatrix {
            i: self.i.values,
            j,
            x: self.x,
        }
    }

    pub fn to_dense_by_idx(
        &self,
        index: impl Fn(usize, usize, usize, usize) -> usize,
        zero: Scalar,
    ) -> Vec<Scalar> {
        let (m, n) = self.get_shape();

        let mut dense = vec![zero; m * n];
        for j in 0..n {
            for k in self.p.get(j)..self.p.get(j + 1) {
                let i = self.i.get(k);
                dense[index(m, n, i, j)] = self.x[k].clone();
            }
        }
        dense
    }

    pub fn to_dense_row_major(&self, zero: Scalar) -> Vec<Scalar> {
        self.to_dense_by_idx(|m, _n, i, j| i + m * j, zero)
    }

    pub fn to_dense_column_major(&self, zero: Scalar) -> Vec<Scalar> {
        self.to_dense_by_idx(|_m, n, i, j| n * i + j, zero)
    }
}

impl<Scalar, Size> SMatrix<Scalar, Size>
where
    Scalar: ScalarT + std::ops::Add<Scalar, Output = Scalar>,
    Size: SizeT,
{
    pub fn dedup_accumulate(self) -> Self {
        self.dedup_by(|old, new| old + new)
    }

    pub fn from_coords_dedup_accumulate(
        shape: (usize, usize),
        A: &CoordsMatrix<Scalar, Size>,
    ) -> Self {
        Self::from_coords_no_dedup(shape, A).dedup_accumulate()
    }
}

// cargo test -- --nocapture
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn matrix_from_coords_dedup_noop_1() {
        let i: Vec<u8> = vec![0, 1, 2, 3, 5];
        let j: Vec<u8> = vec![0, 1, 3, 2, 3];
        let x: Vec<SymbolicScalar> = vec![SymbolicScalar {}; 5];
        //let x: Vec<f32> = vec![10., 20., 30., 40., 50.];
        let coo = CoordsMatrix { i, j, x };
        let m = SMatrix::from_coords_no_dedup((6, 4), &coo);
        let m2 = m.clone().dedup_accumulate();
        assert!(m == m2);
        assert!(SMatrix::from_coords_dedup_accumulate((6, 4), &coo) == m2);
    }

    #[test]
    fn matrix_from_coords_dedup_noop_2() {
        let i: Vec<u16> = vec![0, 1, 1];
        let j: Vec<u16> = vec![0, 1, 1];
        let x: Vec<SymbolicScalar> = vec![SymbolicScalar {}; 3];
        //let x: Vec<f32> = vec![1., 2., 3.];
        let coo = CoordsMatrix { i, j, x };
        let m = SMatrix::from_coords_no_dedup((2, 2), &coo);
        let m2 = m.clone().dedup_accumulate();
        assert!(m != m2);
        assert!(m2.get_nnz() == 2);
        assert!(SMatrix::from_coords_dedup_accumulate((2, 2), &coo) == m2);
    }

    #[test]
    fn matrix_from_coords_dedup_op_1() {
        let i: Vec<u32> = vec![1, 1, 1, 0, 0, 0];
        let j: Vec<u32> = vec![1, 1, 1, 0, 0, 0];
        let x: Vec<f32> = vec![1., 2., 3., 4., 5., 6.];
        let coo = CoordsMatrix { i, j, x };

        let m = SMatrix::from_coords_dedup_accumulate((2, 4), &coo);
        assert!(
            m.get_x().len() == 2
                && (m.get_x()[0] - 15.0).abs() <= 1.0e-10
                && (m.get_x()[1] - 6.0).abs() <= 1.0e-10
        );
        assert!(m.to_dense_row_major(0.).len() == 8);
        assert!(m.to_dense_column_major(0.).len() == 8);
    }
}
