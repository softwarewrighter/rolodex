# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build Commands

```bash
# Development server with hot reload (http://localhost:8080)
trunk serve --port 8080
# or
./scripts/serve.sh

# Production build (outputs to docs/ for GitHub Pages)
./scripts/build.sh

# Run tests (requires wasm-pack)
wasm-pack test --headless --chrome

# Run a single test
wasm-pack test --headless --chrome -- --test <test_name>

# Check compilation
cargo check --target wasm32-unknown-unknown

# Lint
cargo clippy --target wasm32-unknown-unknown

# Format
cargo fmt
```

## Architecture

This is a 3D contact management app built with Rust/Yew (WebAssembly) and Three.js.

### Two-Language Architecture

**Rust/Yew (src/)**: Application logic, state management, UI components
- Compiles to WebAssembly via Trunk
- Yew components handle all UI except 3D rendering
- `wasm-bindgen` bridges Rust <-> JavaScript

**JavaScript (js/rolodex3d.js)**: Three.js 3D visualization
- Imported as ES module via `#[wasm_bindgen(module = "/js/rolodex3d.js")]`
- Handles all 3D rendering, animations, raycasting
- Called from Rust through FFI bindings in `components/rolodex_3d.rs`

### Data Flow

```
User Action -> Yew Component -> AppMsg -> App::update() -> CardStorage -> LocalStorage
                                              |
                                              v
                                         update_cards() -> js/rolodex3d.js -> Three.js
```

### Key Files

| File | Purpose |
|------|---------|
| `src/components/app.rs` | Root component, state management, message handling |
| `src/components/rolodex_3d.rs` | Three.js bindings (wasm-bindgen FFI) |
| `js/rolodex3d.js` | Three.js scene, card rendering, navigation |
| `src/card.rs` | Card data model with search |
| `src/storage.rs` | LocalStorage persistence |

### Three.js Integration Pattern

The `rolodex_3d.rs` component uses conditional compilation to handle tests:

```rust
#[cfg(not(test))]
#[wasm_bindgen(module = "/js/rolodex3d.js")]
extern "C" {
    fn init_rolodex_impl(container_id: &str);
    // ...
}

#[cfg(test)]
fn init_rolodex(_container_id: &str) {
    // No-op in tests
}
```

This allows Rust unit tests to run without loading Three.js.

### 3D Visualization

The "infinite rolodex" in `js/rolodex3d.js`:
- ~5 cards visible in front, others compressed at edges
- Center card pops forward with full details
- Wrap-around navigation (Z -> A, A -> Z)
- Click detection via Three.js raycasting
- Cards rendered as canvas textures on BoxGeometry

## Testing

Tests use `wasm-bindgen-test` and run in a headless browser:

```bash
wasm-pack test --headless --chrome
```

Storage tests in `src/storage.rs` require browser environment for LocalStorage access.

## Build Output

Production builds go to `docs/` for GitHub Pages deployment with `--public-url /rolodex/`.
