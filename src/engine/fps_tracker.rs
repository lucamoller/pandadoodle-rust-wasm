use crate::engine::*;

pub struct FpsTracker {
  mesurement_window_n: usize,
  last_n_frames_dts: VecDeque<F1>,
  total_window_time: F1,
}

impl FpsTracker {
  pub fn new() -> FpsTracker {
    return FpsTracker {
      mesurement_window_n: 60,
      last_n_frames_dts: VecDeque::new(),
      total_window_time: 0.0,
    };
  }

  pub fn update_fps<C: ContextTrait + ?Sized>(&mut self, context: &mut C) {
    self.total_window_time += context.get_dt();
    self.last_n_frames_dts.push_back(*context.get_dt());
    while self.last_n_frames_dts.len() > self.mesurement_window_n {
      self.total_window_time -= self.last_n_frames_dts.pop_front().unwrap();
    }

    let fps = 1000.0 * (self.mesurement_window_n as F1) / self.total_window_time;

    let screen_viewport = context.get_screen_viewport().clone();

    if context.show_fps() {
      context.get_draw_manager().draw_string_viewport(
        &screen_viewport,
        DrawStringArgs {
          text: String::from(format!("fps {:3.1}", fps)),
          position: F2 { x: 0.98, y: 0.045 },
          font_size: 0.05,
          depth: -150.0,
          optional: DrawStringOptionalArgs {
            border: true,
            color: DrawColor {
              r: 255,
              g: 255,
              b: 255,
            },
            alignment: TextAlignment::Right,
            ..Default::default()
          },
        },
      );
    }
  }
}
