use url::form_urlencoded;
use std::collections::HashMap;

pub mod headers;

pub fn map_params(params: form_urlencoded::Parse) -> HashMap<String, Vec<String>> {
    let mut map : HashMap<String, Vec<String>> = HashMap::new();

    for (borrowed_key, value) in params.into_owned() {
        if !map.contains_key(&borrowed_key) {
            map.insert(borrowed_key.clone(), Vec::new());
        } else {
            // Do nothing
        }

        map.get_mut(&borrowed_key).unwrap().push(value.to_string());
    }

    return map;
}
