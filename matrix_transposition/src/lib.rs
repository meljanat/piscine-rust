struct Matrix {
    tuple1: (i32, i32),
    tuple2: (i32, i32),
}

pub fn transpose(m: Matrix) -> Matrix {
    Matrix {
        tuple1: (m.tuple1.0, m.tuple2.0),
        tuple2: (m.tuple1.1, m.tuple2.1),
    }
}