use crate::engine::*;
use crate::game::paint_color::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub const STAGES_PER_BOOK: usize = 25;

pub struct AllStagesData {
  stages: Vec<StageData>,
  // pub stages_data: HashMap<usize, StageData>,
}

// pub type AllStagesData = HashMap<String, StageData>;

#[derive(Serialize, Deserialize)]
pub struct StageDataRaw {
  hint: String,
  record: i32,
  #[serde(rename(deserialize = "score2Stars"))]
  score_2_stars: i32,
  #[serde(rename(deserialize = "score3Stars"))]
  score_3_stars: i32,
  #[serde(rename(deserialize = "stageObjects"))]
  stage_objects: String,
  user: String,
  version: i32,
}

#[derive(Serialize, Deserialize)]
pub struct StageObjectRaw {
  #[serde(rename(deserialize = "type"))]
  object_type: String,
  #[serde(default)]
  x: F1,
  #[serde(default)]
  y: F1,
  #[serde(default)]
  x1: F1,
  #[serde(default)]
  y1: F1,
  #[serde(default)]
  x2: F1,
  #[serde(default)]
  y2: F1,
  #[serde(default)]
  color: String,
  #[serde(default)]
  #[serde(rename(deserialize = "paintAmount"))]
  paint_amount: i32,
  #[serde(default)]
  r: F1,
  #[serde(default)]
  speed: F1,
}

pub struct SourceData {
  pub position: F2,
  pub paint_color: PaintColor,
  pub paint_amount: F1,
}

pub struct GoalData {
  pub position: F2,
  pub paint_color: PaintColor,
}

pub struct MirrorData {
  pub p1: F2,
  pub p2: F2,
}

pub struct MovingSourceData {
  pub position: F2,
  pub radius: F1,
  pub speed: F1,
}

pub struct PortalData {
  pub p1: F2,
  pub p2: F2,
}

#[derive(Default)]
pub struct StageData {
  pub record: i32,
  pub score_2_stars: i32,
  pub score_3_stars: i32,
  pub sources: Vec<SourceData>,
  pub goals: Vec<GoalData>,
  pub mirrors: Vec<MirrorData>,
  pub moving_sources: Vec<MovingSourceData>,
  pub portals: Vec<PortalData>,
}

fn get_paint_color_from_name(paint_color_name: &String) -> PaintColor {
  if paint_color_name == "red" {
    return PaintColor::Red;
  }
  if paint_color_name == "yellow" {
    return PaintColor::Yellow;
  }
  if paint_color_name == "blue" {
    return PaintColor::Blue;
  }
  if paint_color_name == "orange" {
    return PaintColor::Orange;
  }
  if paint_color_name == "purple" {
    return PaintColor::Purple;
  }
  if paint_color_name == "green" {
    return PaintColor::Green;
  }
  if paint_color_name == "gray" {
    return PaintColor::Gray;
  }
  panic!("invalid paint_color_name {}", paint_color_name);
}

impl AllStagesData {
  pub fn new() -> AllStagesData {
    let mut stages_data: HashMap<String, StageData> = HashMap::new();
    let stages_data_raw: HashMap<String, StageDataRaw> =
      serde_json::from_str(STAGES_DATA).expect("failed to parse STAGES_DATA as json");

    for (stage_id, stage_data_raw) in stages_data_raw {
      let stage_objects_raw: Vec<StageObjectRaw> =
        serde_json::from_str(stage_data_raw.stage_objects.as_str())
          .expect("failed to parse stage_objects as json");

      let mut sources: Vec<SourceData> = Vec::new();
      let mut goals: Vec<GoalData> = Vec::new();
      let mut mirrors: Vec<MirrorData> = Vec::new();
      let mut moving_sources: Vec<MovingSourceData> = Vec::new();
      let mut portals: Vec<PortalData> = Vec::new();

      for stage_object_raw in stage_objects_raw.iter() {
        if stage_object_raw.object_type == "source" {
          sources.push(SourceData {
            position: F2 {
              x: stage_object_raw.x,
              y: stage_object_raw.y,
            },
            paint_color: get_paint_color_from_name(&stage_object_raw.color),
            paint_amount: stage_object_raw.paint_amount as F1,
          });
        } else if stage_object_raw.object_type == "goal" {
          goals.push(GoalData {
            position: F2 {
              x: stage_object_raw.x,
              y: stage_object_raw.y,
            },
            paint_color: get_paint_color_from_name(&stage_object_raw.color),
          });
        } else if stage_object_raw.object_type == "mirror" {
          mirrors.push(MirrorData {
            p1: F2 {
              x: stage_object_raw.x1,
              y: stage_object_raw.y1,
            },
            p2: F2 {
              x: stage_object_raw.x2,
              y: stage_object_raw.y2,
            },
          });
        } else if stage_object_raw.object_type == "movingSourceCircle" {
          moving_sources.push(MovingSourceData {
            position: F2 {
              x: stage_object_raw.x,
              y: stage_object_raw.y,
            },
            radius: stage_object_raw.r,
            speed: stage_object_raw.speed,
          });
        } else if stage_object_raw.object_type == "portal" {
          portals.push(PortalData {
            p1: F2 {
              x: stage_object_raw.x1,
              y: stage_object_raw.y1,
            },
            p2: F2 {
              x: stage_object_raw.x2,
              y: stage_object_raw.y2,
            },
          });
        } else {
          console_log_with_div!("{}", stage_object_raw.object_type);
        }
      }

      // let stage_id: usize = stage_id.parse::<usize>().unwrap();

      let stage = StageData {
        record: stage_data_raw.record,
        score_2_stars: if stage_data_raw.score_2_stars == -1 {
          7000
        } else {
          stage_data_raw.score_2_stars
        },
        score_3_stars: if stage_data_raw.score_3_stars == -1 {
          9000
        } else {
          stage_data_raw.score_3_stars
        },
        sources: sources,
        goals: goals,
        mirrors: mirrors,
        moving_sources: moving_sources,
        portals: portals,
      };

      stages_data.insert(stage_id, stage);
    }

    let stages_list: Vec<String> =
      serde_json::from_str(STAGES_LIST_DATA).expect("failed to parse STAGES_LIST_DATA as json");

    let mut stages: Vec<StageData> = Vec::new();
    for stage_id in stages_list.iter() {
      stages.push(
        stages_data
          .remove(stage_id)
          .expect(format!("stage_id {} not found in stages_data", stage_id).as_str()),
      );
    }
    return AllStagesData { stages: stages };
  }

  pub fn get_stage(&self, book_number: usize, stage_number: usize) -> &StageData {
    let stage_index = STAGES_PER_BOOK * book_number + stage_number;
    return &self.stages[stage_index];
  }
}

static STAGES_LIST_DATA: &str = r###"[
  "82"]"###;

static STAGES_DATA: &str = r###"{ "82": {"record": 212, "stageObjects": "[{\"type\":\"source\",\"x\":0.49166666666666664,\"y\":0.8520833333333333,\"color\":\"red\",\"paintAmount\":1000,\"activated\":true},{\"type\":\"goal\",\"x\":0.4895833333333333,\"y\":0.24791666666666667,\"color\":\"red\"}]", "version": 5, "user": "lucamattosmoller@gmail.com", "stageId": 82, "hint": "0.498523,0.30932,1,63,195*0.497303,0.299899,1,64,196*0.499993,0.328205,1,61,193*0.499998,0.318705,1,62,194*0.499927,0.356705,1,58,190*0.499977,0.347205,1,59,191*0.499989,0.337705,1,60,192*0.499446,0.375699,1,56,188*0.499686,0.366202,1,57,189*0.5,0.536911,1,39,171*0.5,0.555911,1,37,169*0.5,0.546411,1,38,170*0.499993,0.584411,1,34,166*0.499996,0.574911,1,35,167*0.499999,0.565411,1,36,168*0.499977,0.593911,1,33,165*0.499961,0.603411,1,32,164*0.499905,0.612911,1,31,163*0.499849,0.622411,1,30,162*0.498611,0.650851,1,27,159*0.498611,0.641351,1,28,160*0.499658,0.631909,1,29,161*0.498611,0.669851,1,25,157*0.498611,0.660351,1,26,158*0.498611,0.688851,1,23,155*0.498611,0.679351,1,24,156*0.498611,0.698352,1,22,154*0.498611,0.707852,1,21,153*0.498611,0.717352,1,20,152*0.498609,0.736352,1,18,150*0.49861,0.726852,1,19,151*0.498601,0.745852,1,17,149*0.498566,0.764852,1,15,147*0.498592,0.755352,1,16,148*0.498252,0.783849,1,13,145*0.498409,0.77435,1,14,146*0.497722,0.793334,1,12,144*0.491667,0.852083,1,0,132*0.498958,0.869628,1,2,134*0.495313,0.860856,1,1,133*0.502778,0.403997,1,53,185*0.501533,0.394579,1,54,186*0.500288,0.385161,1,55,187*0.502778,0.422997,1,51,183*0.502778,0.413497,1,52,184*0.502777,0.441997,1,49,181*0.502777,0.432497,1,50,182*0.502764,0.470497,1,46,178*0.50277,0.460997,1,47,179*0.502777,0.451497,1,48,180*0.502673,0.489497,1,44,176*0.502719,0.479997,1,45,177*0.501823,0.50847,1,42,174*0.50252,0.498996,1,43,175*0.500564,0.527428,1,40,172*0.501127,0.517945,1,41,173*0.500009,0.802555,1,11,143*0.500697,0.81203,1,10,142*0.502778,0.830902,1,8,140*0.501386,0.821505,1,9,141*0.502777,0.840402,1,7,139*0.502775,0.849902,1,6,138*0.50277,0.859402,1,5,137*0.502758,0.868902,1,4,136*0.502604,0.878401,1,3,135~0,788.01", "score2Stars": -1, "score3Stars": -1}}"###;
