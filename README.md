# Statusphere Serverless

A fully serverless ATProto application running on Cloudflare's Developer Platform.

## Introduction

Statusphere is a minimal social media app that demonstrates how to build ATProto applications without managing servers.

This implementation runs entirely on Cloudflare's edge network using Workers, KV, D1, and Durable Objects - no servers required.

**Read the full blog post**: [Serverless Statusphere](https://blog.cloudflare.com/serverless-atproto)

## Deploy it yourself

[![Deploy to Cloudflare](https://deploy.workers.cloudflare.com/button)](https://deploy.workers.cloudflare.com/?url=https%3A%2F%2Fgithub.com%2Finanna-malick%2Fstatusphere-serverless%2Ftree%2Fmain%2Fworker)

## Architecture Overview

The application leverages Cloudflare's serverless platform:

- **Workers**: Deploy code globally in milliseconds, handle HTTP requests and scheduled tasks
- **KV**: Cache frequently-accessed data like DIDs and session state with low latency
- **D1**: Store status updates in a distributed SQL database
- **Durable Objects**: Manage WebSocket connections for real-time updates

## Project Structure

### Core Components

- [`worker/src/lib.rs`](worker/src/lib.rs) - Main Worker entry point and request routing
- [`worker/src/frontend_worker/`](worker/src/frontend_worker/) - HTTP endpoint handlers
- [`worker/src/services/oauth.rs`](worker/src/services/oauth.rs) - ATProto OAuth implementation
- [`worker/src/services/agent.rs`](worker/src/services/agent.rs) - Authenticated ATProto operations
- [`worker/src/durable_object/`](worker/src/durable_object/) - WebSocket server for real-time updates
- [`worker/src/services/jetstream.rs`](worker/src/services/jetstream.rs) - Network event stream integration
- [`worker/src/storage/`](worker/src/storage/) - KV and D1 database abstractions

### Database Schema

- [`worker/migrations/`](worker/migrations/) - D1 database migrations for status storage

### Configuration

- [`worker/wrangler.toml`](worker/wrangler.toml) - Cloudflare Workers configuration
- [`worker/lexicons/status.json`](worker/lexicons/status.json) - ATProto schema for status records

### Optional Components

- [`firehose_listener/`](firehose_listener/) - External process for real-time Jetstream updates

## Learn More

- **Blog Post**: [Serverless Statusphere](https://blog.cloudflare.com/serverless-atproto)
- **ATProto Documentation**: [https://atproto.com/docs](https://atproto.com/docs)
- **Cloudflare Workers**: [https://developers.cloudflare.com/workers/](https://developers.cloudflare.com/workers/)
- **Bluesky**: [https://bsky.social](https://bsky.social)

## Attribution

This implementation was built using [Bailey Townsend's Rust Statusphere example](https://github.com/fatfingers23/rusty_statusphere_example_app) as a reference.
