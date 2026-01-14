# Ralph Wiggum Guide

A guide to using the Ralph Wiggum technique for iterative AI-driven development with Claude Code.

## What is Ralph Wiggum?

The Ralph Wiggum technique, created by Geoffrey Huntley, is an iterative development methodology that runs Claude Code in a continuous loop until a task is complete. Named after the Simpsons character, it embodies persistent iteration despite setbacks.

### Core Concept

At its simplest, Ralph is a bash loop:

```bash
while :; do cat PROMPT.md | claude-code ; done
```

The official Claude Code plugin implements this more elegantly using a **Stop hook** that intercepts exit attempts and feeds the same prompt back. Each iteration:

1. Claude receives the same prompt
2. Works on the task, modifying files
3. Tries to exit
4. Stop hook intercepts and feeds the same prompt again
5. Claude sees its previous work in files
6. Iterates until completion criteria are met

### Key Philosophy

- **"Deterministically bad in an undeterministic world"** - failures are predictable and informative
- **Iteration > Perfection** - don't aim for perfect on first try; let the loop refine
- **Failures are data** - use them to tune prompts
- **Operator skill matters** - success depends on writing good prompts

## Installation

The ralph-wiggum plugin is available from the official Claude Code plugins:

```bash
/install-plugin ralph-wiggum
```

## Commands

### /ralph-loop

Start a Ralph loop in your current session.

```bash
/ralph-loop "<prompt>" --max-iterations <n> --completion-promise "<text>"
```

**Options:**
- `--max-iterations <n>` - Maximum iterations before auto-stop (recommended as safety net)
- `--completion-promise <text>` - Phrase that signals completion (matched via `<promise>` tag)

### /cancel-ralph

Cancel an active Ralph loop:

```bash
/cancel-ralph
```

## Completion Promises

To signal completion, Claude outputs a `<promise>` tag:

```
<promise>TASK COMPLETE</promise>
```

The stop hook detects this tag. Without it (or `--max-iterations`), Ralph runs indefinitely.

## Writing Effective Prompts

### 1. Clear Completion Criteria

**Bad:**
```
Build a todo API and make it good.
```

**Good:**
```markdown
Build a REST API for todos.

When complete:
- All CRUD endpoints working
- Input validation in place
- Tests passing (coverage > 80%)
- README with API docs
- Output: <promise>COMPLETE</promise>
```

### 2. Incremental Goals

**Bad:**
```
Create a complete e-commerce platform.
```

**Good:**
```markdown
Phase 1: User authentication (JWT, tests)
Phase 2: Product catalog (list/search, tests)
Phase 3: Shopping cart (add/remove, tests)

Output <promise>COMPLETE</promise> when all phases done.
```

### 3. Self-Correction Instructions

**Bad:**
```
Write code for feature X.
```

**Good:**
```markdown
Implement feature X following TDD:
1. Write failing tests
2. Implement feature
3. Run tests
4. If any fail, debug and fix
5. Refactor if needed
6. Repeat until all green
7. Output: <promise>COMPLETE</promise>
```

### 4. Escape Hatches

Always use `--max-iterations` as a safety net:

```bash
/ralph-loop "Try to implement feature X" --max-iterations 20
```

In your prompt, include what to do if stuck:
```markdown
After 15 iterations, if not complete:
- Document what's blocking progress
- List what was attempted
- Suggest alternative approaches
- Output: <promise>BLOCKED</promise>
```

## When to Use Ralph

**Good for:**
- Well-defined tasks with clear success criteria
- Tasks requiring iteration and refinement (e.g., getting tests to pass)
- Greenfield projects where you can walk away
- Tasks with automatic verification (tests, linters, type checkers)

**Not good for:**
- Tasks requiring human judgment or design decisions
- One-shot operations
- Tasks with unclear success criteria
- Production debugging (use targeted debugging instead)

## Security Considerations

Ralph often requires elevated permissions to work autonomously. Options:

1. **Pre-approve specific commands** in `.claude/settings.local.json`
2. **Use `--dangerously-skip-permissions`** (only in sandboxed environments)
3. **Run in disposable VMs** for maximum isolation

**Warning:** Never run Ralph with skip-permissions on a machine with sensitive data or production access.

## Cost Considerations

Autonomous loops consume tokens. A 50-iteration loop on a large codebase can cost $50-100+ in API credits. Mitigate by:

- Setting conservative `--max-iterations`
- Writing prompts that converge quickly
- Monitoring usage during long runs

---

# Rolodex Project Examples

Specific Ralph prompts for iterating on this 3D Rolodex contact manager.

## Example 1: Add a New Feature

```bash
/ralph-loop "Add a feature to export contacts as CSV.

Requirements:
1. Add 'Export CSV' button to the UI
2. Generate valid CSV with headers: Name, Company, Email, Phone
3. Trigger browser download when clicked
4. Add test coverage for the export logic

Verification:
- Run: trunk build
- Run: wasm-pack test --headless --chrome
- Manually verify button appears and download works

Output <promise>EXPORT COMPLETE</promise> when all requirements met and tests pass." --max-iterations 15 --completion-promise "EXPORT COMPLETE"
```

## Example 2: Fix Failing Tests

```bash
/ralph-loop "Fix all failing tests in this project.

Process:
1. Run: wasm-pack test --headless --chrome
2. Analyze failures
3. Fix the root cause (not just the test)
4. Re-run tests
5. Repeat until all pass

Output <promise>TESTS GREEN</promise> when all tests pass." --max-iterations 20 --completion-promise "TESTS GREEN"
```

## Example 3: Improve 3D Visualization

```bash
/ralph-loop "Improve the 3D rolodex card readability.

Current issues:
- Cards at edges are hard to read
- Text overlaps on small viewports

Requirements:
1. Improve card spacing algorithm in js/rolodex3d.js
2. Ensure center card is always fully readable
3. Add responsive handling for small screens
4. Test on viewport widths: 320px, 768px, 1280px

Verification:
- Run: trunk build
- Visual inspection at each viewport size

Output <promise>VISUALIZATION IMPROVED</promise> when cards are readable at all sizes." --max-iterations 25 --completion-promise "VISUALIZATION IMPROVED"
```

## Example 4: Refactor with Type Safety

```bash
/ralph-loop "Add comprehensive type safety to the Rust codebase.

Tasks:
1. Run: cargo clippy --target wasm32-unknown-unknown
2. Fix all clippy warnings
3. Add explicit type annotations where inference is ambiguous
4. Ensure no unwrap() calls without error context

Verification:
- cargo clippy returns no warnings
- cargo check passes
- All tests pass

Output <promise>TYPES CLEAN</promise> when clippy is happy and tests pass." --max-iterations 15 --completion-promise "TYPES CLEAN"
```

## Example 5: Performance Optimization

```bash
/ralph-loop "Optimize Three.js rendering performance.

Focus areas in js/rolodex3d.js:
1. Reduce unnecessary re-renders
2. Implement object pooling for cards
3. Optimize texture updates
4. Add requestAnimationFrame throttling if needed

Verification:
- No visible jank when scrolling through 100+ cards
- Console shows no WebGL warnings
- Memory usage stable over time

Output <promise>PERFORMANCE OPTIMIZED</promise> when smooth scrolling achieved." --max-iterations 20 --completion-promise "PERFORMANCE OPTIMIZED"
```

## Tips for This Project

1. **Always run `trunk build`** as verification - catches Rust and JS integration issues
2. **Use `wasm-pack test --headless --chrome`** for automated test verification
3. **Check both Rust (`src/`) and JavaScript (`js/`)** - this is a two-language architecture
4. **The 3D visualization is in `js/rolodex3d.js`** - Three.js changes go there
5. **State management is in `src/components/app.rs`** - Yew component logic lives here

## Learn More

- [Original Ralph technique](https://ghuntley.com/ralph/)
- [Official plugin documentation](https://github.com/anthropics/claude-code/blob/main/plugins/ralph-wiggum/README.md)
- [Ralph Orchestrator](https://github.com/mikeyobrien/ralph-orchestrator)
