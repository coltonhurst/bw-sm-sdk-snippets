# bw-sm-sdk-snippets

This repository contains basic code and instructions for creating a secret in various programming languages using the Bitwarden SDK.

## Languages

- [C#](./sdk-cs/instructions.md)
- [Go](./sdk-go/instructions.md)
- [Rust](./sdk-rust/instructions.md)

## Environment Variables

For the examples in this repository, we set the following environment variables:
- `BW_IDENTITY_URL`
- `BW_API_URL`
- `BW_ORGANIZATION_ID`
- `BW_ACCESS_TOKEN`
- `BW_PROJECT_ID`

Here is an example script you can use:

```bash
#!/usr/bin/env bash

export BW_IDENTITY_URL="https://identity.bitwarden.com"
export BW_API_URL="https://api.bitwarden.com"
export BW_ORGANIZATION_ID="<your-organization-id-here>"
export BW_ACCESS_TOKEN="<your-access-token-here>"
export BW_PROJECT_ID="<your-project-id-here>"
```

Example usage: `source bw_setup_script.sh`
