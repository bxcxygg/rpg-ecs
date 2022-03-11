use bevy::prelude::Component;
use defaults::Defaults;
use gdnative::export::Export;
use gdnative::prelude::*;
use std::ops::{Deref, DerefMut};

/// Stats Component.
#[derive(Component, Defaults, ToVariant, FromVariant, Copy, Clone)]
pub struct Stats {
    #[def = "4"]
    pub max_health: i32,
    #[def = "4"]
    pub health: i32,
}
impl Export for Stats {
    type Hint = ();
    fn export_info(_hint: Option<Self::Hint>) -> ExportInfo {
        ExportInfo::new(VariantType::Dictionary)
    }
}

/// Roll Component.
#[derive(Component, Defaults, ToVariant, FromVariant, Copy, Clone)]
pub struct Roll {
    #[def = "120.0"]
    pub roll_speed: f32,
    pub roll_velocity: Vector2,
}
impl Export for Roll {
    type Hint = ();
    fn export_info(_hint: Option<Self::Hint>) -> ExportInfo {
        ExportInfo::new(VariantType::Dictionary)
    }
}

/// Accelerates Component.
#[derive(Component, Defaults, ToVariant, FromVariant, Copy, Clone)]
pub struct Acceleration {
    #[def = "80.0"]
    pub max_speed: f32,
    #[def = "500.0"]
    pub acceleration: f32,
}
impl Export for Acceleration {
    type Hint = ();
    fn export_info(_hint: Option<Self::Hint>) -> ExportInfo {
        ExportInfo::new(VariantType::Dictionary)
    }
}

/// Friction Component.
#[derive(Component, Defaults, ToVariant, FromVariant, Copy, Clone)]
pub struct Friction {
    #[def = "400.0"]
    pub friction: f32,
}
impl Export for Friction {
    type Hint = ();
    fn export_info(_hint: Option<Self::Hint>) -> ExportInfo {
        ExportInfo::new(VariantType::Dictionary)
    }
}

/// Velocity Component.
#[derive(Component, Default, Copy, Clone)]
pub struct Velocity {
    pub velocity: Vector2,
}

impl Velocity {
    #[inline]
    fn into_inner(self) -> Vector2 {
        self.velocity
    }
}

impl Deref for Velocity {
    type Target = Vector2;

    fn deref(&self) -> &Self::Target {
        &self.velocity
    }
}

impl DerefMut for Velocity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.velocity
    }
}
