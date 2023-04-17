use crate::prelude::*;
/*
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

// performs y += ax where a is a matrix, x is a spase column, y is a dense lane workspace

pub fn add_ax<ScalarY, ScalarA, ScalarX, Size>(
    y: &mut DLaneWorkspace<ScalarY, Size>,
    a: &SMatrix<ScalarA, Size>,
    x: SLaneView<ScalarX, Size>,
)
where
    ScalarY: ScalarT + std::ops::AddAssign<ScalarY>,
    ScalarA: ScalarT + std::ops::Mul<ScalarX, Output = ScalarY>,
    ScalarX: ScalarT,
    Size: SizeT,
{
    debug_assert!(a.n == x.m);
    debug_assert!(a.m == y.m());

    for kx in 0..x.x.len() {
        let j = x.i.get(kx);
        for ka in a.p.get(j)..a.p.get(j + 1) {
            // yi += aij + xj
            // note : if y is write-indexed by j instead of i, we calculate y += AT x
            // maybe implement it some day ?
            let i = a.i.get(ka);
            let aij_plus_xj = a.x[ka].clone() * x.x[kx].clone();
            if y.dense_exist.values[i] {
                y.dense_x[i] += aij_plus_xj;
            } else {
                y.dense_exist.values[i] = true;
                y.sparse_i.values.push(Size::to_underlying(i));
                y.dense_x[i] = aij_plus_xj;
            }
        }
    }
}


// compute C = A*B

pub fn mat_mul<ScalarA, ScalarB, ScalarC, Size>(
    a: &SMatrix<ScalarA, Size>,
    b: &SMatrix<ScalarB, Size>,
) -> Result<SMatrix<ScalarC, Size>, String>
where
    ScalarA: ScalarT + std::ops::Mul<ScalarB, Output = ScalarC>,
    ScalarB: ScalarT,
    ScalarC: ScalarT + std::ops::AddAssign<ScalarC>,
    Size: SizeT,
{
    (a.n == b.m).then_some(()).ok_or(format!("Matrix Multiply failed : (a.n = {:?}) != (b.m = {:?})", a.n, b.m))?;
    let cn = a.n;
    let cm = b.m;
    // each column is computed independently
    // if we have a workspace per thread, this can be multiprocessed
    // with no determinism loss or race condition
    // only the gathering into the compact result is inter-thread coupled
    let mut cp = IdxStorage::from(vec![Size::to_underlying(0); cn + 1]);
    let mut ci = IdxStorage::from(vec![]);
    let mut cx = vec![];
    let mut ws = DLaneWorkspace::new(cn);
    for j in 0..cn {
        add_ax(&mut ws, &a, SLaneView::from_matrix_lane(&b, j));
        for i in &ws.sparse_i.values {
            let i = i.from_underlying();
            cx.push(ws.dense_x[i].clone());
            ws.dense_exist.values[i] = false;
        }
        cp.set(j+1, cp.get(j) + ws.sparse_i.values.len());
        ci.values.append(&mut ws.sparse_i.values);
    }

    Ok(SMatrix {
        m:cm,
        n:cn,
        p:cp,
        i:ci,
        x:cx,
    })
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn matrix_dot_1() {
        let i: Vec<u32> = vec![1, 1, 0, 2, 2, 2];
        let j: Vec<u32> = vec![1, 1, 0, 0, 1, 2];
        let x: Vec<f32> = vec![1., 2., 2., 2., 3., 4.];
        let coo = CoordsMatrix { i, j, x };
        let m = SMatrix::from_coords_dedup_accumulate((3, 3), &coo);
        let x = vec![1., 10., 100.];
        let i = vec![0, 1, 2];
        let x = SLaneView::from_raw(3, &i, &x);
        // m = (2 0 0) \ (0 3 0) \ (2 3 4)
        // x = (1 10 100)
        let should_be = vec![2., 30., 432.]; 
        let mut y = DLaneWorkspace::new(3);
        add_ax(&mut y, &m, x);
        assert!(y.dense_x.len() == 3 && y.sparse_i.values.len() == 3);
        assert!(y.dense_x.iter().zip(should_be).map(|tup| (tup.0 - tup.1).abs()).sum::<f32>() < 1.0e-10);

        let x: Vec<f32> = vec![];
        let i = vec![];
        let x = SLaneView::from_raw(3, &i, &x);
        let mut y = DLaneWorkspace::new(3);
        add_ax(&mut y, &m, x);
        assert!(y.sparse_i.values.is_empty())
    }
    #[test]
    fn matrix_mul_1() {
        let i: Vec<u32> = vec![0, 1, 2, 1, 2, 2];
        let j: Vec<u32> = vec![0, 0, 0, 1, 1, 2];
        let x: Vec<f32> = vec![10., 1., 1., 1., 1., 100.];
        let coo = CoordsMatrix { i, j, x };
        let m = SMatrix::from_coords_dedup_accumulate((3, 3), &coo);
        let res = mat_mul(&m, &m).unwrap();
        let should_be = vec![100.,11.,111.,1.,101.,10000.]; 
        assert!(res.get_x().len() == 6);
        assert!(res.get_x().iter().zip(should_be).map(|tup| (tup.0 - tup.1).abs()).sum::<f32>() < 1.0e-10);
    }
}
