use crate::engine::*;
use crate::game::paint_color::PaintColor;
use std::rc::Rc;

pub struct TextureManager {
  pub background_wood: Rc<Texture>,
  pub background_canvas: Rc<Texture>,
  pub background_cat_bot: Rc<Texture>,
  pub background_cat_top: Rc<Texture>,
  pub background_rabbit_bot: Rc<Texture>,
  pub background_rabbit_top: Rc<Texture>,
  pub background_wolf_bot: Rc<Texture>,
  pub background_wolf_top: Rc<Texture>,
  pub chat_bar_cat: Rc<Texture>,
  pub chat_bar_panda: Rc<Texture>,
  pub chat_bar_rabbit: Rc<Texture>,
  pub chat_bar_wolf: Rc<Texture>,
  pub chat_char_cat: Rc<Texture>,
  pub chat_char_panda: Rc<Texture>,
  pub chat_char_rabbit: Rc<Texture>,
  pub chat_char_wolf: Rc<Texture>,
  pub circle40: Rc<Texture>,
  pub collect0star: Rc<Texture>,
  pub collect1star: Rc<Texture>,
  pub collect2star: Rc<Texture>,
  pub collect3star: Rc<Texture>,
  pub cross: Rc<Texture>,
  pub dot: Rc<Texture>,
  pub flare: Rc<Texture>,
  pub gui_awesome: Rc<Texture>,
  pub gui_book_cat: Rc<Texture>,
  pub gui_book_panda: Rc<Texture>,
  pub gui_book_panda2: Rc<Texture>,
  pub gui_book_rabbit: Rc<Texture>,
  pub gui_book_wolf: Rc<Texture>,
  pub gui_btn_back: Rc<Texture>,
  pub gui_btn_back_pressed: Rc<Texture>,
  pub gui_btn_menu: Rc<Texture>,
  pub gui_btn_menu_pressed: Rc<Texture>,
  pub gui_btn_music: Rc<Texture>,
  pub gui_btn_music_off: Rc<Texture>,
  pub gui_btn_sound: Rc<Texture>,
  pub gui_btn_sound_off: Rc<Texture>,
  pub gui_btn_restart: Rc<Texture>,
  pub gui_btn_restart_pressed: Rc<Texture>,
  pub gui_btn_undo: Rc<Texture>,
  pub gui_btn_undo_pressed: Rc<Texture>,
  pub gui_btn_wood: Rc<Texture>,
  pub gui_btn_wood_pressed: Rc<Texture>,
  pub gui_btn_wood_menu: Rc<Texture>,
  pub gui_btn_wood_menu_pressed: Rc<Texture>,
  pub gui_btn_wood_play: Rc<Texture>,
  pub gui_btn_wood_play_pressed: Rc<Texture>,
  pub gui_btn_wood_restart: Rc<Texture>,
  pub gui_btn_wood_restart_pressed: Rc<Texture>,
  pub gui_btn_wood_skip: Rc<Texture>,
  pub gui_btn_wood_skip_pressed: Rc<Texture>,
  pub gui_credits: Rc<Texture>,
  pub gui_cursor: Rc<Texture>,
  pub gui_glow_bot: Rc<Texture>,
  pub gui_glow_top: Rc<Texture>,
  pub gui_img_lock: Rc<Texture>,
  pub gui_img_twinkle_next: Rc<Texture>,
  pub gui_nice: Rc<Texture>,
  pub gui_paper: Rc<Texture>,
  pub gui_scroll: Rc<Texture>,
  pub gui_stage_icon: Rc<Texture>,
  pub gui_stage_icon_pressed: Rc<Texture>,
  pub gui_stage_icon_lock: Rc<Texture>,
  pub gui_very_good: Rc<Texture>,
  pub gui_volume: Rc<Texture>,
  pub goals_blue: Rc<Texture>,
  pub goals_blue_fill: Rc<Texture>,
  pub goals_gray: Rc<Texture>,
  pub goals_gray_fill: Rc<Texture>,
  pub goals_green: Rc<Texture>,
  pub goals_green_fill: Rc<Texture>,
  pub goals_orange: Rc<Texture>,
  pub goals_orange_fill: Rc<Texture>,
  pub goals_purple: Rc<Texture>,
  pub goals_purple_fill: Rc<Texture>,
  pub goals_red: Rc<Texture>,
  pub goals_red_fill: Rc<Texture>,
  pub goals_yellow: Rc<Texture>,
  pub goals_yellow_fill: Rc<Texture>,
  pub icon_original: Rc<Texture>,
  pub mancha: Rc<Texture>,
  pub medal: Rc<Texture>,
  pub mini_star: Rc<Texture>,
  pub portal1: Rc<Texture>,
  pub portal1_glow: Rc<Texture>,
  pub portal2: Rc<Texture>,
  pub portal2_glow: Rc<Texture>,
  pub portal3: Rc<Texture>,
  pub portal3_glow: Rc<Texture>,
  pub pixel: Rc<Texture>,
  pub source_blue: Rc<Texture>,
  pub source_blue_empty: Rc<Texture>,
  pub source_light: Rc<Texture>,
  pub source_moving: Rc<Texture>,
  pub source_red: Rc<Texture>,
  pub source_red_empty: Rc<Texture>,
  pub source_yellow: Rc<Texture>,
  pub source_yellow_empty: Rc<Texture>,
  pub star: Rc<Texture>,
  pub star_active_bright: Rc<Texture>,
  pub star_empty: Rc<Texture>,
  pub star_l: Rc<Texture>,

  pub landing_loader: TextureLoader,
  pub loader: TextureLoader,
}

impl TextureManager {
  pub fn new(document: Rc<web_sys::Document>) -> TextureManager {
    let mut landing_loader = TextureLoader::new(document.clone());
    let mut loader = TextureLoader::new(document);

    return TextureManager {
      background_wood: landing_loader.register(TextureParams {
        src: String::from("/static/gui_madeira800.png"),
        ..Default::default()
      }),
      background_canvas: loader.register(TextureParams {
        src: String::from("/static/canvasFinal480.png"),
        ..Default::default()
      }),
      background_cat_bot: loader.register(TextureParams {
        src: String::from("/static/backCatBot.png"),
        ..Default::default()
      }),
      background_cat_top: loader.register(TextureParams {
        src: String::from("/static/backCatTop.png"),
        ..Default::default()
      }),
      background_rabbit_bot: loader.register(TextureParams {
        src: String::from("/static/backRabbitBot.png"),
        ..Default::default()
      }),
      background_rabbit_top: loader.register(TextureParams {
        src: String::from("/static/backRabbitTop.png"),
        ..Default::default()
      }),
      background_wolf_bot: loader.register(TextureParams {
        src: String::from("/static/backWolfBot.png"),
        ..Default::default()
      }),
      background_wolf_top: loader.register(TextureParams {
        src: String::from("/static/backWolfTop.png"),
        ..Default::default()
      }),
      chat_bar_cat: loader.register(TextureParams {
        src: String::from("/static/gui_chatBarCat.png"),
        ..Default::default()
      }),
      chat_bar_panda: loader.register(TextureParams {
        src: String::from("/static/gui_chatBarPanda.png"),
        ..Default::default()
      }),
      chat_bar_rabbit: loader.register(TextureParams {
        src: String::from("/static/gui_chatBarRabbit.png"),
        ..Default::default()
      }),
      chat_bar_wolf: loader.register(TextureParams {
        src: String::from("/static/gui_chatBarWolf.png"),
        ..Default::default()
      }),
      chat_char_cat: loader.register(TextureParams {
        src: String::from("/static/chars_cat.png"),
        ..Default::default()
      }),
      chat_char_panda: loader.register(TextureParams {
        src: String::from("/static/chars_panda.png"),
        ..Default::default()
      }),
      chat_char_rabbit: loader.register(TextureParams {
        src: String::from("/static/chars_rabbit.png"),
        ..Default::default()
      }),
      chat_char_wolf: loader.register(TextureParams {
        src: String::from("/static/chars_wolf.png"),
        ..Default::default()
      }),
      circle40: loader.register(TextureParams {
        src: String::from("/static/circle40.png"),
        ..Default::default()
      }),
      collect0star: loader.register(TextureParams {
        src: String::from("/static/collect0star.png"),
        ..Default::default()
      }),
      collect1star: loader.register(TextureParams {
        src: String::from("/static/collect1star.png"),
        ..Default::default()
      }),
      collect2star: loader.register(TextureParams {
        src: String::from("/static/collect2star.png"),
        ..Default::default()
      }),
      collect3star: loader.register(TextureParams {
        src: String::from("/static/collect3star.png"),
        ..Default::default()
      }),
      cross: loader.register(TextureParams {
        src: String::from("/static/cross.png"),
        ..Default::default()
      }),
      dot: loader.register(TextureParams {
        src: String::from("/static/dot.png"),
        ..Default::default()
      }),
      // flare: Texture::new(document, gl.clone(), true, "/static/flare.png"),
      flare: loader.register(TextureParams {
        src: String::from("/static/flare_small.png"),
        optional: TextureParamsOptional {
          color_alpha_cache: Some(ColorAlphaCacheParams {
            colors: [
              PaintColor::NoColor,
              PaintColor::Red,
              PaintColor::Yellow,
              PaintColor::Blue,
              PaintColor::Orange,
              PaintColor::Green,
              PaintColor::Purple,
              PaintColor::Gray,
            ]
            .iter()
            .map(|color| color.get_draw_color())
            .collect(),
            opacity_levels: 10,
          }),
        },
      }),
      gui_awesome: loader.register(TextureParams {
        src: String::from("/static/gui_awesome.png"),
        ..Default::default()
      }),
      gui_book_cat: loader.register(TextureParams {
        src: String::from("/static/gui_bookCat.png"),
        ..Default::default()
      }),
      gui_book_panda: loader.register(TextureParams {
        src: String::from("/static/gui_bookPanda.png"),
        ..Default::default()
      }),
      gui_book_panda2: loader.register(TextureParams {
        src: String::from("/static/gui_bookPanda_ii.png"),
        ..Default::default()
      }),
      gui_book_rabbit: loader.register(TextureParams {
        src: String::from("/static/gui_bookRabbit.png"),
        ..Default::default()
      }),
      gui_book_wolf: loader.register(TextureParams {
        src: String::from("/static/gui_bookWolf.png"),
        ..Default::default()
      }),
      gui_btn_back: loader.register(TextureParams {
        src: String::from("/static/gui_btnBack.png"),
        ..Default::default()
      }),
      gui_btn_back_pressed: loader.register(TextureParams {
        src: String::from("/static/gui_btnBack_pressed.png"),
        ..Default::default()
      }),
      gui_btn_menu: loader.register(TextureParams {
        src: String::from("/static/gui_btnMenu.png"),
        ..Default::default()
      }),
      gui_btn_menu_pressed: loader.register(TextureParams {
        src: String::from("/static/gui_btnMenu_pressed.png"),
        ..Default::default()
      }),
      gui_btn_music: loader.register(TextureParams {
        src: String::from("/static/gui_btnMusic.png"),
        ..Default::default()
      }),
      gui_btn_music_off: loader.register(TextureParams {
        src: String::from("/static/gui_btnMusicNo.png"),
        ..Default::default()
      }),
      gui_btn_sound: loader.register(TextureParams {
        src: String::from("/static/gui_btnSound.png"),
        ..Default::default()
      }),
      gui_btn_sound_off: loader.register(TextureParams {
        src: String::from("/static/gui_btnSoundNo.png"),
        ..Default::default()
      }),
      gui_btn_restart: loader.register(TextureParams {
        src: String::from("/static/gui_btnRestart.png"),
        ..Default::default()
      }),
      gui_btn_restart_pressed: loader.register(TextureParams {
        src: String::from("/static/gui_btnRestart_pressed.png"),
        ..Default::default()
      }),
      gui_btn_undo: loader.register(TextureParams {
        src: String::from("/static/gui_btnUndo.png"),
        ..Default::default()
      }),
      gui_btn_undo_pressed: loader.register(TextureParams {
        src: String::from("/static/gui_btnUndo_pressed.png"),
        ..Default::default()
      }),
      gui_btn_wood: landing_loader.register(TextureParams {
        src: String::from("/static/gui_buttonWood.png"),
        ..Default::default()
      }),
      gui_btn_wood_pressed: landing_loader.register(TextureParams {
        src: String::from("/static/gui_buttonWood_pressed.png"),
        ..Default::default()
      }),
      gui_btn_wood_menu: loader.register(TextureParams {
        src: String::from("/static/gui_btnWoodMenu.png"),
        ..Default::default()
      }),
      gui_btn_wood_menu_pressed: loader.register(TextureParams {
        src: String::from("/static/gui_btnWoodMenu_pressed.png"),
        ..Default::default()
      }),
      gui_btn_wood_play: loader.register(TextureParams {
        src: String::from("/static/gui_btnWoodPlay.png"),
        ..Default::default()
      }),
      gui_btn_wood_play_pressed: loader.register(TextureParams {
        src: String::from("/static/gui_btnWoodPlay_pressed.png"),
        ..Default::default()
      }),
      gui_btn_wood_restart: loader.register(TextureParams {
        src: String::from("/static/gui_btnWoodRestart.png"),
        ..Default::default()
      }),
      gui_btn_wood_restart_pressed: loader.register(TextureParams {
        src: String::from("/static/gui_btnWoodRestart_pressed.png"),
        ..Default::default()
      }),
      gui_btn_wood_skip: loader.register(TextureParams {
        src: String::from("/static/gui_btnWoodSkip.png"),
        ..Default::default()
      }),
      gui_btn_wood_skip_pressed: loader.register(TextureParams {
        src: String::from("/static/gui_btnWoodSkip_pressed.png"),
        ..Default::default()
      }),
      gui_credits: loader.register(TextureParams {
        src: String::from("/static/gui_credits.png"),
        ..Default::default()
      }),
      gui_cursor: loader.register(TextureParams {
        src: String::from("/static/gui_cursor.png"),
        ..Default::default()
      }),
      gui_glow_bot: loader.register(TextureParams {
        src: String::from("/static/gui_glowBot.png"),
        ..Default::default()
      }),
      gui_glow_top: loader.register(TextureParams {
        src: String::from("/static/gui_glowTop.png"),
        ..Default::default()
      }),
      gui_img_lock: loader.register(TextureParams {
        src: String::from("/static/gui_imgLock.png"),
        ..Default::default()
      }),
      gui_img_twinkle_next: loader.register(TextureParams {
        src: String::from("/static/gui_imgTwinkleNext.png"),
        ..Default::default()
      }),
      gui_nice: loader.register(TextureParams {
        src: String::from("/static/gui_nice.png"),
        ..Default::default()
      }),
      gui_paper: loader.register(TextureParams {
        src: String::from("/static/gui_paper.png"),
        ..Default::default()
      }),
      gui_scroll: loader.register(TextureParams {
        src: String::from("/static/gui_scroll.png"),
        ..Default::default()
      }),
      gui_stage_icon: loader.register(TextureParams {
        src: String::from("/static/gui_stageIcon.png"),
        ..Default::default()
      }),
      gui_stage_icon_pressed: loader.register(TextureParams {
        src: String::from("/static/gui_stageIcon_pressed.png"),
        ..Default::default()
      }),
      gui_stage_icon_lock: loader.register(TextureParams {
        src: String::from("/static/gui_stageIconLock.png"),
        ..Default::default()
      }),
      gui_very_good: loader.register(TextureParams {
        src: String::from("/static/gui_verygood.png"),
        ..Default::default()
      }),
      gui_volume: loader.register(TextureParams {
        src: String::from("/static/gui_volume.png"),
        ..Default::default()
      }),
      goals_blue: loader.register(TextureParams {
        src: String::from("/static/goals_blue.png"),
        ..Default::default()
      }),
      goals_blue_fill: loader.register(TextureParams {
        src: String::from("/static/goals_blue_fill.png"),
        ..Default::default()
      }),
      goals_gray: loader.register(TextureParams {
        src: String::from("/static/goals_gray.png"),
        ..Default::default()
      }),
      goals_gray_fill: loader.register(TextureParams {
        src: String::from("/static/goals_gray_fill.png"),
        ..Default::default()
      }),
      goals_green: loader.register(TextureParams {
        src: String::from("/static/goals_green.png"),
        ..Default::default()
      }),
      goals_green_fill: loader.register(TextureParams {
        src: String::from("/static/goals_green_fill.png"),
        ..Default::default()
      }),
      goals_orange: loader.register(TextureParams {
        src: String::from("/static/goals_orange.png"),
        ..Default::default()
      }),
      goals_orange_fill: loader.register(TextureParams {
        src: String::from("/static/goals_orange_fill.png"),
        ..Default::default()
      }),
      goals_purple: loader.register(TextureParams {
        src: String::from("/static/goals_purple.png"),
        ..Default::default()
      }),
      goals_purple_fill: loader.register(TextureParams {
        src: String::from("/static/goals_purple_fill.png"),
        ..Default::default()
      }),
      goals_red: loader.register(TextureParams {
        src: String::from("/static/goals_red.png"),
        ..Default::default()
      }),
      goals_red_fill: loader.register(TextureParams {
        src: String::from("/static/goals_red_fill.png"),
        ..Default::default()
      }),
      goals_yellow: loader.register(TextureParams {
        src: String::from("/static/goals_yellow.png"),
        ..Default::default()
      }),
      goals_yellow_fill: loader.register(TextureParams {
        src: String::from("/static/goals_yellow_fill.png"),
        ..Default::default()
      }),
      icon_original: landing_loader.register(TextureParams {
        src: String::from("/static/icon_original.png"),
        ..Default::default()
      }),
      mancha: loader.register(TextureParams {
        src: String::from("/static/mancha.png"),
        ..Default::default()
      }),
      medal: loader.register(TextureParams {
        src: String::from("/static/medal.png"),
        ..Default::default()
      }),
      mini_star: loader.register(TextureParams {
        src: String::from("/static/miniStar.png"),
        ..Default::default()
      }),
      pixel: landing_loader.register(TextureParams {
        src: String::from("/static/pixel.png"),
        ..Default::default()
      }),
      portal1: loader.register(TextureParams {
        src: String::from("/static/portal1.png"),
        ..Default::default()
      }),
      portal1_glow: loader.register(TextureParams {
        src: String::from("/static/portal1_glow.png"),
        ..Default::default()
      }),
      portal2: loader.register(TextureParams {
        src: String::from("/static/portal2.png"),
        ..Default::default()
      }),
      portal2_glow: loader.register(TextureParams {
        src: String::from("/static/portal2_glow.png"),
        ..Default::default()
      }),
      portal3: loader.register(TextureParams {
        src: String::from("/static/portal3.png"),
        ..Default::default()
      }),
      portal3_glow: loader.register(TextureParams {
        src: String::from("/static/portal3_glow.png"),
        ..Default::default()
      }),
      source_blue: loader.register(TextureParams {
        src: String::from("/static/sourceBlue.png"),
        ..Default::default()
      }),
      source_blue_empty: loader.register(TextureParams {
        src: String::from("/static/sourceBlueEmpty.png"),
        ..Default::default()
      }),
      source_light: loader.register(TextureParams {
        src: String::from("/static/sourceLight.png"),
        ..Default::default()
      }),
      source_moving: loader.register(TextureParams {
        src: String::from("/static/sourceMoving.png"),
        ..Default::default()
      }),
      source_red: loader.register(TextureParams {
        src: String::from("/static/sourceRed.png"),
        ..Default::default()
      }),
      source_red_empty: loader.register(TextureParams {
        src: String::from("/static/sourceRedEmpty.png"),
        ..Default::default()
      }),
      source_yellow: loader.register(TextureParams {
        src: String::from("/static/sourceYellow.png"),
        ..Default::default()
      }),
      source_yellow_empty: loader.register(TextureParams {
        src: String::from("/static/sourceYellowEmpty.png"),
        ..Default::default()
      }),
      star: loader.register(TextureParams {
        src: String::from("/static/especial.png"),
        ..Default::default()
      }),
      star_active_bright: loader.register(TextureParams {
        src: String::from("/static/star_active_bright.png"),
        ..Default::default()
      }),
      star_empty: loader.register(TextureParams {
        src: String::from("/static/especialEmpty.png"),
        ..Default::default()
      }),
      star_l: loader.register(TextureParams {
        src: String::from("/static/especialL.png"),
        ..Default::default()
      }),
      landing_loader: landing_loader,
      loader: loader,
    };
  }

  pub fn loaded_landing(&mut self) -> bool {
    return self.landing_loader.all_loaded();
  }

  pub fn loaded(&mut self) -> bool {
    return self.loader.all_loaded();
  }
}
