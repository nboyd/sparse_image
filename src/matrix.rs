//Simple matrix type. Dumb.
use std::ops::*;
use std::vec::*;
use std::fmt::*;

#[derive(Clone)]
pub struct Matrix{
    data : Vec<f64>,
    n_cols : usize,
    n_rows : usize,
}

impl Matrix{
    pub fn linear_index(&self, i : usize, j : usize) -> usize {
        self.n_cols*i + j
    }
    pub fn new(data : Vec<f64>, n_cols : usize, n_rows : usize) -> Matrix{
        Matrix{data: data, n_cols : n_cols, n_rows : n_rows}
    }
    pub fn add(&mut self, rhs: Matrix) -> &mut Matrix {
        assert!(self.n_cols == rhs.n_cols && self.n_rows == rhs.n_rows);
        for i in 0..self.data.len(){
            self.data[i] += rhs.data[i];
        }
        self
    }
}

impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f,"{}x{} matrix:\n", self.n_rows, self.n_cols);
        let s = self.data.chunks(self.n_cols).map(|c| "\t".to_string()+&(c.iter().map(|n| n.to_string()).
            collect::<Vec<String>>().connect("\t"))).collect::<Vec<String>>().connect("\n");
        f.write_str(&s)
    }
}


impl Index<(usize,usize)> for Matrix{
    type Output = f64;
    fn index(&self, (i,j): (usize,usize)) -> &f64 {
        let index = self.linear_index(i,j);
        &self.data[index]
    }
}

impl IndexMut<(usize,usize)> for Matrix{
    fn index_mut<'b>(&'b mut self, (i,j): (usize,usize)) -> &'b mut f64 {
        let index = self.linear_index(i,j);
        &mut self.data[index]
    }
}
