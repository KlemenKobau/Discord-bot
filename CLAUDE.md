# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Discord bot for a Kendo Discord server, built in Rust using the Serenity framework. The bot implements reaction-based role management, where users can self-assign roles by reacting to specific messages with emojis.

**Status**: Successfully migrated from Shuttle.rs to self-hosted deployment with automated CI/CD.

**Recent Updates**:
- **2025-12-07**: Migrated from Loki to OpenTelemetry for observability (supports Grafana, Jaeger, Honeycomb, etc.)
- **2025-11-25**: Split monolithic codebase into clean modular architecture (config, handler, roles, logging modules)
- **2025-11-12**: Migrated from Shuttle.rs to self-hosted deployment with Docker support and CI/CD
- Set up GitHub Actions pipeline for automatic builds to GHCR (ghcr.io)
- Updated all dependencies to latest versions (Serenity 0.12.4, Tokio 1.48.0)

## Commands

### Development
- `cargo build` - Build the project
- `cargo run` - Run the bot locally (requires `.env` file with `DISCORD_TOKEN`)
- `cargo check` - Quick check for compilation errors without building
- `cargo clippy` - Run the linter for Rust best practices
- `cargo fmt` - Format the code

### Deployment

**Direct Binary Deployment:**
- Build: `cargo build --release`
- Run: `./target/release/kobi-kendo-discord-bot` (requires `DISCORD_TOKEN` environment variable)

**Docker Deployment (Recommended):**
- Build image: `docker build -t kobi-kendo-discord-bot .`
- Run container: `docker run -e DISCORD_TOKEN=your_token kobi-kendo-discord-bot`
- Or use docker-compose: `docker-compose up -d`

**Production Considerations:**
- Use systemd service for binary deployment or Docker/Docker Compose for containerized deployment
- Ensure the bot has network access to Discord's API endpoints
- Consider implementing health checks and automatic restarts
- Monitor logs with `docker logs -f kobi-kendo-discord-bot` or systemd journal

## Architecture

The codebase follows Rust best practices with a modular structure separating concerns into distinct modules.

### Project Structure

```
src/
├── main.rs           - Application entry point and configuration
├── lib.rs            - Library root, exports public modules
├── config.rs         - Configuration management from environment
├── handler.rs        - Discord event handler implementation
├── roles.rs          - Role management logic and utilities
└── logging.rs        - Logging initialization (console + Loki)
```

### Core Modules

**[src/main.rs](src/main.rs)** - Entry Point
- Loads configuration from environment variables
- Initializes logging system
- Sets up Discord bot with role reaction configurations
- Handles graceful shutdown on Ctrl-C

**[src/config.rs](src/config.rs)** - Configuration
- `Config` struct for all environment-based configuration
- Validates required environment variables (DISCORD_TOKEN)
- Handles optional OpenTelemetry OTLP settings
- Helper methods: `has_otlp()`, `parse_otlp_headers()`

**[src/handler.rs](src/handler.rs)** - Event Handler
- `BotHandler` implements Serenity's `EventHandler` trait
- Monitors `reaction_add` and `reaction_remove` events
- Supports multiple role reaction configurations
- Automatically matches reactions to configured roles

**[src/roles.rs](src/roles.rs)** - Role Management
- `RoleReaction` struct for configuring message/role/emoji mappings
- `get_member()` - Retrieves Discord member from reaction with error handling
- `add_role()` / `remove_role()` - Clean interfaces for role management
- Extensible design for adding multiple role reactions

**[src/logging.rs](src/logging.rs)** - Logging Setup
- Initializes tracing subscriber with console output
- Optional OpenTelemetry OTLP integration for distributed tracing and logging
- Supports custom headers for authentication (Grafana Cloud, Honeycomb, etc.)
- Graceful fallback to console-only if OTLP is not configured

### Role Reaction System

The bot uses a flexible role reaction system:
- Configuration via `RoleReaction` instances in [src/main.rs](src/main.rs)
- Each reaction links: message ID + emoji → role ID
- Flow: User reacts → Handler validates → Role assigned/removed
- Easy to extend for multiple roles by adding to the `role_reactions` vector

### Dependencies

- **serenity (v0.12.4)** - Discord API library with cache, client, gateway, and rustls_backend features
- **tokio (v1.48.0)** - Async runtime with macros, multi-threaded runtime, and signal handling
- **tracing (v0.1.41)** - Structured logging
- **tracing-subscriber (v0.3)** - Logging subscriber for formatting and output
- **tracing-opentelemetry (v0.28)** - OpenTelemetry integration for distributed tracing
- **opentelemetry (v0.27)** - OpenTelemetry API for traces and metrics
- **opentelemetry_sdk (v0.27)** - OpenTelemetry SDK with Tokio runtime support
- **opentelemetry-otlp (v0.27)** - OTLP exporter with gRPC support
- **tonic (v0.12)** - gRPC client library for OTLP
- **anyhow (v1.0.100)** - Error handling
- **dotenvy (v0.15.7)** - Environment variable loading from .env file

### Configuration

**Environment Variables**:
- `DISCORD_TOKEN` (required) - Bot authentication token from Discord Developer Portal
  - For local development: Copy `.env.example` to `.env` and add your token
  - For production: Set as system environment variable
- `OTLP_ENDPOINT` (optional) - OpenTelemetry OTLP endpoint URL for traces and logs
  - Local Jaeger: `http://localhost:4317`
  - Local OTEL Collector: `http://localhost:4317`
  - Grafana Cloud: `https://otlp-gateway-prod-XXX.grafana.net/otlp`
  - Honeycomb: `https://api.honeycomb.io:443`
  - If not set, logs will only be written to console/stdout
- `OTLP_HEADERS` (optional) - Authentication headers for OTLP endpoint (format: `key1=value1,key2=value2`)
  - Grafana Cloud: `Authorization=Basic <base64_instance_id:token>`
  - Honeycomb: `x-honeycomb-team=your_api_key`
  - Generic API Key: `Authorization=Bearer your_token`
- `ENVIRONMENT` (optional) - Environment label for traces and logs (defaults to `production`)
  - Common values: `development`, `staging`, `production`

**Gateway Intents** ([src/main.rs:29-32](src/main.rs#L29-L32)):
- `GUILD_MESSAGES` - Access to message data in guild channels
- `GUILD_MESSAGE_REACTIONS` - Required for monitoring reaction add/remove events
- `MESSAGE_CONTENT` - Access to message content (privileged intent - must be enabled in Discord Developer Portal)
- `AUTO_MODERATION_CONFIGURATION` - Auto-moderation features

**Note**: `MESSAGE_CONTENT` is a privileged intent. You must enable it in the Discord Developer Portal under your bot's settings (Bot → Privileged Gateway Intents).

### Adding New Role Reactions

The modular design makes adding new role reactions simple:

1. **Add constants** for the new role at the top of [src/main.rs](src/main.rs):
   ```rust
   const NEW_MESSAGE_ID: u64 = 1234567890;
   const NEW_ROLE_ID: u64 = 9876543210;
   const NEW_EMOJI: &str = "🎮";
   ```

2. **Add to role_reactions vector** in `main()`:
   ```rust
   let role_reactions = vec![
       RoleReaction::new(MONITORED_MESSAGE, ANIME_ROLE_ID, ANIME_ROLE_EMOJI),
       RoleReaction::new(NEW_MESSAGE_ID, NEW_ROLE_ID, NEW_EMOJI),
   ];
   ```

That's it! The handler automatically processes all configured role reactions. No need to modify any other code.

### Logging and Observability

The bot uses the `tracing` crate for structured logging with optional OpenTelemetry OTLP integration:

**Log Levels**:
- `info!` - Successful role assignments/removals and bot startup
- `warn!` - Missing user/guild IDs or member lookup failures
- `error!` - Discord API errors when modifying roles

**OpenTelemetry Integration** (optional):
- Set `OTLP_ENDPOINT` environment variable to enable distributed tracing and logging
- Traces and logs are sent via OTLP with resource attributes: `service.name=discord-bot`, `service.environment=<ENVIRONMENT>`
- The bot continues to log to console even when OTLP is enabled
- If OTLP endpoint is unreachable, logs still appear in console (no data loss)
- Supports multiple backends: Grafana Cloud, Jaeger, Honeycomb, Datadog, and any OTLP-compatible collector

**Example OpenTelemetry Setups**:

**Grafana Cloud:**
```bash
# In .env file
OTLP_ENDPOINT=https://otlp-gateway-prod-XXX.grafana.net/otlp
OTLP_HEADERS=Authorization=Basic <base64_instance_id:token>
ENVIRONMENT=production

# Or with Docker
docker run \
  -e DISCORD_TOKEN=xxx \
  -e OTLP_ENDPOINT=https://otlp-gateway-prod-XXX.grafana.net/otlp \
  -e OTLP_HEADERS="Authorization=Basic <base64_token>" \
  ghcr.io/klemenkobau/discord-bot:latest
```

**Honeycomb:**
```bash
# In .env file
OTLP_ENDPOINT=https://api.honeycomb.io:443
OTLP_HEADERS=x-honeycomb-team=your_api_key
ENVIRONMENT=production
```

**Local Jaeger (for development):**
```bash
# Start Jaeger with OTLP support
docker run -d --name jaeger \
  -p 4317:4317 \
  -p 16686:16686 \
  jaegertracing/all-in-one:latest

# In .env file
OTLP_ENDPOINT=http://localhost:4317
ENVIRONMENT=development

# View traces at http://localhost:16686
```

**Local OpenTelemetry Collector:**
```bash
# In .env file
OTLP_ENDPOINT=http://localhost:4317
ENVIRONMENT=production
```

## Migration Notes

### Code Refactoring to Modular Architecture (2025-11-25)

**What Changed**:
- ✅ Split monolithic `main.rs` into clean, focused modules
- ✅ Created `src/lib.rs` as library root exposing public API
- ✅ Extracted `config.rs` for environment variable management
- ✅ Extracted `handler.rs` for Discord event handling logic
- ✅ Extracted `roles.rs` for role management utilities
- ✅ Extracted `logging.rs` for logging initialization
- ✅ Improved extensibility - adding new role reactions now requires only 2 lines of code
- ✅ Better separation of concerns following Rust best practices
- ✅ Maintained backward compatibility - no configuration changes needed
- ✅ All tests passing with `cargo check` and `cargo clippy`

**Benefits**:
- **Maintainability**: Each module has a single, clear responsibility
- **Testability**: Individual modules can be unit tested in isolation
- **Extensibility**: Easy to add new features without modifying existing code
- **Readability**: Smaller files with focused functionality
- **Reusability**: Core logic can be used as a library by other projects

### Shuttle.rs to Self-Hosted Migration (2025-11-12)

**What Changed**:
- ✅ Removed Shuttle.rs platform dependencies (shuttle-serenity, shuttle-runtime, shuttle-secrets)
- ✅ Changed from Shuttle's `#[shuttle_runtime::main]` macro to standard `#[tokio::main]`
- ✅ Replaced `shuttle_secrets::SecretStore` with dotenvy for environment variable loading
- ✅ Removed GitHub Actions workflow for Shuttle deployment
- ✅ Updated all dependencies to latest versions (Serenity 0.12.4, Tokio 1.48.0, etc.)
- ✅ Added graceful shutdown handling with Ctrl-C signal support
- ✅ Implemented tracing-subscriber for better logging output

**Current Setup**:
- The bot now runs as a standalone binary
- Can be deployed to any server with Rust runtime or as a Docker container
- Uses `.env` file for local development (copy from `.env.example`)
- Includes graceful shutdown on Ctrl-C
- All tests passing with `cargo check` and `cargo clippy`

### Docker Setup

**Files**:
- [Dockerfile](Dockerfile) - Multi-stage build for optimized image size
- [.dockerignore](.dockerignore) - Excludes unnecessary files from build context
- [docker-compose.yml](docker-compose.yml) - Simplified deployment configuration

**Docker Image Details**:
- Uses multi-stage build (builder + runtime)
- Builder: rust:1.84-slim with build dependencies
- Runtime: debian:bookworm-slim with minimal dependencies
- Runs as non-root user (botuser, UID 1000)
- Final image contains only the compiled binary and runtime dependencies

**Quick Start with Docker**:
1. Create `.env` file with `DISCORD_TOKEN=your_token_here`
2. Run: `docker-compose up -d`
3. Check logs: `docker-compose logs -f`
4. Stop: `docker-compose down`

**Building and Publishing**:

The project uses GitHub Actions to automatically build and publish Docker images to GitHub Container Registry (GHCR) on every push to main and on version tags.

Manual publishing (if needed):
```bash
# Build the image
docker build -t kobi-kendo-discord-bot:latest .

# Tag for GitHub Container Registry
docker tag kobi-kendo-discord-bot:latest ghcr.io/klemenkobau/discord-bot:latest

# Login to GHCR (requires GitHub personal access token with packages:write permission)
echo $GITHUB_TOKEN | docker login ghcr.io -u USERNAME --password-stdin

# Push to registry
docker push ghcr.io/klemenkobau/discord-bot:latest
```

**Using Pre-built Images from GHCR**:
```bash
# Pull the latest image
docker pull ghcr.io/klemenkobau/discord-bot:latest

# Run it
docker run -d -e DISCORD_TOKEN=your_token ghcr.io/klemenkobau/discord-bot:latest
```

## Continuous Integration

### GitHub Actions Workflow

The project includes a CI/CD pipeline ([.github/workflows/docker-publish.yml](.github/workflows/docker-publish.yml)) that:
- Builds Docker images on every push to main
- Publishes images to GitHub Container Registry (ghcr.io)
- Creates tags for version releases (when you push a git tag like `v1.0.0`)
- Uses Docker layer caching for faster builds
- Only pushes images on main branch (not on pull requests)

**Workflow triggers:**
- Push to `main` branch → builds and pushes `latest` and `sha-<commit>` tags
- Push version tag (e.g., `v1.0.0`) → builds and pushes version-specific tags
- Pull requests → builds but doesn't push (validation only)

**Available image tags:**
- `ghcr.io/klemenkobau/discord-bot:latest` - Latest build from main
- `ghcr.io/klemenkobau/discord-bot:main` - Main branch tag
- `ghcr.io/klemenkobau/discord-bot:sha-<commit>` - Specific commit from any branch
- `ghcr.io/klemenkobau/discord-bot:v1.0.0` - Specific version (when tagged)
- `ghcr.io/klemenkobau/discord-bot:1.0` - Major.minor version
- `ghcr.io/klemenkobau/discord-bot:1` - Major version only

**Creating a release:**
```bash
# Tag a new version
git tag -a v1.0.0 -m "Release v1.0.0"
git push origin v1.0.0

# GitHub Actions will automatically build and push the versioned images
```

### Making the Container Registry Public

To allow anyone to pull Docker images without authentication:

1. **Wait for first build to complete:**
   - Go to: https://github.com/KlemenKobau/Discord-bot/actions
   - Wait for "Build and Publish Docker Image" workflow to finish

2. **Navigate to package settings:**
   - Go to: https://github.com/KlemenKobau/Discord-bot/pkgs/container/discord-bot
   - Or: GitHub profile → Packages → discord-bot

3. **Change visibility:**
   - Click "Package settings" (right side)
   - Scroll to "Danger Zone" section
   - Click "Change visibility"
   - Select "Public"
   - Confirm the change

4. **Verify public access:**
   ```bash
   # Test pulling without authentication
   docker pull ghcr.io/klemenkobau/discord-bot:latest
   ```

**Note:** Making the package public means anyone can pull the Docker images without needing GitHub authentication. This is recommended for open-source projects.
