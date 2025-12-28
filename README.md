# Rolodex

A 3D rolodex contact management application built with Rust/Yew and Three.js.

![Screenshot](images/screenshot.png?ts=1735426628000)

## Live Demo

[Try the Live Demo](https://softwarewrighter.github.io/rolodex/?ts=1735426628000)

## Features

- 3D rotating rolodex visualization using Three.js
- Add, edit, and delete contact cards
- Search contacts by name, company, email, or phone
- Alphabetically sorted contact list
- Click cards in 3D view to edit
- Touchpad/scroll wheel navigation
- Local storage persistence
- Populate with fake test data (100 cards)

## Technology Stack

- **Frontend**: Rust/Yew (WebAssembly)
- **3D Visualization**: Three.js
- **Build Tool**: Trunk
- **Storage**: Browser LocalStorage

## Development

### Prerequisites

- Rust (latest stable)
- wasm-pack
- trunk (`cargo install trunk`)

### Build

```bash
./scripts/build.sh
```

### Serve Locally

```bash
./scripts/serve.sh
```

### Deploy

```bash
./scripts/deploy.sh
```

## Documentation

- [Development Process](documentation/process.md)
- [Development Tools](documentation/tools.md)

## License

MIT License - Copyright 2025 Software Wrighter
