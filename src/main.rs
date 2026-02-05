use clap::{Parser, Subcommand};
use colored::*;
use std::collections::HashSet;
use winreg::RegKey;
use winreg::enums::*;

/// Command line interface structure for the winpath utility
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

/// Available commands for the winpath utility
#[derive(Subcommand)]
enum Commands {
    /// List the PATH environment variable
    List,
    /// Add a new path to the PATH environment variable
    Add {
        /// The path to add to the PATH environment variable
        path: String,
    },
}

/// Main function that handles command line arguments and executes the appropriate command
fn main() {
    // Registry key name for the PATH environment variable
    let key = "PATH";
    // Parse command line arguments
    let cli = Cli::parse();
    // Open the HKEY_CURRENT_USER registry key
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    // Open the Environment subkey with read/write permissions
    let env = hkcu.open_subkey_with_flags("Environment", KEY_SET_VALUE | KEY_QUERY_VALUE);

    // Match on the command and execute the appropriate action
    match &cli.command {
        Commands::List => match env {
            Ok(registro) => {
                match registro.get_value::<String, _>("PATH") {
                    Ok(v) => {
                        // HashSet to track seen paths and detect duplicates
                        let mut seen = HashSet::new();
                        let mut duplicates = Vec::new();

                        // Iterate through each path in the PATH variable
                        for (i, parte) in v.split(";").enumerate() {
                            if parte.is_empty() {
                                continue;
                            }
                            // Print the path with index and colored formatting
                            println!("{}: {}", (i + 1).to_string().blue(), parte.trim().yellow());

                            // Remove trailing backslash for normalization
                            let mut ruta = parte.trim();
                            if ruta.ends_with('\\') {
                                ruta = &ruta[..ruta.len() - 1]; // remove last character
                            }

                            // Check for duplicates
                            if !seen.insert(ruta) {
                                duplicates.push(parte);
                            }
                        }

                        // Print any duplicate paths found
                        for d in duplicates {
                            println!("Duplicates: {}", d.red());
                        }
                    }
                    Err(e) => println!("Error getting {key} value: {e}"),
                }
            }
            Err(e) => println!("Couldn't interpret {key}: {e}"),
        },
        Commands::Add { path } => {
            match env {
                Ok(registro) => match registro.get_value::<String, _>("PATH") {
                    Ok(mut v) => {
                        // Check if the path already exists in the PATH variable
                        for (_i, parte) in v.split(";").enumerate() {
                            if parte.is_empty() {
                                continue;
                            }

                            // Normalize existing path by removing trailing backslash
                            let mut ruta = parte.trim();
                            if ruta.ends_with('\\') {
                                ruta = &ruta[..ruta.len() - 1]; // remove last character
                            }
                            // Normalize new path by removing trailing backslash
                            let mut new_path = path.trim();
                            if new_path.ends_with('\\') {
                                new_path = &new_path[..new_path.len() - 1];
                            }

                            // Check if the path already exists (case-insensitive)
                            if ruta.eq_ignore_ascii_case(new_path) {
                                println!("Path {} is already in the path", path.red());
                                return;
                            }
                        }
                        // Add the new path to the PATH variable
                        v.push_str(&format!(";{}", path));
                        registro.set_value("PATH", &v).unwrap();
                        println!("Path {} added to the path", path.green());
                    }
                    Err(e) => print!("Error {e}"),
                },
                Err(e) => println!("Error {e}"),
            }
        }
    }
}
