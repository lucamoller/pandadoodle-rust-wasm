use crate::engine::*;

pub struct DrawManager {
  draw_backend: Canvas2dDrawBackend,
  queued: Vec<DrawArgs>,
  device_pixel_ratio: F1,
  canvas_size: F2,
}

impl DrawManager {
  pub fn new(
    draw_backend: Canvas2dDrawBackend,
    screen_size: &F2,
    device_pixel_ratio: &F1,
  ) -> DrawManager {
    return DrawManager {
      draw_backend: draw_backend,
      queued: Vec::new(),
      device_pixel_ratio: *device_pixel_ratio,
      canvas_size: screen_size * device_pixel_ratio,
    };
  }

  pub fn update_screen_size(&mut self, screen_size: &F2, device_pixel_ratio: &F1) {
    self.device_pixel_ratio = *device_pixel_ratio;
    self.canvas_size = screen_size * device_pixel_ratio;
  }

  pub fn execute_draws(&mut self) {
    self.draw_backend.clear_screen(&self.canvas_size);

    self
      .queued
      .sort_by(|a, b| b.get_depth().partial_cmp(a.get_depth()).unwrap());

    for args in self.queued.iter() {
      match args {
        DrawArgs::Image(image_args) => {
          self
            .draw_backend
            .execute_image_draw(image_args, &self.canvas_size);
        }
        DrawArgs::String(string_args) => {
          self.draw_backend.execute_string_draw(
            string_args,
            &self.device_pixel_ratio,
            &self.canvas_size,
          );
        }
        DrawArgs::GradientBox(gradient_box_args) => {
          self
            .draw_backend
            .execute_gradient_box_draw(gradient_box_args, &self.canvas_size);
        }
      };
    }
    self.queued.clear();
  }

  pub fn draw_canvas(&mut self, args: DrawImageArgs) {
    self.queued.push(DrawArgs::Image(args));
  }

  pub fn draw_screen(&mut self, mut args: DrawImageArgs) {
    args.position *= &self.device_pixel_ratio;
    args.size *= &self.device_pixel_ratio;
    self.draw_canvas(args);
  }

  pub fn draw_gradient_box_canvas(&mut self, args: DrawGradientBoxArgs) {
    self.queued.push(DrawArgs::GradientBox(args));
  }

  pub fn draw_gradient_box_screen(&mut self, mut args: DrawGradientBoxArgs) {
    args.position *= &self.device_pixel_ratio;
    args.size *= &self.device_pixel_ratio;
    self.draw_gradient_box_canvas(args);
  }

  pub fn convert_viewport_into_canvas_draw_args(
    &self,
    viewport: &Viewport,
    mut args: DrawImageArgs,
  ) -> DrawImageArgs {
    args.position = viewport.viewport_to_canvas(&args.position);
    args.size = viewport.viewport_to_canvas_ratio(&args.size);
    return args;
  }

  pub fn draw_viewport(&mut self, viewport: &Viewport, args: DrawImageArgs) {
    self.draw_canvas(self.convert_viewport_into_canvas_draw_args(viewport, args));
  }

  fn draw_string_canvas(&mut self, args: DrawStringArgs) {
    self.queued.push(DrawArgs::String(args));
  }

  pub fn draw_string_viewport(&mut self, viewport: &Viewport, mut args: DrawStringArgs) {
    args.position = viewport.viewport_to_canvas(&args.position);
    args.font_size = viewport.viewport_to_canvas_ratio_y(&args.font_size);
    self.draw_string_canvas(args);
  }
}
