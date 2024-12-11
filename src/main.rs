use std::env;
use std::fs::{File, remove_file, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write as _};
use std::path::PathBuf;

mod commands;
use commands::{Cli, Commands, match_command};

mod task;
use task::{Task, read_task, find_lowest_available_id, write_task, update_task, delete_task, list_tasks};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <command> <file_name> [content]", args[0]);
        return Ok(());
    }

    let command = &args[1];
    let file_name = PathBuf::from(&args[2]);
    let content = if args.len() > 3 {
        args[3..].join(" ")
    } else {
        String::new()
    };

    let cli: Cli = match_command(command, file_name, content);

    match cli.command {
        Commands::Create { file_name, content } => {
            match File::open(&file_name) {
                Ok(_) => {
                    let mut file = match OpenOptions::new()
                    .read(true)
                    .write(true)
                    .create(true)
                    .open(&file_name) {
                        Ok(file) => file,
                        Err(why) => panic!("couldn't open {:?}: {}", file_name, why),
                    };

                    let id = match find_lowest_available_id(&mut file) {
                        Ok(id) => id,
                        Err(why) => panic!("couldn't find lowest available id: {}", why),
                    };
            
                    let task = Task::new(id, content);
            
                    match write_task(&mut file, task) {
                        Ok(_) => print!("Task {:?} created successfully", file_name),
                        Err(why) => panic!("couldn't write to {:?}: {}", file_name, why),
                    }
                },
                Err(why) => panic!("couldn't create {:?}: {}", file_name, why),
            }
            
            
        }
        Commands::Read { file_name, content } => {
            match File::open(&file_name) {
                Ok(mut file) => {
                    let mut contents = String::new();

                    match file.read_to_string(&mut contents) {
                        Ok(_) => {
                            print!("Task: {:?}", read_task(&mut file, content.parse().unwrap()));
                        },
                        Err(why) => panic!("couldn't read {:?}: {}", file_name, why),
                    }
                },
                Err(why) => panic!("couldn't open {:?}: {}", file_name, why),
            }
        }
        Commands::Update { file_name, content } => {
            match OpenOptions::new()
                .read(true)
                .write(true)
                .open(&file_name) {
                Ok(mut file) => {
                    let properties: Vec<&str> = content.split(" ").collect();
                    match update_task(
                        &mut file,
                        properties[0].parse().unwrap(),
                        Some("description".to_owned()),
                        Some(properties[1].to_string())
                    ) {
                    Ok(_) => print!("File {:?} updated successfully", file_name),
                        Err(why) => panic!("couldn't write to {:?}: {}", file_name, why),
                    }
                },
                Err(why) => panic!("couldn't open {:?}: {}", file_name, why),
                
            }
        }
        Commands::Delete { file_name, content } => {
            match OpenOptions::new()
                .read(true)
                .write(true)
                .open(&file_name) {
                Ok(mut file) => {
                    match delete_task(&mut file, content.parse().unwrap()) {
                        Ok(_) => (),
                        Err(why) => panic!("couldn't delete {:?}: {}", file_name, why),
                    }
                },
                Err(why) => panic!("couldn't open {:?}: {}", file_name, why),
            }
        }
        Commands::UpdateStatus { file_name, content } => {
            match OpenOptions::new()
            .read(true)
                .write(true)
                .open(&file_name) {
                Ok(mut file) => {
                    let properties: Vec<&str> = content.split(" ").collect();
                    let task_id = properties[0].parse().unwrap();
                    let status = properties[1].to_string();

                    match update_task(
                        &mut file, task_id,
                        Some("status".to_owned()),
                        Some(status)
                    ) {
                    Ok(_) => print!("Task {:?} updated successfully", file_name),
                        Err(why) => panic!("couldn't update {:?}: {}", file_name, why),
                    }
                },
                Err(why) => panic!("couldn't open {:?}: {}", file_name, why),
            }
        }
        Commands::CreateFile { file_name } => {
            match File::create(&file_name) {
                Ok(_) => {
                    let mut file = OpenOptions::new().write(true).open(&file_name).unwrap();
                    file.seek(SeekFrom::Start(0))?;
                    writeln!(file, "{}", "[]")?;
                
                    print!("File {:?} created successfully", file_name)
                },
                Err(why) => panic!("couldn't create {:?}: {}", file_name, why),
            }
        }
        Commands::ReadFile { file_name } => {
            match File::open(&file_name) {
                Ok(mut file) => {
                    match list_tasks(&mut file) {
                        Ok(_) => (),
                        Err(why) => panic!("couldn't read {:?}: {}", file_name, why),
                        
                    }
                },
                Err(why) => panic!("couldn't open {:?}: {}", file_name, why),
            }
        }
        Commands::DeleteFile { file_name } => {
            match remove_file(&file_name) {
                Ok(_) => print!("File {:?} deleted successfully", file_name),
                Err(why) => panic!("couldn't delete {:?}: {}", file_name, why),
            }
        }
       
    }

    Ok(())
}
