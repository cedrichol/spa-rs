#![allow(non_snake_case)]

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SMatrix<ScalarT, SizeT = usize> {
    m: SizeT,
    n: SizeT,
    p: Vec<SizeT>,
    i: Vec<SizeT>,
    x: Vec<ScalarT>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoordsMatrix<ScalarT, SizeT = usize> {
    pub i: Vec<SizeT>,
    pub j: Vec<SizeT>,
    pub x: Vec<ScalarT>,
}

#[derive(Clone, Default, PartialEq, Eq, Debug)]
pub struct SymbolicScalar();

impl std::ops::Add<SymbolicScalar> for SymbolicScalar {
    type Output = Self;
    fn add(self, _other: Self) -> Self {
        SymbolicScalar {}
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

// SizeT must be big enough to either hold N, M, or nnz.
// For instance a 200*200 matrix with 260 entries cannot be represented with u8

impl<ScalarT, SizeT> SMatrix<ScalarT, SizeT>
where
    ScalarT: Clone + Default,
    SizeT: Copy + TryFrom<usize> + TryInto<usize>,
{

    pub fn get_shape(&self) -> (usize, usize) {
        (load(self.m), load(self.n))
    }

    pub fn get_x(&self) -> &[ScalarT] {
        &self.x
    }

    pub fn get_nnz(&self) -> usize {
        debug_assert!(self.i.len() == self.x.len());
        self.i.len()
    }

    pub fn from_coords_no_dedup(shape: (usize, usize), A: &CoordsMatrix<ScalarT, SizeT>) -> Self {
        let Ai = &A.i;
        let Aj = &A.j;
        let Ax = &A.x;

        assert!(
            Ai.len() == Aj.len() && Aj.len() == Ax.len(),
            "Ai, Aj and Ax must be the same lenght"
        );

        let nnz = Ai.len();
        let (M, N) = shape;
        let mut Bp = vec![store(0); N + 1];
        let mut Bi = vec![store(0); nnz]; // maybe they can be uninit ?
        let mut Bx = vec![ScalarT::default(); nnz]; // idem (even worse)

        // make p: the cumulative sum of indices of each column
        for j in Aj {
            let j = load(*j);
            Bp[j] = store(1 + load(Bp[j]));
        }

        let mut cumsum = 0;
        #[allow(clippy::needless_range_loop)]
        for j in 0..N {
            let temp = load(Bp[j]);
            Bp[j] = store(cumsum);
            cumsum += temp;
        }

        // make	new i,x from the given COOs
        // uses p as as the global index
        for k in 0..nnz {
            let j = load(Aj[k]);
            let global_i = load(Bp[j]);
            Bp[j] = store(load(Bp[j]) + 1);
            Bi[global_i] = Ai[k];
            Bx[global_i] = Ax[k].clone();
        }

        // at the end p has to be shifted to be restored
        Bp.pop()
            .unwrap_or_else(|| panic!("That should be impossible : N + 1 > 0"));
        Bp.insert(0, store(0));

        // CSR representation (possible duplicates, unsorted column)
        Self {
            m: store(M),
            n: store(N),
            p: Bp,
            i: Bi,
            x: Bx,
        }
    }

    pub fn dedup_by(mut self, reduce: impl Fn(ScalarT, ScalarT) -> ScalarT) -> Self {
        let mut last_seen_at = vec![store::<SizeT>(0); load(self.m)];
        let mut writeidx = 0;
        for col in 0..self.p.len() - 1 {
            let readrange = load(self.p[col])..load(self.p[col + 1]);
            self.p[col] = store(writeidx);

            for readidx in readrange {
                let i = load(self.i[readidx]);
                let is_duplicate = load(last_seen_at[i]) > load(self.p[col]);
                if is_duplicate {
                    let to_add_to = load(last_seen_at[i]) - 1;
                    self.x[to_add_to] = reduce(self.x[to_add_to].clone(), self.x[readidx].clone());
                } else {
                    self.i[writeidx] = self.i[readidx];
                    self.x[writeidx] = self.x[readidx].clone();
                    writeidx += 1;
                    last_seen_at[i] = store(writeidx);
                }
            }
        }
        let last = self.p.len() - 1;
        self.p[last] = store(writeidx);
        self.i.truncate(writeidx);
        self.x.truncate(writeidx);
        self
    }

    pub fn to_coords(self) -> CoordsMatrix<ScalarT, SizeT> {
        let mut j = Vec::<SizeT>::with_capacity(self.i.len());
        for k in 0..self.p.len() - 1 {
            j.extend(
                std::iter::repeat::<SizeT>(store(k)).take(load(self.p[k + 1]) - load(self.p[k])),
            )
        }
        CoordsMatrix {
            i: self.i,
            j,
            x: self.x,
        }
    }

    pub fn to_dense_by_idx(
        &self,
        index: impl Fn(usize, usize, usize, usize) -> usize,
    ) -> Vec<ScalarT> {
        let (m, n) = self.get_shape();

        let mut dense = vec![ScalarT::default(); m * n];
        for j in 0..self.p.len() - 1 {
            for k in load(self.p[j])..load(self.p[j + 1]) {
                let i = load(self.i[k]);
                dense[index(m, n, i, j)] = self.x[k].clone();
            }
        }
        dense
    }

    pub fn to_dense_row_major(&self) -> Vec<ScalarT> {
        self.to_dense_by_idx(|m, _n, i, j| i + m * j)
    }

    pub fn to_dense_column_major(&self) -> Vec<ScalarT> {
        self.to_dense_by_idx(|_m, n, i, j| n * i + j)
    }
}

impl<ScalarT, SizeT> SMatrix<ScalarT, SizeT>
where
    ScalarT: Clone + Default + std::ops::Add<ScalarT, Output = ScalarT>,
    SizeT: Copy + TryFrom<usize> + TryInto<usize>,
{
    pub fn dedup_accumulate(self) -> Self {
        self.dedup_by(|old, new| old + new)
    }

    pub fn from_coords_dedup_accumulate(
        shape: (usize, usize),
        A: &CoordsMatrix<ScalarT, SizeT>,
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
        assert!(m.to_dense_row_major().len() == 8);
        assert!(m.to_dense_column_major().len() == 8);
    }
}
