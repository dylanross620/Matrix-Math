use std::io;

const DELTA : f64 = 0.002;

pub struct Mat {
    rows : u32,
    cols : u32,
    data : Vec<f64>
}

impl Mat {
    pub fn new(rows : u32, cols : u32) -> Mat {
        let mut res = Mat{rows, cols, data: Vec::new()};
        if rows == 0 || cols == 0 {
            return res;
        }

        println!("Enter values for a {}x{} matrix:", rows, cols);

        for _ in 0..(rows*cols) {
            let mut num = String::new();
            io::stdin().read_line(&mut num).expect("Failed to read line");
            let num : Vec<&str> = num.trim().split_whitespace().collect();
            let num : f64 = match num[0].parse() { //Ensure that, in the case of spaces within input, only first number will be used as opposed to a 0
                Ok(n) => n,
                Err(_) => 0.0
            };
            res.data.push(num);
        }

        return res;
    }

    pub fn copy(other: &Mat) -> Mat {
        Mat{data: other.data.to_vec(), ..*other}
    }

    pub fn identity(n : u32) -> Mat {
        let mut res = Mat{rows: n, cols: n, data: Vec::with_capacity((n * n) as usize)};

        for i in 0..(n*n) {
            if i % (n+1) == 0 {
                res.data.push(1.0);
            }
            else {
                res.data.push(0.0);
            }
        }

        res
    }

    pub fn add(mat1: &Mat, mat2: &Mat) -> Option<Mat> {
        if mat1.rows != mat2.rows || mat1.cols != mat2.cols {
            return None;
        }

        let mut res = Mat{data: Vec::new(), ..*mat1};
        for i in 0..(res.rows*res.cols) as usize{
            res.data.push(mat1.data[i] + mat2.data[i]);
        }
        return Some(res);
    }

    pub fn sub(mat1: &Mat, mat2: &Mat) -> Option<Mat> {
        if mat1.rows != mat2.rows || mat1.cols != mat2.cols {
            return None;
        }

        let mut res = Mat{data: Vec::new(), ..*mat1};
        for i in 0..(res.rows*res.cols) as usize{
            res.data.push(mat1.data[i] - mat2.data[i]);
        }
        return Some(res);
    }

    pub fn mult(mat1: &Mat, mat2: &Mat) -> Option<Mat> {
        if mat1.cols != mat2.rows {
            return None;
        }

        let mut res = Mat{rows: mat1.rows, cols: mat2.cols, data: Vec::new()};
        for r in 0..res.rows {
            for c in 0..res.cols {
                let mut sum : f64 = 0.0;
                for k in 0..mat1.cols {
                    sum += mat1.get(r, k) * mat2.get(k, c);
                }
                res.data.push(sum);
            }
        }

        return Some(res);
    }

    //Really should never return None, but returns Option for consistency and just in case a reason comes up I'm not thinking of
    pub fn mult_scalar(mat: &Mat, scalar: &f64) -> Option<Mat> {
        let mut res = Mat::copy(mat);

        for i in 0..res.data.len() {
            res.data[i] *= scalar;
        }

        Some(res)
    }

    fn switch_rows(&mut self, row1: u32, row2: u32) {
        if row1 >= self.rows || row2 >= self.rows {
            println!("Attempted row switch out of bounds");
            return;
        }

        for i in 0..self.cols {
            let tmp = self.get(row1, i);
            self.set(row1, i, self.get(row2, i));
            self.set(row2, i, tmp);
        }
    }

    //Will set so that dest = mult*src
    fn add_rows(&mut self, dest: u32, src: u32, mult: f64) {
        if src >= self.rows || dest >= self.rows {
            println!("Attempted to add rows out of bounds");
            return;
        }

        for i in 0..self.cols {
            let cur = self.get(dest, i);
            self.set(dest, i, cur + mult * self.get(src, i));
        }
    }

    //Multiply a row by a scalar
    fn mult_row(&mut self, row: u32, mult: f64) {
        if row >= self.rows {
            println!("Attempt to multiply out of bounds row");
            return;
        }

        for i in (row * self.cols)..((row+1) * self.cols) {
            self.data[i as usize] *= mult;
        }
    }

    fn set(&mut self, row: u32, col: u32, val: f64) {
        if (row * self.cols + col) as usize >= self.data.len() {
            println!("Attempted set out of bounds");
            return;
        }

        self.data[(row * self.cols + col) as usize] = val;
    }

    fn get(&self, row: u32, col: u32) -> f64 {
        if (row * self.cols + col) as usize >= self.data.len() {
            println!("Attempted access out of bounds");
            return 0.0;
        }

        self.data[(row * self.cols + col) as usize]
    }

    pub fn num_rows(&self) -> u32 {
        self.rows
    }

    pub fn num_cols(&self) -> u32 {
        self.cols
    }

    pub fn rref(mat: &Mat, inverse: bool) -> Option<Mat> {
        if mat.rows != mat.cols && inverse {
            println!("Unable to take inverse of non-square matrix");
            return None;
        }

        let mut res = Mat::copy(mat);

        let mut inv = if inverse {
            Mat::identity(mat.rows)
        } else {
            Mat::new(0, 0)
        };

        let mut pivot_col : u32 = 0;
        let mut pivot_row : u32 = 0;

        while pivot_row < res.rows && pivot_col < res.cols {
            //Check for valid pivot
            if res.get(pivot_row, pivot_col).abs() < DELTA {
                let mut swapped = false;
                for i in pivot_row..res.rows { //If not a valid pivot, check all rows below for a valid pivot to swap with
                    if res.get(i, pivot_col).abs() >= DELTA {
                        res.switch_rows(pivot_row, i);
                        if inverse {
                            inv.switch_rows(pivot_row, i);
                        }
                        swapped = true;
                        break;
                    }
                }

                if !swapped { //If unable to swap below, matrix is singular. Either return or increment pivot column
                    if inverse {
                        println!("Unable to take inverse of non full-rank matrix");
                        return None;
                    }
                    pivot_col += 1;
                    continue;
                }
            }

            //We have a valid pivot. Make pivot equal to 1 and Adjust all other rows
            let pivot = res.get(pivot_row, pivot_col);
            if (1.0 - pivot).abs() >= DELTA {
                res.mult_row(pivot_row, 1.0 / pivot);
                if inverse {
                    inv.mult_row(pivot_row, 1.0 / pivot);
                }
            }

            for r in 0..res.rows {
                if r == pivot_row {
                    continue;
                }

                let mult = -1.0 * res.get(r, pivot_col);
                res.add_rows(r, pivot_row, mult);
                if inverse {
                    inv.add_rows(r, pivot_row, mult);
                }
            }

            pivot_row += 1;
            pivot_col += 1;
        }

        if inverse {
            Some(inv)
        }
        else {
            Some(res)
        }
    }

    pub fn print(&self) {
        //Find largest column size
        let mut col_sizes : Vec<usize> = vec![0; self.cols as usize]; //Creates a vector initialized with 0's for every column
        for i in 0..self.data.len() as usize{
            if self.data[i].to_string().len() > col_sizes[i % self.cols as usize] {
                col_sizes[i % self.cols as usize] = self.data[i].to_string().len();
            }
        }

        //Print
        for i in 0..self.data.len() as u32 {
            if i % self.cols == 0 {
                print!("[");
            }

            let mut col_width = col_sizes[i as usize % self.cols as usize];
            col_width -= self.data[i as usize].to_string().len();   //Calculate number of spaces to right align within column

            print!("{}{}", " ".repeat(col_width), self.data[i as usize]);   //Print necessary spaces then value

            if i % self.cols == self.cols - 1 {
                println!("]");
            }
            else {
                print!(" ");
            }
        }
    }
}