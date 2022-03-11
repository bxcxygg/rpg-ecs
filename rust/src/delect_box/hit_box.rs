use bevy::prelude::{Bundle, Component};
use defaults::Defaults;
use gdnative::api::Area2D;
use gdnative::export::Export;
use gdnative::prelude::*;
use gdrust::macros::*;

/// Damage Component.
#[derive(Component, Defaults, ToVariant, FromVariant, Copy, Clone)]
pub struct Damage {
    #[def = "1"]
    pub damage: i32,
}
impl Export for Damage {
    type Hint = ();
    fn export_info(_hint: Option<Self::Hint>) -> ExportInfo {
        ExportInfo::new(VariantType::Dictionary)
    }
}

/// Knockback Component.
#[derive(Component, Default, Copy, Clone)]
pub struct Knockback {
    pub vector: Vector2,
}

/// HitBox Component.
#[derive(Component, Defaults, Copy, Clone)]
pub struct HitBox {
    #[def = "Area2D::new().into_shared()"]
    pub owner: Ref<Area2D>,
}

#[gdrust(extends = Area2D)]
#[derive(Bundle, Default, Copy, Clone)]
pub struct HitBoxBundle {
    #[default(HitBox { owner: _owner.claim()})]
    pub hitbox: HitBox,
    #[export]
    pub damage: Damage,
    pub knockback: Knockback,
}
#[methods]
impl HitBoxBundle {}
