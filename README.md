# 🛡️ DevGuard

> Catch broken configs before they break your app.

DevGuard is a fast, zero-config `.env` scanner for Node.js projects. It validates your environment variables and warns you about weak secrets, invalid ports, malformed URLs, and empty values — before you ship.

Built with Rust. Fast by default.

---

## ✨ Features

- 🔍 Scans your `.env` file instantly
- ❌ Detects weak secrets (e.g. `JWT_SECRET` too short)
- ❌ Catches invalid port values (e.g. `PORT=abc`)
- ❌ Flags malformed URLs (e.g. `DATABASE_URL=localhost`)
- ⚠️ Warns about empty or malformed variables
- ✅ Clean, readable CLI output

---

## 🚀 Installation

```bash
npx devguard
```

That's it. No installation needed.

---

## 📦 Usage

Place a `.env` file in your project root, then run:

```bash
# Scan default .env
npx devguard check
 
# Scan a custom path
npx devguard check --path ./apps/backend/.env
```

### Example `.env`

```env
PORT=abc
JWT_SECRET=123
DATABASE_URL=localhost
NODE_ENV=development
```

### Example output

```bash
🔍 DevGuard - scanning .env...

❌ PORT -> must be a number
❌ JWT_SECRET -> must be greater than or equal to 32 characters
❌ DATABASE_URL -> must start with http://, https://, postgres://, mysql://

⚠️  3 issue(s) found
```

When everything looks good:

```bash
🔍 DevGuard - scanning .env...

✅ All checks passed! Your .env looks good!
```

---

## 🧠 How It Works

DevGuard scans your `.env` file line by line and runs pattern-based validation rules:

| Pattern | Rule |
| ------- | ---- |
| Key contains `SECRET` | Value must be ≥ 32 characters |
| Key contains `PORT` | Value must be a valid number |
| Key contains `URL` | Value must start with a valid protocol |

No config needed. Just run it.

---

## 🗺️ Roadmap

- [x] `.env` parser
- [x] Pattern-based validation engine
- [x] CLI output with colors
- [x] `npx devguard` via npm
- [x] `--path` option for custom `.env` paths
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
