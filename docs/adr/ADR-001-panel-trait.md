# ADR-001: SettingsPanel as a trait object

**Status:** Accepted  
**Date:** 2024-01-01

## Context

The number and type of settings panels will grow over time. We need a
registration mechanism that doesn't require modifying `SettingsApp` for each
new panel.

## Decision

`SettingsPanel` is a `dyn Trait`. Panels are registered as `Box<dyn SettingsPanel>`.
`SettingsApp::register_panel` accepts any conforming type.

## Consequences

- New panels can be added without touching `SettingsApp` (open-closed).
- Dynamic dispatch overhead is negligible for a settings UI (not a hot path).
- Panels must implement `fmt::Debug` for diagnostics.
