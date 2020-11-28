use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RocketSiloRocket {
    /// effects_fade_in_end_distance :: double
    effects_fade_in_end_distance: f64,    

    /// effects_fade_in_start_distance :: double
    effects_fade_in_start_distance: f64,    

    /// engine_starting_speed :: double
    engine_starting_speed: f64,    

    /// flying_acceleration :: double
    flying_acceleration: f64,    

    /// flying_speed :: double
    flying_speed: f64,    

    /// full_render_layer_switch_distance :: double
    full_render_layer_switch_distance: f64,    

    /// inventory_size :: ItemStackIndex
    inventory_size: ItemStackIndex,    

    /// rising_speed :: double
    rising_speed: f64,    

    /// rocket_flame_animation :: Animation
    rocket_flame_animation: Animation,    

    /// rocket_flame_left_animation :: Animation
    rocket_flame_left_animation: Animation,    

    /// rocket_flame_left_rotation :: float
    rocket_flame_left_rotation: f32,    

    /// rocket_flame_right_animation :: Animation
    rocket_flame_right_animation: Animation,    

    /// rocket_flame_right_rotation :: float
    rocket_flame_right_rotation: f32,    

    /// rocket_glare_overlay_sprite :: Sprite
    rocket_glare_overlay_sprite: Sprite,    

    /// rocket_launch_offset :: vector
    rocket_launch_offset: Vector2<f32>,    

    /// rocket_render_layer_switch_distance :: double
    rocket_render_layer_switch_distance: f64,    

    /// rocket_rise_offset :: vector
    rocket_rise_offset: Vector2<f32>,    

    /// rocket_shadow_sprite :: Sprite
    rocket_shadow_sprite: Sprite,    

    /// rocket_smoke_bottom1_animation :: Animation
    rocket_smoke_bottom1_animation: Animation,    

    /// rocket_smoke_bottom2_animation :: Animation
    rocket_smoke_bottom2_animation: Animation,    

    /// rocket_smoke_top1_animation :: Animation
    rocket_smoke_top1_animation: Animation,    

    /// rocket_smoke_top2_animation :: Animation
    rocket_smoke_top2_animation: Animation,    

    /// rocket_smoke_top3_animation :: Animation
    rocket_smoke_top3_animation: Animation,    

    /// rocket_sprite :: Sprite
    rocket_sprite: Sprite,    

    /// rocket_visible_distance_from_center :: double
    rocket_visible_distance_from_center: f64,    

    /// shadow_fade_out_end_ratio :: double
    shadow_fade_out_end_ratio: f64,    

    /// shadow_fade_out_start_ratio :: double
    shadow_fade_out_start_ratio: f64,    

    /// shadow_slave_entity :: string
    shadow_slave_entity: String,    

    /// dying_explosion :: string (optional)
    dying_explosion: Option<String>,    

    /// flying_trigger :: TriggerEffect (optional)
    flying_trigger: Option<TriggerEffect>,    

    /// glow_light :: LightDefinition (optional)
    glow_light: Option<LightDefinition>,    

    /// result_items :: table of table (optional)
    result_items: Option<Vec<Vec<Todo>>>,

    /// rocket_initial_offset :: vector (optional)
    rocket_initial_offset: Option<Vector2<f32>>,    

}

impl Prototype for RocketSiloRocket {
    const TYPE: Option<&'static str> = Some("rocket-silo-rocket");
}


