# Quiver

> _Your grip on every API._

Quiver is a local-first desktop app that combines a secure API key vault with a lightweight HTTP client. Store secrets encrypted on your machine, then fire requests without ever exposing keys to plain text.

Built with Tauri, React, and Rust. No cloud. No telemetry. No fluff.

---

## Features

### Vault

- Store API keys, tokens, and secrets encrypted at rest (AES-256-GCM)
- Master password derived with Argon2id — your key never leaves memory
- Organize entries by workspace and environment (dev / staging / production)
- One-click copy with auto-clear clipboard after 30 seconds
- Vault locks automatically on app close

### Tester

- Full HTTP client — GET, POST, PUT, PATCH, DELETE, HEAD, OPTIONS
- Reference vault entries directly in request auth — keys never touch JS state
- Request collections and history
- JSON body editor with pretty-print response viewer
- Response timing, status, headers, and size at a glance
- Environment switcher — vault entries resolve per environment automatically

---

## Stack

| Layer              | Technology                |
| ------------------ | ------------------------- |
| Desktop framework  | Tauri v2                  |
| Frontend           | React + TypeScript + Vite |
| Routing            | TanStack Router           |
| State              | Zustand                   |
| Styling            | Tailwind CSS v4           |
| HTTP client (Rust) | reqwest                   |
| Database           | SQLite via sqlx           |
| Encryption         | AES-256-GCM + Argon2id    |

---

## Getting Started

### Prerequisites

- [Node.js](https://nodejs.org/) 18+
- [Rust](https://rustup.rs/) (stable)
- [Tauri prerequisites](https://tauri.app/start/prerequisites/) for your OS

### Install

```bash
git clone https://github.com/yourname/quiver
cd quiver
npm install
```

### Dev

```bash
npm run tauri dev
```

### Build

```bash
npm run tauri build
```

---

## Security Model

- Secrets are encrypted with **AES-256-GCM** before being written to the local SQLite database.
- The master password is never stored — it is used to derive the encryption key via **Argon2id** on each unlock.
- The derived key lives only in Rust memory (`AppState`) and is cleared on lock or app exit.
- Vault values are **never exposed to JavaScript** — the Rust layer injects them directly into HTTP requests at runtime.
- Request history stores only vault entry references, not raw secret values.

---

## Roadmap

- [ ] Encrypted export / import (portable vault backup)
- [ ] Request chaining (use response values in the next request)
- [ ] Environment variable interpolation in URLs and headers
- [ ] WebSocket and SSE support
- [ ] Collections import from Postman / Insomnia
- [ ] Keyboard-first navigation

---

## License

MIT
