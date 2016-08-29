use url::form_urlencoded;
use std::collections::HashMap;
use url::Url;

pub mod headers;

pub struct Params {
    map : HashMap<String, Vec<String>>
}

impl Params {
    pub fn new(url: &Url) -> Params {
        let mut map : HashMap<String, Vec<String>> = HashMap::new();

        for (key, value) in url.query_pairs().into_owned() {
            if !map.contains_key(&key) {
                map.insert(String::from(key.clone()), Vec::new());
            } else {

            }

            map.get_mut(&key).unwrap().push(value);
        }

        return Params {
            map : map
        }
    }

    pub fn get_first_param(&self, name: &String) -> Option<String> {
        let params = self.map.get(name);

        params.and_then(|x| {
           if x.is_empty() {
               None
           } else {
               Some(x.first().unwrap().clone())
           }
        })
    }
}

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
