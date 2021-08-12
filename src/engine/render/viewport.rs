use crate::engine::*;

#[derive(Default)]
pub struct Viewport {
  pub viewport_position_on_screen: F2,
  pub viewport_size_on_screen: F2,
  viewport_to_screen_ratio: F2,

  pub viewport_position_on_canvas: F2,
  pub viewport_size_on_canvas: F2,
  pub viewport_to_canvas_ratio: F2,

  pub viewport_yx_ratio: F1,

  pub screen_top_left_corner: F2,
  pub screen_bottom_right_corner: F2,
  pub screen_center: F2,
  pub screen_size: F2,
}

impl Viewport {
  pub fn new(
    viewport_position_on_screen: &F2,
    viewport_size_on_screen: &F2,
    screen_size: &F2,
    device_pixel_ratio: &F1,
  ) -> Viewport {
    let viewport_yx_ratio = viewport_size_on_screen.y / viewport_size_on_screen.x;
    let viewport_to_screen_ratio = F2 {
      x: viewport_size_on_screen.x,
      y: viewport_size_on_screen.y / viewport_yx_ratio,
    };

    let mut viewport = Viewport {
      viewport_position_on_screen: *viewport_position_on_screen,
      viewport_size_on_screen: *viewport_size_on_screen,
      viewport_to_screen_ratio: viewport_to_screen_ratio,

      viewport_position_on_canvas: viewport_position_on_screen * device_pixel_ratio,
      viewport_size_on_canvas: viewport_size_on_screen * device_pixel_ratio,
      viewport_to_canvas_ratio: viewport_to_screen_ratio * device_pixel_ratio,

      viewport_yx_ratio: viewport_yx_ratio,
      ..Default::default()
    };

    viewport.screen_top_left_corner = viewport.screen_to_viewport(&F2 { x: 0.0, y: 0.0 });
    viewport.screen_bottom_right_corner = viewport.screen_to_viewport(screen_size);
    viewport.screen_center =
      &(&viewport.screen_top_left_corner + &viewport.screen_bottom_right_corner) * &0.5;
    viewport.screen_size = &viewport.screen_bottom_right_corner - &viewport.screen_top_left_corner;
    return viewport;
  }

  pub fn viewport_to_screen(&self, input: &F2) -> F2 {
    return F2 {
      x: input.x * self.viewport_to_screen_ratio.x + self.viewport_position_on_screen.x,
      y: input.y * self.viewport_to_screen_ratio.y + self.viewport_position_on_screen.y,
    };
  }

  pub fn viewport_to_canvas(&self, input: &F2) -> F2 {
    return F2 {
      x: input.x * self.viewport_to_canvas_ratio.x + self.viewport_position_on_canvas.x,
      y: input.y * self.viewport_to_canvas_ratio.y + self.viewport_position_on_canvas.y,
    };
  }

  pub fn screen_to_viewport(&self, input: &F2) -> F2 {
    return F2 {
      x: (input.x - self.viewport_position_on_screen.x) / self.viewport_to_screen_ratio.x,
      y: (input.y - self.viewport_position_on_screen.y) / self.viewport_to_screen_ratio.y,
    };
  }

  pub fn viewport_to_screen_ratio(&self, input: &F2) -> F2 {
    return F2 {
      x: input.x * self.viewport_to_screen_ratio.x,
      y: input.y * self.viewport_to_screen_ratio.y,
    };
  }

  pub fn viewport_to_canvas_ratio(&self, input: &F2) -> F2 {
    return F2 {
      x: input.x * self.viewport_to_canvas_ratio.x,
      y: input.y * self.viewport_to_canvas_ratio.y,
    };
  }

  pub fn viewport_to_canvas_ratio_y(&self, input: &F1) -> F1 {
    return input * self.viewport_to_canvas_ratio.y;
  }

  pub fn screen_to_viewport_ratio(&self, input: &F2) -> F2 {
    return F2 {
      x: input.x / self.viewport_to_screen_ratio.x,
      y: input.y / self.viewport_to_screen_ratio.y,
    };
  }

  pub fn is_inside_viewport(&self, input: &F2) -> bool {
    return input.x >= 0.0 && input.x <= 1.0 && input.y >= 0.0 && input.y <= self.viewport_yx_ratio;
  }

  pub fn move_inside_viewport(&self, input: &F2) -> F2 {
    return F2 {
      x: F1Util::move_within_range(&input.x, &0.0, &1.0),
      y: F1Util::move_within_range(&input.y, &0.0, &self.viewport_yx_ratio),
    };
  }
}
