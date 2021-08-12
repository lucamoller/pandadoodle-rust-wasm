use crate::engine::*;

pub struct StateAndCheckpoint<T> {
  state: T,
  checkpoint: u32,
}

pub struct StateHistory<T> {
  states: RefCell<Vec<StateAndCheckpoint<T>>>,
  created_checkpoint: Cell<u32>,
}

impl<T> StateHistory<T> {
  pub fn new(current_checkpoint: u32) -> StateHistory<T> {
    return StateHistory {
      states: RefCell::new(Vec::new()),
      created_checkpoint: Cell::new(current_checkpoint),
    };
  }

  // Register the object state before the checkpoint.
  pub fn register_state_internal(&self, checkpoint: u32, state: T) {
    let mut states = self.states.borrow_mut();
    if match states.last() {
      Some(last_state) => last_state.checkpoint != checkpoint,
      None => true,
    } {
      states.push(StateAndCheckpoint {
        state: state,
        checkpoint: checkpoint,
      });
    }
  }

  // Rollback to state before the checkpoint
  pub fn rollback_changes(&self, checkpoint: &u32) -> Option<T> {
    let mut states = self.states.borrow_mut();
    if states.last().is_some() && states.last().unwrap().checkpoint >= *checkpoint {
      let mut target_state = states.pop();
      while states.last().is_some() && states.last().unwrap().checkpoint >= *checkpoint {
        target_state = states.pop();
      }
      return target_state.map(|stage_and_checkpoint| stage_and_checkpoint.state);
    }
    return None;
  }

  pub fn exists_before(&self, checkpoint: &u32) -> bool {
    return *checkpoint > self.created_checkpoint.get();
  }
}
