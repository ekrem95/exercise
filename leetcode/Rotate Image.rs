impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        for i in 0..matrix.len() {
            for j in i..matrix[i].len() {
                let ib = matrix[i].clone();
                matrix[i][j] = matrix[j][i];
                matrix[j][i] = ib[j];
            }
        }
        for i in 0..matrix.len() {
            matrix[i].reverse();
        }
    }
}