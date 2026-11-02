# Extraction Log — SigmaOS-settings

## Source
Extracted from: `SigmaOS-desktop` monorepo, `apps/settings/` subtree.

## What Was Extracted

| Module | Source path | Notes |
|--------|------------|-------|
| display.rs | apps/settings/src/panels/display.rs | Orientation + brightness extracted |
| sound.rs | apps/settings/src/panels/sound.rs | PipeWire calls stubbed |
| network.rs | apps/settings/src/panels/network.rs | NM calls stubbed |
| lib.rs core | apps/settings/src/lib.rs | SettingsStore, SettingsPanel trait |

## Decisions Made During Extraction

1. **PipeWire/NM deps removed from lib** — stubs keep the crate testable in CI.
2. **Accessibility panel moved to separate module** — will be added in next iteration.
3. **Privacy settings** — stored in SettingsStore with `privacy.*` keys; no separate panel yet.

## Known TODOs

- [ ] Real PipeWire device enumeration
- [ ] Real NetworkManager D-Bus calls
- [ ] Accessibility panel (font scale, high contrast, magnifier)
- [ ] User accounts panel
- [ ] Locale / timezone panel with system calls

## Date
2024-01-01
