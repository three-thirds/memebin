# Memebin :p

## Your memes finally in one place :p

![Backend](https://img.shields.io/badge/Backend-Rust-green)
![Frontend](https://img.shields.io/badge/Frontend-Svelte-red)
![Front Backend](https://img.shields.io/badge/FrontBackend-Typescript-blue)

## Features

- **Drag and drop** Anything anywhere
- **Simple popup** with a quick keybind
- **Dark and Light mode** slider

## Insallation

1. Go to the [**Releases**](https://github.com/three-thirds/memebin/releases) page
2. Download the installer for your OS on the latest release

## Some Known Issues

Tauri and AppImages are NOTORIOUSLY famously bad combinations (we realized this a bit too late)
So if on linux, please try out installing from .deb or .rpm files. 
The app has been tested on Fedora 44, if it does not seem to work dm @Chish on slack to help try to fix it
Thank you.

## Local Development

```bash
#clone repo
git clone https://github.com/three-thirds/memebin.git
cd memebin

#install dependencies
pnpm install

# run app in dev mode
pnpm tauri dev
```

### Dependencies

- [Rust](https://rustup.rs)
- [Tauri Prequisites](https://tauri.app/start/prerequisites/)

## Built with

- [Tauri](https://tauri.app)
- [Svelte](https://svelte.dev)
- [Tailwind CSS](https://tailwindcss.com)
- [shadcn-svelte](https://shaDcn-svelte.com)
- [Rust](https://rustup.rs)

Made with ❤️ by [Three Thirds](https://github.com/three-thirds) for [Third Space](https://thirdspace.hackclub.com)
