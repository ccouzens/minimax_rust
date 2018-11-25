pub trait MinMaxGame: Sized {
    fn finished(&self) -> Option<i8>;
    fn moves(&self, player: bool) -> Vec<Self>;
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
    pub fn minimax<G: MinMaxGame>(game: &G, player: bool) -> i8 {
        match game.finished() {
            Some(score) => score,
            None => {
                let moves = game.moves(player);
                let following_scores = moves.iter().map(|g| minimax::<G>(&g, !player));
                if player {
                    following_scores.max()
                } else {
                    following_scores.min()
                }.unwrap_or(0)
            }
        }
    }

    pub fn next<G: MinMaxGame>(game: &G, player: bool) -> Option<G> {
        match game.finished() {
            Some(_) => None,
            None => {
                let moves = game.moves(player).into_iter();
                let key = |game: &G| minimax::<G>(game, !player);
                if player {
                    moves.max_by_key(key)
                } else {
                    moves.min_by_key(key)
                }
            }
        }
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
