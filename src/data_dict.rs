#[derive(Debug, Clone)]
pub enum VarDictType {
    TempCentigrade(f64),
}

#[derive(Debug, Clone)]

pub struct DataTag {
    pub key: String,
    pub summary: String,
    pub description: Option<String>,
    pub default: VarDictType,
}

pub fn build_key(module: &str, name: &str) -> String {
    let mod_split: Option<&str> = module.rsplit(':').next();
    let mod_name: &str = match mod_split {
        Some(text) => text,
        None => module,
    };
    let s: String = format!("{}>{}", mod_name, name);
    s
}

pub fn strip_name(name: &str) -> String {
    let name_split: Option<&str> = name.rsplit(':').next();
    let new_name: &str = match name_split {
        Some(text) => text,
        None => name,
    };
    let s: String = new_name.to_string();
    s
}

impl std::fmt::Display for VarDictType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
