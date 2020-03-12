use std::io;

pub struct Mat {
    rows : u32,
    cols : u32,
    data : Vec<i64>
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
            let num = match num.trim().parse() {
                Ok(n) => n,
                Err(_) => 0
            };
            res.data.push(num);
        }

        return res;
    }

    pub fn copy(other: &Mat) -> Mat {
        Mat{data: other.data.to_vec(), ..*other}
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
                let mut sum : i64 = 0;
                for k in 0..mat1.cols {
                    sum += mat1.get(r, k) * mat2.get(k, c);
                }
                res.data.push(sum);
            }
        }

        return Some(res);
    }

    fn get(&self, row: u32, col: u32) -> i64 {
        self.data[(row * self.cols + col) as usize]
    }

    pub fn print(&self) {
        //Find largest column size
        let mut col_sizes : Vec<usize> = vec![0; self.cols as usize];
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