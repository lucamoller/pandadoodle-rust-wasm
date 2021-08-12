use crate::engine::*;

pub struct Emitter {
  pub live_forever: Cell<bool>,
  pub time_remaining: Cell<F1>,
  pub interval: F1,
  pub time: Cell<F1>,

  pub position: RefCell<F2>,
  pub start_angle_range: F1,

  pub time_to_live_particle: F1,

  pub speed: RefCell<F2>,
  pub end_speed: F2,
  pub range_speed: F2,
  pub time_speed_change: F1,

  pub size: F2,
  pub end_size: F2,
  pub range_size: F2,
  pub time_size_change: F1,

  pub rotation: F1,
  pub end_rotation: F1,
  pub range_rotation: F1,
  pub time_rotation_change: F1,

  pub opacity: F1,
  pub end_opacity: F1,
  pub range_opacity: F1,
  pub time_opacity_change: F1,

  pub color: Cell<DrawColor>,
  pub texture: Rc<Texture>,
  pub depth: F1,

  pub start_position_delta: F2,
  pub rand_region_size: F2,
  pub additive_blending: bool,

  pub particles: Shared<Vec<Particle>>,
}

impl Emitter {
  pub fn update<C: ContextTrait + ?Sized>(&self, context: &mut C) {
    self.time.set(self.time.get() - context.get_dt());
    self
      .time_remaining
      .set(self.time_remaining.get() - context.get_dt());
    while self.time.get() <= 0.0 {
      self.time.set(self.time.get() + self.interval);

      let speed_angle = self.start_angle_range * random::get_random_from_minus1_to_1();

      let end_speed = F2::rotate_new(
        &(self.end_speed + self.range_speed * random::get_random_from_minus1_to_1()),
        &speed_angle,
      );
      let d_speed = (end_speed - *self.speed.borrow()) * (1.0 / self.time_speed_change);

      let end_size = self.end_size + self.range_size * random::get_random_from_minus1_to_1();
      let d_size = (end_size - self.size) * (1.0 / self.time_size_change);

      let end_rotation =
        self.end_rotation + self.range_rotation * random::get_random_from_minus1_to_1();
      let d_rotation = (end_rotation - self.rotation) * (1.0 / self.time_rotation_change);

      let end_opacity =
        self.end_opacity + self.range_opacity * random::get_random_from_minus1_to_1();
      let d_opacity = (end_opacity - self.opacity) * (1.0 / self.time_opacity_change);

      self.particles.borrow_mut().push(Particle {
        time_remaining: self.time_to_live_particle,
        position: self.start_position_delta
          + random::get_random_in_rectangular_region(
            &self.position.borrow(),
            &self.rand_region_size,
          ),

        speed: F2::rotate_new(&self.speed.borrow(), &speed_angle),
        time_start_speed: self.time_speed_change,
        d_speed: d_speed,

        size: self.size,
        time_start_size: self.time_size_change,
        d_size: d_size,

        rotation: self.rotation,
        time_start_rotation: self.time_rotation_change,
        d_rotation: d_rotation,

        opacity: self.opacity,
        time_start_opacity: self.time_opacity_change,
        d_opacity: d_opacity,

        color: self.color.get(),
        texture: self.texture.clone(),
        depth: self.depth,
        additive_blending: self.additive_blending,
      });
    }
  }
}
