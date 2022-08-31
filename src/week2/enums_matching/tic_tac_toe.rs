use weblab::weblab;

#[weblab(programming_assignment)]
/// Last week, you implemented a 1-player version of tic tac toe. In this assignment, you need to implement the full game.
/// This board is now represented using `[[Square; 3]; 3]`, which is a 3x3 array of `Square`s.
///
/// - A `Square` is defined as an enum that is either `X`, `O` or `Empty`.
///
/// Implement a function that is given a board, and checks if any player has won.
///
/// Hint: It may help to define a helper function `has_won(Square, Square, Square) -> bool` that checks if any player has won in a certain row.
#[weblab(title = "Tic Tac Toe - 2 Player")]
#[weblab(weight = 2)]
mod assignment {
    #[weblab(solution)]
    mod solution {
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub enum Square {
            X,
            O,
            Empty,
        }

        pub fn check_board(board: [[Square; 3]; 3]) -> bool {
            fn has_won(a: Square, b: Square, c: Square) -> bool {
                if a == Square::Empty {
                    return false;
                }
                a == b && b == c
            }

            //Consider board[x][y]

            //Check horizontal
            for y in 0..3 {
                if has_won(board[0][y], board[1][y], board[2][y]) {
                    return true;
                }
            }

            //Check vertical
            #[allow(clippy::needless_range_loop)]
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

            false
        }
    }

    #[allow(unused_variables)]
    #[weblab(solution_template)]
    mod solution_template {
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub enum Square {
            X,
            O,
            Empty,
        }

        pub fn check_board(board: [[Square; 3]; 3]) -> bool {
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
                [Square::Empty, Square::Empty, Square::X],
                [Square::Empty, Square::X, Square::X],
                [Square::X, Square::Empty, Square::X],
            ];
            assert_eq!(check_board(board), true);
        }

        solution_only! {
            #[test]
            fn test_empty_board() {
                let board = [
                    [Square::Empty, Square::Empty, Square::Empty],
                    [Square::Empty, Square::Empty, Square::Empty],
                    [Square::Empty, Square::Empty, Square::Empty],
                ];
                assert_eq!(check_board(board), false);
            }
            #[test]
            fn test_full_board() {
                let board = [
                    [Square::X, Square::X, Square::X],
                    [Square::X, Square::X, Square::X],
                    [Square::X, Square::X, Square::X],
                ];
                assert_eq!(check_board(board), true);
            }
            #[test]
            fn test_horizontal_x() {
                let board = [
                    [Square::Empty, Square::Empty, Square::Empty],
                    [Square::X, Square::X, Square::X],
                    [Square::Empty, Square::Empty, Square::Empty],
                ];
                assert_eq!(check_board(board), true);
            }
            #[test]
            fn test_vertical_x() {
                let board = [
                    [Square::Empty, Square::X, Square::Empty],
                    [Square::Empty, Square::X, Square::Empty],
                    [Square::Empty, Square::X, Square::Empty],
                ];
                assert_eq!(check_board(board), true);
            }
            #[test]
            fn test_diagonal_x() {
                let board = [
                    [Square::Empty, Square::Empty, Square::X],
                    [Square::Empty, Square::X, Square::Empty],
                    [Square::X, Square::Empty, Square::Empty],
                ];
                assert_eq!(check_board(board), true);
            }
            #[test]
            fn test_horizontal_o() {
                let board = [
                    [Square::Empty, Square::Empty, Square::Empty],
                    [Square::O, Square::O, Square::O],
                    [Square::Empty, Square::Empty, Square::Empty],
                ];
                assert_eq!(check_board(board), true);
            }
            #[test]
            fn test_vertical_o() {
                let board = [
                    [Square::Empty, Square::O, Square::Empty],
                    [Square::Empty, Square::O, Square::Empty],
                    [Square::Empty, Square::O, Square::Empty],
                ];
                assert_eq!(check_board(board), true);
            }
            #[test]
            fn test_diagonal_o() {
                let board = [
                    [Square::Empty, Square::Empty, Square::O],
                    [Square::Empty, Square::O, Square::Empty],
                    [Square::O, Square::Empty, Square::Empty],
                ];
                assert_eq!(check_board(board), true);
            }


            pub fn check_board_ref(board: [[Square; 3]; 3]) -> bool {
                fn has_won(a: Square, b: Square, c: Square) -> bool {
                    if a == Square::Empty {
                        return false;
                    }
                    a == b && b == c
                }

                //Consider board[x][y]

                //Check horizontal
                for y in 0..3 {
                    if has_won(board[0][y], board[1][y], board[2][y]) {
                        return true;
                    }
                }

                //Check vertical
                #[allow(clippy::needless_range_loop)]
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

                false
            }

            #[test]
            fn test_all_boards() {
                use itertools::*;

                let x = [Square::Empty, Square::X, Square::O];
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
