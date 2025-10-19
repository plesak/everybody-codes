// use std::fs;

use std::fmt::Display;
use array2d::Array2D;

pub fn print_array2d<T: Display>(array: &Array2D<T>) {
    for row in array.rows_iter() {
        for col in row {
            print!("{}", col);
        }
        println!();
    }
}