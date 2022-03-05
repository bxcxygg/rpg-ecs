use gdrust::gd_ecs_controller;

use crate::plugins::GamePluginGroup;

mod components;
mod delect_box;
mod events;
mod player;
mod plugins;

gd_ecs_controller!(GamePluginGroup);
