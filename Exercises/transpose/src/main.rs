/************************************************************************************/
/*                                  NESTED ARRAYS                                   */
/************************************************************************************/
/* Use an array such as the above to write a function transposethat                 */
/* transposes a matrix (turns rows into columns):                                   */
/*                                                                                  */
/*           | 1 2 3 |     | 1 4 7 |                                                */
/* transpose | 4 5 6 |  =  | 2 5 8 |                                                */
/*           | 7 8 9 |     | 3 6 9 |                                                */
/*                                                                                  */
/************************************************************************************/


const MATRIX_SIZE: usize = 3;

fn transpose(matrix: [[i32; MATRIX_SIZE]; MATRIX_SIZE]) -> [[i32; MATRIX_SIZE]; MATRIX_SIZE] {

    let mut t_matrix = [[0; MATRIX_SIZE]; MATRIX_SIZE];

    for row_index in 0..=MATRIX_SIZE {
        let mut col_index: usize = row_index;
        while col_index < MATRIX_SIZE {
            if col_index == row_index {
                t_matrix[row_index][col_index] = matrix[row_index][col_index];
                col_index += 1;
                continue;
            }

            t_matrix[col_index][row_index] = matrix[row_index][col_index];
            t_matrix[row_index][col_index] = matrix[col_index][row_index];
            col_index += 1;
        }
    }

    return t_matrix;
}

fn main() {
    let matrix = [
    [101, 102, 103], // <-- the comment makes rustfmt add a newline
    [201, 202, 203],
    [301, 302, 303],
    ];

    println!("Original:");
    for row in matrix {
        println!("{row:?}");
    }

    let transposed = transpose(matrix);
    println!("\nTransposed:");
    for row in transposed {
        println!("{row:?}");
    }
}
