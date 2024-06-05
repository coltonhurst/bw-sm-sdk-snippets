# bw-sm-sdk-snippets

This repository contains build instructions and example code for creating a secret in every supported language wrapper of the Bitwarden SDK.

## Languages

- [Golang](./sdk-go/instructions.md)

## Environment Variables

For the examples in this repository, we set the following environment variables:
- `BW_API_URL`
- `BW_IDENTITY_URL`
- `BW_ACCESS_TOKEN`
- `BW_ORGANIZATION_ID`
- `BW_PROJECT_ID`

Here is an example script you can use:

```bash
#!/bin/bash

export BW_API_URL="https://api.bitwarden.com"
export BW_IDENTITY_URL="https://identity.bitwarden.com"
export BW_ACCESS_TOKEN="<your-access-token-here>"
export BW_ORGANIZATION_ID="<your-organization-id-here>"
export BW_PROJECT_ID="<your-project-id-here>"
```

Example usage: `source bw_setup_script.sh`