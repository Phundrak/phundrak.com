# GitHub Actions Workflows

## Docker Image Publishing

The `publish-docker.yml` workflow automatically builds and publishes Docker images for the backend service using Nix.

### Triggers and Tagging Strategy

| Event        | Condition                   | Published Tags         | Example           |
|--------------+-----------------------------+------------------------+-------------------|
| Tag push     | Tag pushed to `main` branch | `latest` + version tag | `latest`, `1.0.0` |
| Branch push  | Push to `develop` branch    | `develop`              | `develop`         |
| Pull request | PR opened or updated        | `pr<number>`           | `pr12`            |
| Branch push  | Push to `main` (no tag)     | `latest`               | `latest`          |

### Required Secrets

Configure these secrets in your repository settings (`Settings` → `Secrets and variables` → `Actions`):

| Secret Name         | Description                                 | Example Value                           |
|---------------------+---------------------------------------------+-----------------------------------------|
| `DOCKER_USERNAME`   | Username for Docker registry authentication | `phundrak`                              |
| `DOCKER_PASSWORD`   | Password or token for Docker registry       | Personal Access Token (PAT) or password |
| `CACHIX_AUTH_TOKEN` | (Optional) Token for Cachix caching         | Your Cachix auth token                  |

#### For GitHub Container Registry (ghcr.io)

1. Create a Personal Access Token (PAT):
   - Go to GitHub Settings → Developer settings → Personal access tokens → Tokens (classic)
   - Click "Generate new token (classic)"
   - Select scopes: `write:packages`, `read:packages`, `delete:packages`
   - Copy the generated token

2. Add secrets:
   - `DOCKER_USERNAME`: Your GitHub username
   - `DOCKER_PASSWORD`: The PAT you just created

#### For Docker Hub

1. Create an access token:
   - Go to Docker Hub → Account Settings → Security → Access Tokens
   - Click "New Access Token"
   - Set permissions to "Read, Write, Delete"
   - Copy the generated token

2. Add secrets:
   - `DOCKER_USERNAME`: Your Docker Hub username
   - `DOCKER_PASSWORD`: The access token you just created

#### For Gitea Registry (e.g., labs.phundrak.com)

1. Create an access token in Gitea:
   - Log in to your Gitea instance
   - Go to Settings (click your avatar → Settings)
   - Navigate to Applications → Manage Access Tokens
   - Click "Generate New Token"
   - Give it a descriptive name (e.g., "Phundrak Labs Docker Registry")
   - Select the required permissions:
     - `write:package` - Required to publish packages
     - `read:package` - Required to pull packages
   - Click "Generate Token"
   - Copy the generated token immediately (it won't be shown again)

2. Add secrets:
   - `DOCKER_USERNAME`: Your Gitea username
   - `DOCKER_PASSWORD`: The access token you just created

Note: Gitea's container registry is accessed at `https://your-gitea-instance/username/-/packages`

#### For Other Custom Registries

1. Obtain credentials from your registry administrator

2. Add secrets:
   - `DOCKER_USERNAME`: Your registry username
   - `DOCKER_PASSWORD`: Your registry password or token

### Configuring Cachix (Build Caching)

Cachix is a Nix binary cache that dramatically speeds up builds by caching build artifacts. The workflow supports configurable Cachix settings.

#### Environment Variables

Configure these in the workflow's `env` section or as repository variables:

| Variable           | Description                                    | Default Value | Example            |
|--------------------+------------------------------------------------+---------------+--------------------|
| `CACHIX_NAME`      | Name of the Cachix cache to use                | `devenv`      | `phundrak-dot-com` |
| `CACHIX_SKIP_PUSH` | Whether to skip pushing artifacts to the cache | `true`        | `false`            |

#### Option 1: Pull from Public Cache Only

If you only want to pull from a public cache (no pushing):

1. Set environment variables in the workflow:
   ```yaml
   env:
     CACHIX_NAME: devenv  # or any public cache name
     CACHIX_SKIP_PUSH: true
   ```

2. No `CACHIX_AUTH_TOKEN` secret is needed

This is useful when using public caches like `devenv` or `nix-community`.

#### Option 2: Use Your Own Cache (Recommended for Faster Builds)

To cache your own build artifacts for faster subsequent builds:

1. Create a Cachix cache:
   - Go to https://app.cachix.org
   - Sign up and create a new cache (e.g., `your-project-name`)
   - Free for public/open-source projects

2. Get your auth token:
   - In Cachix, go to your cache settings
   - Find your auth token under "Auth tokens"
   - Copy the token

3. Add your cache configuration to `flake.nix`:
   ```nix
   nixConfig = {
     extra-trusted-public-keys = [
       "devenv.cachix.org-1:w1cLUi8dv3hnoSPGAuibQv+f9TZLr6cv/Hm9XgU50cw="
       "your-cache-name.cachix.org-1:YOUR_PUBLIC_KEY_HERE"
     ];
     extra-substituters = [
       "https://devenv.cachix.org"
       "https://your-cache-name.cachix.org"
     ];
   };
   ```

4. Configure the workflow:
   - Edit `.github/workflows/publish-docker.yml`:
     ```yaml
     env:
       CACHIX_NAME: your-cache-name
       CACHIX_SKIP_PUSH: false
     ```
   - Or set as repository variables in GitHub/Gitea

5. Add your auth token as a secret:
   - Go to repository `Settings` → `Secrets and variables` → `Actions`
   - Add secret `CACHIX_AUTH_TOKEN` with your token

#### Benefits of Using Your Own Cache

- **Faster builds**: Subsequent builds reuse cached artifacts (Rust dependencies, compiled binaries)
- **Reduced CI time**: Can reduce build time from 10+ minutes to under 1 minute
- **Cost savings**: Less compute time means lower CI costs
- **Shared across branches**: All branches benefit from the same cache

### Configuring the Docker Registry

The target registry is set via the `DOCKER_REGISTRY` environment variable in the workflow file. To change it:

1. Edit `.github/workflows/publish-docker.yml`
2. Modify the `env` section:

```yaml
env:
  DOCKER_REGISTRY: ghcr.io  # Change to your registry (e.g., docker.io, labs.phundrak.com)
  IMAGE_NAME: phundrak/phundrak-dot-com-backend
```

Or set it as a repository variable:
- Go to `Settings` → `Secrets and variables` → `Actions` → `Variables` tab
- Add `DOCKER_REGISTRY` with your desired registry URL

### Image Naming

Images are published with the name: `${DOCKER_REGISTRY}/${IMAGE_NAME}:${TAG}`

For example:
- `labs.phundrak.com/phundrak/phundrak-dot-com-backend:latest`
- `labs.phundrak.com/phundrak/phundrak-dot-com-backend:1.0.0`
- `labs.phundrak.com/phundrak/phundrak-dot-com-backend:develop`
- `labs.phundrak.com/phundrak/phundrak-dot-com-backend:pr12`

### Local Testing

To test the Docker image build locally:

```bash
# Build the image with Nix
nix build .#backendDockerLatest

# Load it into Docker
docker load < result

# Run the container (image name comes from Cargo.toml package.name)
docker run -p 3100:3100 phundrak/phundrak-dot-com-backend:latest
```

### Troubleshooting

#### Authentication Failures

If you see authentication errors:
1. Verify your `DOCKER_USERNAME` and `DOCKER_PASSWORD` secrets are correct
2. For ghcr.io, ensure your PAT has the correct permissions
3. Check that the `DOCKER_REGISTRY` matches your credentials

#### Build Failures

If the Nix build fails:
1. Test the build locally first: `nix build .#backendDockerLatest`
2. Check the GitHub Actions logs for specific error messages
3. Ensure all dependencies in `flake.nix` are correctly specified

#### Image Not Appearing in Registry

1. Verify the workflow completed successfully in the Actions tab
2. Check that the registry URL is correct
3. For ghcr.io, images appear at: `https://github.com/users/USERNAME/packages/container/IMAGE_NAME`
4. Ensure your token has write permissions
