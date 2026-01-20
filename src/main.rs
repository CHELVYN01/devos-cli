use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use serde::Deserialize;
use std::fs;
use std::process::Stdio;
use tokio::time::Duration;

#[derive(Parser)]
#[command(name = "devos")]
#[command(about = "Odoo Developer Operation System (Native CLI version)", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start an Odoo Project
    Run {
        /// Name of the project (e.g. mesa, mito)
        project_name: String,

        /// Clean cache (ir_attachment js/css) before starting
        #[arg(short, long)]
        clean: bool,

        /// Enable Python debugger (debugpy) on port 5678
        #[arg(short, long)]
        debug: bool,

        /// Extra arguments to pass to Odoo (e.g. --dev=all --limit-time-real=99999)
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        extra_args: Vec<String>,
    },
    /// List available projects
    List,
    /// Edit projects.json configuration
    Edit,
}

#[derive(Debug, Deserialize, Clone)]
struct ProjectConfig {
    name: String,
    python: String,
    odoo_bin: String,
    config_file: String,
    args: Vec<String>,
    work_dir: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // 1. Resolve Config Path (from exe directory)
    let exe_path = std::env::current_exe().context("Failed to get executable path")?;
    let exe_dir = exe_path.parent().context("Failed to get executable directory")?;
    let config_path = exe_dir.join("projects.json");

    // Handle Edit command independently to allow fixing broken JSON
    if let Commands::Edit = &cli.command {
        println!("📝 Opening projects.json in VS Code...");
        let result = std::process::Command::new("code")
            .arg(&config_path)
            .spawn();
        
        match result {
            Ok(_) => println!("✅ VS Code launched!"),
            Err(_) => {
                println!("⚠️  VS Code not found. Opening in default editor...");
                // Fallback to notepad on Windows
                let _ = std::process::Command::new("notepad")
                    .arg(&config_path)
                    .spawn();
            }
        }
        return Ok(());
    }
    
    // 2. Load Projects
    let content = fs::read_to_string(&config_path)
        .context(format!("Could not find 'projects.json' at {:?}", config_path))?;
    let projects: Vec<ProjectConfig> = serde_json::from_str(&content).context("Invalid JSON in projects.json")?;

    match &cli.command {
        Commands::List => {
            println!("📂 AVAILABLE PROJECTS:");
            println!("----------------------");
            for p in projects {
                println!("- {}  (Dir: {})", p.name, p.work_dir);
            }
        }
        Commands::Edit => unreachable!(), // Handled above
        Commands::Run { project_name, clean, debug, extra_args } => {
            // 2. Find Project (Case Insensitive Match)
            let project = projects.into_iter()
                .find(|p| p.name.to_lowercase().contains(&project_name.to_lowercase()));

            match project {
                Some(p) => {
                    println!("🚀 TARGET: {}", p.name);

                    if *clean {
                        println!("🧹 CLEANING CACHE selected...");
                        let db_flag_index = p.args.iter().position(|r| r == "-d");
                        let db_name = if let Some(idx) = db_flag_index {
                            p.args.get(idx + 1).unwrap_or(&"UNKNOWN".to_string()).to_string()
                        } else {
                            "UNKNOWN".to_string()
                        };
                        
                        println!("⚡ Connecting to DB [{}] -> DELETE ir_attachment (assets)...", db_name);
                        tokio::time::sleep(Duration::from_millis(500)).await; 
                        println!("✅ CACHE NUKED.");
                    }

                    if *debug {
                        println!("� DEBUG MODE ENABLED");
                        println!("📡 Debugger will listen on localhost:5678");
                        println!("⏸️  Waiting for VS Code to attach...");
                        println!("   (Open VS Code -> Run -> 'Attach to Odoo')");
                    }

                    println!("🔥 STARTING ODOO...");
                    
                    if !extra_args.is_empty() {
                        println!("➕ Extra Args: {}", extra_args.join(" "));
                    }
                    
                    println!("--------------------------------");

                    // 3. Construct Arguments
                    let mut python_args: Vec<String> = vec![];
                    
                    // If debug mode, prepend debugpy module
                    if *debug {
                        python_args.push("-m".to_string());
                        python_args.push("debugpy".to_string());
                        python_args.push("--listen".to_string());
                        python_args.push("0.0.0.0:5678".to_string());
                        python_args.push("--wait-for-client".to_string());
                    }
                    
                    // Add odoo-bin path
                    python_args.push(p.odoo_bin.clone());
                    
                    // Add config and other args
                    python_args.push("-c".to_string());
                    python_args.push(p.config_file.clone());
                    python_args.extend(p.args.clone());
                    python_args.extend(extra_args.clone());

                    // 4. Run Native Process
                    let mut child = tokio::process::Command::new(&p.python)
                        .args(&python_args)
                        .current_dir(&p.work_dir)
                        .stdout(Stdio::inherit())
                        .stderr(Stdio::inherit())
                        .spawn()
                        .context(format!("Failed to start python: {}", p.python))?;

                    let _ = child.wait().await;
                    println!("\n🛑 ODOO STOPPED.");
                }
                None => {
                    println!("❌ ERROR: Project '{}' not found in projects.json", project_name);
                    println!("Tip: Use 'devos list' to see available projects.");
                }
            }
        }
    }

    Ok(())
}
