use weblab::weblab;

#[weblab(programming_assignment)]
/// Tic-tac-toe is a paper-and-pencil game for two players who take turns marking the spaces in a three-by-three grid with X or O.
/// The player who succeeds in placing three of their marks in a horizontal, vertical, or diagonal row is the winner.
/// For more information see: https://en.wikipedia.org/wiki/Tic-tac-toe
///
/// For this assignment, consider a board from a one-player variant of the game, using only crosses.
/// So, the player wins by placing three crosses in a horizontal, vertical, or diagonal row.
/// This board is represented using `[[bool; 3]; 3]`, which is a 3x3 array of booleans.
/// A square has a cross if the corresponding boolean is true, and is empty otherwise.
/// Squares of 2d arrays can be accessed using `board[x][y]`
///
/// Implement a function that is given a board, and checks if the player has won.
#[weblab(title = "Tic Tac Toe - 1 Player")]
mod assignment {
    #[weblab(solution)]
    mod solution {
        pub fn check_board(board: [[bool; 3]; 3]) -> bool {
            //Consider board[x][y]

            //Check horizontal
            for y in 0..3 {
                if board[0][y] && board[1][y] && board[2][y] {
                    return true;
                }
            }

            //Check vertical
            #[allow(clippy::needless_range_loop)]
            for x in 0..3 {
                if board[x][0] && board[x][1] && board[x][2] {
                    return true;
                }
            }

            //Check diagonal
            if board[0][0] && board[1][1] && board[2][2] {
                return true;
            }
            if board[2][0] && board[1][1] && board[0][2] {
                return true;
            }

            false
        }
    }

    #[allow(unused_variables)]
    #[weblab(solution_template)]
    mod solution_template {
        pub fn check_board(board: [[bool; 3]; 3]) -> bool {
            todo!()
        }
    }

    #[weblab(test)]
    mod test {
        use super::solution::*;
        use weblab::{solution_only, template_only};

        #[test]
        fn test_board() {
            // --x
            // -xx
            // x-x
            let board = [
                [false, false, true],
                [false, true, true],
                [true, false, true],
            ];
            assert_eq!(check_board(board), true);
        }

        solution_only! {
            #[test]
            fn test_empty_board() {
                let board = [
                    [false, false, false],
                    [false, false, false],
                    [false, false, false],
                ];
                assert_eq!(check_board(board), false);
            }
            #[test]
            fn test_full_board() {
                let board = [
                    [true, true, true],
                    [true, true, true],
                    [true, true, true],
                ];
                assert_eq!(check_board(board), true);
            }
            #[test]
            fn test_horizontal() {
                let board = [
                    [false, false, false],
                    [true, true, true],
                    [false, false, false],
                ];
                assert_eq!(check_board(board), true);
            }
            #[test]
            fn test_vertical() {
                let board = [
                    [false, true, false],
                    [false, true, false],
                    [false, true, false],
                ];
                assert_eq!(check_board(board), true);
            }

            #[test]
            fn test_diagonal_1() {
                let board2 = [
                    [true, false, false],
                    [false, true, false],
                    [false, false, true],
                ];
                assert_eq!(check_board(board2), true);
            }

            #[test]
            fn test_diagonal_2() {
                let board = [
                    [false, false, true],
                    [false, true, false],
                    [true, false, false],
                ];
                assert_eq!(check_board(board), true);
            }

            #[test]
            fn test_boundaries_2() {
                let board = [
                    [true, true, false],
                    [true, false, false],
                    [false, false, true],
                ];
                assert_eq!(check_board(board), false);
            }

            #[test]
            fn test_boundary_2() {
                let board2 = [
                    [false, true, true],
                    [false, true, false],
                    [false, false, false],
                ];
                assert_eq!(check_board(board2), false);
            }

            pub fn check_board_ref(board: [[bool; 3]; 3]) -> bool {
                //Consider board[x][y]

                //Check horizontal
                for y in 0..3 {
                    if board[0][y] && board[1][y] && board[2][y] {
                        return true;
                    }
                }

                //Check vertical
                #[allow(clippy::needless_range_loop)]
                for x in 0..3 {
                    if board[x][0] && board[x][1] && board[x][2] {
                        return true;
                    }
                }

                //Check diagonal
                if board[0][0] && board[1][1] && board[2][2] {
                    return true;
                }
                if board[2][0] && board[1][1] && board[0][2] {
                    return true;
                }

                false
            }

            #[test]
            fn test_all_boards() {
                for i in 0..(2<<9) {
                    let board = [
                        [i & (2<<0) != 0, i & (2<<1) != 0, i & (2<<2) != 0],
                        [i & (2<<3) != 0, i & (2<<4) != 0, i & (2<<5) != 0],
                        [i & (2<<6) != 0, i & (2<<7) != 0, i & (2<<8) != 0],
                    ];
                    assert_eq!(check_board(board), check_board_ref(board));
                }
            }
        }
    }
}
