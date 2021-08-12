use crate::engine::*;

pub struct DrawDepths {
  pub black_border: F1,
  pub overlay: F1,
  pub front_board: F1,
  pub ui: F1,
  pub over: F1,
  pub source_label: F1,
  pub source: F1,
  pub goal: F1,
  pub barrier: F1,
  pub mirror: F1,
  pub path: F1,
  pub background: F1,
}

impl DrawDepths {
  pub fn new() -> DrawDepths {
    let mut depth = 0.0;
    let mut get_new_depth = || -> F1 {
      depth += 10.0;
      return depth.clone();
    };

    return DrawDepths {
      black_border: get_new_depth(),
      overlay: get_new_depth(),
      front_board: get_new_depth(),
      ui: get_new_depth(),
      over: get_new_depth(),
      source_label: get_new_depth(),
      source: get_new_depth(),
      goal: get_new_depth(),
      barrier: get_new_depth(),
      mirror: get_new_depth(),
      path: get_new_depth(),
      background: get_new_depth(),
    };
  }
}
