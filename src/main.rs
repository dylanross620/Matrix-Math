mod mat;

use mat::Mat;
use std::io;
use std::collections::HashMap;

fn create_mat(input: &[&str]) -> Option<Mat> {
    if input.len() < 5 || input[2] != "=" {
        return None;
    }

    let rows : u32 = input[3].parse().expect("Row amount must be a positive integer");
    let cols : u32 = input[4].parse().expect("Column amount must be a positive integer");

    Some(Mat::new(rows, cols))
}

fn add_mats(input: &[&str], mats: &HashMap<String, Mat>) -> Option<Mat>{
    let mat1 = mats.get(input[0]);
    let mat2 = mats.get(input[2]);

    let tmp = Mat::new(0,0);

    let mut error = false;

    let mat1 = match mat1 {
        Some(m) => m,
        None => {println!("Unable to find matrix {}", input[0]); error = true; &tmp}
    };
    let mat2 = match mat2 {
        Some(m) => m,
        None => {println!("Unable to find matrix {}", input[2]); error = true; &tmp}
    };

    if error {
        return None;
    }
    
    let res = if input[1] == "+" {
        Mat::add(&mat1, &mat2)
    } else {
        Mat::sub(&mat1, &mat2)
    };

    match res {
        Some(m) => Some(m),
        None => {println!("Unable to add/subtract matrices {} and {}", input[0], input[2]); None}
    }
}

fn mult_mats(input: &[&str], mats: &HashMap<String, Mat>) -> Option<Mat>{
    let mat1 = mats.get(input[0]);
    let mat2 = mats.get(input[2]);

    let tmp = Mat::new(0,0);

    let mut error = false;

    let mat1 = match mat1 {
        Some(m) => m,
        None => {println!("Unable to find matrix {}", input[0]); error = true; &tmp}
    };
    let mat2 = match mat2 {
        Some(m) => m,
        None => {println!("Unable to find matrix {}", input[2]); error = true; &tmp}
    };

    if error {
        return None;
    }

    match Mat::mult(&mat1, &mat2) {
        Some(m) => Some(m),
        None => {println!("Unable to multiply matrices {} and {}", input[0], input[2]); None}
    }
}

fn parse_secondary_input (input: &[&str], mats: &HashMap<String, Mat>) -> Option<Mat>{
    if input.len() == 1 {
        return match mats.get(input[0]) {
            Some(m) => Some(Mat::copy(m)),
            None => {println!("Unable to find matrix {}", input[0]); None}
        };
    }
    else if input.len() >= 3 && (input[1] == "+" || input[1] == "-") {
        return add_mats(&input, &mats);
    }
    else if input.len() >= 3 && input[1] == "*" {
        return mult_mats(&input, &mats);
    }
    else {
        println!("Invalid command");
    }
    None
}

fn main() {
    let mut mats : HashMap<String, Mat> = HashMap::new();

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();
        
        if input.to_lowercase() == "q" || input.to_lowercase() == "quit" {
            break;
        }

        let input : Vec<&str> = input.split_whitespace().collect();

        if input[0] == "let" { //Create a new matrix
            let res = if input.len() == 5 {
                create_mat(&input)
            } else {
                parse_secondary_input(&input[3..], &mats)
            };

            match res {
                Some(m) => {mats.insert(String::from(input[1]), m); println!("Successfully created matrix {}", input[1])},
                None => println!("Error creating matrix {}", input[1])
            };
        }
        else if input[0] == "print" { //Print existing matrix
            if input.len() < 2 {
                println!("Not enough aruments given for print\n");
                continue;
            }

            let res = mats.get(&String::from(input[1]));
            match res {
                Some(mat) => mat.print(),
                None => println!("Matrix {} not found", input[1])
            };
        }
        else { //Neither creating nore printing anything existing. Go straight to operation and print the temporary result
            match parse_secondary_input(&input, &mats) {
                Some(mat) => mat.print(),
                None => ()
            };
        }

        println!("");   //Extra spacing for readability
    }
}
