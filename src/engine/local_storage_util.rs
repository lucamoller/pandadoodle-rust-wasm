pub struct LocalStorageUtil {}

impl LocalStorageUtil {
  pub fn read<T: serde::de::DeserializeOwned>(
    local_storage: &web_sys::Storage,
    key: &str,
  ) -> Option<T> {
    return local_storage
      .get_item(key)
      .expect(&format!("local_storage.get_item({}) failed", key))
      .map(|value_txt| {
        serde_json::from_str(value_txt.as_str()).expect(&format!(
          "failed to parse local_storage.get_item({}) as json",
          key
        ))
      });
  }

  pub fn write<T: serde::Serialize>(local_storage: &web_sys::Storage, key: &str, value: &T) {
    local_storage
      .set_item(
        key,
        serde_json::to_string(value)
          .expect(&format!("serde_json::to_string({}) failed.", key))
          .as_str(),
      )
      .expect("local_storage.set_item failed.");
  }
}
