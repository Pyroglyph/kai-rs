use std::fmt;

#[derive(Debug)]
pub enum Model {
    HuggingFace(String, String),
    Local(String),
}

impl fmt::Display for Model {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Model::HuggingFace(owner, model) => {
                write!(f, "{owner}/{model}")
            }
            Model::Local(path) => {
                write!(f, "{path}")
            }
        }
    }
}

impl From<&str> for Model {
    fn from(value: &str) -> Self {
        // TODO: Improve this
        if value.starts_with("./") {
            let path = value.split_at(2);
            Model::Local(String::from(path.1))
        } else {
            let values: Vec<&str> = value.split('/').collect();
            Model::HuggingFace(String::from(values[0]), String::from(values[1]))
        }
    }
}
impl From<String> for Model {
    fn from(value: String) -> Self {
        Model::from(value.as_str())
    }
}

impl serde::Serialize for Model {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
