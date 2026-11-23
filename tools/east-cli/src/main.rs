use clap::{Parser, Subcommand};
use colored::Colorize;
use std::{
    env, fs,
    path::{Path, PathBuf},
    process::{Command, ExitCode},
};

#[derive(Parser, Debug)]
#[command(name = "east", version, about = "Developer toolkit for East Engine")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Display engine and project information
    Info,
    /// Validate the expected East Engine project structure
    Check,
    /// Check Rust, Cargo, and engine prerequisites
    Doctor,
    /// Run the engine
    Run {
        /// Build and run with optimizations
        #[arg(long)]
        release: bool,
    },
    /// Compile the engine
    Build {
        /// Build an optimized release executable
        #[arg(long)]
        release: bool,
    },
    /// Remove Cargo build artifacts
    Clean,
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match execute(cli.command) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{} {error}", "error:".red().bold());
            ExitCode::FAILURE
        }
    }
}

fn execute(command: Commands) -> Result<(), String> {
    let root = find_project_root()?;
    match command {
        Commands::Info => info(&root),
        Commands::Check => check(&root),
        Commands::Doctor => doctor(&root),
        Commands::Run { release } => cargo(&root, "run", release),
        Commands::Build { release } => cargo(&root, "build", release),
        Commands::Clean => cargo(&root, "clean", false),
    }
}

fn find_project_root() -> Result<PathBuf, String> {
    let start = env::current_dir().map_err(|e| format!("cannot read current directory: {e}"))?;
    for candidate in start.ancestors() {
        if is_engine_root(candidate) {
            return Ok(candidate.to_path_buf());
        }
    }
    Err("could not find an East Engine checkout (expected Cargo.toml and src/main.rs). Run this command from inside the repository.".into())
}

fn is_engine_root(path: &Path) -> bool {
    path.join("Cargo.toml").is_file()
        && path.join("src/main.rs").is_file()
        && path.join("src/renderer").is_dir()
}

fn info(root: &Path) -> Result<(), String> {
    let manifest = fs::read_to_string(root.join("Cargo.toml"))
        .map_err(|e| format!("cannot read Cargo.toml: {e}"))?;
    let package = manifest.lines()
        .find_map(|line| line.strip_prefix("name = "))
        .unwrap_or("\"unknown\"").trim_matches('"');
    let version = manifest.lines()
        .find_map(|line| line.strip_prefix("version = "))
        .unwrap_or("\"unknown\"").trim_matches('"');

    println!("{}", "East Engine".bold().cyan());
    println!("  Package:  {package}");
    println!("  Version:  {version}");
    println!("  Location: {}", root.display());
    println!("  Renderer: wgpu");
    println!("  Window:   winit");
    println!("  Language: Rust");
    Ok(())
}

fn check(root: &Path) -> Result<(), String> {
    let required = [
        "Cargo.toml", "src/main.rs", "src/app.rs", "src/renderer/mod.rs",
        "src/shader.wgsl", "src/camera.rs", "src/entity.rs", "src/particle.rs",
        "src/flow_field.rs", "assets", "assets/textures", "assets/fonts",
    ];

    println!("{}", "Checking East Engine project...".bold());
    let mut missing = Vec::new();
    for item in required {
        if root.join(item).exists() {
            println!("  {} {item}", "✓".green());
        } else {
            println!("  {} {item}", "✗".red());
            missing.push(item);
        }
    }

    if missing.is_empty() {
        println!("{}", "Project structure looks good.".green().bold());
        Ok(())
    } else {
        Err(format!("{} required path(s) missing.", missing.len()))
    }
}

fn doctor(root: &Path) -> Result<(), String> {
    println!("{}", "East Engine environment diagnostics".bold().cyan());
    let rustc = Command::new("rustc").arg("--version").output()
        .map_err(|e| format!("Rust compiler not found: {e}"))?;
    if !rustc.status.success() {
        return Err(format!("rustc failed: {}", String::from_utf8_lossy(&rustc.stderr).trim()));
    }
    println!("  {} {}", "✓ rustc".green(), String::from_utf8_lossy(&rustc.stdout).trim());

    let cargo = Command::new("cargo").arg("--version").output()
        .map_err(|e| format!("Cargo not found: {e}"))?;
    if !cargo.status.success() {
        return Err("Cargo version check failed.".into());
    }
    println!("  {} {}", "✓ cargo".green(), String::from_utf8_lossy(&cargo.stdout).trim());

    for (label, path) in [
        ("Engine manifest", "Cargo.toml"),
        ("WGSL shader", "src/shader.wgsl"),
        ("Texture assets", "assets/textures"),
        ("Font assets", "assets/fonts"),
    ] {
        if root.join(path).exists() {
            println!("  {} {label}", "✓".green());
        } else {
            println!("  {} {label} ({path})", "✗".red());
        }
    }
    println!("{}", "Diagnostics complete.".green().bold());
    Ok(())
}

fn cargo(root: &Path, action: &str, release: bool) -> Result<(), String> {
    let mut command = Command::new("cargo");
    command.arg(action);
    if release {
        command.arg("--release");
    }
    println!("{}", format!("Running: cargo {action}{}", if release { " --release" } else { "" }).bold());
    let status = command.current_dir(root).status()
        .map_err(|e| format!("failed to start Cargo: {e}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("cargo {action} failed with status {status}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identifies_engine_root() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        assert!(is_engine_root(&root));
    }

    #[test]
    fn rejects_unrelated_directory() {
        assert!(!is_engine_root(Path::new("/")));
    }
}
