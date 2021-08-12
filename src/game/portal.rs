use crate::engine::*;
use crate::game::stages_data::*;
use crate::*;

pub enum PortalType {
  Portal1,
  Portal2,
  Portal3,
}

pub struct Portal {
  entity_base: EntityBase,
  portal_type: PortalType,
  pub endpoint1: PortalEndpoint,
  pub endpoint2: PortalEndpoint,
  state_history: StateHistory<()>,
}

pub struct PortalEndpoint {
  pub position: F2,
}

impl Portal {
  pub fn new(_context: &Context, portal_data: &PortalData, portal_type: usize) -> Rc<Portal> {
    return Rc::new(Portal {
      entity_base: EntityBase::new(),
      portal_type: match portal_type {
        0 => PortalType::Portal1,
        1 => PortalType::Portal2,
        _ => PortalType::Portal3,
      },
      endpoint1: PortalEndpoint {
        position: portal_data.p1,
      },
      endpoint2: PortalEndpoint {
        position: portal_data.p2,
      },
      state_history: StateHistory::new(0),
    });
  }
}

impl EffectManagerTrait<Context> for Portal {
  fn get_effect_manager(&self) -> Option<&EffectManager> {
    return None;
  }
}

impl EntityTrait<Context> for Portal {
  type State = ();

  fn get_base(&self) -> &EntityBase {
    return &self.entity_base;
  }

  fn get_state_history(&self) -> &StateHistory<Self::State> {
    return &self.state_history;
  }

  fn apply_state(&self, _state: Self::State) {}

  fn get_current_state(&self) -> Self::State {
    return ();
  }

  fn to_remove(&self) -> bool {
    return false;
  }

  fn update(&self, _context: &mut Context) {}

  fn draw(&self, context: &mut Context) {
    let time = *context.get_latest_timestamp();
    let size = F2 { x: 0.14, y: 0.14 };
    let (tex, tex_glow) = match self.portal_type {
      PortalType::Portal1 => (
        context.texture_manager.portal1.clone(),
        context.texture_manager.portal1_glow.clone(),
      ),
      PortalType::Portal2 => (
        context.texture_manager.portal2.clone(),
        context.texture_manager.portal2_glow.clone(),
      ),
      PortalType::Portal3 => (
        context.texture_manager.portal3.clone(),
        context.texture_manager.portal3_glow.clone(),
      ),
    };

    let black = DrawColor { r: 0, g: 0, b: 0 };
    context.draw_manager.draw_viewport(
      &context.game_viewport,
      DrawImageArgs {
        source: DrawSource::Texture(tex.clone()),
        position: self.endpoint1.position,
        size: size,
        depth: context.draw_depths.mirror,
        optional: DrawImageOptionalArgs {
          opacity: context.stage_opacity.get(),
          color: black,
          ..Default::default()
        },
      },
    );
    context.draw_manager.draw_viewport(
      &context.game_viewport,
      DrawImageArgs {
        source: DrawSource::Texture(tex.clone()),
        position: self.endpoint2.position,
        size: size,
        depth: context.draw_depths.mirror,
        optional: DrawImageOptionalArgs {
          opacity: context.stage_opacity.get(),
          color: black,
          ..Default::default()
        },
      },
    );

    let glow_color = DrawColor {
      r: 80,
      g: 200,
      b: 255,
    };
    let glow_opacity = (0.5 + 0.5 * ((time * 0.002).sin())) * context.stage_opacity.get();
    context.draw_manager.draw_viewport(
      &context.game_viewport,
      DrawImageArgs {
        source: DrawSource::Texture(tex_glow.clone()),
        position: self.endpoint1.position,
        size: size,
        depth: context.draw_depths.mirror - 0.1,
        optional: DrawImageOptionalArgs {
          opacity: glow_opacity,
          color: glow_color,
          composite_operation: Some(String::from("lighter")),
          ..Default::default()
        },
      },
    );
    context.draw_manager.draw_viewport(
      &context.game_viewport,
      DrawImageArgs {
        source: DrawSource::Texture(tex_glow.clone()),
        position: self.endpoint2.position,
        size: size,
        depth: context.draw_depths.mirror - 0.1,
        optional: DrawImageOptionalArgs {
          opacity: glow_opacity,
          color: glow_color,
          composite_operation: Some(String::from("lighter")),
          ..Default::default()
        },
      },
    );
  }
}

impl CircleShape for PortalEndpoint {
  fn get_center<'a>(&'a self) -> &'a F2 {
    return &self.position;
  }

  fn get_radius<'a>(&'a self) -> &'a F1 {
    return &0.03;
  }
}
