use image::Image;
use color::Color;
use matrix::Matrix;
mod image;
mod color;
mod matrix;

fn main(){
    let mut matrix1 = Matrix::new(4,4);
    matrix1.matrixArray = vec![vec![1,2,3],vec![4,5,6]];
    let mut matrix2 = Matrix::new(4,4);
    matrix2.matrixArray = vec![vec![7,8],vec![9,10],vec![11,12]];
    matrix2.multiply_matrixes(matrix1);
    println!("{}", matrix2);
    matrix2 = Matrix::new(4,4);
    matrix2.identity();
    println!("{}", matrix2);
}