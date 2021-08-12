use crate::engine::*;
use crate::game::paint_path::*;
use crate::*;
use std::collections::HashSet;

const MIRROR_DRAW_DX: F1 = 0.014;

pub struct Mirror {
  entity_base: EntityBase,
  p1: F2,
  p2: F2,
  direction: F2,
  length: F1,
  paths_created: RefCell<HashSet<HashablePointer<PaintPath>>>,

  cached_canvas: CachedCanvasBackend,
  state_history: StateHistory<MirrorState>,
}

pub struct MirrorState {
  paths_created: HashSet<HashablePointer<PaintPath>>,
}

impl Mirror {
  pub fn new(context: &Context, p1: F2, p2: F2) -> Rc<Mirror> {
    let mut direction = p2 - p1;
    direction.normalize();
    let length = (p2 - p1).length();
    return Rc::new(Mirror {
      entity_base: EntityBase::new(),
      p1: p1,
      p2: p2,
      direction: direction,
      length: length,
      paths_created: RefCell::new(HashSet::new()),
      cached_canvas: CachedCanvasBackend::new(&context.get_canvas_size()),
      state_history: StateHistory::new(0),
    });
  }

  pub fn touch(&self, context: &Context, current_checkpoint: u32, paint_path: &PaintPath) {
    if self.should_create_path_reflection(paint_path) {
      self.register_current_state(current_checkpoint);

      let symmetric_path = SymmetricPaintPath::new(
        context,
        paint_path.paint_color.get(),
        self.p1,
        self.direction,
        current_checkpoint,
      );
      self
        .paths_created
        .borrow_mut()
        .insert(HashablePointer::from(&symmetric_path.paint_path));
      paint_path.symmetric_paths.add(symmetric_path.clone());
      paint_path
        .touched_mirror_last_path
        .borrow_mut()
        .insert(HashablePointer::from(self), symmetric_path);
    }
  }

  pub fn should_create_path_reflection(&self, paint_path: &PaintPath) -> bool {
    if self
      .paths_created
      .borrow()
      .contains(&HashablePointer::from(paint_path))
    {
      return false;
    }

    if let Some(path) = paint_path
      .touched_mirror_last_path
      .borrow()
      .get(&HashablePointer::from(self))
    {
      if !path.disabled.get() {
        return false;
      }
    }

    return true;
  }
}

impl EffectManagerTrait<Context> for Mirror {
  fn get_effect_manager(&self) -> Option<&EffectManager> {
    return None;
  }
}

impl EntityTrait<Context> for Mirror {
  type State = MirrorState;

  fn get_base(&self) -> &EntityBase {
    return &self.entity_base;
  }

  fn get_state_history(&self) -> &StateHistory<Self::State> {
    return &self.state_history;
  }

  fn apply_state(&self, state: Self::State) {
    self.paths_created.replace(state.paths_created);
  }

  fn get_current_state(&self) -> Self::State {
    return MirrorState {
      paths_created: self.paths_created.borrow().clone(),
    };
  }

  fn to_remove(&self) -> bool {
    return false;
  }

  fn update(&self, _context: &mut Context) {}

  fn draw(&self, context: &mut Context) {
    self
      .cached_canvas
      .check_canvas_size_changed(&context.get_canvas_size());
    if self.cached_canvas.check_clear_cache() {
      let mut x = 0.0;
      let size = context
        .texture_manager
        .pixel
        .get_size_from_width(5.0 / 480.0);

      while x < self.length {
        let position = self.p1 + self.direction * x;
        self.cached_canvas.draw_backend.execute_image_draw(
          &mut context.draw_manager.convert_viewport_into_canvas_draw_args(
            &context.game_viewport,
            DrawImageArgs {
              source: DrawSource::Texture(context.texture_manager.pixel.clone()),
              position: position,
              size: size,
              depth: context.draw_depths.mirror,
              optional: DrawImageOptionalArgs {
                color: DrawColor {
                  r: (133.0 / 2.0) as u8,
                  g: (199.0 / 2.0) as u8,
                  b: (191.0 / 2.0) as u8,
                },
                ..Default::default()
              },
            },
          ),
          &self.cached_canvas.canvas_size.get(),
        );
        x += MIRROR_DRAW_DX;
      }
    }

    context.draw_manager.draw_screen(DrawImageArgs {
      source: DrawSource::Canvas(self.cached_canvas.canvas.clone()),
      position: F2 { x: 0.0, y: 0.0 },
      size: context.screen_size,
      depth: context.draw_depths.mirror,
      optional: DrawImageOptionalArgs {
        anchor_point: F2 { x: 0.0, y: 0.0 },
        opacity: context.stage_opacity.get(),
        ..Default::default()
      },
    });
  }
}

impl SegmentShape for Mirror {
  fn get_p1<'a>(&'a self) -> &'a F2 {
    return &self.p1;
  }

  fn get_p2<'a>(&'a self) -> &'a F2 {
    return &self.p2;
  }
}
