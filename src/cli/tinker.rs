use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use comfy_table::Table;
use crate::database::connection::DatabasePool;
use crate::database::model::Model;
use crate::app::models::user::User;

pub async fn run(pool: &DatabasePool) -> Result<(), Box<dyn std::error::Error>> {
    let mut rl = DefaultEditor::new()?;
    // Load history jika ada
    let _ = rl.load_history(".lumina_tinker_history");

    println!("--------------------------------------------------");
    println!("   Lumina Tinker — Interactive Console");
    println!("--------------------------------------------------");
    println!("Type 'help' for available commands.");
    println!("Type 'exit' or 'quit' to close the session.");
    println!();

    loop {
        let readline = rl.readline("lumina> ");
        match readline {
            Ok(line) => {
                let input = line.trim();
                if input.is_empty() { continue; }
                
                let _ = rl.add_history_entry(input);

                match input {
                    "exit" | "quit" => break,
                    "help" => {
                        println!("Available Commands:");
                        println!("  User::all      Fetch all records from users table");
                        println!("  User::count    Count total records in users table");
                        println!("  sql <query>    Run a raw SQL query");
                        println!("  clear          Clear the terminal screen");
                        println!("  help           Show this help message");
                        println!("  exit/quit      Exit the console");
                    },
                    "clear" => {
                        print!("{}[2J{}[1;1H", 27 as char, 27 as char);
                    },
                    "User::all" => {
                        match User::all(pool).await {
                            Ok(users) => {
                                let mut table = Table::new();
                                table.set_header(vec!["ID", "Name", "Email", "Role"]);
                                for user in users {
                                    table.add_row(vec![
                                        user.id.to_string(),
                                        user.name,
                                        user.email,
                                        user.role,
                                    ]);
                                }
                                println!("{table}");
                            },
                            Err(e) => println!("❌ Error: {}", e),
                        }
                    },
                    "User::count" => {
                        match User::all(pool).await {
                            Ok(users) => println!("📊 Total users: {}", users.len()),
                            Err(e) => println!("❌ Error: {}", e),
                        }
                    },
                    cmd if cmd.starts_with("sql ") => {
                        let query = &cmd[4..];
                        println!("🔍 Executing SQL: {}", query);
                        // Untuk raw SQL, kita bisa menggunakan sqlx secara langsung
                        match sqlx::query(query).execute(&pool.pool).await {
                            Ok(res) => println!("✅ Success! Rows affected: {}", res.rows_affected()),
                            Err(e) => println!("❌ SQL Error: {}", e),
                        }
                    },
                    _ => {
                        println!("❓ Unknown command: '{}'. Type 'help' for assistance.", input);
                    }
                }
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    let _ = rl.save_history(".lumina_tinker_history");
    println!("Bye!");
    Ok(())
}
