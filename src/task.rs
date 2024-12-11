use std::collections::HashMap;
use std::fs::{File};
use std::io::{Read, Write, Seek, SeekFrom};
use std::time::{SystemTime, UNIX_EPOCH};

macro_rules! var_name_and_value {
    ($name:ident) => {
        (stringify!($name), $name)
    };
}

#[derive(Debug)]
pub struct Task {
    id: u64,
    description: String,
    status: String,
    created_at: String,
    updated_at: String,
}

impl Task {
    pub fn new(id: u64, description: String) -> Task {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        Self {
            id,
            description,
            status: "todo".to_string(),
            created_at: now.to_string(),
            updated_at: now.to_string(),
        }
    }

    pub fn to_json(&self) -> String {
        format!(
            "{{\"id\":{},\"description\":\"{}\",\"status\":\"{}\",\"created_at\":{},\"updated_at\":{}}}",
            self.id, self.description, self.status, self.created_at, self.updated_at
        )
    }

    pub fn from_json(json: &str) -> Option<Self> {
        let parts: Vec<&str> = json
            .trim_matches(|c| c == '{' || c == '}')
            .split(",")
            .collect();

        let mut map = HashMap::new();
        for part in parts {
            let key_value_pair: Vec<&str> = part.split(':').collect();
            if key_value_pair.len() == 2 {
                let [key, value] = key_value_pair.try_into().unwrap();
                map.insert(
                    key.trim_matches('"').to_string(),
                    value.trim_matches('"').to_string()
                );
            }
        }

        Some(Self {
            id: map.get("id")?.parse().ok()?,
            description: map.get("description")?.clone(),
            status: map.get("status")?.clone(),
            created_at: map.get("created_at")?.parse().ok()?,
            updated_at: map.get("updated_at")?.parse().ok()?,
        })
    }

    pub fn update(&mut self, key: Option<String>, value: Option<String>) {
        if let Some(val) = value {
            match key.as_deref() {
                Some("description") => self.description = val,
                Some("status") => self.status = val,
                _ => (),
            }
        }

        self.updated_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .to_string();
    }
}

fn get_file_items(file: &mut File) -> Result<Vec<String>, std::io::Error> {
    file.seek(SeekFrom::Start(0))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut content = String::new();
    if let (Some(start), Some(end)) = (contents.find('['), contents.rfind(']')) {
        let json_content = &contents[start..=end];

        if json_content.starts_with('[') && json_content.ends_with(']') {
            content = json_content.trim_matches(|c| c == '[' || c == ']').to_owned();
        } else {
            println!("Extracted content is not a valid JSON array.");
        }
    } else {
        println!("No valid JSON array found in the file.");
    }

    let items: Vec<String> = content
            .split("},")
            .filter(|item| item.len() > 0)
            .map(|item| {
                let mut item = item.to_string();
                if !item.ends_with('}') && !item.ends_with('\n') {
                    item.push('}');
                }

                item = item.replace("\n", "");
                item = item.replace("\t", "");
                item = item.replace(" ", "");

                item
            })
            .collect();

    Ok(items)
}

pub fn read_task(file: &mut File, task_id: u64) -> Result<Option<Task>, std::io::Error> {
    let items = get_file_items(file)?;

    for item in items {
        if let Some(task) = Task::from_json(&item) {
            if task.id == task_id {
                return Ok(Some(task));
            }
        }
    }

    Ok(None)
}

pub fn write_task(file: &mut File, task: Task) -> Result<(), std::io::Error> {
    let mut items = get_file_items(file)?;

    items.push(task.to_json());
    let new_contents = format!("[{}]", items.join(","));

    file.seek(SeekFrom::Start(0))?;
    file.set_len(0)?;
    writeln!(file, "{}", new_contents)?;

    Ok(())
}

pub fn update_task(file: &mut File, task_id: u64, field: Option<String>, value: Option<String>) -> Result<(), std::io::Error> {    
    let mut items = get_file_items(file)?;

    items = items
        .iter()
        .map(|item| {
            if let Some(mut task) = Task::from_json(item) {
                if task.id == task_id {
                    task.update(field.clone(), value.clone());
                    println!("Updating task: {:?}", task);
                }
                task.to_json()
            } else {
                item.to_string()
            }
        })
        .collect();

    let new_contents = format!("[{}]", items.join(","));

    file.seek(SeekFrom::Start(0))?;
    file.set_len(0)?;
    writeln!(file, "{}", new_contents)?;
        
    Ok(())
}

pub fn delete_task(file: &mut File, task_id: u64) -> Result<(), std::io::Error> {
    let items = get_file_items(file)?;

    let items: Vec<String> = items
        .iter()
        .filter(|item| {
            if let Some(task) = Task::from_json(item) {
                return task.id != task_id;
            }
            true
        })
        .map(|item| item.to_string())
        .collect();

    let new_contents = format!("[{}]", items.join(","));

    print!("{}", new_contents);
    file.seek(SeekFrom::Start(0))?;
    file.set_len(0)?;
    writeln!(file, "{}", new_contents)?;
    
    Ok(())
}

pub fn list_tasks(file: &mut File) -> Result<(), std::io::Error> {
    let items = get_file_items(file)?;

    items.iter().for_each(|item| {
        if let Some(task) = Task::from_json(item) {
            println!("{:?}", task);
        }
    });

    Ok(())
}

pub fn find_lowest_available_id(file: &mut File) -> Result<u64, std::io::Error> {
    let items = get_file_items(file)?;

    let mut ids = Vec::new();
    items.iter().for_each(|item| {
        if let Some(task) = Task::from_json(item) {
            ids.push(task.id);
        }
    });

    ids.sort();
    let mut id = 0;
    for i in ids {
        if i == id {
            id += 1;
        } else {
            break;
        }
    }

    Ok(id)
}
