# hop 🪄

A fast and minimal CLI tool to manage and connect to frequently used SSH servers.

> Skip the copy-paste hustle. Just `hop` into your server.

---

## ✨ Features

* Add, remove, and list SSH server entries
* Connect to servers by alias or name
* Support custom ports and SSH usernames
* Store configuration locally (JSON or TOML)
* Shell autocompletion for commands and aliases
* Optional: SSH key generation + setup
* Lightweight and fast (built in Rust)

---

## 📦 Installation

> ⚠️ In development. For now, clone the repo and build it.

```bash
git clone https://github.com/yourname/hop-cli.git
cd hop-cli
cargo build --release
./target/release/hop
```

---

## 📁 Configuration

The config file is stored at:

```
~/.config/hop/servers.json
```

Example format:

```json
[
  {
    "name": "prod-db",
    "user": "forge",
    "ip": "192.168.1.20",
  },
  {
    "name": "staging-web",
    "user": "ubuntu",
    "ip": "192.168.1.21",
  }
]
```

---

## 🦪 Usage

### ➕ Add a Server

```bash
hop add --name prod-db --alias db1 --user forge --ip 192.168.1.20 --port 22
```

### 📋 List All Servers

```bash
hop list
```

### 🚀 Connect to Server

```bash
hop connect db1
```

This will run:

```bash
ssh forge@192.168.1.20 -p 22
```

### ❌ Remove a Server

```bash
hop remove db1
```

### ✏️ Edit a Server

```bash
hop edit db1
```

---

## 🛡️ Security

* No private key is stored or managed by default.
* You can optionally integrate SSH key generation and copying with `hop keygen`.
* In future, GPG encryption for the config file may be added.

---

## 🛠️ Development Plan

### Tech Stack

* **Language**: Rust
* **CLI Framework**: [clap](https://docs.rs/clap), [serde](https://serde.rs/)
* **Storage**: JSON (later: optional TOML, encrypted)

### File Structure

```
src/
├── main.rs
├── cli.rs           # Command handling
├── config.rs        # Config reading/writing
├── models.rs        # Server model structs
├── ssh.rs           # Actual SSH connect logic
└── utils.rs
```

---

## ⚠️ Roadmap

* [x] Add/list/remove/edit servers
* [x] Connect using `ssh` command
* [ ] Fuzzy match & autocomplete
* [ ] Keygen support
* [ ] GPG encryption for config
* [ ] Export/import server configs

---

## 💡 Example Ideas

```bash
hop connect prod-db         # using name
hop connect db1             # using alias
hop add --name dev --ip 1.2.3.4 --user root --alias devbox
```

---

## 🧙 Tip for Cursor

You can instruct Cursor to:

* Bootstrap this project using Rust and `clap`
* Scaffold file structure listed above
* Write the `Server` model and config parser first
* Then implement `hop add`, `hop list`, and `hop connect`

---

## 📄 License

MIT
