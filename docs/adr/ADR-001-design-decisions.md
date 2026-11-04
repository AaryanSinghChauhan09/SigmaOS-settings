# ADR-001: Settings Design Decisions

**Date:** 2024-01-01  
**Status:** Accepted  
**Component:** SigmaOS-settings

---

## Context

SigmaOS needs a settings UI that covers all standard desktop configuration areas with a consistent, testable, headless-friendly architecture.

## Decisions

### 1. `SettingsPanel` trait for each category

**Rationale:** Interface segregation — each panel (display, sound, network) has its own type with focused responsibilities. The `SettingsApp` orchestrates them without knowing their internals.

### 2. `SettingsStore` is a flat key-value map

**Rationale:** Simple, serializable, and compatible with GSettings-like backends. Keys use `<panel>.<setting>` naming (`display.brightness`).

### 3. Validation in `apply()`, not in setters

**Rationale:** Setters are cheap to call during UI interaction. Validation fires only when the user confirms, matching the GTK settings panel UX model.

### 4. PipeWire and NetworkManager behind feature flags

**Rationale:** CI can build and test the settings logic without the system daemons. Integration tests with real daemons run separately.

### 5. No GTK in lib types

**Rationale:** Panel structs are pure Rust data — no GTK widget pointers. The GTK4 UI layer constructs widgets from panel state, keeping tests headless.

## Consequences

- All 15+ settings categories can be tested without a desktop session.
- D-Bus/PipeWire calls are isolated to adapter types, not in panel logic.
- Settings persistence can target GSettings, TOML files, or a custom store.
