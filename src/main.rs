use clap::{Parser, Subcommand};
use colored::*;
use std::collections::HashSet;
use winreg::RegKey;
use winreg::enums::*;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Listar el path
    List,
    Add {
        path: String,
    },
}

fn main() {
    let key = "PATH";
    let cli = Cli::parse();
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let env = hkcu.open_subkey_with_flags("Environment", KEY_SET_VALUE | KEY_QUERY_VALUE);
    match &cli.command {
        Commands::List => match env {
            Ok(registro) => {
                match registro.get_value::<String, _>("PATH") {
                    Ok(v) => {
                        let mut seen = HashSet::new();
                        let mut duplicates = Vec::new();
                        for (i, parte) in v.split(";").enumerate() {
                            if parte.is_empty() {
                                continue;
                            }
                            println!("{}: {}", (i + 1).to_string().blue(), parte.trim().yellow());

                            let mut ruta = parte.trim();
                            if ruta.ends_with('\\') {
                                ruta = &ruta[..ruta.len() - 1]; // quita último caracter
                            }

                            if !seen.insert(ruta) {
                                duplicates.push(parte);
                            }
                        }

                        for d in duplicates {
                            println!("Duplicados: {}", d.red());
                        }
                    }
                    Err(e) => println!("Error al obtener el valor {key}: {e}"),
                }
            }
            Err(e) => println!("couldn't interpret {key}: {e}"),
        },
        Commands::Add { path } => {
            match env {
                Ok(registro) => match registro.get_value::<String, _>("PATH") {
                    Ok(mut v) => {
                        for (_i, parte) in v.split(";").enumerate() {
                            if parte.is_empty() {
                                continue;
                            }

                            let mut ruta = parte.trim();
                            if ruta.ends_with('\\') {
                                ruta = &ruta[..ruta.len() - 1]; // quita último caracter
                            }
                            let mut new_path = path.trim();
                            if new_path.ends_with('\\') {
                                new_path = &new_path[..new_path.len() - 1];
                            }

                            if ruta.eq_ignore_ascii_case(new_path) {
                                println!("Path {} is already in the path", path.red());
                                return;
                            }
                        }
                        v.push_str(&format!(";{}", path));
                        registro.set_value("PATH", &v).unwrap();
                        println!("Path {} added to the path", path.green());
                    }
                    Err(e) => print!("E {e}"),
                },
                Err(e) => println!("Error {e}"),
            }
        }
    }
}
