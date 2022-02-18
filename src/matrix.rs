struct Matrix{
    pub matrixArray: vec!<vec!<i32>>;
}

impl Matrix{
    fn new(row: usize = 4, col: usize = 4) -> Matrix{
        Matrix{matrix = vec![vec![0; row]; col],}
    }

    fn multiply_matrixes(&self, m1: Matrix){

    }

    fn identity(&self){
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