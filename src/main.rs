use image::Image;
use color::Color;
use matrix::Matrix;
mod image;
mod color;
mod matrix;
mod draw;

const XRES: i32 = 500;
const YRES: i32 = 500;

fn main(){
    let mut image = Image::new(XRES as usize, YRES as usize);
    let mut color = Color::new_color(0, 255, 0);
    let mut m1 = Matrix::new(4,4);
    let mut m2 = Matrix::new(0,0);
    let mut edges = Matrix::new(1,0);

    println!("Testing add_edge. Adding (1, 2, 3), (4, 5, 6) m2 =");
    m2.add_edge(1.0,2.0,3.0,4.0,5.0,6.0);
    m2.print_matrix();

    println!("Testing ident. m1 = ");
    m1.identity();
    m1.print_matrix();

    println!("Testing matrix_mult. m1 * m2 =");
    m2.multiply_matrixes(m1);
    m2.print_matrix();

    println!("Testing Matrix mult. m1 =");
    m1 = Matrix::new(0,0);
    m1.add_edge(1.0,2.0,3.0,4.0,5.0,6.0);
    m1.add_edge(7.0,8.0,9.0,10.0,11.0,12.0);
    m1.print_matrix();

    println!("\nTesting Matrix mult. m1 * m2 =");
    m2.multiply_matrixes(m1);
    m2.print_matrix();

    edges.add_edge(50.0, 450.0, 0.0, 100.0, 450.0, 0.0);
    edges.add_edge(50.0, 450.0, 0.0, 50.0, 400.0, 0.0);
    edges.add_edge(100.0, 450.0, 0.0, 100.0, 400.0, 0.0);
    edges.add_edge(100.0, 400.0, 0.0, 50.0, 400.0, 0.0);

    edges.add_edge(200.0, 450.0, 0.0, 250.0, 450.0, 0.0);
    edges.add_edge(200.0, 450.0, 0.0, 200.0, 400.0, 0.0);
    edges.add_edge(250.0, 450.0, 0.0, 250.0, 400.0, 0.0);
    edges.add_edge(250.0, 400.0, 0.0, 200.0, 400.0, 0.0);

    edges.add_edge(150.0, 400.0, 0.0, 130.0, 360.0, 0.0);
    edges.add_edge(150.0, 400.0, 0.0, 170.0, 360.0, 0.0);
    edges.add_edge(130.0, 360.0, 0.0, 170.0, 360.0, 0.0);

    edges.add_edge(100.0, 340.0, 0.0, 200.0, 340.0, 0.0);
    edges.add_edge(100.0, 320.0, 0.0, 200.0, 320.0, 0.0);
    edges.add_edge(100.0, 340.0, 0.0, 100.0, 320.0, 0.0);
    edges.add_edge(200.0, 340.0, 0.0, 200.0, 320.0, 0.0);  

    image.draw_lines(edges, color);

    image.create_file("bob.ppm".to_owned());
}