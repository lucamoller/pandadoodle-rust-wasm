use crate::context::Context;
use crate::context::UiEvent;
use crate::game_ui::*;
use crate::*;

pub struct GameUiManager {
  ui_manager: Rc<UiManager>,
}

impl GameUiManager {
  pub fn new(context: &Context) -> GameUiManager {
    context.ui_events.add_event(UiEvent::LoadLandingPage);
    return GameUiManager {
      ui_manager: UiManager::new(context),
    };
  }

  pub fn process_touch(&mut self, context: &mut Context, screen_touch: &ScreenTouch) {
    self.ui_manager.process_touch(context, screen_touch);
  }

  pub fn process_back_button(&self, context: &mut Context) -> InputState {
    return self.ui_manager.process_back_button(context);
  }

  pub fn update(&self, context: &mut Context) {
    while let Some(ui_event) = context.ui_events.consume_event() {
      console_log_with_div!("ui_event: {:?}", ui_event);
      match ui_event {
        UiEvent::LoadLandingPage => {
          self
            .ui_manager
            .switch_to_page(Rc::new(LandingPageUiRoot::new(context)), context);
        }
        UiEvent::LoadMainMenu => {
          context.statsig_bindings.log_event("main_menu_view");
          self
            .ui_manager
            .switch_to_page(Rc::new(MainMenuUiRoot::new(context)), context);
        }
        UiEvent::LoadMenuChooseStage => {
          context
            .statsig_bindings
            .log_event("menu_choose_stages_view");
          self
            .ui_manager
            .push_page_on_stack(MenuChooseStageUiRoot::new(context), context);
        }
        UiEvent::LoadGame(load_game_params) => {
          let in_game_ui = IngameUiRoot::new(context);
          in_game_ui.load_game(context, load_game_params);
          self.ui_manager.push_page_on_stack(in_game_ui, context);
        }
      }
    }
    self.ui_manager.update(context);
  }

  pub fn draw(&self, context: &mut Context) {
    self.ui_manager.draw(context);
  }
}
