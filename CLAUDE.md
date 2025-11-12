# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Discord bot for a Kendo Discord server, built in Rust using the Serenity framework. The bot implements reaction-based role management, where users can self-assign roles by reacting to specific messages with emojis.

**Status**: Successfully migrated from Shuttle.rs to self-hosted deployment with automated CI/CD.

**Recent Session Work (2025-11-12)**:
- Created comprehensive CLAUDE.md documentation
- Updated all dependencies to latest versions (Serenity 0.12.4, Tokio 1.48.0, etc.)
- Migrated from Shuttle.rs to self-hosted deployment with dotenvy
- Created multi-stage Dockerfile for optimized builds
- Set up GitHub Actions CI/CD pipeline for automatic Docker builds
- Published to GitHub Container Registry (ghcr.io)
- Created v1.0.0 release tag
- Fixed Docker tag format issue in workflow (sha- prefix)

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

### Core Components

**Bot Event Handler** ([src/main.rs](src/main.rs))
- Implements Serenity's `EventHandler` trait
- Monitors `reaction_add` and `reaction_remove` events
- Uses helper functions to validate reactions and manage members

**Role Management System**
- Reaction-based self-service role assignment
- Configuration via constants:
  - `MONITORED_MESSAGE` - The message ID that the bot watches for reactions
  - `ANIME_ROLE_ID` - The role ID to assign/remove
  - `ANIME_ROLE_EMOJI` - The emoji users react with
- Flow: User reacts → Bot validates → Bot assigns/removes role

**Key Functions**
- `is_message_relevant_for_bot()` - Filters reactions to only process the configured message and emoji
- `get_member()` - Retrieves Discord member object from reaction data with error handling
- `reaction_add()` / `reaction_remove()` - Event handlers that assign or remove roles

### Dependencies

- **serenity (v0.12.4)** - Discord API library with cache, client, gateway, and rustls_backend features
- **tokio (v1.48.0)** - Async runtime with macros, multi-threaded runtime, and signal handling
- **tracing (v0.1.41)** - Structured logging
- **tracing-subscriber (v0.3)** - Logging subscriber for formatting and output
- **anyhow (v1.0.100)** - Error handling
- **dotenvy (v0.15.7)** - Environment variable loading from .env file

### Configuration

**Environment Variables**:
- `DISCORD_TOKEN` - Bot authentication token from Discord Developer Portal
  - For local development: Copy `.env.example` to `.env` and add your token
  - For production: Set as system environment variable

**Gateway Intents** ([src/main.rs:123-126](src/main.rs#L123-L126)):
- `GUILD_MESSAGES` - Access to message data
- `GUILD_MESSAGE_REACTIONS` - Access to reaction events
- `MESSAGE_CONTENT` - Access to message content
- `AUTO_MODERATION_CONFIGURATION` - Auto-moderation features

### Adding New Role Reactions

To add support for additional role-based reactions:
1. Add new constants for the role ID and emoji at the top of [src/main.rs](src/main.rs)
2. Extend `is_message_relevant_for_bot()` to handle multiple emoji types
3. Modify `reaction_add()` and `reaction_remove()` to map emojis to their respective role IDs
4. Consider refactoring to use a HashMap or match statement for multiple role mappings

### Logging

The bot uses the `tracing` crate for structured logging:
- `info!` - Successful role assignments/removals and bot startup
- `warn!` - Missing user/guild IDs or member lookup failures
- `error!` - Discord API errors when modifying roles

## Migration Notes

### Shuttle.rs to Self-Hosted Migration (Completed)

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
