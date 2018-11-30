pub trait MinMaxGame: Sized {
    fn finished(&self) -> Option<i8>;
    fn moves(&self, player: bool) -> Vec<Self>;
}

#[derive(Default, Clone)]
struct Connect4Game {
    board: [[Option<bool>; 7]; 6],
}

impl std::str::FromStr for Connect4Game {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, (Self::Err)> {
        let mut squares = s.chars().filter_map(|c| match c {
            ' ' => Some(None),
            'O' => Some(Some(true)),
            'X' => Some(Some(false)),
            _ => None,
        });
        let mut game = Connect4Game::default();
        for row in (0..6).rev() {
            for column in 0..7 {
                game.board[row][column] = squares.next().unwrap_or(None);
            }
        }
        Ok(game)
    }
}

impl std::fmt::Debug for Connect4Game {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<")?;
        for row in (0..6).rev() {
            for col in 0..7 {
                let square = match self.board[row][col] {
                    None => ' ',
                    Some(true) => 'O',
                    Some(false) => 'X',
                };
                write!(f, "{}", square)?;
            }
            if row > 0 {
                write!(f, "┃")?;
            }
        }
        write!(f, ">")
    }
}

impl MinMaxGame for Connect4Game {
    fn finished(&self) -> Option<i8> {
        let board = self.board;

        for (value, player) in [(1, Some(true)), (-1, Some(false))].iter().cloned() {
            let vertical_search = || {
                for column in 0..7 {
                    let mut count = 0;
                    for row in board.iter() {
                        let square = row[column];
                        count = if square == player { count + 1 } else { 0 };
                        if count >= 4 {
                            return Some(value);
                        }
                    }
                }
                None
            };
            let horizontal_search = || {
                for row in board.iter() {
                    let mut count = 0;
                    for square in row.iter().cloned() {
                        count = if square == player { count + 1 } else { 0 };
                        if count >= 4 {
                            return Some(value);
                        }
                    }
                }
                None
            };
            let north_east_diagonal_search = || {
                for diagonal in 3..9usize {
                    let mut count = 0;
                    let mut column = diagonal.saturating_sub(5);
                    while column < 7 && column <= diagonal {
                        let row = diagonal - column;
                        let square = board[row][column];

                        count = if square == player { count + 1 } else { 0 };
                        if count >= 4 {
                            return Some(value);
                        }
                        column += 1;
                    }
                }
                None
            };
            let north_west_diagonal_search = || {
                for diagonal in 3..9usize {
                    let mut count = 0;
                    let mut column = diagonal.saturating_sub(5);
                    let mut row = 5usize.saturating_sub(diagonal);
                    while column < 7 && row < 6 {
                        let square = board[row][column];
                        count = if square == player { count + 1 } else { 0 };
                        if count >= 4 {
                            return Some(value);
                        }

                        row += 1;
                        column += 1;
                    }
                }
                None
            };
            if let Some(result) = vertical_search()
                .or_else(horizontal_search)
                .or_else(north_east_diagonal_search)
                .or_else(north_west_diagonal_search)
            {
                return Some(result);
            };
        }
        if (0..42).all(|i| board[i / 7][i % 7].is_some()) {
            Some(0)
        } else {
            None
        }
    }

    fn moves(&self, player: bool) -> Vec<Self> {
        let board = self.board;
        let mut moves = Vec::with_capacity(7);
        for column in [3, 2, 4, 1, 5, 0, 6].iter().cloned() {
            for row in (0..6).rev() {
                if board[row][column] == None {
                    let mut new_game = self.clone();
                    new_game.board[row][column] = Some(player);
                    moves.push(new_game);
                    break;
                }
            }
        }
        moves
    }
}

#[derive(Clone, Default, PartialEq)]
struct TicTacToeGame {
    board: [[Option<bool>; 3]; 3],
}

impl std::fmt::Debug for TicTacToeGame {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<")?;
        for row in 0..3 {
            for col in 0..3 {
                let square = match self.board[row][col] {
                    None => ' ',
                    Some(true) => 'O',
                    Some(false) => 'X',
                };
                write!(f, "{}", square)?;
            }
            if row < 2 {
                write!(f, "┃")?;
            }
        }
        write!(f, ">")
    }
}

impl std::str::FromStr for TicTacToeGame {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, (Self::Err)> {
        let mut squares = s.chars().filter_map(|c| match c {
            ' ' => Some(None),
            'O' => Some(Some(true)),
            'X' => Some(Some(false)),
            _ => None,
        });
        let mut s = || squares.next().ok_or("Failed to extract 9 squares");
        Ok(TicTacToeGame {
            board: [[s()?, s()?, s()?], [s()?, s()?, s()?], [s()?, s()?, s()?]],
        })
    }
}

impl MinMaxGame for TicTacToeGame {
    fn finished(&self) -> Option<i8> {
        let board = self.board;
        for (value, player) in [(1, Some(true)), (-1, Some(false))].iter().cloned() {
            if board[0][0] == player && board[1][1] == player && board[2][2] == player {
                return Some(value);
            }
            if board[0][2] == player && board[1][1] == player && board[2][0] == player {
                return Some(value);
            }
            for i in 0..3 {
                if board[0][i] == player && board[1][i] == player && board[2][i] == player {
                    return Some(value);
                }
                if board[i][0] == player && board[i][1] == player && board[i][2] == player {
                    return Some(value);
                }
            }
        }
        if (0..9).all(|i| board[i / 3][i % 3].is_some()) {
            Some(0)
        } else {
            None
        }
    }

    fn moves(&self, player: bool) -> Vec<Self> {
        let board = self.board;
        (0..9)
            .filter(|&i| board[i / 3][i % 3].is_none())
            .map(|i| {
                let mut new_game = self.clone();
                new_game.board[i / 3][i % 3] = Some(player);
                new_game
            }).collect()
    }
}

pub mod min_max_game_strategy {
    use MinMaxGame;

    fn best_pick(a: Option<i8>, b: Option<i8>, maximise: bool) -> Option<i8> {
        match (a, b) {
            (None, None) => None,
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (Some(a), Some(b)) => Some(if maximise {
                std::cmp::max(a, b)
            } else {
                std::cmp::min(a, b)
            }),
        }
    }

    pub fn minimax<G: MinMaxGame>(
        game: &G,
        mut alpha: Option<i8>,
        mut beta: Option<i8>,
        player: bool,
    ) -> (i8, Option<G>) {
        match game.finished() {
            Some(score) => (score, None),
            None => {
                let moves = game.moves(player);
                let mut value = None;
                let mut best_move = None;
                for r#move in moves {
                    let old_value = value;
                    value = best_pick(
                        value,
                        Some(minimax(&r#move, alpha, beta, !player).0),
                        player,
                    );
                    if old_value != value {
                        best_move = Some(r#move);
                    }
                    if player {
                        alpha = best_pick(alpha, value, player);
                    } else {
                        beta = best_pick(beta, value, player);
                    }

                    if let (Some(alpha), Some(beta)) = (alpha, beta) {
                        if alpha >= beta {
                            break;
                        }
                    }
                }
                (value.unwrap_or(0), best_move)
            }
        }
    }

    pub fn next<G: MinMaxGame>(game: &G, player: bool) -> Option<G> {
        minimax(game, None, None, player).1
    }
}

#[cfg(test)]
mod connect_4_tests {
    #[test]
    fn from_str_debug() {
        use Connect4Game;
        for s in [
            "<       ┃       ┃       ┃       ┃       ┃       >",
            "<X   O  ┃       ┃       ┃       ┃       ┃       >",
        ]
            .into_iter()
        {
            let g = s.parse::<Connect4Game>().unwrap();
            assert_eq!(&format!("{:?}", g), s);
        }
    }

    #[test]
    fn finished() {
        use Connect4Game;
        use MinMaxGame;

        let f = |string: &str| string.parse::<Connect4Game>().unwrap().finished();

        // Not finished
        assert_eq!(
            f("<X   O  ┃       ┃       ┃       ┃       ┃       >"),
            None
        );
        // No winner
        assert_eq!(
            f("<OOOXXXO┃XXXOOOX┃OOOXXXO┃XXXOOOX┃OOOXXXO┃XXXOOOX>"),
            Some(0)
        );

        // Vertical
        assert_eq!(
            f("<O   XXX┃O      ┃O      ┃O      ┃       ┃       >"),
            Some(1)
        );

        // Horizontal
        assert_eq!(
            f("<OOOOXXX┃       ┃       ┃       ┃       ┃       >"),
            Some(1)
        );

        // First North East Diagonal
        assert_eq!(
            f("<OXXOX  ┃XOOXX  ┃OXXO   ┃ OOX   ┃  OO   ┃   O   >"),
            Some(1)
        );

        // Diagonal North East left border
        assert_eq!(
            f("<OXXX   ┃ OOX   ┃ OOX   ┃   O   ┃       ┃       >"),
            Some(1)
        );

        // Last Diagonal North East right border
        assert_eq!(
            f("<   OXXX┃   XOOX┃     OO┃      O┃       ┃       >"),
            Some(1)
        );

        // First Diaonal North West Left border
        assert_eq!(
            f("<XXXOX  ┃XOO    ┃OO     ┃O      ┃       ┃       >"),
            Some(1)
        );

        // Last Diagonal North West right border
        assert_eq!(
            f("< X XXXO┃   XOOX┃   OXXO┃   XOO ┃   OO  ┃   O   >"),
            Some(1)
        );

        // Other winner
        assert_eq!(
            f("<OO XXXX┃OO     ┃       ┃       ┃       ┃       >"),
            Some(-1)
        )
    }

    #[test]
    fn complete_game() {
        use min_max_game_strategy::next;
        use Connect4Game;

        let mut game = "<XXXOOO ┃XOO    ┃OX     ┃XO     ┃XX     ┃OO     >"
            .parse::<Connect4Game>()
            .unwrap();
        let mut player = true;
        loop {
            match next(&game, player) {
                Some(g) => game = g,
                None => break,
            }
            println!("{:?}", game);
            player = !player;
        }
        panic!("blah")
    }
}

#[cfg(test)]
mod min_max_strategy_tests {
    #[test]
    fn finishing_move_x() {
        use min_max_game_strategy::next;
        use TicTacToeGame;

        let g = "<O O┃ O ┃X X>".parse::<TicTacToeGame>().unwrap();
        let e = "<O O┃ O ┃XXX>".parse().unwrap();
        assert_eq!(next(&g, false), Some(e));
    }

    #[test]
    fn finishing_move_o() {
        use min_max_game_strategy::next;
        use TicTacToeGame;

        let g = "<O O┃   ┃X X>".parse::<TicTacToeGame>().unwrap();
        let e = "<OOO┃   ┃X X>".parse().unwrap();
        assert_eq!(next(&g, true), Some(e));
    }

    // #[test]
    // fn complete_game() {
    //     use min_max_game_strategy::next;
    //     use TicTacToeGame;

    //     let mut game = TicTacToeGame::default();
    //     let mut player = true;
    //     loop {
    //         match next(&game, player) {
    //             Some(g) => game = g,
    //             None => break,
    //         }
    //         println!("{:?}", game);
    //         player = !player;
    //     }
    //     panic!("blah")
    // }
}

#[cfg(test)]
mod tic_tac_toe_game_tests {
    #[test]
    fn finished() {
        use MinMaxGame;
        use TicTacToeGame;

        let f = |string: &str| string.parse::<TicTacToeGame>().unwrap().finished();

        // Not finished
        assert_eq!(f("<O O┃   ┃ X >"), None);
        // Horizontal
        assert_eq!(f("<OOO┃   ┃ XX>"), Some(1));
        assert_eq!(f("<   ┃OOO┃ XX>"), Some(1));
        assert_eq!(f("<   ┃ XX┃OOO>"), Some(1));
        // Vertical
        assert_eq!(f("<O  ┃O  ┃OXX>"), Some(1));
        assert_eq!(f("< O ┃ O ┃XOX>"), Some(1));
        assert_eq!(f("<  O┃  O┃XXO>"), Some(1));
        // Diagonal
        assert_eq!(f("<O  ┃ O ┃XXO>"), Some(1));
        assert_eq!(f("<  O┃ O ┃OXX>"), Some(1));
        // Other player
        assert_eq!(f("<O O┃ O ┃XXX>"), Some(-1));
        // No win
        assert_eq!(f("<OXO┃XOO┃XOX>"), Some(0));
    }

    #[test]
    fn debug() {
        use TicTacToeGame;
        assert_eq!(
            format!("{:?}", TicTacToeGame::default()),
            "<   ┃   ┃   >"
        );
    }

    #[test]
    fn moves() {
        use std::str::FromStr;
        use MinMaxGame;
        use TicTacToeGame;
        let states: Vec<TicTacToeGame> = TicTacToeGame::from_str("<  O┃   ┃ X >")
            .unwrap()
            .moves(true);
        assert_eq!(
            states,
            vec!(
                "<O O┃   ┃ X >".parse().unwrap(),
                "< OO┃   ┃ X >".parse().unwrap(),
                "<  O┃O  ┃ X >".parse().unwrap(),
                "<  O┃ O ┃ X >".parse().unwrap(),
                "<  O┃  O┃ X >".parse().unwrap(),
                "<  O┃   ┃OX >".parse().unwrap(),
                "<  O┃   ┃ XO>".parse().unwrap(),
            )
        );
    }

    #[test]
    fn from_str() {
        use TicTacToeGame;
        let game = "<  O┃   ┃ X >";
        assert_eq!(
            format!("{:?}", game.parse::<TicTacToeGame>().unwrap()),
            game
        );
    }
}
