# Kendo Discord Bot

A Discord bot for the Kendo Discord server that implements reaction-based role management. Users can self-assign roles by reacting to specific messages with emojis.

## Features

- ✅ Reaction-based role assignment
- ✅ Automatic role removal on reaction removal
- ✅ Graceful shutdown handling
- ✅ Structured logging with tracing
- ✅ Docker support for easy deployment

## Quick Start

### Using Docker (Recommended)

1. Create a `.env` file with your Discord token:
   ```bash
   cp .env.example .env
   # Edit .env and add your DISCORD_TOKEN
   ```

2. Start the bot using docker-compose:
   ```bash
   docker-compose up -d
   ```

3. View logs:
   ```bash
   docker-compose logs -f
   ```

4. Stop the bot:
   ```bash
   docker-compose down
   ```

### Using Cargo (Local Development)

1. Create a `.env` file:
   ```bash
   cp .env.example .env
   # Edit .env and add your DISCORD_TOKEN
   ```

2. Run the bot:
   ```bash
   cargo run
   ```

## Configuration

The bot requires a `DISCORD_TOKEN` environment variable. Get your bot token from the [Discord Developer Portal](https://discord.com/developers/applications).

### Current Configuration

- **Monitored Message ID**: Set in `src/main.rs` (MONITORED_MESSAGE constant)
- **Role ID**: Set in `src/main.rs` (ANIME_ROLE_ID constant)
- **Emoji**: Set in `src/main.rs` (ANIME_ROLE_EMOJI constant)

## Building for Production

### Docker

```bash
# Build the image
docker build -t kobi-kendo-discord-bot:latest .

# Run the container
docker run -e DISCORD_TOKEN=your_token kobi-kendo-discord-bot
```

### Binary

```bash
# Build release binary
cargo build --release

# Run the binary
DISCORD_TOKEN=your_token ./target/release/kobi-kendo-discord-bot
```

## Development

- `cargo build` - Build the project
- `cargo run` - Run the bot locally
- `cargo check` - Quick check for compilation errors
- `cargo clippy` - Run the linter
- `cargo fmt` - Format the code

## License

Licensed under the MIT License. See [LICENSE](LICENSE) for details.
