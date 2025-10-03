# Ridicrypt GUI

A cross‑platform desktop GUI (Tauri + SvelteKit + Rust) for inspecting and decrypting your locally downloaded Ridibooks library via the `ridicrypt-core` backend.

> You already have the data on your machine; this app just helps you look at it and export it comfortably.

## Features

- Local library discovery (multi‑user aware)
- Fast incremental refresh (polling every few seconds)
- Per‑user filtering & sorting (title / author / recently downloaded)
- Bulk selection with export action
- One‑click open (decrypts to a temp file and launches with system default)
- On‑demand export (single or batch) with sensible default filenames
- Temporary decrypted files auto‑cleaned on window close
- Internationalization via Paraglide + message bundles (many locales included)
- Theming & persisted locale preference
- Sandboxed, lightweight native shell (Tauri 2.x)

## Architecture at a Glance

- Frontend: SvelteKit (SPA mode via `adapter-static`, no Node SSR) + Tailwind (v4) + DaisyUI
- Native layer: Rust (Tauri) exposing async commands:
  - `get_library`
  - `decrypt`
  - `get_temp_book_path`
- Core logic delegated to `ridicrypt-core` crate (sibling path core)
- Temp workspace under system temp: `ridi-re.ridicrypt.gui`
- Communication: Tauri invoke bridge returning structured `CommandResult<T>`

## Security & Scope

All operations are local. No external network calls are made by the core logic. Decryption uses keys already present in your Ridibooks installation. If those keys are absent or unreadable the app will fail fast with a dialog.

## Prerequisites

- Node.js (LTS 20+ recommended) & pnpm
- Rust toolchain (stable, includes Cargo)  
- Tauri dependencies for your OS (see https://v2.tauri.app/start/prerequisites/)
- A local Ridibooks installation with valid data (otherwise initialization will error)

## Getting Started

Clone (assuming monorepo or correct relative `ridicrypt-core` path exists):

```pwsh
git clone <your-fork-url>
cd ridicrypt/gui
pnpm install
```

### Development (hot reload)

```pwsh
pnpm tauri dev
```

This runs:
- Frontend dev server (Vite) on an internal port
- Tauri shell that loads the dev URL

### Type Checking / Lint-ish

```pwsh
pnpm check
```

(Uses `svelte-check` with the project `tsconfig`.)

### Build (Production App)

```pwsh
pnpm tauri build
```

Outputs platform bundles (DMG/MSI/AppImage/etc. depending on your OS and configured targets).

### Frontend Only Static Build

```pwsh
pnpm build
```

Emits static assets to build (used as `frontendDist` by Tauri).

## Usage Flow

1. Launch the app.
2. On first run it initializes:
   - Extracts global key pieces
   - Prepares a clean temp directory
3. Library view auto‑refreshes every few seconds.
4. Click a book card:
   - Temp decrypted file is created
   - System default viewer opens
5. Export:
   - Single: Use the export button on the card
   - Batch: Enter selection mode, pick books, apply Export
6. Close the window: temp decrypted files are wiped.

If something fails early (e.g., global key lookup), a blocking error dialog appears and the process exits.

## Internationalization (i18n)

- Messages live under messages and are bundled through Paraglide.
- Browser language (or base language) is auto‑detected on first launch; user choice is persisted via the store.
- Add a new locale: provide a JSON message file and update Paraglide registry if needed, then rebuild.

## Project Structure (Key Bits)

```
src/
  routes/              SvelteKit entry (+layout as SPA bootstrap)
  lib/components/      UI components (common + library views)
  lib/paraglide/       i18n runtime bindings
src-tauri/
  src/                 Rust commands, runtime init, cleanup
  tauri.conf.json      App/product metadata
  Cargo.toml           Native crate manifest
messages/              Localized message JSON
build/                 Generated static site (production assets)
```

## Rust Native Layer Overview

- runtime.rs: Key derivation, library scanning, temp path management, decryption dispatch
- bindings.rs: Tauri command wrappers (async -> blocking macro)
- utils.rs: `CommandResult<T>` helper and blocking executor
- Temp file naming: hashed `book_id + owner_id` to produce stable ephemeral filenames

## Commands Contract

- get_library → JSON string mapping `userId -> { bookId -> metadata+storage }`
- decrypt(key_path, file_path, target_path) → writes decrypted file
- get_temp_book_path(book_id, owner_id, format) → deterministic temp path (unused files cleaned on exit)

Errors surface as `{ success: false, error: "message" }` so the UI can show notifications.

## Styling & UI

- Tailwind v4 + DaisyUI for component primitives
- Dark/Light theme toggling (selector component)
- Grid auto‑fills book cards; responsive without SSR

## Scripts (from package.json)

```text
dev              Vite dev server
build            Production build (static)
preview          Preview static build
check            Sync + type check
check:watch      Continuous type checking
tauri            Tauri CLI passthrough
machine-translate Inlang machine translation automation
```

## Adding a Feature (Example Playbook)

1. Define a new Rust command in bindings.rs.
2. Implement logic in a dedicated Rust module or runtime.rs.
3. Add it to `invoke_handler![]` in lib.rs.
4. Expose in the Svelte layer via `invoke("your_command", { ... })`.
5. Type the result (narrow shape) in the calling component.

## Troubleshooting

| Symptom | Likely Cause | Fix |
| ------- | ------------ | --- |
| Startup fatal dialog about global key | Ridibooks not installed / locked files | Ensure Ridibooks app ran at least once and data path is accessible |
| Empty library | No user datastore or missing permissions | Verify local Ridibooks data folder and file permissions |
| Export fails intermittently | Antivirus interfering with temp or target dir | Whitelist the temp directory / target path |
| Temp files persist | App crash before destruction | Manually clear your system temp folder |

## Contributing

PRs welcome:
1. Fork & branch
2. Keep commits focused
3. Add concise comments where logic is non-obvious
4. Test on at least one platform (Windows/Linux/macOS if possible)
5. Follow existing code style (formatter defaults)

## License

CC0 1.0 Universal (Public Domain Dedication). See the `LICENSE` file for the full legal text.

In plain words: to the extent possible under law, the authors have waived all copyright and related rights in this project. You can copy, modify, distribute, and use it (including commercially) without asking permission. No warranties.

## Disclaimer

Intended for lawful personal use with legitimately obtained content. You are responsible for complying with local laws and platform terms.

## Future Ideas

- Smarter diff-based refresh (drop polling)
- Progress bars for bulk export
- Search & tag filtering
- Plugin hook for custom export pipelines

> Build small, ship early, decrypt locally.
