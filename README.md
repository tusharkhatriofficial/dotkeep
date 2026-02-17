<p align="center">
  <h1 align="center">dotkeep</h1>
  <p align="center">
    <strong>Your .env files, encrypted locally. No accounts. No cloud. Just works.</strong>
  </p>
  <p align="center">
    <a href="#install">Install</a> &bull;
    <a href="#quick-start">Quick Start</a> &bull;
    <a href="#commands">Commands</a> &bull;
    <a href="#why-dotkeep">Why dotkeep?</a> &bull;
    <a href="#security">Security</a>
  </p>
</p>

<br>

You have **12 projects**. Each has a `.env` file. Some share the same `DATABASE_URL`. You copy-paste secrets between them. One day you `rm -rf` the wrong folder. The secrets are gone forever.

**dotkeep fixes this in 30 seconds:**

```bash
dotkeep init                  # set a master password
cd ~/code/my-saas && dotkeep add my-saas     # encrypt & store .env
cd ~/code/api && dotkeep add api             # again
cd ~/code/landing && dotkeep add landing     # and again

# Two months later, new laptop:
dotkeep use my-saas           # .env restored instantly
```

That's it. Every `.env` value is encrypted with **AES-256-GCM**, stored in an encrypted **SQLCipher** database, and unlocked with a single master password you keep in your head.

<br>

## See it in action

```
$ dotkeep list

  dotkeep projects (4)

┌─────────────────┬──────┬──────────┐
│ Project         │ Vars │ Modified │
├─────────────────┼──────┼──────────┤
│ my-saas         │ 28   │ 2h ago   │
│ api             │ 45   │ 3h ago   │
│ landing-page    │ 12   │ 1d ago   │
│ worker          │ 8    │ 2d ago   │
└─────────────────┴──────┴──────────┘

$ dotkeep search DATABASE_URL

  Found DATABASE_URL in 3 projects:
  |-- my-saas: postgresql://localhost/saas_dev
  |-- api: postgresql://localhost/api_dev
  |-- worker: postgresql://localhost/worker_dev

$ dotkeep inspect my-saas

  Project: my-saas (28 variables)

┌──────────────────┬──────────────────────────────┐
│ Key              │ Value                        │
├──────────────────┼──────────────────────────────┤
│ DATABASE_URL     │ postgresql://localhost/*****  │
│ REDIS_URL        │ redis://localhost:6379/*****  │
│ STRIPE_SECRET_KEY│ ********                     │
│ APP_PORT         │ 3000                         │
│ DEBUG            │ true                         │
└──────────────────┴──────────────────────────────┘

$ dotkeep use my-saas
  ✅ Wrote 28 variables to .env
```

<br>

<h2 id="install">Install</h2>

```bash
cargo install dotkeep
```

**Pre-built binaries** (Linux, macOS, Windows):

```bash
# macOS
brew install tusharkhatriofficial/tap/dotkeep

# Linux
curl -sSf https://github.com/tusharkhatriofficial/dotkeep/releases/latest/download/dotkeep-linux-amd64.tar.gz | tar -xzv
sudo mv dotkeep /usr/local/bin/

# Windows (PowerShell)
Invoke-WebRequest -Uri "https://github.com/tusharkhatriofficial/dotkeep/releases/latest/download/dotkeep-windows-amd64.exe" -OutFile "dotkeep.exe"
```

<br>

<h2 id="quick-start">Quick Start</h2>

**1. Create your vault**
```bash
$ dotkeep init
  Creating a new vault. Choose a master password.
  Enter master password: ••••••••
  Confirm master password: ••••••••
  ✅ Vault created at ~/.dotkeep/vault.db
```

**2. Store a project's `.env`**
```bash
$ cd ~/code/my-saas
$ dotkeep add my-saas
  ✅ Added project my-saas with 28 variables
```

**3. Restore it anywhere, anytime**
```bash
$ cd ~/code/my-saas
$ dotkeep use my-saas
  ✅ Wrote 28 variables to .env
```

Done. Your secrets are safe.

<br>

<h2 id="commands">Commands</h2>

### Core workflow

| Command | What it does |
|---|---|
| `dotkeep init` | Create encrypted vault, set master password |
| `dotkeep add <name>` | Scan `.env` in current dir, encrypt & store |
| `dotkeep use <name>` | Restore `.env` from vault to current dir |
| `dotkeep list` | Show all projects in a table |
| `dotkeep remove <name>` | Delete a project from the vault |

### Inspect & compare

| Command | What it does |
|---|---|
| `dotkeep inspect <name>` | Show variables (secrets auto-masked) |
| `dotkeep diff <proj1> <proj2>` | Compare two projects side-by-side |
| `dotkeep search <key>` | Find which projects use a key (e.g. `DATABASE_URL`) |
| `dotkeep unused <name>` | Detect dead variables not referenced in code |
| `dotkeep validate <name>` | Catch mistakes: invalid ports, malformed URLs |
| `dotkeep types <name>` | Infer variable types (string, number, boolean, URL) |

### Secrets & sharing

| Command | What it does |
|---|---|
| `dotkeep secrets set KEY=VALUE` | Store an encrypted shared secret |
| `dotkeep secrets list` | List all secrets (masked) |
| `dotkeep secrets link <key> <project>` | Link a secret to a project |
| `dotkeep sync <from> <to>` | Copy common vars between projects |
| `dotkeep export <name>` | Export project as encrypted `.envvault` file |
| `dotkeep import <file>` | Import from `.envvault` file |

### Backup & restore

| Command | What it does |
|---|---|
| `dotkeep backup` | Export entire vault as encrypted backup |
| `dotkeep restore <file>` | Restore vault from backup file |
| `dotkeep status` | Show currently active project |
| `dotkeep recent` | Switch to a recently used project |

### Terminal UI

```bash
dotkeep tui
```

Full-screen interface built with [ratatui](https://github.com/ratatui/ratatui). Navigate projects, edit variables, search, and sync — all from your terminal.

```
┌─ dotkeep ──────────────────────────────┐
│ 📁 Projects (4)                        │
│                                        │
│ ▶ my-saas       28 vars    2h ago  🟢  │
│   api           45 vars    3h ago  🟢  │
│   landing-page  12 vars    1d ago  🟡  │
│   worker         8 vars    2d ago  🟢  │
│ ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ │
│ / Search    e Edit    s Sync    q Quit │
└────────────────────────────────────────┘
```

<br>

<h2 id="why-dotkeep">Why dotkeep?</h2>

### The problem

Every developer has this:

```
~/code/
├── my-saas/.env          # 28 vars, half are copy-pasted from api/
├── api/.env              # 45 vars, STRIPE_KEY duplicated in 3 places
├── landing/.env          # forgot to update DATABASE_URL after migration
├── worker/.env           # is this the right REDIS_URL?
└── side-project/.env     # what's even in here?
```

You can't search across them. You can't compare them. You can't back them up. You copy-paste secrets and pray nothing breaks.

### The solution

```
~/.dotkeep/vault.db       # one encrypted file, all your secrets
```

### vs. the alternatives

| | **dotkeep** | **Doppler** | **Infisical** | **direnv** |
|---|---|---|---|---|
| **Cost** | Free forever | $20+/month | $10+/month | Free |
| **Storage** | Local (your machine) | Cloud | Cloud | `.envrc` files |
| **Encryption** | AES-256-GCM + SQLCipher | Server-side | Server-side | None |
| **Account required** | No | Yes | Yes | No |
| **Cross-project search** | Yes | Limited | Limited | No |
| **Dead var detection** | Yes | No | No | No |
| **TUI** | Yes | No | No | No |
| **Binary size** | ~3 MB | ~40 MB | ~35 MB | ~5 MB |
| **Startup time** | ~5 ms | ~45 ms | ~30 ms | ~12 ms |

<br>

<h2 id="security">Security</h2>

```
Master Password
      │
      ▼  PBKDF2-HMAC-SHA256 (100,000 iterations) + random salt
      │
 Derived Key (32 bytes)
      │
      ├──▶ SQLCipher (encrypts entire database file)
      │
      └──▶ AES-256-GCM (encrypts each variable value individually)
```

- **Master password is never stored.** Only a verification hash derived via PBKDF2.
- **Double encryption.** The database file itself is encrypted (SQLCipher), AND each value inside is encrypted separately (AES-256-GCM with unique nonces).
- **Zero plaintext on disk.** After `dotkeep use` writes your `.env`, no secrets remain in the vault unencrypted.
- **Tamper detection.** GCM mode guarantees integrity — any modification to ciphertext is detected.
- **Crypto stack:** [`ring`](https://github.com/briansmith/ring) (same library used by Cloudflare, Fastly, and rustls).

<br>

## How it works

```
$ dotkeep add my-saas

  1. Reads .env from current directory
  2. Parses KEY=VALUE pairs (handles quotes, comments, multiline)
  3. Encrypts each value with AES-256-GCM using your derived key
  4. Stores in ~/.dotkeep/vault.db (encrypted SQLCipher database)

$ dotkeep use my-saas

  1. Unlocks vault with your master password
  2. Decrypts each variable value
  3. Writes clean .env file to current directory
```

<br>

## Tech stack

| Component | Library | Why |
|---|---|---|
| CLI parsing | [clap](https://github.com/clap-rs/clap) | Best-in-class Rust CLI framework |
| Encryption | [ring](https://github.com/briansmith/ring) | Industry-standard cryptography |
| Database | [rusqlite](https://github.com/rusqlite/rusqlite) + SQLCipher | Encrypted SQLite, zero setup |
| Terminal UI | [ratatui](https://github.com/ratatui/ratatui) | Modern TUI framework |
| Password input | [rpassword](https://github.com/conradkleinespel/rpassword) | Secure hidden input |

<br>

## Contributing

```bash
git clone https://github.com/tusharkhatriofficial/dotkeep.git
cd dotkeep
cargo build
cargo test
cargo run -- init
```

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

<br>

## License

MIT — do whatever you want.

<br>

---

<p align="center">
  <strong>Stop copy-pasting secrets. Start keeping them safe.</strong>
  <br>
  <code>cargo install dotkeep</code>
</p>
