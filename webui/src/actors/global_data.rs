use crate::{*, interop::{get_global_data, set_global_data}};

/// Container for global data persisted to memory only
/// 
/// Data is stored in memory. Browser refreshing or page loads will always start with empty data.
/// For persistent memory stored in the browser, look at `webui::get_user_storage_data` and `webui::set_user_storage_data`
pub struct GlobalData;

impl GlobalData {
	/// Save data to global storage.
	/// 
	/// Passed value is serialized into json and stored in memory.
	/// Success will return Ok(json)
	pub fn set_data<T>(key: String, value: T) -> Result<String, WebUIError>
		where T: serde::Serialize
	{
		let result = serde_json::to_string(&value);
		match result {
			Ok(json) => {
				set_global_data(key, json.to_string());
				Ok(json)
			},
			Err(error) => {
				jslog!("Error in GlobalData.set_data: {:?}", error);
				Err(WebUIError::JsonSerializeError)
			}
		}
	}

	/// Get data stored in memory from set_data method
	pub fn get_data<T>(key: String) -> Result<T, WebUIError>
		where T: for<'a> serde::Deserialize<'a>
	{
		let json = get_global_data(key);
		if json.is_empty() {
			return Err(WebUIError::MissingData);
		}
		let data = serde_json::from_str::<T>(&json);
		match data {
			Ok(value) => Ok(value),
			Err(error) => {
				jslog!("Error in GlobalData.get_data: {:?}", error);
				Err(WebUIError::JsonParseError)
			}
		}
	}
}