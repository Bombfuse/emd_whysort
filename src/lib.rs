use std::collections::HashMap;

use emerald::{serde::Deserialize, Aseprite, Emerald, Entity, Label, Sprite, Transform, World};

pub fn init(emd: &mut Emerald) {
    emd.loader().register_component::<YSort>("y_sort");
}

fn default_offset() -> u32 {
    1
}

#[derive(Deserialize, PartialEq, Eq, Hash, Clone)]
#[serde(crate = "emerald::serde")]
pub struct YSort {
    #[serde(default)]
    layer: u32,

    #[serde(default = "default_offset")]
    spacer: u32,

    /// Only sort this entity once, then remove the YSort component.
    #[serde(default)]
    once: bool,
}

pub fn y_sort_system(world: &mut World) {
    let mut entities_by_layer: HashMap<YSort, Vec<Entity>> = HashMap::new();
    let mut to_remove = Vec::new();
    for (id, y_sort_key) in world.query::<&YSort>().iter() {
        let entities = entities_by_layer
            .entry(y_sort_key.clone())
            .or_insert(Vec::new());
        entities.push(id);
        if y_sort_key.once {
            to_remove.push(id);
        }
    }
    for key in entities_by_layer.keys() {
        let mut entities_and_positions: Vec<(Entity, Transform)> = Vec::new();
        let base = key.layer as f32;
        let spacer = key.spacer as f32;
        let mut delta = spacer;
        if let Some(entities) = entities_by_layer.get(&key) {
            for entity in entities {
                match world.get::<&mut Transform>(entity.clone()) {
                    Ok(position) => entities_and_positions.push((*entity, *position)),
                    Err(_) => {}
                }
            }
        }
        entities_and_positions
            .sort_by(|a, b| b.1.translation.y.partial_cmp(&a.1.translation.y).unwrap());
        for (entity, _) in &mut entities_and_positions {
            match world.get::<&mut Aseprite>(entity.clone()) {
                Ok(mut aseprite) => {
                    aseprite.z_index = base + delta;
                    delta += spacer;
                }
                Err(_) => {}
            }
            match world.get::<&mut Sprite>(entity.clone()) {
                Ok(mut sprite) => {
                    sprite.z_index = base + delta;
                    delta += spacer;
                }
                Err(_) => {}
            }
            match world.get::<&mut Label>(entity.clone()) {
                Ok(mut label) => {
                    label.z_index = base + delta;
                    delta += spacer;
                }
                Err(_) => {}
            }
        }
    }

    for id in to_remove {
        world.remove_one::<YSort>(id).ok();
    }
}
