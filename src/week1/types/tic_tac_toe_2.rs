use weblab::weblab;

#[weblab(programming_assignment)]
/// Last assignment, you implemented a 1-player version of tic tac toe. In this assignment, you need to implement the full game.
/// This board is now represented using `[[char; 3]; 3]`, which is a 3x3 array of characters.
///
/// - A cross is represented using the character 'x'
/// - A nought is represented using the character 'o' (not 0!)
/// - An empty square is represented using the character ' ' (space)
/// - You can assume all characters in the array will be 'x', 'o' or ' '
///
/// Implement a function that is given a board, and checks if any player has won.
///
/// Hint: It may help to define a helper function `has_won(char, char, char) -> bool` that checks if any player has won in a certain row.
#[weblab(title = "Tic Tac Toe - 2 Player")]
mod assignment {
    #[weblab(solution)]
    mod solution {
        pub fn check_board(board: [[char; 3]; 3]) -> bool {
            fn has_won(a: char, b: char, c: char) -> bool {
                if a == ' ' {
                    return false;
                }
                return a == b && b == c;
            }

            //Consider board[x][y]

            //Check horizontal
            for y in 0..3 {
                if has_won(board[0][y], board[1][y], board[2][y]) {
                    return true;
                }
            }

            //Check vertical
            for x in 0..3 {
                if has_won(board[x][0], board[x][1], board[x][2]) {
                    return true;
                }
            }

            //Check diagonal
            if has_won(board[0][0], board[1][1], board[2][2]) {
                return true;
            }
            if has_won(board[2][0], board[1][1], board[0][2]) {
                return true;
            }

            return false;
        }
    }

    #[allow(unused_variables)]
    #[weblab(solution_template)]
    mod solution_template {
        pub fn check_board(board: [[char; 3]; 3]) -> bool {
            todo!()
        }
    }

    #[weblab(test)]
    mod test {
        use super::solution::*;
        use weblab::{solution_only, template_only};

        pub fn check_board_ref(board: [[char; 3]; 3]) -> bool {
            fn has_won(a: char, b: char, c: char) -> bool {
                if a == ' ' {
                    return false;
                }
                return a == b && b == c;
            }

            //Consider board[x][y]

            //Check horizontal
            for y in 0..3 {
                if has_won(board[0][y], board[1][y], board[2][y]) {
                    return true;
                }
            }

            //Check vertical
            for x in 0..3 {
                if has_won(board[x][0], board[x][1], board[x][2]) {
                    return true;
                }
            }

            //Check diagonal
            if has_won(board[0][0], board[1][1], board[2][2]) {
                return true;
            }
            if has_won(board[2][0], board[1][1], board[0][2]) {
                return true;
            }

            return false;
        }

        #[test]
        fn test_board() {
            // --x
            // -xx
            // x-x
            let board = [
                [' ', ' ', 'x'],
                [' ', 'x', 'x'],
                ['x', ' ', 'x'],
            ];
            assert_eq!(check_board(board), true);
        }

        solution_only! {
            #[test]
            fn test_empty_board() {
                let board = [
                    [' ', ' ', ' '],
                    [' ', ' ', ' '],
                    [' ', ' ', ' '],
                ];
                assert_eq!(check_board(board), false);
            }
            #[test]
            fn test_full_board() {
                let board = [
                    ['x', 'x', 'x'],
                    ['x', 'x', 'x'],
                    ['x', 'x', 'x'],
                ];
                assert_eq!(check_board(board), true);
            }
            #[test]
            fn test_horizontal_x() {
                let board = [
                    [' ', ' ', ' '],
                    ['x', 'x', 'x'],
                    [' ', ' ', ' '],
                ];
                assert_eq!(check_board(board), true);
            }
            #[test]
            fn test_vertical_x() {
                let board = [
                    [' ', 'x', ' '],
                    [' ', 'x', ' '],
                    [' ', 'x', ' '],
                ];
                assert_eq!(check_board(board), true);
            }
            #[test]
            fn test_diagonal_x() {
                let board = [
                    [' ', ' ', 'x'],
                    [' ', 'x', ' '],
                    ['x', ' ', ' '],
                ];
                assert_eq!(check_board(board), true);
            }
            #[test]
            fn test_horizontal_o() {
                let board = [
                    [' ', ' ', ' '],
                    ['o', 'o', 'o'],
                    [' ', ' ', ' '],
                ];
                assert_eq!(check_board(board), true);
            }
            #[test]
            fn test_vertical_o() {
                let board = [
                    [' ', 'o', ' '],
                    [' ', 'o', ' '],
                    [' ', 'o', ' '],
                ];
                assert_eq!(check_board(board), true);
            }
            #[test]
            fn test_diagonal_o() {
                let board = [
                    [' ', ' ', 'o'],
                    [' ', 'o', ' '],
                    ['o', ' ', ' '],
                ];
                assert_eq!(check_board(board), true);
            }
            #[test]
            fn test_all_boards() {
                use itertools::*;

                let x = [' ', 'x', 'o'];
                for (a,b,c,d,e,f,g,h,i) in iproduct!(x,x,x,x,x,x,x,x,x) {
                    let board = [
                        [a,b,c],
                        [d,e,f],
                        [g,h,i]
                    ];
                    assert_eq!(check_board(board), check_board_ref(board));
                }
            }
        }
    }
}
