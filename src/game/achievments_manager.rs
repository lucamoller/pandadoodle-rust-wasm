use crate::engine::*;
use crate::game::stages_data::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

static PANDA_DOODLE_ACHIEVMENTS: &str = "PandaDoodleAchievments";
const BOOK_COUNT: usize = 5;
const STARS_REQUIRED_TO_UNLOCK_BOOK: usize = 50;
const MAX_STARS_PER_STAGE: usize = 3;

pub struct AchievmentsManager {
  pub local_storage: Rc<web_sys::Storage>,
  pub achievments_data: RefCell<AchievmentsData>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct AchievmentsData {
  #[serde(default)]
  stars_collected: HashMap<usize, usize>,
  #[serde(default)]
  scores: HashMap<usize, i32>,
}

fn load_achievments_data(local_storage: &web_sys::Storage) -> AchievmentsData {
  return LocalStorageUtil::read(local_storage, PANDA_DOODLE_ACHIEVMENTS).unwrap_or_default();
}

impl AchievmentsManager {
  pub fn new(local_storage: Rc<web_sys::Storage>) -> AchievmentsManager {
    return AchievmentsManager {
      local_storage: local_storage.clone(),
      achievments_data: RefCell::new(load_achievments_data(local_storage.as_ref())),
    };
  }

  fn store(&self) {
    LocalStorageUtil::write(
      self.local_storage.as_ref(),
      PANDA_DOODLE_ACHIEVMENTS,
      self.achievments_data.borrow().deref(),
    );
  }

  pub fn is_stage_available(&self, book_number: usize, stage_number: usize) -> bool {
    if stage_number == 0 {
      return true;
    }

    let prev_stage_index = self.get_stage_index(book_number, stage_number) - 1;
    return self
      .achievments_data
      .borrow()
      .stars_collected
      .contains_key(&prev_stage_index);
  }

  pub fn get_stage_index(&self, book_number: usize, stage_number: usize) -> usize {
    return book_number * STAGES_PER_BOOK + stage_number;
  }

  pub fn get_stage_stars(&self, book_number: usize, stage_number: usize) -> usize {
    if let Some(stars) = self
      .achievments_data
      .borrow()
      .stars_collected
      .get(&self.get_stage_index(book_number, stage_number))
    {
      return *stars;
    }
    return 0;
  }

  pub fn get_total_stars(&self) -> usize {
    return self
      .achievments_data
      .borrow()
      .stars_collected
      .values()
      .sum();
  }

  pub fn is_book_available(&self, book_number: usize) -> bool {
    if book_number == 0 {
      return true;
    }
    return self.get_total_stars() > self.get_stars_required_for_book(book_number);
  }

  pub fn get_stars_required_for_book(&self, book_number: usize) -> usize {
    return book_number * STARS_REQUIRED_TO_UNLOCK_BOOK;
  }

  pub fn get_total_existing_medals(&self) -> usize {
    return BOOK_COUNT;
  }

  pub fn get_stars_per_book(&self, book_number: usize) -> usize {
    let mut result = 0;
    for stage_number in 0..STAGES_PER_BOOK {
      let stage_index = self.get_stage_index(book_number, stage_number);
      result += match self
        .achievments_data
        .borrow()
        .stars_collected
        .get(&stage_index)
      {
        Some(stars_collected) => *stars_collected,
        None => 0,
      };
    }
    return result;
  }

  pub fn get_total_existing_stars_per_book(&self) -> usize {
    return MAX_STARS_PER_STAGE * STAGES_PER_BOOK;
  }

  pub fn has_medal(&self, book_number: usize) -> bool {
    return self.get_stars_per_book(book_number) == self.get_total_existing_stars_per_book();
  }

  pub fn get_total_medals(&self) -> usize {
    let mut result = 0;
    for book_number in 0..BOOK_COUNT {
      if self.has_medal(book_number) {
        result += 1;
      }
    }
    return result;
  }

  pub fn get_score(&self, book_number: usize, stage_number: usize) -> i32 {
    let stage_index = self.get_stage_index(book_number, stage_number);
    return match self.achievments_data.borrow().scores.get(&stage_index) {
      Some(score) => *score,
      None => 0,
    };
  }

  pub fn set_score(
    &self,
    book_number: usize,
    stage_number: usize,
    score: i32,
    stars: usize,
  ) -> bool {
    let stage_index = self.get_stage_index(book_number, stage_number);
    self
      .achievments_data
      .replace(load_achievments_data(&self.local_storage));
    let best_score = match self.achievments_data.borrow().scores.get(&stage_index) {
      Some(current_score) => score > *current_score,
      None => true,
    };
    if best_score {
      self
        .achievments_data
        .borrow_mut()
        .scores
        .insert(stage_index, score);
      self
        .achievments_data
        .borrow_mut()
        .stars_collected
        .insert(stage_index, stars);
      self.store();
    }
    return best_score;
  }
}
