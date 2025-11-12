# Kendo Discord Bot

A Discord bot for the Kendo Discord server that implements reaction-based role management. Users can self-assign roles by reacting to specific messages with emojis.

## Features

- ✅ Reaction-based role assignment
- ✅ Automatic role removal on reaction removal
- ✅ Graceful shutdown handling
- ✅ Structured logging with tracing
- ✅ Docker support for easy deployment

## Quick Start

### Using Pre-built Docker Image (Easiest)

Pull and run the pre-built image from GitHub Container Registry:

```bash
# Pull the latest image
docker pull ghcr.io/klemenkobau/discord-bot:latest

# Run the container
docker run -d \
  --name kobi-kendo-discord-bot \
  --restart unless-stopped \
  -e DISCORD_TOKEN=your_discord_token_here \
  ghcr.io/klemenkobau/discord-bot:latest
```

To use a specific version, replace `latest` with the version tag (e.g., `v1.0.0`).

### Using Docker Compose

1. Create a `.env` file with your Discord token:
   ```bash
   cp .env.example .env
   # Edit .env and add your DISCORD_TOKEN
   ```

2. Update `docker-compose.yml` to use the pre-built image:
   ```yaml
   version: '3.8'
   services:
     discord-bot:
       image: ghcr.io/klemenkobau/discord-bot:latest
       container_name: kobi-kendo-discord-bot
       restart: unless-stopped
       environment:
         - DISCORD_TOKEN=${DISCORD_TOKEN}
   ```

3. Start the bot:
   ```bash
   docker-compose up -d
   ```

4. View logs:
   ```bash
   docker-compose logs -f
   ```

5. Stop the bot:
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

### Using Pre-built Images

The easiest way to deploy is using the pre-built images from GitHub Container Registry:

```bash
# Pull the latest image
docker pull ghcr.io/klemenkobau/discord-bot:latest

# Run it
docker run -d \
  --name kobi-kendo-discord-bot \
  --restart unless-stopped \
  -e DISCORD_TOKEN=your_token \
  ghcr.io/klemenkobau/discord-bot:latest
```

**Available Images:**
- `ghcr.io/klemenkobau/discord-bot:latest` - Latest build from main branch
- `ghcr.io/klemenkobau/discord-bot:v1.0.0` - Specific version tags
- `ghcr.io/klemenkobau/discord-bot:main-<sha>` - Specific commit builds

### Building Your Own Docker Image

If you want to build from source:

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
