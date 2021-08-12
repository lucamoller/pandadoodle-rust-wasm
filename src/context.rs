use crate::audio_manager::*;
use crate::draw_depths::*;
use crate::engine::*;
use crate::game::achievments_manager::*;
use crate::game::game_mode::*;
use crate::game::stages_data::AllStagesData;
use crate::game_ui::Book;
use crate::texture_manager::*;
use crate::*;

use wasm_bindgen::JsCast;

static PANDA_DOODLE_AUDIO_SETTINGS: &str = "PandaDoodleAudioSettings";
static PANDA_DOODLE_VIBRATION_SETTINGS: &str = "PandaDoodleVibrationSettings";

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = ["window"], js_name = iOSstandalone)]
  fn ios_standalone() -> bool;
}

#[derive(Clone, Copy, Debug)]
pub enum UiEvent {
  LoadLandingPage,
  LoadMainMenu,
  LoadMenuChooseStage,
  LoadGame(LoadGameParams),
}

#[derive(Clone, Copy, Debug)]
pub struct LoadGameParams {
  pub book: Book,
  pub stage_number: usize,
}

pub struct Context {
  pub window: Rc<web_sys::Window>,

  pub platform_manager: PlatformManager,
  pub running_as_pwa: bool,

  pub canvas: web_sys::HtmlCanvasElement,
  pub screen_size: F2,
  pub device_pixel_ratio: F1,
  pub screen_viewport: Rc<Viewport>,
  pub ui_viewport: Rc<Viewport>,
  pub game_viewport: Rc<Viewport>,

  pub texture_manager: TextureManager,
  pub draw_manager: DrawManager,
  pub draw_depths: DrawDepths,

  pub audio_manager: AudioManager,
  pub audio_player: AudioPlayer,

  pub vibration_manager: VibrationManager,

  pub local_storage: Rc<web_sys::Storage>,
  pub achievments_manager: AchievmentsManager,
  pub stages_data: AllStagesData,

  pub history: Rc<web_sys::History>,

  pub dt_ms: F1,
  pub latest_timestamp_ms: F1,
  pub draw_cycle: u32,

  pub ui_manager_events: Rc<EventManager<UiManagerEvent>>,
  pub artificial_input_events: Rc<EventManager<InputEvent>>,
  pub ui_events: Rc<EventManager<UiEvent>>,
  pub menu_choose_stages_events: Rc<EventManager<MenuChooseStageEvent>>,
  pub stage_opacity: Shared<F1>,
  pub game_mode: RefCell<Option<Rc<GameMode>>>,

  pub show_fps: bool,
  pub statsig_bindings: StatsigBindings,
}

impl Context {
  pub fn new(window: Rc<web_sys::Window>) -> Context {
    let platform_manager = PlatformManager::new(&window);
    let running_as_pwa = is_running_as_pwa(platform_manager.ios());
    console_log_with_div!("running_as_pwa: {}", running_as_pwa);

    let document = Rc::new(window.document().unwrap());

    let canvas = document
      .create_element("canvas")
      .expect("document.create_element failed")
      .dyn_into::<web_sys::HtmlCanvasElement>()
      .expect("dyn_into::<web_sys::HtmlCanvasElement> failed");
    document
      .body()
      .unwrap()
      .append_child(&canvas)
      .expect("document.body.append_child failed");

    let screen_size = get_screen_size(&window);
    let device_pixel_ratio = get_device_pixel_ratio(&window);
    set_canvas_size(&canvas, &screen_size, &device_pixel_ratio);
    let canvas_context = Rc::new(
      canvas
        .get_context("2d")
        .expect("canvas.get_context 2d failed")
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .expect("dyn_into::<web_sys::CanvasRenderingContext2d> failed"),
    );
    let (screen_viewport, ui_viewport, game_viewport) =
      generate_viewports(&screen_size, &device_pixel_ratio);

    let local_storage = Rc::new(
      window
        .local_storage()
        .expect("window.local_storage failed")
        .unwrap(),
    );

    let vibration_manager = VibrationManager::new(
      &window,
      platform_manager.ios(),
      local_storage.clone(),
      PANDA_DOODLE_VIBRATION_SETTINGS.to_string(),
    );

    return Context {
      window: window.clone(),
      platform_manager,
      running_as_pwa,

      canvas,
      screen_size,
      device_pixel_ratio,
      screen_viewport: screen_viewport,
      ui_viewport: ui_viewport,
      game_viewport: game_viewport,

      texture_manager: TextureManager::new(document),
      draw_manager: DrawManager::new(
        Canvas2dDrawBackend::new(canvas_context),
        &screen_size,
        &device_pixel_ratio,
      ),
      draw_depths: DrawDepths::new(),

      audio_manager: AudioManager::new(),
      audio_player: AudioPlayer::new(
        local_storage.clone(),
        PANDA_DOODLE_AUDIO_SETTINGS.to_string(),
      ),

      vibration_manager: vibration_manager,

      local_storage: local_storage.clone(),
      achievments_manager: AchievmentsManager::new(local_storage.clone()),
      stages_data: AllStagesData::new(),

      history: Rc::new(window.history().expect("window.history failed")),

      dt_ms: 0.0,
      latest_timestamp_ms: 0.0,
      draw_cycle: 0,

      ui_manager_events: EventManager::new(),
      artificial_input_events: EventManager::new(),
      ui_events: EventManager::new(),
      menu_choose_stages_events: EventManager::new(),
      stage_opacity: Shared::new(1.0),
      game_mode: RefCell::new(None),

      show_fps: LocalStorageUtil::read(local_storage.as_ref(), "show_fps_key").unwrap_or(false),
      statsig_bindings: StatsigBindings::new(),
    };
  }

  pub fn update_timestamp(&mut self, timestamp_ms: F1) {
    if self.latest_timestamp_ms == 0.0 {
      self.latest_timestamp_ms = timestamp_ms;
    }
    self.dt_ms = (timestamp_ms - self.latest_timestamp_ms) as F1;
    self.latest_timestamp_ms = timestamp_ms;
  }

  pub fn get_canvas_size(&self) -> F2 {
    return self.screen_size * self.device_pixel_ratio;
  }

  pub fn check_screen_updated(&mut self) {
    let screen_size = get_screen_size(&self.window);
    let device_pixel_ratio = get_device_pixel_ratio(&self.window);
    if screen_size != self.screen_size || device_pixel_ratio != self.device_pixel_ratio {
      self.screen_size = screen_size;
      self.device_pixel_ratio = device_pixel_ratio;
      set_canvas_size(&self.canvas, &screen_size, &device_pixel_ratio);
      let (screen_viewport, ui_viewport, game_viewport) =
        generate_viewports(&screen_size, &device_pixel_ratio);
      self.screen_viewport = screen_viewport;
      self.ui_viewport = ui_viewport;
      self.game_viewport = game_viewport;
      self
        .draw_manager
        .update_screen_size(&screen_size, &device_pixel_ratio);
    }
  }
}

fn get_screen_size(window: &web_sys::Window) -> F2 {
  let inner_width = window.inner_width().expect("window.inner_width failed");
  let inner_height = window.inner_height().expect("window.inner_height failed");
  return F2 {
    x: inner_width.as_f64().unwrap() as F1,
    y: inner_height.as_f64().unwrap() as F1,
  };
}

fn get_device_pixel_ratio(window: &web_sys::Window) -> F1 {
  return window.device_pixel_ratio() as F1;
}

fn set_canvas_size(canvas: &web_sys::HtmlCanvasElement, size: &F2, device_pixel_ratio: &F1) {
  canvas
    .style()
    .set_property("width", &format!("{}px", size.x as i32))
    .expect("set_property failed");
  canvas
    .style()
    .set_property("height", &format!("{}px", size.y as i32))
    .expect("set_property failed");
  canvas.set_id("main_canvas");

  let canvas_size = size * device_pixel_ratio;
  canvas.set_width(canvas_size.x as u32);
  canvas.set_height(canvas_size.y as u32);
}

fn generate_viewports(
  screen_size: &F2,
  device_pixel_ratio: &F1,
) -> (Rc<Viewport>, Rc<Viewport>, Rc<Viewport>) {
  let screen_viewport = Rc::new(Viewport::new(
    &F2 { x: 0.0, y: 0.0 },
    screen_size,
    screen_size,
    &device_pixel_ratio,
  ));

  let screen_y_x_ratio = screen_size.y / screen_size.x;

  let ui_y_x_ratio = 720.0 / 480.0;
  let min_screen_y_x_ratio = 720.0 / 480.0;
  let ui_frame_size = if screen_y_x_ratio < min_screen_y_x_ratio {
    F2 {
      x: screen_size.y / ui_y_x_ratio,
      y: screen_size.y,
    }
  } else {
    F2 {
      x: screen_size.x,
      y: screen_size.x * ui_y_x_ratio,
    }
  };
  let ui_frame_position = F2 {
    x: screen_size.x / 2.0 - ui_frame_size.x / 2.0,
    y: screen_size.y / 2.0 - ui_frame_size.y / 2.0,
  };
  let ui_viewport = Rc::new(Viewport::new(
    &ui_frame_position,
    &ui_frame_size,
    &screen_size,
    &device_pixel_ratio,
  ));

  let game_y_x_ratio = 540.0 / 480.0;
  let game_frame_size = F2 {
    x: ui_frame_size.x,
    y: ui_frame_size.x * game_y_x_ratio,
  };
  let game_frame_position = F2 {
    x: screen_size.x / 2.0 - game_frame_size.x / 2.0,
    y: screen_size.y / 2.0 - game_frame_size.y / 2.0 + game_frame_size.y / 40.0,
  };
  let game_viewport = Rc::new(Viewport::new(
    &game_frame_position,
    &game_frame_size,
    &screen_size,
    &device_pixel_ratio,
  ));
  return (screen_viewport, ui_viewport, game_viewport);
}

fn is_running_as_pwa(ios: bool) -> bool {
  return if ios {
    ios_standalone()
  } else {
    let href = web_sys::window()
      .unwrap()
      .location()
      .href()
      .expect("web_sys::window().unwrap().location().href() failed");
    href.contains("mode=pwa")
  };
}

impl ContextTrait for Context {
  fn get_dt(&self) -> &F1 {
    return &self.dt_ms;
  }

  fn get_latest_timestamp(&self) -> &F1 {
    return &self.latest_timestamp_ms;
  }

  fn get_draw_cycle(&self) -> &u32 {
    return &self.draw_cycle;
  }

  fn get_draw_manager(&mut self) -> &mut DrawManager {
    return &mut self.draw_manager;
  }

  fn get_screen_viewport(&self) -> &Rc<Viewport> {
    return &self.screen_viewport;
  }

  fn get_ui_viewport(&self) -> &Rc<Viewport> {
    return &self.ui_viewport;
  }

  fn get_game_viewport(&self) -> &Rc<Viewport> {
    return &self.game_viewport;
  }

  fn get_screen_size(&self) -> &F2 {
    return &self.screen_size;
  }

  fn draw_ui_viewport(&mut self, args: DrawImageArgs) {
    self.draw_manager.draw_viewport(&self.ui_viewport, args);
  }

  fn draw_string_ui_viewport(&mut self, args: DrawStringArgs) {
    self
      .draw_manager
      .draw_string_viewport(&self.ui_viewport, args);
  }

  fn get_pixel_texture(&self) -> Rc<Texture> {
    return self.texture_manager.pixel.clone();
  }

  fn get_front_board_depth(&self) -> F1 {
    return self.draw_depths.front_board;
  }

  fn get_ui_manager_events(&self) -> &Rc<EventManager<UiManagerEventGeneric<Self>>> {
    return &self.ui_manager_events;
  }

  fn play_sound(&mut self, sound: &Rc<Audio>) {
    self.audio_player.play_sound(&sound);
  }

  fn get_platform_manager(&self) -> &PlatformManager {
    return &self.platform_manager;
  }

  fn window(&self) -> &web_sys::Window {
    return &self.window;
  }

  fn local_storage(&self) -> &web_sys::Storage {
    return self.local_storage.as_ref();
  }

  fn show_fps(&self) -> bool {
    return self.show_fps;
  }

  fn toggle_show_fps(&mut self) {
    self.show_fps = !self.show_fps;
    LocalStorageUtil::write(self.local_storage(), "show_fps_key", &self.show_fps);
  }
}
