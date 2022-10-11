use crate::constants;
use crate::types::{GameState, GameStateInstance, GameStatusCheckResult, Move, MoveCheckResult};

#[derive(Debug, Clone)]
pub struct Bagchal {
    pub turn: i8,
    pub goat_counter: i8,
    pub goat_captured: i8,
    pub game_state: GameState,
    pub game_history: Vec<GameStateInstance>,
    pub pgn: String,
    pub prev_move: Option<Move>,
    pub move_reward_tiger: Vec<f32>,
    pub move_reward_goat: Vec<f32>,
    pub trapped_tiger: i8,
}

impl Default for Bagchal {
    fn default() -> Self {
        Self {
            turn: 1,
            goat_counter: 0,
            goat_captured: 0,
            game_state: GameState::NotDecided,
            game_history: [GameStateInstance::default()].to_vec(),
            pgn: "".to_string(),
            prev_move: None,
            move_reward_tiger: [].to_vec(),
            move_reward_goat: [].to_vec(),
            trapped_tiger: 0,
        }
    }
}

impl Bagchal {
    fn board(&self) -> [[i8; 5]; 5] {
        return self.game_history[1].board;
    }

    fn move_count(&self) -> i8 {
        return (self.game_history.len() - 1) as i8;
    }

    fn cord_to_char(num: i8) -> char {
        match num {
            0 => return 'A',
            1 => return 'B',
            2 => return 'C',
            3 => return 'D',
            4 => return 'E',
            _ => return 'X',
        }
    }

    fn char_to_cord(c: char) -> i8 {
        match c {
            'A' => return 0,
            'B' => return 1,
            'C' => return 2,
            'D' => return 3,
            'E' => return 4,
            _ => return -1,
        }
    }

    fn pos_dec(num: i8) -> [i8; 5] {
        match num {
            1 => return [1, 0, 0, 0, 0],
            2 => return [0, 1, 0, 0, 0],
            3 => return [0, 0, 1, 0, 0],
            4 => return [0, 0, 0, 1, 0],
            5 => return [0, 0, 0, 0, 1],
            _ => return [0, 0, 0, 0, 0],
        }
    }

    fn pgn_unit_to_coord(pgn: String) -> Move {
        let source: Option<[i8; 2]>;

        let mut pgn_iter = pgn.chars();

        if pgn_iter.nth(0).unwrap() == 'X' {
            source = None;
        } else {
            source = Some([
                5 - pgn_iter.nth(1).unwrap().to_digit(10).unwrap() as i8,
                Bagchal::char_to_cord(pgn_iter.nth(0).unwrap()),
            ]);
        }

        let destination = [
            5 - pgn_iter.nth(3).unwrap().to_digit(10).unwrap() as i8,
            Bagchal::char_to_cord(pgn_iter.nth(2).unwrap()),
        ];

        return (source, destination);
    }

    fn coord_to_png_unit(source: Option<[i8; 2]>, destination: [i8; 2]) -> String {
        let mut unit = String::new();

        // Source coordinates to PGN
        if source.is_none() {
            unit = "XX".to_string();
        } else {
            unit.push(Bagchal::cord_to_char(source.unwrap()[1]));
            unit.push_str(&(5 - source.unwrap()[0]).to_string());
        };

        // Destination coordinates to PGN
        unit.push(Bagchal::cord_to_char(destination[1]));
        unit.push_str(&(5 - destination[0]).to_string());

        return unit;
    }

    fn check_trapped_tiger(&self) {
        let count = 0;

        for i in 0..5 {
            for j in 0..5 {
                let board = self.board();
                if board[i][j] == -1 {
                    let has_move = false;

                    for k in 0..5 {
                        for l in 0..5 {
                            res = self.check_move()
                        }
                    }
                }
            }
        }
    }

    fn make_move(
        &mut self,
        source: Option<[i8; 2]>,
        target: [i8; 2],
        eval_res: Option<MoveCheckResult>,
    ) -> bool {
        let prev_captured = self.goat_captured;
        let prev_trapped = self.trapped_tiger;

        let move_eval: MoveCheckResult;
        if eval_res.is_none() {
            move_eval = self.check_move(source, target, None);
        } else {
            move_eval = eval_res.unwrap();
        }

        if !move_eval.is_valid {
            return false;
        }

        self.move_reward_goat.push(0f32);
        self.move_reward_tiger.push(0f32);

        let mut new_state = self.board().clone();

        if move_eval.is_place_move {
            new_state[target[0usize] as usize][target[1usize] as usize] = 1;
            self.goat_counter += 1;
        } else {
            if move_eval.is_capture_move {
                let piece = move_eval.capture_piece.unwrap();
                new_state[piece[0] as usize][piece[1] as usize] = 0;
            }

            new_state[source.unwrap()[0usize] as usize][source.unwrap()[1usize] as usize] = 0;
            new_state[target[0usize] as usize][target[1usize] as usize] = self.turn;
        }

        // Change the turn
        self.turn = if self.turn == -1 { 1 } else { -1 };

        // Push to game history
        self.game_history.push(GameStateInstance {
            board: new_state,
            goat_count: self.goat_counter,
            goat_captured: self.goat_captured,
        });

        // Append PGN unit after move
        if self.pgn == "" {
            self.pgn = Bagchal::coord_to_png_unit(source, target);
        } else {
            self.pgn.push('-');
            self.pgn
                .push_str(&Bagchal::coord_to_png_unit(source, target));
        }

        self.prev_move = Some((source, target));

        // Goats captured check
        if prev_captured != self.goat_captured {
            *self.move_reward_goat.last_mut().unwrap() += constants::G_GOAT_CAPTURED;
            *self.move_reward_tiger.last_mut().unwrap() += constants::T_GOAT_CAPTURE;
        }

        // Trapped tiger check
        if prev_trapped < self.trapped_tiger {
            *self.move_reward_goat.last_mut().unwrap() += constants::G_TIGER_TRAP;
            *self.move_reward_tiger.last_mut().unwrap() += constants::T_GOT_TRAPPED;
        } else if prev_trapped > self.trapped_tiger {
            *self.move_reward_goat.last_mut().unwrap() += constants::G_TIGER_ESCAPE;
            *self.move_reward_tiger.last_mut().unwrap() += constants::T_TRAP_ESCAPE;
        }

        // Has game been decided check
        let status_after_move = self.game_status_check();

        if status_after_move.decided {
            if status_after_move.won_by == -1 {
                self.game_state = GameState::TigerWon;
                *self.move_reward_tiger.last_mut().unwrap() += constants::T_WIN;
                *self.move_reward_goat.last_mut().unwrap() += constants::G_LOSE;
            } else {
                self.game_state = GameState::GoatWon;
                *self.move_reward_goat.last_mut().unwrap() += constants::G_WIN;
                *self.move_reward_tiger.last_mut().unwrap() += constants::T_LOSE;
            }
        }

        return true;
    }

    fn check_move(
        &self,
        source: Option<[i8; 2]>,
        target: [i8; 2],
        assuming_turn: Option<i8>,
    ) -> MoveCheckResult {
        let turn: i8;

        if assuming_turn.is_some() {
            turn = assuming_turn.unwrap();
        } else {
            turn = self.turn;
        }

        let m = target[0] as usize;
        let n = target[1] as usize;

        let position = self.board();

        // Place Move Check
        if source.is_none() {
            // If turn is of goat
            if turn == 1 {
                // If all goats have been placed
                if self.goat_counter >= 20 {
                    return MoveCheckResult {
                        is_valid: false,
                        reason: "Cannot place any more goats!".to_string(),
                        ..Default::default()
                    };
                }
                // If all goats haven't been placed
                else {
                    // If target position already has a piece
                    if position[m][n] != 0 {
                        return MoveCheckResult {
                            is_valid: false,
                            reason: "Target already has a piece!".to_string(),
                            ..Default::default()
                        };
                    }
                    // If target doesn't have a piece
                    else {
                        return MoveCheckResult {
                            is_valid: true,
                            reason: "Goat place move!".to_string(),
                            is_place_move: true,
                            ..Default::default()
                        };
                    }
                }
            }
            // If turn is of tiger
            else {
                return MoveCheckResult {
                    is_valid: false,
                    reason: "Tiger can't place!".to_string(),
                    ..Default::default()
                };
            }
        }

        let x = source.unwrap()[0] as usize;
        let y = source.unwrap()[1] as usize;

        // Board boundary check
        if x < 0 || y < 0 || m < 0 || n < 0 || x > 4 || y > 4 || m > 4 || n > 4 {
            return MoveCheckResult {
                is_valid: false,
                reason: "Cannot move outside the board!".to_string(),
                ..Default::default()
            };
        }

        // Game state check
        if self.game_state != GameState::NotDecided {
            return MoveCheckResult {
                is_valid: false,
                reason: "Cannot move after game has been decided!".to_string(),
                ..Default::default()
            };
        }

        // Turn check
        if !((turn == 1 && position[x][y] == 1) || (turn == -1 && position[x][y] == -1)) {
            return MoveCheckResult {
                is_valid: false,
                reason: "Cannot move in other's turn!".to_string(),
                ..Default::default()
            };
        }

        // Goat can't move before placing all goats
        if turn == 1 && self.goat_counter < 20 {
            return MoveCheckResult {
                is_valid: false,
                reason: "Cannot move goat before all goats are placed".to_string(),
                ..Default::default()
            };
        }

        // Target already has a piece check
        if position[m][n] != 0 {
            return MoveCheckResult {
                is_valid: false,
                reason: "Target already has a piece!".to_string(),
                ..Default::default()
            };
        }

        let x_diff_abs = ((x - m) as i8).abs() as usize;
        let y_diff_abs = ((y - n) as i8).abs() as usize;
        let x_diff = m - x;
        let y_diff = n - y;
        let s_sum = x + y;
        let t_sum = m + n;

        // Cannot move distance more than 2 check
        if x_diff_abs > 2 || y_diff_abs > 2 {
            return MoveCheckResult {
                is_valid: false,
                reason: "Cannot move distance more than 2!".to_string(),
                ..Default::default()
            };
        }

        // Source and target check
        if x_diff_abs == 0 && y_diff_abs == 0 {
            return MoveCheckResult {
                is_valid: false,
                reason: "Source and target can't be same!".to_string(),
                ..Default::default()
            };
        }

        // Odd to Odd position check
        if s_sum % 2 != 0 && t_sum % 2 != 0 {
            return MoveCheckResult {
                is_valid: false,
                reason: "Cannot move from odd position to another odd position!".to_string(),
                ..Default::default()
            };
        }

        // Tiger jump over goat check
        if turn == -1 {
            let can_jump = match (x_diff_abs, y_diff_abs) {
                (2, 2) => s_sum % 2 == 0, // Can only jump diagonally from even position
                (2, 0) => true,
                (0, 2) => true,
                _ => false,
            };

            if !can_jump {
                return MoveCheckResult {
                    is_valid: false,
                    reason: "Requested jump move is invalid!".to_string(),
                    ..Default::default()
                };
            }

            let piece_to_capture = [(x + x_diff / 2) as i8, (y + y_diff / 2) as i8];

            // If capture piece is goat
            if position[piece_to_capture[0] as usize][piece_to_capture[1] as usize] == 1 {
                return MoveCheckResult {
                    is_valid: true,
                    is_capture_move: true,
                    capture_piece: Some(piece_to_capture),
                    reason: "Can capture goat!".to_string(),
                    ..Default::default()
                };
            } else {
                return MoveCheckResult {
                    is_valid: false,
                    reason: "Cannot capture tiger!".to_string(),
                    ..Default::default()
                };
            }
        }

        // if turn == -1
        //     && ((x_diff_abs == 2 && y_diff_abs == 0)
        //         || (y_diff_abs == 2 && (x_diff_abs == 0 || x_diff_abs == 2)))
        // {
        //     if x_diff_abs == 2 && y_diff_abs == 2 {
        //         if s_sum % 2 != 0 {
        //             return MoveCheckResult {
        //                 is_valid: false,
        //                 reason: "Cannot jump diagonally from odd position!".to_string(),
        //                 ..Default::default()
        //             };
        //         } else {
        //             let piece_to_capture = [(x + x_diff / 2) as i8, (y + y_diff / 2) as i8];
        //
        //             if position[piece_to_capture[0] as usize][piece_to_capture[1] as usize] == 1 {
        //                 return MoveCheckResult {
        //                     is_valid: true,
        //                     is_capture_move: true,
        //                     capture_piece: Some(piece_to_capture),
        //                     reason: "Can capture goat!".to_string(),
        //                     ..Default::default()
        //                 };
        //             } else {
        //                 return MoveCheckResult {
        //                     is_valid: false,
        //                     reason: "Cannot capture tiger!".to_string(),
        //                     ..Default::default()
        //                 };
        //             }
        //         }
        //     }
        // }

        return MoveCheckResult::default();
    }

    fn goat_can_move(&self) -> bool {
        if self.goat_counter < 20 && self.turn == 1 {
            return true;
        }

        let position = self.board();

        for i in 0i8..5 {
            for j in 0i8..5 {
                if position[i as usize][j as usize] == 1 {
                    for k in -1i8..2 {
                        for l in -1i8..2 {
                            let this_move = self.check_move(Some([i, j]), [i + k, j + l], None);
                            if this_move.is_valid {
                                return true;
                            }
                        }
                    }
                }
            }
        }
        false
    }

    fn tiger_can_move(&self) -> bool {
        let position = self.board();

        for i in 0i8..5 {
            for j in 0i8..5 {
                if position[i as usize][j as usize] == -1 {
                    for k in 0i8..5 {
                        for l in 0i8..5 {
                            let this_move = self.check_move(Some([i, j]), [k, l], None);
                            if this_move.is_valid {
                                return true;
                            }
                        }
                    }
                }
            }
        }
        false
    }

    fn game_status_check(&self) -> GameStatusCheckResult {
        if self.goat_captured >= 5 {
            return GameStatusCheckResult {
                decided: true,
                won_by: -1,
            };
        } else if self.turn == -1 && !self.tiger_can_move() {
            return GameStatusCheckResult {
                decided: true,
                won_by: 1,
            };
        } else if self.turn == 1 && !self.goat_can_move() {
            return GameStatusCheckResult {
                decided: true,
                won_by: -1,
            };
        } else {
            return GameStatusCheckResult {
                decided: false,
                won_by: -1,
            };
        }
    }

    fn get_possible_moves(self) {
        let moves = Vec::<Move>::new();

        let position = self.board();

        if self.turn == -1 {
            for i in 0i8..5 {
                for j in 0i8..5 {
                    if position[i as usize][j as usize] == -1 {
                        for k in 0i8..5 {
                            for l in 0i8..5 {
                                let this_move = self.check_move(Some([i, j]), [i + k, j + l], None);
                                if this_move.is_valid {
                                    let new_move_state = self.clone();
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
