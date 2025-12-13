use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::process::Command;

#[derive(Deserialize)]
struct Config {
    schedule: Vec<Schedule>,
}

#[derive(Deserialize)]
struct Schedule {
    name: String,
    command: String,
    time_window_hours: u64,
}

#[derive(Serialize, Deserialize, Default)]
struct State {
    last_run: HashMap<String, u64>,
}

fn main() {
    let text = fs::read_to_string("schedule.toml").expect("Can't read schedule.toml");
    let config: Config = toml::from_str(&text).expect("Invalid TOML");
    
    let text = fs::read_to_string(".state.json").unwrap_or_default();
    let mut state: State = serde_json::from_str(&text).unwrap_or_default();
    
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    let is_auto = std::env::args().any(|a| a == "-a" || a == "--auto");

    if is_auto {
        // Auto mode: run due tasks
        for schedule in &config.schedule {
            let window_seconds = schedule.time_window_hours * 3600;
            let should_run = match state.last_run.get(&schedule.name) {
                None => true,
                Some(&last_run) => (now - last_run) >= window_seconds,
            };
            
            if should_run {
                println!("Running: {}", schedule.name);
                let success = Command::new("sh")
                    .arg("-c")
                    .arg(&schedule.command)
                    .status()
                    .map(|s| s.success())
                    .unwrap_or(false);
                
                if success {
                    println!("✓");
                    state.last_run.insert(schedule.name.clone(), now);
                }
            }
        }
    } else {
        // Interactive mode: list and pick
        for (i, schedule) in config.schedule.iter().enumerate() {
            let last = match state.last_run.get(&schedule.name) {
                None => "never".to_string(),
                Some(&t) => format!("{}h ago", (now - t) / 3600),
            };
            println!("{}. {} ({})", i + 1, schedule.name, last);
        }
        
        print!("\nTask #: ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        if let Ok(num) = input.trim().parse::<usize>() {
            if num > 0 && num <= config.schedule.len() {
                let schedule = &config.schedule[num - 1];
                println!("Running: {}", schedule.name);
                
                let success = Command::new("sh")
                    .arg("-c")
                    .arg(&schedule.command)
                    .status()
                    .map(|s| s.success())
                    .unwrap_or(false);
                
                if success {
                    println!("✓");
                    state.last_run.insert(schedule.name.clone(), now);
                }
            }
        }
    }

    let json = serde_json::to_string_pretty(&state).unwrap();
    fs::write(".state.json", json).expect("Can't write state");
}