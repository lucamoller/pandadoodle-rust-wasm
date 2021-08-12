use crate::engine::*;

pub struct Particle {
  pub time_remaining: F1,
  pub position: F2,

  pub speed: F2,
  pub time_start_speed: F1,
  pub d_speed: F2,

  pub size: F2,
  pub time_start_size: F1,
  pub d_size: F2,

  pub rotation: F1,
  pub time_start_rotation: F1,
  pub d_rotation: F1,

  pub opacity: F1,
  pub time_start_opacity: F1,
  pub d_opacity: F1,

  pub color: DrawColor,
  pub texture: Rc<Texture>,
  pub depth: F1,

  pub additive_blending: bool,
}

impl Particle {
  pub fn to_remove(&self) -> bool {
    return self.time_remaining < 0.0;
  }

  pub fn update<C: ContextTrait + ?Sized>(&mut self, context: &mut C) {
    self.time_remaining -= context.get_dt();
    self.position += &(self.speed * context.get_dt());

    if self.time_remaining <= self.time_start_speed {
      self.speed += &(self.d_speed * context.get_dt());
    }

    if self.time_remaining <= self.time_start_size {
      self.size += &(self.d_size * context.get_dt());
    }

    if self.time_remaining <= self.time_start_rotation {
      self.rotation += self.rotation * context.get_dt();
    }

    if self.time_remaining <= self.time_start_opacity {
      self.opacity += self.d_opacity * context.get_dt();
      self.opacity = F1Util::move_within_range(&self.opacity, &0.0, &1.0);
    }
  }

  pub fn draw<C: ContextTrait + ?Sized>(&self, context: &mut C, viewport: Rc<Viewport>) {
    context.get_draw_manager().draw_viewport(
      &viewport,
      DrawImageArgs {
        source: DrawSource::Texture(self.texture.clone()),
        position: self.position,
        size: self.size,
        depth: self.depth, // context.draw_depths.source + 0.1
        optional: DrawImageOptionalArgs {
          opacity: self.opacity,
          color: self.color,
          rotation: self.rotation,
          composite_operation: if self.additive_blending {
            Some(String::from("lighter"))
          } else {
            None
          },
          ..Default::default()
        },
      },
    )
  }
}
