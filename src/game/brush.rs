use super::super::context::Context;
use super::paint_color::PaintColor;
use crate::engine::*;

pub struct Brush {
  pub position: F2,
  pub active: bool,

  emitter_fire: Emitter,
  emitter_smoke: Emitter,
}

impl Brush {
  pub fn new(context: &Context, particles: Shared<Vec<Particle>>) -> Brush {
    return Brush {
      position: F2 { x: 0.0, y: 0.0 },
      active: false,

      emitter_fire: Emitter {
        live_forever: Cell::new(true),
        time_remaining: Cell::new(0.0),
        interval: 20.0,
        time: Cell::new(0.0),

        position: RefCell::new(F2 { x: 0.0, y: 0.0 }),
        start_angle_range: 0.2,

        time_to_live_particle: 1000.0,

        speed: RefCell::new(F2 { x: 0.0, y: 0.0 }),
        end_speed: F2 { x: 0.0, y: -0.0002 },
        range_speed: F2 { x: 0.0, y: 0.0 },
        time_speed_change: 1500.0,

        size: F2 { x: 0.15, y: 0.15 } * 0.3,
        end_size: F2 { x: 0.08, y: 0.02 } * 0.3,
        range_size: F2 { x: 0.05, y: 0.05 },
        time_size_change: 500.0,

        rotation: 0.0,
        end_rotation: 0.0,
        range_rotation: 10.0,
        time_rotation_change: 2000.0,

        opacity: 0.6,
        end_opacity: 0.0,
        range_opacity: 0.0,
        time_opacity_change: 250.0,

        color: Cell::new(PaintColor::NoColor.get_draw_color()),
        texture: context.texture_manager.flare.clone(),
        depth: context.draw_depths.source - 0.01,

        start_position_delta: F2 { x: 0.0, y: 0.0 },
        rand_region_size: F2 { x: 0.06, y: 0.06 },
        additive_blending: false,

        particles: particles.clone(),
      },

      emitter_smoke: Emitter {
        live_forever: Cell::new(true),
        time_remaining: Cell::new(0.0),
        interval: 20.0,
        time: Cell::new(0.0),

        position: RefCell::new(F2 { x: 0.0, y: 0.0 }),
        start_angle_range: 0.4,

        time_to_live_particle: 600.0,

        speed: RefCell::new(F2 { x: 0.0, y: 0.0 }),
        end_speed: F2 { x: 0.0, y: -0.0002 },
        range_speed: F2 { x: 0.0, y: 0.0 },
        time_speed_change: 1000.0,

        size: F2 { x: 0.2, y: 0.2 } * 0.3,
        end_size: F2 { x: 0.08, y: 0.02 } * 0.3,
        range_size: F2 { x: 0.05, y: 0.05 },
        time_size_change: 500.0,

        rotation: 0.0,
        end_rotation: 0.0,
        range_rotation: 10.0,
        time_rotation_change: 2000.0,

        opacity: 0.2,
        end_opacity: 0.0,
        range_opacity: 0.0,
        time_opacity_change: 1000.0,

        color: Cell::new(PaintColor::NoColor.get_draw_color()),
        texture: context.texture_manager.flare.clone(),
        depth: context.draw_depths.source - 0.01,

        start_position_delta: F2 { x: 0.0, y: 0.0 },
        rand_region_size: F2 { x: 0.1, y: 0.1 },
        additive_blending: false,

        particles: particles.clone(),
      },
    };
  }

  pub fn set_color(&self, paint_color: &PaintColor) {
    self.emitter_fire.color.set(paint_color.get_draw_color());
    self.emitter_smoke.color.set(paint_color.get_draw_color());
  }

  pub fn update(&self, context: &mut Context) {
    if self.active {
      *self.emitter_fire.position.borrow_mut() = self.position;
      *self.emitter_smoke.position.borrow_mut() = self.position;
      self.emitter_fire.update(context);
      self.emitter_smoke.update(context);
    }
  }
}
