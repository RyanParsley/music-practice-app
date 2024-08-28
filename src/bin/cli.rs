use clap::{Parser, Subcommand};
use music_practice_app::models::{PracticeSession, SheetMusic};
use music_practice_app::storage;
use uuid::Uuid;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        #[clap(short, long)]
        duration: u32,
        #[clap(short, long)]
        notes: Option<String>,
    },
    List,
    View {
        #[clap(short, long)]
        id: Uuid,
    },
    AddSheet {
        #[clap(short, long)]
        name: String,
        #[clap(short, long)]
        content: String,
    },
    ListSheets,
    ViewSheet {
        #[clap(short, long)]
        name: String,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { duration, notes } => {
            let session = PracticeSession {
                id: Uuid::new_v4(),
                date: chrono::Utc::now(),
                duration,
                notes,
            };
            storage::save_practice_session(&session)?;
            println!("Added new practice session with ID: {}", session.id);
        }
        Commands::List => {
            let sessions = storage::list_practice_sessions()?;
            for session in sessions {
                println!("ID: {}, Date: {}, Duration: {} minutes", session.id, session.date, session.duration);
            }
        }
        Commands::View { id } => {
            match storage::read_practice_session(&id) {
                Ok(session) => println!("{:#?}", session),
                Err(_) => println!("Session not found"),
            }
        }
        Commands::AddSheet { name, content } => {
            let sheet = SheetMusic { name, content };
            storage::save_sheet_music(&sheet)?;
            println!("Added new sheet music: {}", sheet.name);
        }
        Commands::ListSheets => {
            let sheets = storage::list_sheet_music()?;
            for sheet in sheets {
                println!("Name: {}", sheet);
            }
        }
        Commands::ViewSheet { name } => {
            match storage::read_sheet_music(&name) {
                Ok(sheet) => println!("{}", sheet.content),
                Err(_) => println!("Sheet music not found"),
            }
        }
    }

    Ok(())
}
