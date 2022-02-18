use image::Image;
use color::Color;
use matrix::Matrix;
use draw::*;
mod image;
mod color;
mod matrix;
mod draw;

const XRES: i32 = 500;
const YRES: i32 = 500;

fn main(){
    let mut image = Image::new(500, 500);
    let mut color = Color::new();
    let mut m1 = Matrix::new(4,4);
    let mut m2 = Matrix::new(0,0);
    let mut edges = Matrix::new(1,0);

    println!("Testing add_edge. Adding (1, 2, 3), (4, 5, 6) m2 =");
    m2.add_edge(1,2,3,4,5,6);
    m2.print_matrix();

    println!("Testing ident. m1 = ");
    m1.identity();
    m1.print_matrix();

    println!("Testing matrix_mult. m1 * m2 =");
    m2.multiply_matrixes(m1);
    m2.print_matrix();

    println!("Testing Matrix mult. m1 =");
    m1 = Matrix::new(0,0);
    m1.add_edge(1,2,3,4,5,6);
    m1.add_edge(7,8,9,10,11,12);
    m1.print_matrix();

    println!("\nTesting Matrix mult. m1 * m2 =");
    m2.multiply_matrixes(m1);
    m2.print_matrix();
}