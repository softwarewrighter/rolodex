# Development Plan

## Project Phases

### Phase 1: Core MVP (Completed)

**Goal**: Basic contact management with 3D visualization

- [x] Project setup (Rust/Yew/WASM)
- [x] Card data model
- [x] LocalStorage persistence
- [x] Basic Yew components (App, CardList, CardForm)
- [x] Three.js integration
- [x] 3D rolodex visualization
- [x] Add/Edit/Delete operations
- [x] Search/filter functionality

### Phase 2: Polish and UX (Completed)

**Goal**: Improve visual quality and user experience

- [x] Infinite rolodex scroll (wrap-around)
- [x] Improved 3D card positioning
- [x] Edge-only selection highlighting
- [x] Prefix-based name filtering
- [x] Test data generation
- [x] GitHub corner link
- [x] Footer with copyright/license/version
- [x] Build scripts for deployment

### Phase 3: Documentation (In Progress)

**Goal**: Comprehensive project documentation

- [x] Architecture documentation
- [x] Product Requirements Document
- [x] Design documentation
- [x] Development plan (this document)
- [ ] Status tracking
- [ ] README updates
- [ ] Process documentation updates

### Phase 4: Future Enhancements (Planned)

**Goal**: Additional features based on user feedback

#### Short Term
- [ ] Keyboard navigation in 3D view
- [ ] Touch/swipe support for mobile
- [ ] Contact photos/avatars
- [ ] Import/Export (JSON, CSV)

#### Medium Term
- [ ] Categories/tags for contacts
- [ ] Multiple rolodex views (by category)
- [ ] Custom card templates
- [ ] Undo/redo support

#### Long Term
- [ ] Progressive Web App (PWA)
- [ ] Optional cloud sync
- [ ] Share contacts feature
- [ ] vCard import/export

## Technical Improvements

### Performance
- [ ] Lazy loading for large contact lists
- [ ] Virtual scrolling in sidebar
- [ ] Texture caching for 3D cards
- [ ] Bundle size optimization

### Code Quality
- [ ] Increase test coverage
- [ ] Add integration tests
- [ ] Add Playwright E2E tests
- [ ] Code documentation improvements

### DevOps
- [ ] GitHub Actions CI/CD
- [ ] Automated testing on PR
- [ ] Automated deployment to GitHub Pages
- [ ] Version management automation

## Known Issues to Address

1. **Mobile Layout**: Not optimized for small screens
2. **Accessibility**: 3D view lacks keyboard navigation
3. **Large Datasets**: Performance untested with 1000+ contacts
4. **Error Handling**: Limited user feedback on storage errors

## Development Guidelines

### Adding New Features

1. Update PRD with feature requirements
2. Update design doc with UI/UX decisions
3. Implement with TDD approach
4. Update architecture doc if structural changes
5. Update status doc with progress

### Bug Fixes

1. Document issue in status.md
2. Write failing test (if applicable)
3. Fix bug
4. Update learnings.md with root cause

### Code Review Checklist

- [ ] Tests pass
- [ ] No clippy warnings
- [ ] Code formatted
- [ ] Documentation updated
- [ ] PR description clear

## Release Checklist

- [ ] All tests pass
- [ ] Build succeeds
- [ ] Screenshot updated
- [ ] Version bumped in Cargo.toml
- [ ] README reflects current state
- [ ] docs/ folder contains latest build
- [ ] Git tag created

## Milestones

| Milestone | Description | Status |
|-----------|-------------|--------|
| v0.1.0 | Initial MVP release | Done |
| v0.2.0 | Polish and UX improvements | Done |
| v0.3.0 | Documentation complete | In Progress |
| v0.4.0 | Mobile support | Planned |
| v0.5.0 | Import/Export | Planned |
| v1.0.0 | Feature complete | Planned |
