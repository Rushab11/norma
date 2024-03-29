use pyo3::prelude::*;

// pub mod bagchal;
// pub mod constants;
// pub mod move_translator;
// pub mod types;
// pub mod utils;
// pub mod traits;
pub mod game;
pub mod helpers;

use game::bagchal::BaghchalRS;
use helpers::{
    types::*,
    utils::{coord_to_png_unit, pgn_unit_to_coord},
};
use pythonize::depythonize;
use serde::{Deserialize, Serialize};

#[pyclass]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Baghchal {
    inner: BaghchalRS,
}

#[pymethods]
impl Baghchal {
    #[new]
    fn python_new(
        turn: Option<i8>,
        goat_counter: Option<i8>,
        goat_captured: Option<i8>,
        game_state: Option<GameStatus>,
        game_history: Option<Vec<PyObject>>,
        // game_history: Option<Vec<HashMap<String, PyObject>>>,
        // game_history: Option<String>,
        pgn: Option<String>,
        prev_move: Option<Option<Move>>,
        move_reward_tiger: Option<Vec<f32>>,
        move_reward_goat: Option<Vec<f32>>,
        trapped_tiger: Option<i8>,
    ) -> PyResult<Self> {
        let mut obj = Self::default();

        if turn.is_some() {
            obj.inner.turn = turn.unwrap();
        };

        if goat_counter.is_some() {
            obj.inner.goat_counter = goat_counter.unwrap();
        };

        if goat_captured.is_some() {
            obj.inner.goat_captured = goat_captured.unwrap();
        };

        if game_state.is_some() {
            obj.inner.game_state = game_state.unwrap();
        };

        if game_history.is_some() {
            Python::with_gil(|py| {
                obj.inner.game_history = game_history
                    .unwrap()
                    .into_iter()
                    .map(|item| depythonize(item.as_ref(py)).unwrap())
                    .collect::<Vec<GameStateInstance>>();
            })
        };

        if pgn.is_some() {
            obj.inner.pgn = pgn.unwrap();
        };

        if prev_move.is_some() {
            obj.inner.prev_move = prev_move.unwrap();
        };

        if move_reward_tiger.is_some() {
            obj.inner.move_reward_tiger = move_reward_tiger.unwrap();
        };

        if move_reward_goat.is_some() {
            obj.inner.move_reward_goat = move_reward_goat.unwrap();
        };

        if trapped_tiger.is_some() {
            obj.inner.trapped_tiger = trapped_tiger.unwrap();
        };

        return Ok(obj);
    }

    #[staticmethod]
    pub fn default() -> Self {
        return Self {
            inner: BaghchalRS::default(),
        };
    }

    #[staticmethod]
    pub fn from_str(serialized: &str) -> Self {
        let deserialized: Self = serde_json::from_str(&serialized).unwrap();

        return deserialized;
    }

    #[staticmethod]
    pub fn pgn_unit_to_coord(pgn: String) -> Move {
        return pgn_unit_to_coord(pgn);
    }

    #[staticmethod]
    pub fn coord_to_png_unit(destination: [i8; 2], source: Option<[i8; 2]>) -> String {
        return coord_to_png_unit(source, destination);
    }

    #[staticmethod]
    pub fn i2m_goat(index: usize) -> Move {
        return BaghchalRS::i2m_goat(index);
    }
    #[staticmethod]
    pub fn i2m_placement(index: usize) -> Move {
        return BaghchalRS::i2m_placement(index);
    }
    #[staticmethod]
    pub fn i2m_tiger(index: usize) -> Move {
        return BaghchalRS::i2m_tiger(index);
    }
    #[staticmethod]
    pub fn m2i_goat(__move__: Move) -> usize {
        return BaghchalRS::m2i_goat(__move__);
    }
    #[staticmethod]
    pub fn m2i_placement(__move__: Move) -> usize {
        return BaghchalRS::m2i_placement(__move__);
    }
    #[staticmethod]
    pub fn m2i_tiger(__move__: Move) -> usize {
        return BaghchalRS::m2i_tiger(__move__);
    }

    pub fn copy(&self) -> Baghchal {
        return self.clone();
    }

    pub fn set_rewards(
        &mut self,
        t_goat_capture: f32,
        t_got_trapped: f32,
        t_trap_escape: f32,
        t_win: f32,
        t_lose: f32,
        t_draw: f32,
        t_move: f32,
        g_goat_captured: f32,
        g_tiger_trap: f32,
        g_tiger_escape: f32,
        g_win: f32,
        g_lose: f32,
        g_draw: f32,
        g_move: f32,
        gt_invalid_move: f32,
    ) {
        return self.inner.set_rewards(
            t_goat_capture,
            t_got_trapped,
            t_trap_escape,
            t_win,
            t_lose,
            t_draw,
            t_move,
            g_goat_captured,
            g_tiger_trap,
            g_tiger_escape,
            g_win,
            g_lose,
            g_draw,
            g_move,
            gt_invalid_move,
        );
    }

    pub fn merged_rewards(&self) -> Vec<f32> {
        return crate::helpers::utils::merge_rewards(
            &self.inner.move_reward_goat,
            &self.inner.move_reward_tiger,
        );
    }

    pub fn to_str(&self) -> String {
        return serde_json::to_string(&self).unwrap();
    }

    pub fn board(&self) -> [[i8; 5]; 5] {
        return self.inner.board();
    }

    pub fn move_count(&self) -> i8 {
        return self.inner.move_count();
    }

    pub fn game_status_check(&self) -> GameStatusCheckResult {
        return self.inner.game_status_check();
    }

    pub fn turn(&self) -> i8 {
        return self.inner.turn;
    }

    pub fn goat_counter(&self) -> i8 {
        return self.inner.goat_counter;
    }

    pub fn goat_captured(&self) -> i8 {
        return self.inner.goat_captured;
    }

    pub fn game_state(&self) -> GameStatus {
        return self.inner.game_state;
    }

    pub fn game_history(&self) -> Vec<GameStateInstance> {
        return self.inner.game_history.clone();
    }

    pub fn pgn(&self) -> &str {
        return &self.inner.pgn;
    }

    pub fn prev_move(&self) -> Option<Move> {
        return self.inner.prev_move;
    }

    pub fn move_reward_tiger(&self) -> Vec<f32> {
        return self.inner.move_reward_tiger.clone();
    }

    pub fn trapped_tiger(&self) -> i8 {
        return self.inner.trapped_tiger;
    }

    pub fn move_reward_goat(&self) -> Vec<f32> {
        return self.inner.move_reward_goat.clone();
    }

    pub fn state_as_inputs(
        &self,
        possible_moves_pre: Option<Vec<PossibleMove>>,
        mode: Option<i8>,
        rotate_board: Option<bool>,
    ) -> Vec<Vec<i8>> {
        return self
            .inner
            .state_as_inputs(possible_moves_pre, mode, rotate_board);
    }

    pub fn state_as_inputs_all_symmetry(
        &self,
        possible_moves_pre: Option<Vec<PossibleMove>>,
    ) -> Vec<Vec<Vec<i8>>> {
        return self.inner.state_as_inputs_all_symmetry(possible_moves_pre);
    }

    pub fn state_as_input_actor(
        &self,
        possible_moves_pre: Option<Vec<PossibleMove>>,
        mode: Option<i8>,
        rotate_board: Option<bool>,
    ) -> Vec<Vec<i8>> {
        return self
            .inner
            .state_as_input_actor(possible_moves_pre, mode, rotate_board);
    }

    pub fn index_to_input(&self, index: usize, symmetry: i8) -> Vec<Vec<i8>> {
        return self.inner.index_to_input(index, symmetry);
    }

    pub fn clear_game(&mut self) {
        return self.inner.clear_game();
    }

    pub fn resign(&mut self, side: i8) -> GameStatusCheckResult {
        return self.inner.resign(side);
    }

    pub fn load_game(&mut self, pgn: String) {
        return self.inner.load_game(pgn);
    }

    pub fn set_game_over_on_invalid(&mut self, state: bool) {
        return self.inner.set_game_over_on_invalid(state);
    }

    pub fn transition_history(&self) -> Vec<TransitionHistoryInstance> {
        return self.inner.transition_history.clone();
    }

    pub fn make_move(
        &mut self,
        target: [i8; 2],
        source: Option<[i8; 2]>,
        eval_res: Option<MoveCheckResult>,
    ) -> MoveCheckResult {
        return self.inner.make_move(source, target, eval_res, true);
    }

    pub fn make_move_with_symmetry(
        &mut self,
        symmetry: i8,
        target: [i8; 2],
        source: Option<[i8; 2]>,
    ) -> MoveCheckResult {
        return self.inner.make_move_with_symmetry(source, target, symmetry);
    }

    pub fn make_move_index(&mut self, index: usize) -> Option<TransitionHistoryInstance> {
        return self.inner.make_move_index(index).cloned();
    }
    pub fn get_possible_moves(&self) -> Vec<PossibleMove> {
        return self.inner.get_possible_moves();
    }
}

#[pymodule]
fn libbaghchal(_py: Python, module: &PyModule) -> PyResult<()> {
    module.add_class::<Baghchal>()?;
    module.add_class::<MoveCheckResult>()?;
    module.add_class::<PossibleMove>()?;
    module.add_class::<GameStateInstance>()?;
    module.add_class::<GameStatusCheckResult>()?;
    module.add_class::<GameStatus>()?;
    module.add_class::<TransitionHistoryInstance>()?;
    module.add_class::<MoveType>()?;
    Ok(())
}
