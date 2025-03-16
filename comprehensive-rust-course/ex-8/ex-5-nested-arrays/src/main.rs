fn main() {
    let matrix = [
        [101, 102, 103], //
        [201, 202, 203],
        [301, 302, 303],
    ];

    dbg!(matrix);

    let transposed: [[i32; 3]; 3] = transpose(matrix);
    dbg!(transposed);
}

fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut to_transpose: [[i32; 3]; 3] = [[0; 3]; 3];

    for i in 0..3 {
      for j in 0..3 {
        to_transpose[j][i] = matrix[i][j];
      }
    }
    
    to_transpose
}
