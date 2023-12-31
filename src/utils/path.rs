#[derive(Clone, Debug)]
pub struct Path {
    pub path: Vec<String>,
}

impl Path {
    pub fn new(path: &str) -> Self {
        let mut ret = Path {
            path: path
                .replace('\\', "/")
                .split('/')
                .map(|s| s.to_string())
                .collect(),
        };
        ret.tidy();
        ret
    }

    pub fn parent(&self) -> Path {
        let mut path = self.path.clone();
        path.pop();
        Path { path }
    }

    pub fn tidy(&mut self) {
        let mut i = 0;
        while i < self.path.len() {
            if self.path[i] == "." {
                self.path.remove(i);
            } else if self.path[i] == ".." {
                if i > 0 {
                    self.path.remove(i);
                    self.path.remove(i - 1);
                    i -= 1;
                } else {
                    i += 1;
                }
            } else {
                i += 1;
            }
        }
    }

    pub fn relative(&self, other: &Path) -> Path {
        let mut path = self.path.clone();
        path.extend(other.path.clone());
        Path { path }
    }

    pub fn relative_to(&self, other: &Path) -> Path {
        let mut path = Vec::new();
        let mut i = 0;
        while i < self.path.len() && i < other.path.len() {
            if self.path[i] != other.path[i] {
                break;
            }
            i += 1;
        }
        for _ in 0..(other.path.len() - i) {
            path.push("..".to_string());
        }
        for j in i..self.path.len() {
            path.push(self.path[j].clone());
        }
        Path { path }
    }

    pub fn as_std_path(&self) -> std::path::PathBuf {
        let mut path = std::path::PathBuf::from("/");
        for p in &self.path {
            if p.ends_with(":") {
                path.push(p.to_owned() + "\\");
            } else {
                path.push(p);
            }
        }
        path
    }
}

impl ToString for Path {
    fn to_string(&self) -> String {
        self.path.join("/")
    }
}
