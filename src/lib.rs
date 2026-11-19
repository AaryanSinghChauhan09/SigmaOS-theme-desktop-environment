// SPDX-License-Identifier: MIT OR Apache-2.0
//! SigmaOS-theme-desktop-environment — Desktop-shell theme (Cinnamon-style).
//!
//! Provides desktop shell styling models: panel dimensions, workspace indicator layout,
//! Catppuccin Mocha color tokens, window decorations, and theme stylesheet generation.
//!
//! Inspired by Omarchy's coherent Wayland aesthetic, Hyprland's rounded borders & blur,
//! and Catppuccin Mocha's warm dark palette.
//!
//! Principles: clamp-bounded dimensions, pure `std`, zero `unsafe`.

#![forbid(unsafe_code)]

use std::fmt;

// ── Error types ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShellThemeError {
    InvalidPanelHeight(u32),
    InvalidOpacity(u8),
    InvalidBorderRadius(u32),
    InvalidThemeVariant(String),
}

impl fmt::Display for ShellThemeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidPanelHeight(h) => write!(f, "panel height {h}px out of bounds (20..=120px)"),
            Self::InvalidOpacity(o) => write!(f, "opacity {o}% out of bounds (0..=100%)"),
            Self::InvalidBorderRadius(r) => write!(f, "border radius {r}px out of bounds (0..=48px)"),
            Self::InvalidThemeVariant(v) => write!(f, "unrecognized theme variant: '{v}'"),
        }
    }
}

impl std::error::Error for ShellThemeError {}

// ── Shell Theme Configuration ────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PanelPosition {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShellVariant {
    MochaDark,
    LatteLight,
    HighContrast,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShellThemeConfig {
    pub name: String,
    pub variant: ShellVariant,
    pub panel_height_px: u32,
    pub panel_opacity_pct: u8,
    pub border_radius_px: u32,
    pub panel_position: PanelPosition,
    pub show_workspace_names: bool,
    pub accent_color: String,
}

impl Default for ShellThemeConfig {
    fn default() -> Self {
        Self {
            name: "SigmaOS Catppuccin Mocha".into(),
            variant: ShellVariant::MochaDark,
            panel_height_px: 36,
            panel_opacity_pct: 92,
            border_radius_px: 12,
            panel_position: PanelPosition::Top,
            show_workspace_names: true,
            accent_color: "#cba6f7".into(), // Mocha Mauve
        }
    }
}

impl ShellThemeConfig {
    pub fn new(name: impl Into<String>) -> Self {
        let mut s = Self::default();
        s.name = name.into();
        s
    }

    pub fn set_panel_height(&mut self, height: u32) -> Result<(), ShellThemeError> {
        if !(20..=120).contains(&height) {
            return Err(ShellThemeError::InvalidPanelHeight(height));
        }
        self.panel_height_px = height;
        Ok(())
    }

    pub fn set_opacity(&mut self, opacity: u8) -> Result<(), ShellThemeError> {
        if opacity > 100 {
            return Err(ShellThemeError::InvalidOpacity(opacity));
        }
        self.panel_opacity_pct = opacity;
        Ok(())
    }

    pub fn set_border_radius(&mut self, radius: u32) -> Result<(), ShellThemeError> {
        if radius > 48 {
            return Err(ShellThemeError::InvalidBorderRadius(radius));
        }
        self.border_radius_px = radius;
        Ok(())
    }

    pub fn generate_css(&self) -> String {
        format!(
            "/* SigmaOS Shell Theme: {} */\n\
             #panel {{\n\
               height: {}px;\n\
               background-color: rgba(30, 30, 46, {:.2});\n\
               border-radius: {}px;\n\
               accent-color: {};\n\
             }}\n",
            self.name,
            self.panel_height_px,
            (self.panel_opacity_pct as f32) / 100.0,
            self.border_radius_px,
            self.accent_color
        )
    }
}

// ── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_values_are_valid() {
        // Arrange & Act
        let cfg = ShellThemeConfig::default();
        // Assert
        assert_eq!(cfg.panel_height_px, 36);
        assert_eq!(cfg.panel_opacity_pct, 92);
        assert_eq!(cfg.border_radius_px, 12);
        assert_eq!(cfg.variant, ShellVariant::MochaDark);
    }

    #[test]
    fn panel_height_validation_bounds() {
        // Arrange
        let mut cfg = ShellThemeConfig::default();
        // Act
        let valid = cfg.set_panel_height(48);
        let too_small = cfg.set_panel_height(10);
        let too_big = cfg.set_panel_height(200);

        // Assert
        assert!(valid.is_ok());
        assert_eq!(cfg.panel_height_px, 48);
        assert_eq!(too_small, Err(ShellThemeError::InvalidPanelHeight(10)));
        assert_eq!(too_big, Err(ShellThemeError::InvalidPanelHeight(200)));
    }

    #[test]
    fn border_radius_validation_bounds() {
        // Arrange
        let mut cfg = ShellThemeConfig::default();
        // Act
        let valid = cfg.set_border_radius(16);
        let invalid = cfg.set_border_radius(60);

        // Assert
        assert!(valid.is_ok());
        assert_eq!(invalid, Err(ShellThemeError::InvalidBorderRadius(60)));
    }

    #[test]
    fn opacity_validation_bounds() {
        // Arrange
        let mut cfg = ShellThemeConfig::default();
        // Act
        let valid = cfg.set_opacity(80);
        let invalid = cfg.set_opacity(101);

        // Assert
        assert!(valid.is_ok());
        assert_eq!(invalid, Err(ShellThemeError::InvalidOpacity(101)));
    }

    #[test]
    fn generate_css_contains_properties() {
        // Arrange
        let cfg = ShellThemeConfig::default();
        // Act
        let css = cfg.generate_css();
        // Assert
        assert!(css.contains("height: 36px;"));
        assert!(css.contains("border-radius: 12px;"));
        assert!(css.contains("accent-color: #cba6f7;"));
    }

    #[test]
    fn error_display_formatting() {
        // Arrange
        let err = ShellThemeError::InvalidPanelHeight(15);
        // Act & Assert
        assert_eq!(format!("{err}"), "panel height 15px out of bounds (20..=120px)");
    }
}
