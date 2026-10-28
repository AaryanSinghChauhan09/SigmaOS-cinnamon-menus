// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-cinnamon-menus - Cinnamon Menus Component
//! Inspired by Linux Mint cinnamon-menus
//! Provides menu functionality following SOLID principles

#![no_std]

/// Menu manager
pub struct CinnamonMenus {
    name: &'static str,
}

impl CinnamonMenus {
    pub fn new() -> Self {
        Self { name: "SigmaOS Cinnamon Menus" }
    }
}

impl Default for CinnamonMenus {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_menus_creation() {
        let menus = super::CinnamonMenus::new();
        assert_eq!(menus.name, "SigmaOS Cinnamon Menus");
    }
}
