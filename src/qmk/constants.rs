use glob::glob;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
struct Keycode {
    group: String,
    key: String,
    label: Option<String>,
    #[serde(default)]
    aliases: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum KeycodeEnum {
    Reset(u8),
    Delete(String),
    Detailed(Keycode)
}

#[derive(Serialize, Deserialize)]
pub struct Keycodes {
    keycodes: HashMap<String, KeycodeEnum>,
}

#[derive(Serialize, Deserialize)]
struct Range {
    define: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum RangeEnum {
    Delete(String),
    Detailed(Range)
}

#[derive(Serialize, Deserialize)]
pub struct Ranges {
    ranges: HashMap<String, RangeEnum>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum KeycodesEnum {
    Keycodes(Keycodes),
    Ranges(Ranges),
}

impl KeycodesEnum {
    pub fn print(&self) {
        match self {
            KeycodesEnum::Keycodes(k) => println!("keycodes, {} items", k.keycodes.len()),
            KeycodesEnum::Ranges(r) => println!("range, {} items", r.ranges.len()),
        }
    }

    pub fn merge(&mut self, new: KeycodesEnum) {
        match self {
            KeycodesEnum::Keycodes(k) => {
                if let KeycodesEnum::Keycodes(new_k) = new {
                    k.merge(new_k)
                } else {
                    panic!("inconsistent state");
                }
            },
            KeycodesEnum::Ranges(r) => {
                if let KeycodesEnum::Ranges(new_r) = new {
                    r.merge(new_r)
                } else {
                    panic!("inconsistent state");
                }
            },
        }
    }
}

impl Ranges {
    pub fn merge(&mut self, new: Ranges) {
        for (key, value) in new.ranges.into_iter() {
            match value {
                RangeEnum::Delete(s) => {
                    assert_eq!(s, "!delete!");
                    self.ranges.remove(&key);
                },
                RangeEnum::Detailed(d) => {
                    match self.ranges.get_mut(&key) {
                        // here, value is of type "KeycodeEnum", this can be delete, which we need to
                        // handle here, or a Keycode, which we can call `merge` on
                        Some(r) => r.merge(d),
                        None => { self.ranges.insert(key, RangeEnum::Detailed(d)); },
                    }
                }
            }
        }
    }
}

impl Keycodes {
    pub fn merge(&mut self, new: Keycodes) {
        if new.keycodes.contains_key("!reset!") {
            println!("jESET");
        }
        for (key, value) in new.keycodes.into_iter() {
            match value {
                KeycodeEnum::Delete(s) => {
                    assert_eq!(s, "!delete!");
                    self.keycodes.remove(&key);
                },
                KeycodeEnum::Reset(_) => (),
                KeycodeEnum::Detailed(d) => {
                    match self.keycodes.get_mut(&key) {
                        // here, value is of type "KeycodeEnum", this can be delete/reset, which we need to
                        // handle here, or a Keycode, which we can call `merge` on
                        //
                        // a Reset value should go with a "!reset!" key
                        // a Delete value means we want to drop the key from self
                        Some(k) => k.merge(d),
                        None => { self.keycodes.insert(key, KeycodeEnum::Detailed(d)); },
                    }
                }
            }
        }
    }
}

impl RangeEnum {
    pub fn merge(&mut self, new: Range) {
        match self {
            RangeEnum::Detailed(s) => s.merge(new),
            RangeEnum::Delete(_) => panic!("destination range should not contain a Delete value"),
        }
    }
}

impl KeycodeEnum {
    pub fn merge(&mut self, new: Keycode) {
        match self {
            KeycodeEnum::Detailed(s) => s.merge(new),
            KeycodeEnum::Reset(_) => panic!("destination keycode should not contain a Reset value"),
            KeycodeEnum::Delete(_) => panic!("destination keycode should not contain a Delete value"),
        }
    }
}

impl Range {
    pub fn merge(&mut self, new: Range) {
    }
}

impl Keycode {
    pub fn merge(&mut self, new: Keycode) {
    }
}

pub fn parse(file: &PathBuf) -> Result<KeycodesEnum, &'static str> {
    let data = match fs::read_to_string(file) {
        Ok(data) => data,
        Err(_) => return Err("Unable to read file"),
    };
    //println!("{data}");

    /*
    let mut ks = Keycodes{
        keycodes: HashMap::new(),
    };
    let k = Keycode {
        group: "quantum".to_string(),
        key: "QK_LAYER_LOCK".to_string(),
        label: "c".to_string(),
        aliases: vec!("QK_LLCK".to_string()),
    };
    ks.keycodes.insert("0x7C7B".to_string(), k);

    let j = serde_json::to_string(&ks).unwrap();
    println!("{}", j);
*/
    if data.len() == 0 {
        println!("empty file");
    }
    match serde_hjson::from_str::<KeycodesEnum>(&data) {
        //Ok(wrapper) => Ok(wrapper.keycodes),
        Ok(keycodes) => { println!("OK"); Ok(keycodes) },
        Err(_) => { println!("ERR"); Err("Unable to parse file") },
    }
    /*
    #[derive(Serialize, Deserialize)]
    struct Keycodes2 {
        keycodes: HashMap<String, String>
    }
    #[derive(Serialize, Deserialize)]
    struct Wrapper2 {
        keycodes: Keycodes2
    }
    let w : Wrapper2 = Wrapper2{
        keycodes: Keycodes2{
            keycodes: HashMap::new(),
        },
    };
    let j = serde_json::to_string(&w).unwrap();
    println!("{}", j);
    Err("Unable to parse file")
*/
}

fn append_val<T>(h: &mut HashMap<String, Vec<T>>, key: &str, val: T) {
    if !h.contains_key(key) {
        h.insert(key.to_string(), Vec::new());
    }
    h.get_mut(key).unwrap().push(val);
}

pub fn gen_file_list(base_path: &PathBuf, versions: &[&str]) -> HashMap<String, Vec<PathBuf>> {
    let mut files = HashMap::new();
    const PREFIX: &str = "keycodes_";
    let base_path = base_path.join("data").join("constants").join("keycodes");
    for version in versions {
        let path = base_path.join(format!("{PREFIX}{version}.hjson"));
        append_val(&mut files, "-", path);

        let prefix = format!("{PREFIX}{version}_");
        let path_glob = base_path.join(format!("{prefix}*.hjson"));
        let glob_iter = glob(&path_glob.to_string_lossy());
        if let Err(_) = glob_iter {
            println!("continue");
            continue;
        }
        for entry in glob_iter.unwrap().filter_map(|e| e.ok()) {
            if let Some(file_name) = entry.file_name() {
                let category = String::from(file_name.to_string_lossy().strip_prefix(&prefix).expect("missing prefix, glob bug?"));
                let category = category.strip_suffix(".hjson").expect("missing suffix, glob bug?");
                append_val(&mut files, &category, entry);
            }
        }
    }
    files
}

pub fn parse_categories(categories: &HashMap<String, Vec<PathBuf>>) {
    for file_list in categories.values() {
        for file in file_list {
            parse(file);
        }
    }
}
