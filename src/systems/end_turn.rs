use crate::prelude::*;

#[system]
// obtains writable access to the TurnState resource
pub fn end_turn(#[resource] turn_state: &mut TurnState) {
    let new_state = match turn_state {
        // nothing to do if awaiting input
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        TurnState::MonsterTurn => TurnState::AwaitingInput
    };
    // Sets turn resource to the chosen value
    // * dereferences the variable, allow us to write to the stored resource
    *turn_state = new_state;// <callout id="co.tbs.turnbased.set" />
}