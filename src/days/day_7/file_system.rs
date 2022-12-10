#[derive(Clone, Debug)]
pub struct Dir {
    pub name: String,
    pub child_dir: Vec<String>,
    pub files: Vec<(String, String)>,
    pub size: usize,
}

impl Dir {
    pub fn new(name: String) -> Self {
        Dir {
            name,
            child_dir: vec![],
            files: vec![],
            size: 0,
        }
    }
}
