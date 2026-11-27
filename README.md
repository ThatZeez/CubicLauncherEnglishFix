# CubicLauncherEnglishFix

An unofficial fork of CubicLauncher for Linux with English as the default language. It stays close to upstream so changes can be merged, and it keeps the Spanish translation.

## About

Based on [CubicLauncher](https://github.com/CubicLauncherDevs/CubicLauncher) by the CubicLauncher developers (<https://cubiclauncher.org>). This fork is not affiliated with or endorsed by them, and it does not plan to contribute changes back to the original project.

It uses Tauri 2, Rust, Svelte 5/SvelteKit, TypeScript, and Bun, and is developed and tested on Linux (CachyOS with Hyprland). It is licensed under [GPL-3.0-only](LICENSE), inherited from upstream.

This fork has no releases. The original project's releases are at <https://github.com/CubicLauncherDevs/CubicLauncher/releases>.

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

## Limitations

The app identifier is still `com.cubiclauncher.launcher` and data still lives under `~/.cubic`, so do not run this fork against data used by an installed original CubicLauncher. The updater still points at the original project's endpoints, so it updates to upstream releases rather than builds of this fork.

## Contributing and license

Build and test instructions are in [CONTRIBUTING.md](CONTRIBUTING.md) and [TESTING.md](TESTING.md). Participation is governed by [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md). The launcher and all workspace crates are [GPL-3.0-only](LICENSE).

Translations for other locales are in the upstream [Translations](https://github.com/CubicLauncherDevs/Translations) repository. Versioning is described in [VERSIONING.md](VERSIONING.md).

Thanks to the CubicLauncher developers for the project this fork builds on.
