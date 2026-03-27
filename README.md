# 🛡️ DevGuard

> Catch broken configs before they break your app.

DevGuard is a fast, zero-config `.env` scanner for Node.js projects. It validates your environment variables and warns you about weak secrets, invalid ports, malformed URLs, and empty values — before you ship.

Built with Rust. Fast by default.

---

## ✨ Features

- 🔍 Scans your `.env` file instantly
- ❌ Detects weak secrets (`SECRET`, `KEY`, `API` too short)
- ❌ Catches invalid port values (e.g. `PORT=abc`)
- ❌ Flags malformed URLs (e.g. `DATABASE_URL=localhost`)
- ❌ Validates `NODE_ENV` values
- ⚠️ Warns about empty, malformed, or missing variables
- ✅ Priority-based rule engine
- ✅ Clean, readable CLI output

---

## 🚀 Installation

```bash
npx @deveko/devguard
```

That's it. No installation needed.

---

## 📦 Usage

Place a `.env` file in your project root, then run:

```bash
# Scan default .env
npx @deveko/devguard check
 
# Scan a custom path
npx @deveko/devguard check --path ./apps/backend/.env
```

### Example `.env`

```env
TEST2
PORT=abc
JWT_SECRET=123
DATABASE_URL=localhost
NODE_ENV=staging
API_KEY=
STRIPE_SECRET_KEY=shortkey
HOST=
CLIENT_ID=

PORT_HOST_KEY=g
```

### Example output

```bash
🔍 DevGuard - scanning .env...

⚠️ 'TEST2' is malformed - missing '='
❌ HOST -> must not be empty
❌ PORT_HOST_KEY -> must be greater than or equal to 32
❌ NODE_ENV -> must be "development" or "production" or "test"
❌ API_KEY -> must not be empty
❌ DATABASE_URL -> must start with http://, https://, postgres://, postgresql://, mysql://, redis://, rediss://, mongodb://, mongodb+srv://, amqp://, amqps://, sqlite://
❌ JWT_SECRET -> must be greater than or equal to 32
❌ CLIENT_ID -> must not be empty
❌ STRIPE_SECRET_KEY -> must be greater than or equal to 32
❌ PORT -> must be a number

⚠️  9 error(s) and 1 warning(s) found
```

When everything looks good:

```bash
🔍 DevGuard - scanning .env...

✅ All checks passed! Your .env looks good!
```

---

## 🧠 How It Works

DevGuard scans your `.env` file line by line and runs pattern-based validation rules with priority ordering:

| Pattern | Rule |
| ------- | ---- |
| Key is `NODE_ENV` | Must be `development`, `production`, or `test` |
| Key contains `SECRET` or `KEY` or `API` | Value must be ≥ 32 characters |
| Key contains `URL` | Must start with a valid protocol (http, postgres, redis, etc.) |
| Key contains `PORT` | Must be a valid number (0-65535) |
| Key contains `HOST` | Must not be empty |
| Key contains `ID` | Must not be empty |

Rules are checked in priority order — first match wins.
No config needed. Just run it.

---

## 🗺️ Roadmap

- [x] `.env` parser
- [x] Pattern-based validation engine
- [x] CLI output with colors
- [x] `npx devguard` via npm
- [x] `--path` option for custom `.env` paths
- [x] Malformed line detection
- [x] Improved error summary
- [x] New validation rules
- [x] Priority system
- [ ] Custom rules via `devguard.config.toml`
- [ ] CI/CD integration
- [ ] GitHub Action
- [ ] VSCode extension
- [ ] Docker config validation
- [ ] Secret leak detection in source files

---

## 🔧 Local Development

```bash
git clone https://github.com/ekojoecovenant/devguard.git
cd devguard
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
