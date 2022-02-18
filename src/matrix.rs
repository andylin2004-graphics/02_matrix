use std::fmt;

pub struct Matrix{
    pub matrixArray: Vec<Vec<i32>>,
}

impl Matrix{
    pub fn new(row: usize, col: usize) -> Matrix{
        Matrix{matrixArray: vec![vec![0; row]; col],}
    }

    pub fn multiply_matrixes(&mut self, m1: Matrix){
        let mut matrix_result = Matrix::new(m1.matrixArray.len(), self.matrixArray[0].len());
        println!("{}", matrix_result);
        for result_i in 0..matrix_result.matrixArray.len(){
            for result_v in 0..matrix_result.matrixArray[result_i].len(){
                for m2_down_num in 0..self.matrixArray.len(){
                    matrix_result.matrixArray[result_i][result_v] += self.matrixArray[m2_down_num][result_v] * m1.matrixArray[result_i][m2_down_num];
                }
            }
        }

        *self = matrix_result;
    }

    fn identity(&mut self){
        for i in 0..self.matrixArray.len(){
            for v in 0..self.matrixArray[0].len(){
                if i == v{
                    self.matrixArray[i][v] = 1;
                }else{
                    self.matrixArray[i][v] = 0;
                }
            }
        }
    }
}

impl fmt::Display for Matrix{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        let mut result: String = "".to_owned();
        for i in 0..self.matrixArray.len(){
            for v in 0..self.matrixArray[i].len(){
                result.push_str(&(format!("{} ",self.matrixArray[i][v]).to_string()));
            }
            result.push_str("\n");
        }
        write!(f, "{}", result)
    }
}