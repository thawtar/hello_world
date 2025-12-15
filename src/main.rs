struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<Vec<f64>>,
}

impl Matrix {
    fn new(rows: usize, cols: usize) -> Self {
        let data = vec![vec![0.0; cols]; rows];
        Matrix { rows, cols, data}

    }

    fn ones(rows: usize, cols: usize) -> Self {
        let data = vec![vec![1.0; cols]; rows];
        Matrix { rows, cols, data }
    }

    fn identity(size: usize) -> Self {
        let mut data = vec![vec![0.0; size]; size];
        for i in 0..size {
            data[i][i] = 1.0;
        }
        Matrix { rows: size, cols: size, data }
    }
}

fn main() {
    println!("Hello, world!");
}
