# Ferrox Config (`ferrox-config`)

`ferrox-config` handles strongly-typed application configuration loading from environment variables (`.env`) and TOML files,
integrating `secrecy` to prevent accidental logging of passwords and API keys.

## Key Features
- 🔒 **`SecretString` Integration**: Protect sensitive database URIs and API keys from accidental printing in logs.
- 📄 **Multi-Source Merging**: Read from `.env`, environment variables, and `config/default.toml` hierarchically.
