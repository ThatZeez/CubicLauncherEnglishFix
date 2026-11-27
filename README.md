# CubicLauncherEnglishFix

An unofficial fork of CubicLauncher for Linux with English as the default language. It stays close to upstream so changes can be merged, and it keeps the Spanish translation.

## About

Based on [CubicLauncher](https://github.com/CubicLauncherDevs/CubicLauncher) by the CubicLauncher developers (<https://cubiclauncher.org>). This fork is not affiliated with or endorsed by them.

It uses Tauri 2, Rust, Svelte 5/SvelteKit, TypeScript, and Bun, and is developed and tested on Linux (CachyOS with Hyprland). It is licensed under [GPL-3.0-only](LICENSE), inherited from upstream.

This fork has no releases. Build it from source. The original project's releases are at <https://github.com/CubicLauncherDevs/CubicLauncher/releases>.

## Changes

- Fresh installs default to English. Existing settings keep their language, and Spanish (`es-ES`) is still bundled.
- Hardcoded Spanish UI text now lives in the translation files, including the missing English `market.detail.gallery` key.
- Backend messages for core typed errors, path validation, and theme import are in English.
- The branch gets periodic merges from upstream `develop`.

## Planned

Not implemented or verified yet:

- English Discord Rich Presence and the remaining backend messages.
- Mod management for installed modpacks; the scope of the `.mrpack` and Modrinth/CurseForge workflows still needs verification.
- Ely.by login, fork-specific app identity and updates, and a distinct visual design.
- Testing Minecraft 26.2 with Fabric.

## Features

- Create, configure, import, and export instances.
- Install Vanilla, Fabric, Forge, NeoForge, Quilt, and OptiFine.
- Sign in with Microsoft or Yggdrasil, or play offline.
- Install mods, resource packs, and shaders from Modrinth and CurseForge, and import modpacks.
- Manage worlds and server lists per instance.
- Choose the Java version and vendor, adjust memory, and view logs.
- Change the theme and language, and show activity in Discord.
- Migrate from the official launcher, MultiMC and compatible forks, XMCL, and SKLauncher 4.

## Build

You need Git, Bun 1.x or newer, Node.js 20 or newer, Rust 1.85 or newer (edition 2024, with Cargo, rustfmt, and Clippy), and the [Tauri 2 system dependencies](https://v2.tauri.app/start/prerequisites/).

```bash
git clone https://github.com/ThatZeez/CubicLauncherEnglishFix.git
cd CubicLauncherEnglishFix
bun install --frozen-lockfile
bun run tauri dev
```

`bun run dev` starts only the frontend; anything that needs the Tauri backend will not work outside Tauri. With Nix, run `nix develop` first. Build with `bun run tauri build`.

Before committing, run `bun run lint`, `bun run check`, `bun run test:all`, `cargo fmt --all --check`, and `cargo clippy --workspace --locked -- -D warnings`. [TESTING.md](TESTING.md) and [CONTRIBUTING.md](CONTRIBUTING.md) have the full details; both are in Spanish.

## Limitations

The app identifier is still `com.cubiclauncher.launcher` and data still lives under `~/.cubic`, so do not run this fork against data used by an installed original CubicLauncher. The updater still points at the original project's endpoints, so it updates to upstream releases rather than builds of this fork.

## Contributing and license

Read [CONTRIBUTING.md](CONTRIBUTING.md), follow [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md), and see [TESTING.md](TESTING.md) for checks. The launcher and all workspace crates are [GPL-3.0-only](LICENSE).

Translations for other locales are in the upstream [Translations](https://github.com/CubicLauncherDevs/Translations) repository. Versioning is described in [VERSIONING.md](VERSIONING.md).

Thanks to the CubicLauncher developers for the project this fork builds on.
