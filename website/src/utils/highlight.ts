import type { BuiltinTheme } from "shiki";

// Single source of truth for the Shiki theme used by every code sample.
// The panes supply their own dark background, so a dark theme keeps token
// colors readable while the shared CSS makes the Shiki background transparent.
export const CODE_THEME: BuiltinTheme = "vitesse-dark";
