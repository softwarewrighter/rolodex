# Design Document

## Design Philosophy

The Rolodex application combines nostalgic UI metaphors with modern web technologies. The 3D rolodex visualization provides an engaging, memorable interface while the sidebar list ensures practical usability.

## Visual Design

### Color Palette

```
Background:     #1a1a2e (Dark blue-gray)
Header/Footer:  #16213e (Darker blue)
Card Background: #ffffff (White)
Card Header:    #1976d2 (Material Blue)
Text Primary:   #ffffff (White on dark)
Text Secondary: #000000 (Black on cards)
Accent:         #4fc3f7 (Light blue)
Selection:      #00ffff (Cyan glow)
Delete Button:  #dc3545 (Red)
```

### Typography

- **Headers**: System sans-serif, bold
- **Body Text**: System sans-serif, regular
- **Card Text**: Arial (canvas rendering)

### Layout

```
+------------------------------------------------------------------+
|  Rolodex                    [+ Add Card] [Populate] [Clear All]  |
+------------------------------------------------------------------+
|  +--------------+  +----------------------------------------+    |
|  | Search...    |  |                                        |    |
|  +--------------+  |              3D Rolodex View           |    |
|  | Card 1  [E][D]  |                                        |    |
|  | Card 2  [E][D]  |            +------------------+        |    |
|  | Card 3  [E][D]  |            |   Contact Name   |        |    |
|  | Card 4  [E][D]  |            |   Company        |        |    |
|  | Card 5  [E][D]  |            |   email@...      |        |    |
|  | ...           |  |            |   (555) 123-4567 |        |    |
|  |               |  |            +------------------+        |    |
|  |               |  |                                        |    |
|  |               |  |         [Previous]  [Next]             |    |
|  +--------------+  +----------------------------------------+    |
+------------------------------------------------------------------+
|  Copyright (c) 2025 Michael A Wright | MIT License | v0.1.0     |
+------------------------------------------------------------------+
```

## 3D Design

### Rolodex Visualization

The 3D view presents an "infinite rolodex" concept:

1. **Center Card**: Fully visible, flat orientation, shows all contact details
2. **Adjacent Cards**: Partially visible, tilted, show names only
3. **Distant Cards**: Compressed toward top/bottom edges
4. **Navigation**: Smooth scroll with wrap-around (infinite loop)

### Card Layout (Canvas Texture)

```
+------------------------------------------+
|  [Blue Header Bar]                       |
|  Contact Name (white, bold)              |
+------------------------------------------+
|  Company Name (blue, bold)               |
|                                          |
|  email@example.com (black)               |
|  (555) 123-4567 (black)                  |
|                                          |
|  Notes text... (gray, smaller)           |
+------------------------------------------+
```

### Selection Highlighting

Selected cards use edge-only highlighting:
- Sides glow cyan (#00ffff)
- Front face has subtle highlight (preserves text readability)
- Clear visual distinction without obscuring content

### Positioning Algorithm

Cards are positioned using a track-based system:

```
Track Position (0-1) -> Screen Position (Y, Z, RotationX)

- trackPos 0.5 = center (eye level)
- trackPos < 0.5 = above center
- trackPos > 0.5 = below center
- Wrap around at 0 and 1
```

**Y Position** (vertical):
- Front cards: Linear spacing (0.55 units apart)
- Distant cards: Compressed using sqrt for gradual compression

**Z Position** (depth):
- Center card pops forward (Z = 4)
- Quadratic falloff for surrounding cards
- Creates clear visual hierarchy

**Rotation** (tilt):
- Center card: Nearly flat
- Adjacent cards: Moderate tilt (0.08 radians per card)
- Distant cards: Steeper tilt (up to 0.5 radians)

## Component Design

### App Component

Central state management using Yew's component model:

```rust
pub struct App {
    cards: Vec<Card>,           // All cards
    filtered_cards: Vec<Card>,  // After search filter
    editing_card: Option<Card>, // Card being edited
    show_form: bool,            // Form visibility
    search_query: String,       // Current filter
    selected_index: Option<usize>, // Selected card index
}
```

### Message Flow

```rust
pub enum AppMsg {
    LoadCards,              // Initial load
    AddCard,                // Open new card form
    EditCard(Card),         // Open edit form
    SelectCardById(String), // 3D click selection
    SaveCard(Card),         // Save form
    DeleteCard(String),     // Delete from list
    DeleteCardFromForm(String), // Delete from form
    ClearAll,               // Clear all cards
    PopulateTestData,       // Generate fake data
    CancelForm,             // Close form
    Search(String),         // Update filter
    SelectCard(usize),      // List selection
}
```

### Form Component

Modal overlay with controlled inputs:
- Each field uses `use_state` hook
- Form values initialized from `props.card`
- Save reconstructs Card from state
- Supports both create (new ID) and edit (preserve ID)

### Three.js Integration

Rust communicates with JavaScript via wasm-bindgen:

```rust
#[wasm_bindgen(module = "/js/rolodex3d.js")]
extern "C" {
    fn initRolodex(container_id: &str);
    fn updateCards(cards_json: &str);
    fn rotateToCard(index: usize);
    // ...
}
```

Callbacks from JavaScript use closures:

```rust
let callback = Closure::wrap(Box::new(move |index: usize, id: String| {
    on_card_click.emit(id);
}) as Box<dyn Fn(usize, String)>);
setCardClickCallback(callback.as_ref().unchecked_ref());
callback.forget(); // Prevent drop
```

## Data Design

### Card Model

```rust
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Card {
    pub id: String,      // UUID v4
    pub name: String,    // Required
    pub email: String,   // Optional
    pub phone: String,   // Optional
    pub company: String, // Optional
    pub notes: String,   // Optional
}
```

### Storage Format

LocalStorage key: `rolodex_cards`

```json
[
  {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "name": "John Doe",
    "email": "john@example.com",
    "phone": "(555) 123-4567",
    "company": "Acme Corp",
    "notes": "Met at conference"
  }
]
```

### Search Algorithm

Prefix-based name matching:

```rust
pub fn matches_search(&self, query: &str) -> bool {
    if query.is_empty() {
        return true;
    }
    self.name.to_lowercase().starts_with(&query.to_lowercase())
}
```

## Interaction Design

### Navigation

1. **Scroll/Wheel**: Navigate through cards (with wrap-around)
2. **Previous/Next Buttons**: Step through cards one at a time
3. **Click Card (3D)**: Select card, open edit form
4. **Click Card (List)**: Select card, sync 3D view, open edit form

### Selection Sync

Bidirectional sync between list and 3D view:
- Clicking in list selects in 3D and opens form
- Clicking in 3D selects in list and opens form
- Both highlight the same card

### Form Interactions

- **Save**: Validate, persist, close form, refresh views
- **Cancel**: Discard changes, close form
- **Delete**: Remove card, close form, refresh views

## Responsive Considerations

Current design is desktop-optimized:
- Sidebar: Fixed 320px width
- Main view: Flexible width
- Minimum viewport: ~1024px recommended

Future mobile considerations:
- Collapsible sidebar
- Touch-friendly 3D navigation
- Larger touch targets

## Accessibility

Current implementation:
- Semantic HTML structure
- High contrast selection colors
- Keyboard form navigation (tab order)

Limitations:
- 3D view not screen reader accessible
- No keyboard navigation in 3D view
- Limited ARIA annotations

## Design Decisions

### Why Prefix Search?

- Intuitive for name lookup (type first letters)
- Fast implementation (no index needed)
- Matches rolodex metaphor (alphabetical tabs)

### Why Edge-Only Highlighting?

- Preserves text readability on selected card
- Clear visual distinction
- Avoids "washed out" appearance of full-card highlighting

### Why Wrap-Around Navigation?

- Matches real rolodex behavior
- Enables continuous browsing
- No "dead ends" at A or Z

### Why LocalStorage?

- No server required
- Instant persistence
- Privacy-friendly (data stays local)
- Sufficient for typical contact list sizes
