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

The bot requires configuration through environment variables:

### Required
- `DISCORD_TOKEN` - Get your bot token from the [Discord Developer Portal](https://discord.com/developers/applications)

### Optional
- `OTLP_ENDPOINT` - OpenTelemetry OTLP endpoint for traces and logs (e.g., `http://localhost:4317`)
- `OTLP_HEADERS` - Authentication headers for OTLP (format: `key1=value1,key2=value2`)
- `ENVIRONMENT` - Environment label for traces and logs (defaults to `production`)

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

## Logging and Observability

The bot supports both console logging and distributed tracing via OpenTelemetry:

- **Console Logging**: Always enabled, writes to stdout
- **OpenTelemetry Integration**: Optional, enable by setting `OTLP_ENDPOINT` environment variable

### Setting up OpenTelemetry

To enable distributed tracing and logging with OpenTelemetry:

**For Grafana Cloud:**

1. Get your Grafana Cloud OTLP credentials:
   - Go to your Grafana Cloud dashboard
   - Navigate to "Connections" → "Add new connection" → "OpenTelemetry"
   - Find your OTLP endpoint (e.g., `https://otlp-gateway-prod-XXX.grafana.net/otlp`)
   - Create an access token under "Security" → "Access Policies" or use existing credentials
   - Encode your credentials: `echo -n "instance_id:token" | base64`

2. Set the environment variables:
   ```bash
   # In .env file
   OTLP_ENDPOINT=https://otlp-gateway-prod-XXX.grafana.net/otlp
   OTLP_HEADERS=Authorization=Basic <base64_encoded_credentials>
   ENVIRONMENT=production
   ```

**For Honeycomb:**

```bash
# In .env file
OTLP_ENDPOINT=https://api.honeycomb.io:443
OTLP_HEADERS=x-honeycomb-team=your_api_key
ENVIRONMENT=production
```

**For Local Development (Jaeger):**

1. Start Jaeger with OTLP support:
   ```bash
   docker run -d --name jaeger \
     -p 4317:4317 \
     -p 16686:16686 \
     jaegertracing/all-in-one:latest
   ```

2. Configure the bot:
   ```bash
   # In .env file
   OTLP_ENDPOINT=http://localhost:4317
   ENVIRONMENT=development
   ```

3. View traces at [http://localhost:16686](http://localhost:16686)

If OpenTelemetry is not configured or unreachable, logs continue to be written to console normally.

## License

Licensed under the MIT License. See [LICENSE](LICENSE) for details.
