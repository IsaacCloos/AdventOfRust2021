use std::{fmt::Display, fs};

pub struct CaveMap {
    pub cave_connections: Vec<CaveConnection>,
    pub path_options: Vec<Vec<String>>,
    pub small_caves_visited_twice: Vec<String>
}

impl From<&str> for CaveMap {
    fn from(input_path: &str) -> Self {
        let connections = fs::read_to_string(input_path)
            .expect("Failed to read file from path")
            .trim()
            .split_whitespace()
            .map(|s| s.split("-").map(String::from).collect::<Vec<_>>())
            .map(|x| CaveConnection {
                start: x[0].to_string(),
                end: x[1].to_string(),
            })
            .collect::<Vec<CaveConnection>>();

        CaveMap {
            cave_connections: connections,
            path_options: vec![],
            small_caves_visited_twice: vec!["start".to_string(), "end".to_string()]
        }
    }
}

pub struct CaveConnection {
    pub start: String,
    pub end: String,
}

impl Display for CaveConnection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}", &self.start, &self.end)
    }
}

#[derive(Clone)]
pub struct CaveDiver {
    pub position: String,
    pub path: Vec<String>,
}

impl CaveDiver {
    /// defaults with "start" position
    pub fn new() -> CaveDiver {
        CaveDiver {
            position: "start".to_string(),
            path: vec!["start".to_string()],
        }
    }

    pub fn scan(&self, cave_map: &mut CaveMap) {
        let options = &self.what_are_my_options(cave_map);

        if options.len() > 0 {
            for option in options {
                let cave_diver = &mut self.clone();
                cave_diver.position = option.to_string();
                cave_diver.path.push(option.to_string());
                cave_diver.scan(cave_map);
            }
            
        } else if &self.position == "end" {
            cave_map.path_options.push(self.path.clone());
        }
    }

    fn what_are_my_options(&self, cave_map: &mut CaveMap) -> Vec<String> {
        cave_map
            .cave_connections
            .iter()
            .map(|conn| {
                if &self.position == "end" {
                    return String::new()
                }
                let mut found_string = String::new();
                if &self.position == &conn.start {
                    found_string = conn.end.clone()
                } else if &self.position == &conn.end {
                    found_string = conn.start.clone()
                }
                if !self.path.contains(&found_string) || found_string.to_uppercase() == found_string {
                    found_string
                } else {
                    if self.path.contains(&found_string) && found_string.to_lowercase() == found_string && !cave_map.small_caves_visited_twice.contains(&found_string) {
                        cave_map.small_caves_visited_twice.push(found_string.clone());
                        return found_string
                    }
                    String::new()
                }
            })
            .filter(|x| x.len() > 0)
            .collect()
    }
}

impl Display for CaveDiver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Cave Diver Details:\nPosition\t{}\nPath\t\t{:?}",
            &self.position, &self.path
        )
    }
}

impl From<&str> for CaveDiver {
    fn from(start: &str) -> Self {
        CaveDiver {
            position: start.to_string(),
            path: vec![start.to_string()],
        }
    }
}




