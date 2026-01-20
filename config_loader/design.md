
## 🔁 1. How configuration sources are loaded and merged
 `config` crate’s builder API. The crate provides built‑in support for reading from files and environment variables and layering them into a final configuration object

---

## 🔁 2. Merge & Precedence Rules

Values are merged in a deterministic order based on how the sources are added to the builder:

1. **Defaults** — lowest priority  
2. **Configuration file** — overrides defaults  
3. **Environment variables** — overrides file and defaults

This precedence ensures that environment variables always take priority when present


For example, if the configuration file specifies:
```json
{
"port": 5000
}

if environment contains 9000, like below

CONFIG_LOADER__PORT=9000

The final value will be 9000 only.

---


## 🔁 3. Error Modeling and Reporting Strategy

Errors during configuration loading are represented using a custom ConfigLoaderError enum rather than panicking. The loader returns a Result<T, ConfigLoaderError> so callers must explicitly handle failures, which improves robustness and clarity. This follows Rust best practices for error handling, where errors are propagated using Result "?".

Our error type includes:

1.Build errors from the config crate, for issues like failing to parse or load value from env/files.

2.Validation errors for semantic problems such as invalid IP addresses or required fields being empty.


## 🔁 4. Configuration Validation

1.String fields must not be empty.

2.IP address fields must parse as valid network addresses.

3.Port values must be non‑zero.