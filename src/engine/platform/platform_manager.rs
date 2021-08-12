pub struct PlatformManager {
  ios: bool,
}

impl PlatformManager {
  pub fn new(window: &web_sys::Window) -> PlatformManager {
    let ios = {
      let user_agent = window
        .navigator()
        .user_agent()
        .expect("window().navigator().user_agent() call failed.");
      user_agent.contains("iPhone") || user_agent.contains("iPad")
    };
    return PlatformManager { ios: ios };
  }

  pub fn ios(&self) -> bool {
    return self.ios;
  }
}
