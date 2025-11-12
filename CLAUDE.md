# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Discord bot for a Kendo Discord server, built in Rust using the Serenity framework. The bot implements reaction-based role management, where users can self-assign roles by reacting to specific messages with emojis.

**Status**: Successfully migrated from Shuttle.rs to self-hosted deployment.

## Commands

### Development
- `cargo build` - Build the project
- `cargo run` - Run the bot locally (requires `.env` file with `DISCORD_TOKEN`)
- `cargo check` - Quick check for compilation errors without building
- `cargo clippy` - Run the linter for Rust best practices
- `cargo fmt` - Format the code

### Deployment (Self-Hosted)
- Build: `cargo build --release`
- Run: `./target/release/kobi-kendo-discord-bot` (requires `DISCORD_TOKEN` environment variable)
- Recommended: Use systemd service or Docker container for production
- Ensure the bot has network access to Discord's API endpoints

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
