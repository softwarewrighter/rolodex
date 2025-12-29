# Product Requirements Document (PRD)

## Product Overview

**Product Name**: Rolodex

**Description**: A 3D contact management application that provides a nostalgic rolodex-style interface for storing and browsing contact information.

**Target Users**: Anyone who wants a visually engaging way to manage personal or professional contacts.

## Goals and Objectives

### Primary Goals

1. Provide an intuitive 3D interface for browsing contacts
2. Enable quick contact creation, editing, and deletion
3. Support fast searching and filtering of contacts
4. Persist data locally without requiring a server

### Success Metrics

- Users can add and find contacts within 3 clicks
- 3D navigation is smooth (60fps)
- Search returns results instantly (< 100ms)
- Application loads in under 2 seconds

## Features

### Core Features (MVP)

#### F1: Contact Management

**Description**: Create, read, update, and delete contact cards.

**Requirements**:
- [x] Add new contact with name, email, phone, company, notes
- [x] Edit existing contact information
- [x] Delete individual contacts
- [x] Clear all contacts
- [x] Unique ID for each contact (UUID)

**Acceptance Criteria**:
- Form validates required fields (name)
- Changes persist after page reload
- Delete requires no confirmation (direct action)

#### F2: 3D Visualization

**Description**: Display contacts in a 3D rolodex-style view.

**Requirements**:
- [x] 3D scene with rolodex stand
- [x] Cards displayed in vertical stack arrangement
- [x] ~5 cards visible in front, others compressed at edges
- [x] Center card shows full details (name, company, email, phone, notes)
- [x] Surrounding cards show names only
- [x] Smooth scroll/wheel navigation
- [x] Wrap-around navigation (Z -> A, A -> Z)
- [x] Click card to select and edit

**Acceptance Criteria**:
- 3D view renders without errors
- Navigation is smooth (no jitter)
- Selected card is clearly highlighted
- Click opens edit form

#### F3: Search/Filter

**Description**: Filter contacts by name prefix.

**Requirements**:
- [x] Search input in sidebar
- [x] Filter by name prefix (starts with)
- [x] Case-insensitive matching
- [x] Real-time filtering as user types
- [x] Both list and 3D view reflect filter

**Acceptance Criteria**:
- Typing "jo" shows only names starting with "Jo"
- Empty search shows all contacts
- Results update immediately

#### F4: Contact List

**Description**: Sidebar list of all contacts.

**Requirements**:
- [x] Alphabetically sorted list
- [x] Show name, company, email, phone
- [x] Edit and Delete buttons per card
- [x] Click to select (syncs with 3D view)
- [x] Highlight selected card

**Acceptance Criteria**:
- List stays sorted after additions/edits
- Selection syncs bidirectionally with 3D view
- Scroll to show all contacts

#### F5: Data Persistence

**Description**: Store contacts in browser LocalStorage.

**Requirements**:
- [x] Save contacts to LocalStorage
- [x] Load contacts on app start
- [x] Handle storage errors gracefully

**Acceptance Criteria**:
- Contacts persist after page reload
- Contacts persist after browser restart
- No data loss during normal operations

### Additional Features (Implemented)

#### F6: Test Data Generation

**Description**: Populate app with fake contacts for testing.

**Requirements**:
- [x] Generate 100 fake contacts
- [x] Realistic names, emails, phones, companies
- [x] Single-click population

**Acceptance Criteria**:
- Button generates 100 contacts
- Existing contacts are cleared first
- Generated data looks realistic

#### F7: Visual Polish

**Requirements**:
- [x] GitHub corner link to source
- [x] Footer with copyright, license, version info
- [x] Responsive header layout
- [x] Dark theme

### Future Features (Not Implemented)

#### F8: Export/Import

- Export contacts to JSON/CSV
- Import contacts from file
- Backup/restore functionality

#### F9: Categories/Tags

- Assign categories to contacts
- Filter by category
- Color-coded category indicators

#### F10: Photo Support

- Add profile photos to contacts
- Display photos on 3D cards
- Fallback to initials/avatar

#### F11: Cloud Sync

- Optional cloud backup
- Sync across devices
- Account management

## Non-Functional Requirements

### Performance

- Initial load < 2 seconds
- 3D rendering at 60fps
- Search results < 100ms
- Smooth animations (no jank)

### Compatibility

- Modern browsers (Chrome, Firefox, Safari, Edge)
- WebGL support required
- Desktop-first (mobile not prioritized)

### Accessibility

- Keyboard navigation (future)
- Screen reader support (limited - 3D view not accessible)
- High contrast selection highlighting

### Security

- No server-side data storage
- All data local to browser
- No external API calls
- No tracking or analytics

## Technical Constraints

- Must run entirely in browser (no backend)
- WebAssembly for application logic
- Three.js for 3D rendering
- LocalStorage for persistence (5-10MB limit)

## Out of Scope

- Mobile-optimized layout
- Offline PWA capabilities
- Multi-user/sharing features
- Contact import from external sources (vCard, Google, etc.)
- Full-text search (only name prefix)
