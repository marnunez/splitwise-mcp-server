# Deployment Instructions for Claude

## Deploying to simpleserver

The application is deployed on the VPS (simpleserver) using Docker Compose.

### Quick Deploy Command

After GitHub Actions completes the build:

```bash
ssh simpleserver 'cd /opt/splitwise-mcp-server && docker-compose pull && docker-compose down && docker-compose up -d && docker network connect proxy splitwise-mcp'
```

### Step-by-Step Deployment

1. **Wait for GitHub Actions to complete:**
   ```bash
   cd ~/claude-assistant/splitwise-mcp-server
   gh run list --limit 1
   # Get the run ID and watch it:
   gh run watch <RUN_ID> --exit-status
   ```

2. **SSH into the server:**
   ```bash
   ssh simpleserver
   ```

3. **Navigate to the project directory:**
   ```bash
   cd /opt/splitwise-mcp-server
   ```

4. **Pull the latest Docker image:**
   ```bash
   docker-compose pull
   ```

5. **Restart the container:**
   ```bash
   docker-compose down
   docker-compose up -d
   ```

6. **Connect to proxy network (for Traefik):**
   ```bash
   docker network connect proxy splitwise-mcp
   ```

### Important Notes

- The server uses Docker Compose, NOT plain Docker commands
- The project is located in `/opt/splitwise-mcp-server`
- Environment files (.env and .env.oauth) are already on the server
- The container needs to be connected to the `proxy` network for Traefik reverse proxy
- The service is accessible at: https://splitwise.morenomcps.duckdns.org/mcp
- Authorization header uses the Bearer token from .env.oauth

### Checking Status

```bash
ssh simpleserver 'docker ps | grep splitwise'
```

### Viewing Logs

```bash
ssh simpleserver 'docker logs splitwise-mcp --tail 50'
```