use std::path::PathBuf;

pub struct Cli {
    pub command: Commands,
}

pub enum Commands {
    Create {
        file_name: PathBuf,
        content: String,
    },
    Read {
        file_name: PathBuf,
        content: String,
    },
    Update {
        file_name: PathBuf,
        content: String,
    },
    Delete {
        file_name: PathBuf,
        content: String,
    },
    UpdateStatus {
        file_name: PathBuf,
        content: String,
    },
    CreateFile {
        file_name: PathBuf,
    },
    ReadFile {
        file_name: PathBuf,
    },
}

pub fn match_command(command: &str, file_name: PathBuf, content: String) -> Cli {
    match command {
        "create" => Cli {
            command: Commands::Create {
                file_name,
                content,
            },
        },
        "read" => Cli {
            command: Commands::Read {
                file_name,
                content,
            },
        },
        "update" => Cli {
            command: Commands::Update {
                file_name,
                content,
            },
        },
        "delete" => Cli {
            command: Commands::Delete {
                file_name,
                content,
            },
        },
        "update-status" => Cli {
            command: Commands::UpdateStatus {
                file_name,
                content,
            },
        },
        "create-file" => Cli {
            command: Commands::CreateFile {
                file_name,
            },
        },
        "read-file" => Cli {
            command: Commands::ReadFile {
                file_name,
            },
        },
        _ => {
            eprintln!("Invalid command {}", command);
            std::process::exit(1);
        },
    }
}
