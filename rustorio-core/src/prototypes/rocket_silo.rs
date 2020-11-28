use serde::{Serialize, Deserialize};
        
use super::{Prototype, type_stubs::*};
        
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RocketSilo {
    /// active_energy_usage :: Energy
    active_energy_usage: Energy,    

    /// arm_01_back_animation :: Animation
    arm_01_back_animation: Animation,    

    /// arm_02_right_animation :: Animation
    arm_02_right_animation: Animation,    

    /// arm_03_front_animation :: Animation
    arm_03_front_animation: Animation,    

    /// base_day_sprite :: Sprite
    base_day_sprite: Sprite,    

    /// base_front_sprite :: Sprite
    base_front_sprite: Sprite,    

    /// door_back_open_offset :: vector
    door_back_open_offset: Vector2<f32>,    

    /// door_back_sprite :: Sprite
    door_back_sprite: Sprite,    

    /// door_front_open_offset :: vector
    door_front_open_offset: Vector2<f32>,    

    /// door_front_sprite :: Sprite
    door_front_sprite: Sprite,    

    /// door_opening_speed :: double
    door_opening_speed: f64,    

    /// energy_usage :: Energy
    energy_usage: Energy,    

    /// hole_clipping_box :: BoundingBox
    hole_clipping_box: BoundingBox,    

    /// hole_light_sprite :: Sprite
    hole_light_sprite: Sprite,    

    /// hole_sprite :: Sprite
    hole_sprite: Sprite,    

    /// idle_energy_usage :: Energy
    idle_energy_usage: Energy,    

    /// lamp_energy_usage :: Energy
    lamp_energy_usage: Energy,    

    /// light_blinking_speed :: double
    light_blinking_speed: f64,    

    /// red_lights_back_sprites :: Sprite
    red_lights_back_sprites: Sprite,    

    /// red_lights_front_sprites :: Sprite
    red_lights_front_sprites: Sprite,    

    /// rocket_entity :: string
    rocket_entity: String,    

    /// rocket_glow_overlay_sprite :: Sprite
    rocket_glow_overlay_sprite: Sprite,    

    /// rocket_parts_required :: uint32
    rocket_parts_required: u32,    

    /// rocket_shadow_overlay_sprite :: Sprite
    rocket_shadow_overlay_sprite: Sprite,    

    /// satellite_animation :: Animation
    satellite_animation: Animation,    

    /// satellite_shadow_animation :: Animation
    satellite_shadow_animation: Animation,    

    /// shadow_sprite :: Sprite
    shadow_sprite: Sprite,    

    /// silo_fade_out_end_distance :: double
    silo_fade_out_end_distance: f64,    

    /// silo_fade_out_start_distance :: double
    silo_fade_out_start_distance: f64,    

    /// times_to_blink :: uint8
    times_to_blink: u8,    

    /// alarm_sound :: Sound (optional)
    alarm_sound: Option<Sound>,    

    /// alarm_trigger :: TriggerEffect (optional)
    alarm_trigger: Option<TriggerEffect>,    

    /// base_engine_light :: LightDefinition (optional)
    base_engine_light: Option<LightDefinition>,    

    /// base_light :: LightDefinition (optional)
    base_light: Option<LightDefinition>,    

    /// base_night_sprite :: Sprite (optional)
    base_night_sprite: Option<Sprite>,    

    /// clamps_off_sound :: Sound (optional)
    clamps_off_sound: Option<Sound>,    

    /// clamps_off_trigger :: TriggerEffect (optional)
    clamps_off_trigger: Option<TriggerEffect>,    

    /// clamps_on_sound :: Sound (optional)
    clamps_on_sound: Option<Sound>,    

    /// clamps_on_trigger :: TriggerEffect (optional)
    clamps_on_trigger: Option<TriggerEffect>,    

    /// doors_sound :: Sound (optional)
    doors_sound: Option<Sound>,    

    /// doors_trigger :: TriggerEffect (optional)
    doors_trigger: Option<TriggerEffect>,    

    /// flying_sound :: Sound (optional)
    flying_sound: Option<Sound>,    

    /// raise_rocket_sound :: Sound (optional)
    raise_rocket_sound: Option<Sound>,    

    /// raise_rocket_trigger :: TriggerEffect (optional)
    raise_rocket_trigger: Option<TriggerEffect>,    

    /// rocket_result_inventory_size :: ItemStackIndex (optional)
    rocket_result_inventory_size: Option<ItemStackIndex>,    

}

impl Prototype for RocketSilo {
    const TYPE: Option<&'static str> = Some("rocket-silo");
}


