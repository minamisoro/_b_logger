#!/usr/bin/env python3
"""
Sync config.toml values to frontend .env files
Run this after changing config.toml to update environment variables
"""

import tomllib
from pathlib import Path


def main():
    # Paths
    root_dir = Path(__file__).parent.parent
    config_path = root_dir / "config.toml"
    web_env_path = root_dir / "frontend" / "web" / ".env"
    admin_env_path = root_dir / "frontend" / "admin" / ".env"

    # Read config.toml
    with open(config_path, "rb") as f:
        config = tomllib.load(f)

    # Build server URL
    server_host = config["server"]["host"]
    server_port = config["server"].get("port")
    if server_port:
        server_url = f"http://{server_host}:{server_port}"
    else:
        server_url = server_host

    # Generate .env content for web frontend
    web_port = config["frontend"]["web"].get("port", 5173)
    web_env_content = f"""# Blogger Web Frontend - Environment Configuration
# This file is auto-generated from config.toml - DO NOT EDIT MANUALLY
# Run 'python scripts/sync-config.py' to regenerate

# API Base URL
VITE_API_BASE_URL={server_url}

# Dev server port
VITE_PORT={web_port}
"""

    # Generate .env content for admin frontend
    admin_port = config["frontend"]["admin"].get("port", 5174)
    admin_env_content = f"""# Blogger Admin Frontend - Environment Configuration
# This file is auto-generated from config.toml - DO NOT EDIT MANUALLY
# Run 'python scripts/sync-config.py' to regenerate

# API Base URL
VITE_API_BASE_URL={server_url}

# Dev server port
VITE_PORT={admin_port}
"""

    # Write .env files
    web_env_path.write_text(web_env_content)
    admin_env_path.write_text(admin_env_content)

    print("✅ Config synced successfully!")
    print(f"  - {web_env_path}")
    print(f"  - {admin_env_path}")


if __name__ == "__main__":
    main()
