pub fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    [
        [matrix[0][0], matrix[1][0], matrix[2][0]],
        [matrix[0][1], matrix[1][1], matrix[2][1]],
        [matrix[0][2], matrix[1][2], matrix[2][2]],
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transpose_3x3_unit() {
        let matrix = [[1, 0, 0], [0, 1, 0], [0, 0, 1]];

        let ret = transpose(matrix);
        assert_eq!(ret, [[1, 0, 0], [0, 1, 0], [0, 0, 1],]);
    }

    #[test]
    fn transpose_3x3_random() {
        let matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];

        let ret = transpose(matrix);
        assert_eq!(ret, [[1, 4, 7], [2, 5, 8], [3, 6, 9],]);
    }
}
