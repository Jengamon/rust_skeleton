use super::actions::Action;
use super::states::{GameState, RoundState, TerminalState};
use std::error::Error;

pub trait PokerBot {
    type Error: Error;

    fn handle_new_round(&mut self, gs: &GameState, rs: &RoundState, player_index: usize) -> Result<(), Self::Error>;
    fn handle_round_over(&mut self, gs: &GameState, ts: &TerminalState, player_index: usize) -> Result<(), Self::Error>;
    fn get_action(&mut self, gs: &GameState, rs: &RoundState, player_index: usize) -> Result<Action, Self::Error>;
}
