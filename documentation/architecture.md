# Architecture

## Overview

Rolodex is a 3D contact management application built with a WebAssembly frontend and JavaScript 3D rendering. The application provides a nostalgic rolodex-style interface for managing contacts with modern web technologies.

## Technology Stack

```
+------------------+     +------------------+     +------------------+
|   Rust/Yew       |     |   Three.js       |     |   Browser        |
|   (WebAssembly)  | <-> |   (3D Rendering) | <-> |   LocalStorage   |
+------------------+     +------------------+     +------------------+
        |                        |                        |
        v                        v                        v
   UI Components            3D Visualization         Data Persistence
```

### Frontend (Rust/Yew -> WebAssembly)

- **Framework**: Yew 0.21 with Client-Side Rendering (CSR)
- **Build Tool**: Trunk
- **Target**: wasm32-unknown-unknown

### 3D Visualization (JavaScript/Three.js)

- **Library**: Three.js (imported via ES modules)
- **Integration**: wasm-bindgen for Rust<->JS interop
- **Rendering**: WebGL via Three.js

### Storage

- **Persistence**: Browser LocalStorage via gloo-storage
- **Format**: JSON serialization with serde

## Directory Structure

```
rolodex/
+-- src/
|   +-- lib.rs              # WASM entry point
|   +-- card.rs             # Card data model
|   +-- storage.rs          # LocalStorage operations
|   +-- test_data.rs        # Fake data generation
|   +-- three_js.rs         # Three.js bindings
|   +-- components/
|       +-- mod.rs          # Component exports
|       +-- app.rs          # Main application component
|       +-- card_form.rs    # Card edit/create form
|       +-- card_list.rs    # Sidebar card list
|       +-- rolodex_3d.rs   # 3D view wrapper
|       +-- search_bar.rs   # Search/filter input
+-- js/
|   +-- rolodex3d.js        # Three.js 3D visualization
+-- index.html              # HTML entry point
+-- Trunk.toml              # Trunk build configuration
+-- Cargo.toml              # Rust dependencies
+-- docs/                   # Build output (GitHub Pages)
+-- scripts/                # Build and deployment scripts
+-- documentation/          # Project documentation
```

## Component Architecture

### Yew Components

```
App (root)
+-- Header
|   +-- Add Card Button
|   +-- Populate Test Data Button
|   +-- Clear All Button
+-- Main
|   +-- Sidebar
|   |   +-- SearchBar
|   |   +-- CardList
|   +-- MainView
|       +-- Rolodex3D
+-- Footer
+-- CardForm (modal, conditional)
```

### Data Flow

```
User Action -> Yew Component -> AppMsg -> App::update() -> State Change
                                                              |
                                                              v
                                                        CardStorage
                                                              |
                                                              v
                                                        LocalStorage
```

## Key Modules

### card.rs

Defines the `Card` struct with fields:
- `id`: UUID string (unique identifier)
- `name`: Contact name
- `email`: Email address
- `phone`: Phone number
- `company`: Company name
- `notes`: Free-text notes

Provides `matches_search()` for prefix-based name filtering.

### storage.rs

`CardStorage` handles all LocalStorage operations:
- `load_cards()`: Retrieve all cards
- `save_cards()`: Persist card array
- `add_card()`: Add new card
- `update_card()`: Update existing card
- `delete_card()`: Remove card by ID
- `clear_all()`: Remove all cards

### three_js.rs

wasm-bindgen bindings to JavaScript functions:
- `initRolodex()`: Initialize 3D scene
- `updateCards()`: Update card meshes
- `rotateToCard()`: Navigate to specific card
- `rotateNext()`/`rotatePrev()`: Navigate with wrapping
- `setCardClickCallback()`: Register click handler
- `disposeRolodex()`: Cleanup resources

### js/rolodex3d.js

Three.js implementation:
- Scene setup with lighting
- Rolodex stand geometry
- Card mesh generation with canvas textures
- Infinite scroll with wrap-around navigation
- Click detection via raycasting
- Smooth animation with requestAnimationFrame

## 3D Visualization Design

### Infinite Rolodex Concept

The 3D view displays cards in an "infinite rolodex" style:
- ~5 cards visible in the front view
- Cards compress toward edges (top/bottom)
- Center card pops forward and shows full details
- Surrounding cards show only names
- Scroll/wheel navigation wraps around (Z -> A, A -> Z)

### Card Positioning Algorithm

```javascript
function getConveyorPosition(trackPos) {
    // Calculate offset from center (-0.5 to 0.5)
    let offset = trackPos - CENTER_OFFSET;

    // Y position: front cards spread out, distant cards compress
    // Z position: center card pops forward, others recede
    // Rotation: center card flat, others tilt away

    return { y, z, rotationX };
}
```

### Selection Highlighting

Selected cards are highlighted with:
- Cyan edge glow (emissive material on sides)
- Subtle front face highlight (low intensity to preserve readability)

## Build Pipeline

```
Rust Source -> cargo build -> WASM -> Trunk Bundle -> docs/
     |                                     |
     v                                     v
  Cargo.toml                          index.html
                                      *.js
                                      *.wasm
                                      styles.css
```

### Build Commands

```bash
# Development build
trunk serve --port 8080

# Production build
./scripts/build.sh

# Output to docs/ for GitHub Pages
```

## Security Considerations

- No server-side component (all data stays in browser)
- No external API calls
- LocalStorage data is browser-specific and not shared
- No authentication required (single-user, local app)

## Performance Considerations

- WASM compiled with LTO and opt-level "z" for size
- Three.js renders only visible cards in detail
- Card textures generated once per card update
- Smooth 60fps animation via requestAnimationFrame
- Debounced scroll input for navigation
