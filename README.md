# Scheduler

Run commands on a schedule that handles computer sleep.

## How It Works

Every 5 minutes, checks and runs commands that are due based on time windows.

```
schedule.toml        → Define commands & time windows
scheduler program    → Checks which are due, runs them
.state.json         → Remembers last run times
```

## Usage

**Interactive Mode (default)** - List tasks and pick one to run:
```bash
./target/release/scheduler
```

**Auto Mode** - Run all due tasks (for launchd):
```bash
./target/release/scheduler -a
```

## Setup

```bash
# 1. Build
cargo build --release

# 2. Create your schedule config
cp schedule.toml.example schedule.toml
# Edit schedule.toml with your tasks

# 3. Install (uses -a for auto mode)
cp com.user.scheduler.plist ~/Library/LaunchAgents/
launchctl load ~/Library/LaunchAgents/com.user.scheduler.plist
```

## Logs

```bash
tail -f ~/Library/Logs/scheduler.log
```

## Note

Works with ANY command (rsync, scripts, whatever).