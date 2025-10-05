#!/usr/bin/env python3
"""
API Generation Wrapper Script

This script orchestrates the API generation pipeline:
1. Generate OpenAPI spec from Rust backend (generate-openapi)
2. Generate TypeScript types from OpenAPI spec (openapi-typescript)
3. Setup is complete - openapi-fetch uses the generated types

Usage:
    python3 wrapper.py
"""

import subprocess
import sys
import os
from pathlib import Path

# Define paths
PROJECT_ROOT = Path(__file__).parent.parent
SWAGGER_JSON = PROJECT_ROOT / "frontend" / "lib" / "swagger.json"
GENERATED_TYPES = PROJECT_ROOT / "frontend" / "lib" / "api.ts"

def run_command(cmd, description, cwd=None):
    """Run a command and handle errors."""
    print(f"\n{'='*60}")
    print(f"Running: {description}")
    print(f"Command: {' '.join(cmd)}")
    print(f"{'='*60}")

    try:
        result = subprocess.run(
            cmd,
            cwd=cwd or PROJECT_ROOT,
            check=True,
            capture_output=True,
            text=True
        )

        if result.stdout:
            print(result.stdout)

        print(f"✓ {description} completed successfully")
        return True

    except subprocess.CalledProcessError as e:
        print(f"✗ {description} failed", file=sys.stderr)
        print(f"Error: {e.stderr}", file=sys.stderr)
        return False

def main():
    """Main execution flow."""
    print("Starting API generation pipeline...")

    # Step 1: Generate OpenAPI spec from Rust backend
    if not run_command(
        ["cargo", "run", "-p", "blogger-scripts", "--bin", "generate-openapi"],
        "Generate OpenAPI spec (Rust)",
        cwd=PROJECT_ROOT
    ):
        print("\n✗ Pipeline failed at step 1: generate-openapi", file=sys.stderr)
        sys.exit(1)

    # Verify swagger.json was created
    if not SWAGGER_JSON.exists():
        print(f"\n✗ Expected output file not found: {SWAGGER_JSON}", file=sys.stderr)
        sys.exit(1)

    print(f"\n✓ OpenAPI spec generated at: {SWAGGER_JSON}")

    # Step 2: Generate TypeScript types from OpenAPI spec
    if not run_command(
        ["npx", "openapi-typescript", str(SWAGGER_JSON), "-o", str(GENERATED_TYPES)],
        "Generate TypeScript types (openapi-typescript)",
        cwd=PROJECT_ROOT
    ):
        print("\n✗ Pipeline failed at step 2: openapi-typescript", file=sys.stderr)
        sys.exit(1)

    # Verify types were generated
    if not GENERATED_TYPES.exists():
        print(f"\n✗ Expected output file not found: {GENERATED_TYPES}", file=sys.stderr)
        sys.exit(1)

    print(f"\n✓ TypeScript types generated at: {GENERATED_TYPES}")

    # Step 3: Info about openapi-fetch
    print(f"\n{'='*60}")
    print("✓ API generation pipeline completed successfully!")
    print(f"{'='*60}")
    print("\nGenerated files:")
    print(f"  - OpenAPI spec: {SWAGGER_JSON.relative_to(PROJECT_ROOT)}")
    print(f"  - TypeScript types: {GENERATED_TYPES.relative_to(PROJECT_ROOT)}")
    print("\nUsage in your frontend code:")
    print("  import createClient from 'openapi-fetch';")
    print("  import type { paths } from '@/lib/api';")
    print("  ")
    print("  const client = createClient<paths>({ baseUrl: 'http://localhost:3000' });")
    print("  const { data, error } = await client.GET('/api/timeline');")

    return 0

if __name__ == "__main__":
    sys.exit(main())
