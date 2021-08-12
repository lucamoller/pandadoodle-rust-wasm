use crate::context::Context;
use crate::engine::*;
use crate::game::mirror::*;
use crate::game::paint_color::PaintColor;
use crate::game::paint_point::PaintPoint;
use crate::game::portal::*;
use crate::game::source::Source;
use crate::*;

pub struct PaintPath {
  entity_base: EntityBase,
  pub paint_color: Cell<PaintColor>,
  pub point_count: Cell<i32>,
  pub last_point: RefCell<Weak<PaintPoint>>,
  pub path_on_hold: Cell<bool>,
  pub disabled: Cell<bool>,
  pub active: Cell<bool>,

  pub touched_mirror_last_path: RefCell<HashMap<HashablePointer<Mirror>, Rc<SymmetricPaintPath>>>,
  pub symmetric_paths: Rc<EntityManager<SymmetricPaintPath>>,
  pub activated_portal: RefCell<Option<Rc<Portal>>>,

  state_history: StateHistory<PaintPathState>,
}

pub struct PaintPathState {
  pub paint_color: PaintColor,
  pub last_point: Weak<PaintPoint>,
  pub path_on_hold: bool,
  pub disabled: bool,
  pub active: bool,
  pub activated_portal: Option<Rc<Portal>>,

  pub touched_mirror_last_path: HashMap<HashablePointer<Mirror>, Rc<SymmetricPaintPath>>,
  pub symmetric_paths: EntityManager<SymmetricPaintPath>,
}

impl PaintPath {
  pub fn new(_context: &Context, paint_color: &PaintColor, current_checkpoint: u32) -> PaintPath {
    let entity_base = EntityBase::new();
    let symmetric_paths = EntityManager::new_within_parent_entity(&entity_base);
    let state_history = StateHistory::new(current_checkpoint);
    return PaintPath {
      entity_base,
      paint_color: Cell::new(*paint_color),
      point_count: Cell::new(0),
      last_point: RefCell::new(Weak::new()),
      path_on_hold: Cell::new(false),
      disabled: Cell::new(false),
      active: Cell::new(true),
      touched_mirror_last_path: RefCell::new(HashMap::new()),
      symmetric_paths,
      activated_portal: RefCell::new(None),
      state_history: state_history,
    };
  }

  pub fn release_from_hold(&self, current_checkpoint: &u32) {
    self.register_current_state(*current_checkpoint);
    self.path_on_hold.set(false);
  }

  pub fn put_on_hold(&self, _current_checkpoint: &u32) {
    self.path_on_hold.set(true);
  }
}

impl EffectManagerTrait<Context> for PaintPath {
  fn get_effect_manager(&self) -> Option<&EffectManager> {
    return None;
  }
}

impl EntityTrait<Context> for PaintPath {
  type State = PaintPathState;

  fn get_base(&self) -> &EntityBase {
    return &self.entity_base;
  }

  fn get_state_history(&self) -> &StateHistory<Self::State> {
    return &self.state_history;
  }

  fn apply_state(&self, state: Self::State) {
    self.paint_color.set(state.paint_color);
    self.last_point.replace(state.last_point);
    self.path_on_hold.set(state.path_on_hold);
    self.disabled.set(state.disabled);
    self.active.set(state.active);
    self.activated_portal.replace(state.activated_portal);
    self
      .touched_mirror_last_path
      .replace(state.touched_mirror_last_path);
    self.symmetric_paths.replace(state.symmetric_paths);
  }

  fn get_current_state(&self) -> Self::State {
    return PaintPathState {
      paint_color: self.paint_color.get(),
      last_point: self.last_point.borrow().clone(),
      path_on_hold: self.path_on_hold.get(),
      disabled: self.disabled.get(),
      active: self.active.get(),
      activated_portal: self.activated_portal.borrow().clone(),
      touched_mirror_last_path: self.touched_mirror_last_path.borrow().clone(),
      symmetric_paths: self.symmetric_paths.as_ref().clone(),
    };
  }

  fn to_remove(&self) -> bool {
    return false;
  }

  fn update(&self, _context: &mut Context) {}

  fn draw(&self, _context: &mut Context) {}
}

pub struct SourcedPaintPath {
  paint_path: PaintPath,
  pub source: Weak<Source>,
  emitter_fire: Emitter,
  emitter_light: Emitter,
}

impl SourcedPaintPath {
  pub fn new(
    context: &Context,
    source: &Rc<Source>,
    particles: Shared<Vec<Particle>>,
    current_checkpoint: u32,
  ) -> Rc<SourcedPaintPath> {
    return Rc::new(SourcedPaintPath {
      paint_path: PaintPath::new(context, &source.paint_color, current_checkpoint),
      source: Rc::downgrade(source),
      emitter_fire: Emitter {
        live_forever: Cell::new(true),
        time_remaining: Cell::new(0.0),
        interval: 80.0,
        time: Cell::new(0.0),

        position: RefCell::new(F2 { x: 0.0, y: 0.0 }),
        start_angle_range: 0.4,

        time_to_live_particle: 1000.0,

        speed: RefCell::new(F2 { x: 0.0, y: 0.0 }),
        end_speed: F2 { x: 0.0, y: -0.0002 },
        range_speed: F2 { x: 0.0, y: 0.0 },
        time_speed_change: 1000.0,

        size: F2 { x: 0.2, y: 0.2 } * 0.6,
        end_size: F2 { x: 0.08, y: 0.02 } * 0.6,
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

        color: Cell::new(source.paint_color.get_draw_color()),
        texture: context.texture_manager.flare.clone(),
        depth: context.draw_depths.source - 0.01,

        start_position_delta: F2 { x: 0.0, y: -0.05 },
        rand_region_size: F2 { x: 0.1, y: 0.1 },
        additive_blending: false,

        particles: particles.clone(),
      },

      emitter_light: Emitter {
        live_forever: Cell::new(true),
        time_remaining: Cell::new(0.0),
        interval: 80.0,
        time: Cell::new(0.0),

        position: RefCell::new(F2 { x: 0.0, y: 0.0 }),
        start_angle_range: 0.2,

        time_to_live_particle: 1500.0,

        speed: RefCell::new(F2 { x: 0.0, y: 0.0 }),
        end_speed: F2 { x: 0.0, y: -0.0002 },
        range_speed: F2 { x: 0.0, y: 0.0 },
        time_speed_change: 1500.0,

        size: F2 { x: 0.15, y: 0.15 } * 0.6,
        end_size: F2 { x: 0.08, y: 0.02 } * 0.6,
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

        color: Cell::new(source.paint_color.get_draw_color()),
        texture: context.texture_manager.flare.clone(),
        depth: context.draw_depths.source - 1.01,

        start_position_delta: F2 { x: 0.0, y: 0.03 },
        rand_region_size: F2 { x: 0.06, y: 0.06 },
        additive_blending: true,

        particles: particles.clone(),
      },
    });
  }
}

impl EffectManagerTrait<Context> for SourcedPaintPath {
  fn get_effect_manager(&self) -> Option<&EffectManager> {
    return self.paint_path.get_effect_manager();
  }
}

impl EntityTrait<Context> for SourcedPaintPath {
  type State = PaintPathState;

  fn get_base(&self) -> &EntityBase {
    return &self.entity_base;
  }

  fn get_state_history(&self) -> &StateHistory<Self::State> {
    return self.paint_path.get_state_history();
  }

  fn apply_state(&self, state: Self::State) {
    self.paint_path.apply_state(state);
  }

  fn get_current_state(&self) -> Self::State {
    return self.paint_path.get_current_state();
  }

  fn to_remove(&self) -> bool {
    return false;
  }

  fn update(&self, context: &mut Context) {
    if self.paint_path.path_on_hold.get() && self.active.get() {
      match self.paint_path.last_point.borrow().upgrade() {
        Some(last_point) => {
          *self.emitter_fire.position.borrow_mut() = last_point.position;
          *self.emitter_light.position.borrow_mut() = last_point.position;
          self
            .emitter_fire
            .color
            .set(self.paint_path.paint_color.get().get_draw_color());
          self
            .emitter_light
            .color
            .set(self.paint_path.paint_color.get().get_draw_color());
          self.emitter_fire.update(context);
          self.emitter_light.update(context);
        }
        None => {
          console_log_with_div!("self.last_point.upgrade is None");
        }
      }
    }
  }

  fn draw(&self, _context: &mut Context) {}
}

impl Deref for SourcedPaintPath {
  type Target = PaintPath;

  fn deref(&self) -> &Self::Target {
    return &self.paint_path;
  }
}

pub struct SymmetricPaintPath {
  pub paint_path: PaintPath,

  symmetry_point: F2,
  symmetry_direction: F2,
}

impl SymmetricPaintPath {
  pub fn new(
    context: &Context,
    paint_color: PaintColor,
    symmetry_point: F2,
    symmetry_direction: F2,
    current_checkpoint: u32,
  ) -> Rc<SymmetricPaintPath> {
    return Rc::new(SymmetricPaintPath {
      paint_path: PaintPath::new(context, &paint_color, current_checkpoint),
      symmetry_point: symmetry_point,
      symmetry_direction: symmetry_direction,
    });
  }

  pub fn get_symmetric_point(&self, point: &F2) -> F2 {
    let mut p = point - self.symmetry_point;
    let x = F2::dotp(&p, &self.symmetry_direction);
    p -= &(self.symmetry_direction * x);
    return point - (p * 2.0);
  }
}

impl EffectManagerTrait<Context> for SymmetricPaintPath {
  fn get_effect_manager(&self) -> Option<&EffectManager> {
    return self.paint_path.get_effect_manager();
  }
}

impl EntityTrait<Context> for SymmetricPaintPath {
  type State = PaintPathState;

  fn get_base(&self) -> &EntityBase {
    return &self.entity_base;
  }

  fn get_state_history(&self) -> &StateHistory<Self::State> {
    return self.paint_path.get_state_history();
  }

  fn apply_state(&self, state: Self::State) {
    self.paint_path.apply_state(state);
  }

  fn get_current_state(&self) -> Self::State {
    return self.paint_path.get_current_state();
  }

  fn to_remove(&self) -> bool {
    return false;
  }

  fn update(&self, _context: &mut Context) {}

  fn draw(&self, _context: &mut Context) {}
}

impl Deref for SymmetricPaintPath {
  type Target = PaintPath;

  fn deref(&self) -> &Self::Target {
    return &self.paint_path;
  }
}
