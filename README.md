# 🛡️ GuardStack

> Catch broken configs before they break your app.

GuardStack is a fast, zero-config project security scanner for Node.js projects. It validates your environment variable, detects hardcoded secrets, and catches misconfigurations - before you ship.

Built with Rust. Fast by default.

---

## ✨ Features

- 🔍 Scans your `.env` file instantly
- ❌ Detects weak secrets (`SECRET`, `KEY`, `API` too short)
- ❌ Detects common dummy values (e.g., `123456`, `password`)
- ❌ Flags insecure configurations (e.g., `CORS_ORIGIN=*`, `NODE_TLS_REJECT_UNAUTHORIZED=0`)
- ❌ Catches invalid port values (e.g. `PORT=abc`)
- ❌ Flags malformed URLs (e.g. `DATABASE_URL=localhost`)
- ❌ Validates `NODE_ENV` values (including `staging`)
- ⚠️ Warns about empty, malformed, or missing variables
- ✅ Priority-based rule engine
- ✅ Auto-generates `.env.example` from `.env`
- ✅ Clean, readable CLI output
- 🖥️ Cross platform (Windows, Mac, Linux)
- 🤖 Auto-builds via GitHub Actions
- ⚙️ Custom rules via `guardstack.config.toml`
- 🔄 Custom rules override built-in rules
- 🔐 Scans source files for hardcoded secrets
- 🚀 Parallel scanning for blazing fast performance
- 🔍 Detects real secret patterns (Stripe, GitHub, AWS, Slack, and more)
- 💬 Differentiates code leaks from comment leaks

---

## 🚀 Installation

```bash
npx guardstack
```

That's it. Works on Windows, Mac and Linux. No installation needed.

---

## 📦 Usage

Place a `.env` file in your project root, then run:

```bash
# Scan default .env
npx guardstack check
 
# Scan a custom path
npx guardstack check --path ./apps/backend/.env

# Generate .env.example from .env
npx guardstack init

# Use custom config path
npx guardstack check --config ./custom/guardstack.config.toml

# Scan current directory for secret leaks
npx guardstack scan

# Scan custom path
npx guardstack scan --path ./src
```

### Example `guardstack.config.toml`

```toml
[[rules]]
pattern = "TOKEN"
rule = "min_length"
value = "64"
message = "must be at least 64 characters"

[[rules]]
pattern = "NODE_ENV"
rule = "one_of"
value = "staging,production,development"
message = "must be staging, production or development"
```

### Example `.env`

```env
TEST2
PORT=abc
JWT_SECRET=123
DATABASE_URL=localhost
```

### Example output

```bash
🔍 GuardStack - scanning .env...

=== Warning(s) ===
⚠️  'TEST2' is malformed - missing '='

=== Error(s) ===
❌ PORT -> must be a number
❌ JWT_SECRET -> must be greater than or equal to 32
❌ DATABASE_URL -> must start with http://, https://, postgres://...

=== Missing(s) ===
❌ REDIS_URL -> missing required variable

⚠️  4 error(s) and 1 warning(s) found
```

When everything looks good:

```bash
🔍 GuardStack - scanning .env...

✅ All checks passed! Your .env looks good!
```

---

## 🧠 How It Works

GuardStack runs three checks on your project:

**1. Parse Check**
Scans `.env` line by line for malformed entries

**2. Validation Check**
Runs pattern-based rules with priority ordering:

| Pattern | Rule |
| ------- | ---- |
| Key is `NODE_ENV` | Must be `development`, `production`, `staging`, or `test` |
| Key contains `SECRET` or `KEY` or `API` | Value must be ≥ 32 characters |
| Key contains `URL` | Must start with a valid protocol (http, postgres, redis, etc.) |
| Key contains `PORT` | Must be a valid number (0-65535) |
| Key contains `HOST` | Must not be empty |
| Key contains `ID` | Must not be empty |
| Any Key | Must not be a common dummy value (e.g., `password`) |
| Key is `CORS_ORIGIN` | Must not be `*` |
| Key is `NODE_TLS_REJECT_UNAUTHORIZED` | Must not be `0` |

**3. Missing Keys Check**
Compares `.env` against `.env.example` - any key in `.env.example` missing from  `.env` is flagged!!

**4. Custom Rules(optional)**
Create `guardstack.config.toml` in your project root:

| Rule type | Description |
| --------- | ----------- |
| `min_length` | Value must be at least N characters |
| `one_of` | Value must be one of the specified options |

Custom rules override built-in rules with matching patterns!!

---

## 🗺️ Roadmap

- [x] `.env` parser
- [x] Pattern-based validation engine
- [x] CLI output with colors
- [x] `npx guardstack` via npm
- [x] `--path` option for custom `.env` paths
- [x] Malformed line detection
- [x] Improved error summary
- [x] New validation rules
- [x] Priority system
- [x] `guardstack init` -> auto-generate `.env.example`
- [x] Missing required keys detection
- [x] Sectioned output (Warnings, Errors, Missing)
- [x] CI/CD integration via GitHub Action
- [x] Cross platform binaries (Windows, Mac, Linux)
- [x] Custom rules via `guardstack.config.toml`
- [x] Secret leak detection in source files
- [x] Parallel scanning for performance
- [x] Heuristic-based false positive reduction
- [ ] VSCode extension
- [ ] Docker config validation

---

## 🔧 Local Development

```bash
git clone https://github.com/ekojoecovenant/guardstack.git
cd guardstack
cargo build --release
node cli.js check
```

---

## 🤝 Contributing

Contributions are welcome! Here's how to get started:

1. Fork the repo
2. Create a feature branch

    ```bash
    git checkout -b feature/your-feature-name
    ```

3. Make your changes
4. Run the project locally to test

    ```bash
    cargo build --release
    node cli.js check
    ```

5. Open a Pull Request with a clear description of what you changed and why

Please keep PRs focused — one feature or fix per PR.

---

## 📄 License

MIT — use it, build on it, ship it.

---

<p align="center">Built with 🦀 Rust — by <a href="https://github.com/ekojoecovenant">ℭ𝔬𝔳𝔢</a></p>
