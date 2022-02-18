pub struct Matrix{
    pub matrixArray: Vec<Vec<i32>>,
}

impl Matrix{
    fn new(row: Option<usize>, col: Option<usize>) -> Matrix{
        let row = row.unwrap_or(4);
        let col = col.unwrap_or(4);
        Matrix{matrixArray: vec![vec![0; row]; col],}
    }

    fn multiply_matrixes(&self, m1: Matrix){
        
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