# Design system

## Direction

Material Design 3 applied to the Windows 10 Settings information architecture.

## Color

- Seed: Windows accent color, read from the registry with a `#0078D4` fallback.
- Roles: primary, on-primary, primary-container, secondary, surface,
  surface-container-low/high/highest, outline, outline-variant, error,
  inverse-surface, and scrim.
- Light and dark schemes are generated with Material color utilities (HCT).
- The app follows the Windows app theme by default and allows an override.

## Typography

Inter, subset to Latin and punctuation.

| Role | Size / line height | Weight |
| --- | --- | --- |
| Headline | 32 / 40 | 600 |
| Title large | 22 / 28 | 600 |
| Title medium | 16 / 24 | 500 |
| Body large | 16 / 24 | 400 |
| Body medium | 14 / 20 | 400 |
| Label large | 14 / 20 | 500 |
| Label small | 11 / 16 | 500 |

## Shape

| Token | Radius |
| --- | --- |
| Extra small | 4 |
| Small | 8 |
| Medium | 12 |
| Large | 16 |
| Extra large | 28 |

## Elevation

Levels 0 to 5 using offset and blur shadows. Settings rows use level 0 or 1;
dialogs use level 3; the navigation drawer uses level 1.

## Layout

- Top app bar: 64 px.
- Navigation drawer: 320 px; collapses to a rail on narrow windows.
- Content: max width 960 px, 24 px padding, 8 px spacing grid.
- Settings row: 64 to 72 px.

## Motion

- Page transition: 200 ms.
- Switch and slider: 150 ms.
- Ripple is limited to interaction feedback and must stop when idle.
