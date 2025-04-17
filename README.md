# HIIT - High-Intensity Interval Training

A web application for managing and tracking high-intensity interval training
workouts. Built with Rust and Leptos, deployed on CloudFlare Workers.

## Project Overview

This application provides a minimalistic, responsive interface for executing
HIIT workouts. It's built as a full-stack Rust application using:

- Leptos framework for reactive UI components
- Rust compiled to WebAssembly for client-side logic
- CloudFlare Workers for serverless deployment
- Tailwind CSS for styling
- Nix for reproducible development environments

## Installation

### Installing Nix

Determinate Systems has the best Nix:

```bash
# Install Nix using Determinate Systems installer
curl -fsSL https://install.determinate.systems/nix | sh -s -- install --determinate
```

### Cloning the Repository

```bash
git clone https://github.com/lemmih/hiit.git
cd hiit
```

## Development

### Running the Site Locally

To preview the site before deployment:

```bash
nix run .#preview
# Visit http://localhost:8787 in your browser
```

Development is best done by running two commands at the same time:

```bash
nix run .#local-dev
```

```bash
nix run .#wrangler -- dev --env local
# Visit http://localhost:8787 in your browser
```

### Building the Project

```bash
# Create a production build
nix build .#hiit

# Create a development build with better debugging
nix build .#hiit-dev
```

## Testing

### Running Checks

```bash
# Run all checks (formatting, linting, etc.)
nix flake check
```

### Running E2E Tests

E2E tests require a local installation of Firefox.

```bash
nix run .#e2e
```

## Deployment

### CloudFlare Deployment

The application is configured to deploy to CloudFlare Workers. 

#### Prerequisites for Deployment

- CloudFlare account with Workers enabled
- Wrangler CLI authenticated with your CloudFlare account

#### Custom Deployment

To deploy to your own CloudFlare account:

1. Update the `wrangler.toml` file with your domain and route information:

```toml
routes = [
  { pattern = "your-domain.com", zone_name = "your-domain.com", custom_domain = true },
]
```

2. Deploy using Wrangler:

```bash
# Authenticate with CloudFlare (if not already done)
nix run .#wrangler login

# Deploy to CloudFlare
nix run .#deploy
```

#### Setting Up Required Environment Variables

If your application requires environment variables (secrets, API keys, etc.):

```bash
# Set environment variables
nix run .#wrangler secret put SECRET_NAME
# Then enter your secret value when prompted
```

## License

See the [LICENSE](LICENSE) file for details.
