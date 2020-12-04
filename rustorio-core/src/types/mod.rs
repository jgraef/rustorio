//! See [1] or [2]
//!
//! [1] https://lua-api.factorio.com/latest/Concepts.html
//! [2] https://wiki.factorio.com/index.php?search=Types
//!

mod bounding_box;
mod collision_mask;

use std::{
    convert::TryFrom,
    fmt::{self, Display, Formatter},
    str::FromStr,
    sync::Arc,
};

use derive_more::{AsMut, AsRef, Display, From, Into};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use palette::LinSrgba;
use mlua::{Value, Table};

use crate::prototypes::{
    ammo_category::AmmoCategory, artillery_flare::ArtilleryFlare, autoplace_control::AutoplaceControl, damage_type::DamageType, equipment::Equipment,
    fuel_category::FuelCategory, fluid::Fluid,
    tile::Tile, Prototype,
};

pub use bounding_box::BoundingBox;
pub use collision_mask::CollisionMask;
pub use nalgebra::{Vector2, Vector3, Point2, Point3};

use rustorio_data::{FromLuaValue, FromLuaTable, Error, to_option, to_result};


pub type Color = LinSrgba<f32>;

pub type SpriteSizeType = i16;

pub type SpriteSize = Vector2<f32>;

pub fn size_from_fields(table: &Table) -> Result<Vector2<SpriteSizeType>, Error> {
    match table.get::<_, Value>("size")? {
        Value::Nil => {
            log::debug!("size_from_fields: size=nil");
            let width = to_option(table.get("width")?)?.unwrap_or_default();
            let height = to_option(table.get("height")?)?.unwrap_or_default();
            Ok(Vector2::new(width, height))
        },
        Value::Table(table) => {
            log::debug!("size_from_fields: size is table");
            Ok(Vector2::from_lua_table(table)?)
        },
        value => {
            log::debug!("size_from_fields: size is value");
            let x = SpriteSizeType::from_lua_value(value)?;
            let y = x.clone();
            Ok(Vector2::new(x, y))
        },
    }

}

pub fn position_from_fields(table: &Table) -> Result<Vector2<SpriteSizeType>, Error> {
    let x: SpriteSizeType = to_option(table.get::<_, Value>("x")?)?.unwrap_or_default();
    let y: SpriteSizeType = to_option(table.get::<_, Value>("y")?)?.unwrap_or_default();

    if x == 0 && y == 0 {
        Ok(to_option(table.get::<_, Value>("size")?)?.unwrap_or_default())
    }
    else {
        Ok(Vector2::default())
    }
}

pub fn dice_from_fields(_table: &Table) -> Result<Vector2<i16>, Error> {
    // TODO
    Ok(Vector2::default())
}


/// Reference to a prototype instance
///
/// During data-phase this is loaded with the prototype's name. After the data-phase this can be resolved to the actual
/// prototype instance.
///
#[derive(Debug, Serialize, Deserialize)]
#[serde(from="String", into="String")]
pub enum PrototypeRef<P: Prototype> {
    Name(String),

    // TODO: I think we might actually attach the prototypes to entities later, so we might store the entity ID here.
    //       Otherwise maybe an `Arc` to the object?
    Ref(Arc<P>),
}

impl<P: Prototype> From<String> for PrototypeRef<P> {
    fn from(s: String) -> Self {
        Self::Name(s)
    }
}

impl<P: Prototype> From<PrototypeRef<P>> for String {
    fn from(p: PrototypeRef<P>) -> Self {
        match p {
            PrototypeRef::Name(s) => s,
            PrototypeRef::Ref(r) => r.name().to_owned(),
        }
    }
}

impl<P: Prototype> Clone for PrototypeRef<P> {
    fn clone(&self) -> Self {
        match self {
            PrototypeRef::Name(s) => PrototypeRef::Name(s.clone()),
            PrototypeRef::Ref(r) => PrototypeRef::Ref(Arc::clone(r)),
        }
    }
}

impl<P: Prototype> FromLuaValue for PrototypeRef<P> {
    fn from_lua_value(value: Value) -> Result<Self, Error> {
        let s = String::from_lua_value(value)?;
        Ok(PrototypeRef::Name(s))
    }
}


#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct AnimatedVector {
    pub frames: Vec<Vector2<f32>>,
    pub render_layer: Option<RenderLayer>,
}

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaValue)]
pub enum RunMode {
    Forward,
    Backward,
    ForwardThenBackward,
}

impl Default for RunMode {
    fn default() -> Self {
        Self::Forward
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
// TODO: This contains all fields from Sprite. So we could just use that flattened
pub struct Animation {
    #[lua(flatten)]
    sprite: Sprite,

    #[lua(default)]
    pub run_mode: RunMode,

    #[lua(default_with="1")]
    pub frame_count: u32, // NOTE: Can't be 0

    /// Once the specified number of pictures is loaded, other pictures are loaded on other line. This is to allow having longer animations in matrix, to avoid pictures with too big width. The game engine limits the width of any input picture to 2048px, so it is compatible with most graphics cards.
    #[lua(default)]
    pub line_length: u32,

    /// Modifier of the animation playing speed, the default is 1, which means one animation frame per tick (60 fps). The speed of playing can often vary depending on the usage (output of steam engine for example). Has to be greater than 0.
    #[lua(default_with="1.")]
    pub animation_speed: f32,

    #[lua(default)]
    pub max_advance: Option<f32>,

    #[lua(default_with="1")]
    pub repeat_count: u8, // NOTE: Can't be 0

    #[lua(default)]
    pub frame_sequence: Option<AnimationFrameSequence>,

    #[lua(default)]
    pub stripes: Vec<Stripe>,
}

impl Animation {
    fn load_hr_version(value: Value) -> Result<Option<Box<Animation>>, Error> {
        let animation: Option<Animation> = to_option(value)?;
        Ok(animation.map(Box::new))
    }
}

pub type Animation4Way = FourWay<Animation>;

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum CardinalDirection {
    North,
    East,
    South,
    West,
}

impl Default for CardinalDirection {
    fn default() -> Self {
        CardinalDirection::North
    }
}

impl From<CardinalDirection> for f32 {
    fn from(dir: CardinalDirection) -> f32 {
        // TODO Verify this
        match dir {
            CardinalDirection::North => 0.,
            CardinalDirection::East => 0.25,
            CardinalDirection::South => 0.5,
            CardinalDirection::West => 0.75,
        }
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Direction {
    fn to_cardinal(&self) -> Option<CardinalDirection> {
        match self {
            Direction::North => Some(CardinalDirection::North),
            Direction::East => Some(CardinalDirection::East),
            Direction::South => Some(CardinalDirection::South),
            Direction::West => Some(CardinalDirection::West),
            _ => None,
        }
    }
}

impl FromLuaValue for Direction {
    fn from_lua_value(value: Value) -> Result<Self, Error> {
        let x = u32::from_lua_value(value)?;
        match x {
            0 => Ok(Direction::North),
            1 => Ok(Direction::NorthEast),
            2 => Ok(Direction::East),
            3 => Ok(Direction::SouthEast),
            4 => Ok(Direction::South),
            5 => Ok(Direction::SouthWest),
            6 => Ok(Direction::West),
            7 => Ok(Direction::NorthWest),
            _ => Err(Error::other(format!("Expected direction value between 0 and 7, but got {}", x))),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct FourWay<T> {
    pub north: T,
    pub east: T,
    pub south: T,
    pub west: T,
}

impl<T> FourWay<T> {
    fn new(north: T, east: T, south: T, west: T) -> Self {
        Self {
            north,
            east,
            south,
            west,
        }
    }

    fn get(&self, dir: CardinalDirection) -> &T {
        match dir {
            CardinalDirection::North => &self.north,
            CardinalDirection::East => &self.east,
            CardinalDirection::South => &self.south,
            CardinalDirection::West => &self.west,
        }
    }
}

impl<T: FromLuaValue> FromLuaTable for FourWay<T> {
    fn from_lua_table(table: Table) -> Result<Self, Error> {
        Ok(Self {
            north: rustorio_data::to_result(table.get::<_, Value>("north")?, || Error::missing_field("north"))?,
            east: rustorio_data::to_result(table.get::<_, Value>("east")?, || Error::missing_field("east"))?,
            south: rustorio_data::to_result(table.get::<_, Value>("south")?, || Error::missing_field("south"))?,
            west: rustorio_data::to_result(table.get::<_, Value>("west")?, || Error::missing_field("west"))?,
        })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SingleOr4Way<T> {
    Single(T),
    FourWay(FourWay<T>),
}

impl<T: FromLuaValue> SingleOr4Way<T> {
    fn get(&self, dir: CardinalDirection) -> &T {
        match self {
            SingleOr4Way::Single(x) => x,
            SingleOr4Way::FourWay(four_way) => four_way.get(dir),
        }
    }
}

impl<T: FromLuaValue> FromLuaValue for SingleOr4Way<T> {
    fn from_lua_value(value: Value) -> Result<Self, Error> {
        match value {
            Value::Table(table) => Ok(SingleOr4Way::FourWay(FourWay::from_lua_table(table)?)),
            value => Ok(SingleOr4Way::Single(T::from_lua_value(value)?)),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, From, Into, AsRef, AsMut, rustorio_data_derive::FromLuaValue)]
pub struct AnimationFrameSequence(Vec<u16>);

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct AnimationSheet {
    #[lua(flatten)]
    pub animation: Animation, // TODO: Does not use `layers` and `filename` is mandatory.

    pub variation_count: u32,

    #[lua(default_with = "1")]
    pub frame_count: u32,

    pub line_length: Option<u32>, // Default is `frame_count`
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AnimationVariations {
    Animation(Animation),
    Animations(Vec<Animation>),
    Sheet(AnimationSheet),
    Sheets(Vec<AnimationSheet>),
}

impl FromLuaTable for AnimationVariations {
    fn from_lua_table(_table: Table) -> Result<Self, Error> {
        unimplemented!()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct AnimationElement {
    #[lua(default)]
    render_layer: RenderLayer,

    secondary_draw_order: i8,

    #[lua(default_with = "true")]
    draw_as_sprite: bool,

    #[lua(default)]
    draw_as_light: bool,

    #[lua(default)]
    apply_tint: bool,

    #[lua(default_with = "true")]
    always_draw: bool,

    animation: Animation,
}

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct BaseAttackParameters {
    pub range: f32,

    pub cooldown: f32,

    #[lua(default)]
    pub min_range: f32,

    #[lua(default_with = "1.")]
    pub turn_range: f32,

    #[lua(default)]
    pub fire_penalty: f32,

    #[lua(default)]
    pub rotate_penalty: f32,

    #[lua(default)]
    pub health_penalty: f32,

    pub min_attack_distance: Option<f32>, // default `range`

    #[lua(default_with = "1.")]
    pub damage_modifier: f32,

    #[lua(default_with = "1.")]
    pub ammo_consumption_modifier: f32,

    #[lua(default)]
    pub cooldown_deviation: f32,

    #[lua(default)]
    pub warmup: u32,

    #[lua(default)]
    pub lead_target_for_projectile_speed: f32,

    pub movement_slow_down_cooldown: Option<f32>, // default `cooldown`

    #[lua(default_with = "1.")]
    pub movement_slow_down_factor: f64,

    pub ammo_type: Option<AmmoType>, // Mandatory if ammo_category is not specified

    pub sound: Option<LayeredSound>,

    pub animation: Option<RotatedAnimation>,

    pub cyclic_sound: Option<CyclicSound>,

    #[lua(default)]
    pub use_shooter_direction: bool,

    // TODO `ammo_categories` and `ammo_category` can be merged like we did with flex_vec.
    #[lua(default)]
    pub ammo_categories: Vec<PrototypeRef<AmmoCategory>>,
    pub ammo_category: Option<PrototypeRef<AmmoCategory>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SpecializedAttackParameters {
    Projectile(ProjectileAttackParameters),
    Beam(BeamAttackParameters),
    Stream(StreamAttackParameters),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AttackParameters {
    pub base: BaseAttackParameters,

    pub specialized: SpecializedAttackParameters,
}

impl FromLuaTable for AttackParameters {
    fn from_lua_table(_table: Table) -> Result<Self, Error> {
        unimplemented!()
    }
}


#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct ProjectileAttackParameters {
    #[lua(default)]
    pub projectile_center: Vector2<f32>,

    #[lua(default)]
    pub projectile_creation_distance: f32,

    pub shell_particle: CircularParticleCreationSpecification,

    pub projectile_creation_parameters: CircularParticleCreationSpecification,

    #[lua(default)]
    pub projectile_orientation_offset: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct BeamAttackParameters {
    #[lua(default)]
    pub source_direction_count: u32,

    pub source_offset: Vector2<f32>,
}

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct StreamAttackParameters {
    #[lua(default)]
    pub fluid_consumption: f32,

    pub gun_barrel_length: f32,

    pub projectile_creation_parameters: CircularParticleCreationSpecification,

    pub gun_center_shift: SingleOr4Way<Vector2<f32>>,

    pub fluids: Vec<StreamAttackFluids>,
}

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct StreamAttackFluids {
    pub r#type: String,

    #[lua(default_with = "1.")]
    pub damage_modifier: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct CircularParticleCreationSpecification {
    pub name: String,

    pub starting_frame_speed: f32,

    #[lua(default_with = "0.25")]
    pub direction: f32,

    #[lua(default)]
    pub direction_deviation: f32,

    #[lua(default_with = "1.0")]
    pub speed: f32,

    #[lua(default)]
    pub speed_deviation: f32,

    #[lua(default)]
    pub starting_frame_speed_deviation: f32,

    #[lua(default_with = "1.0")]
    pub height: f32,

    #[lua(default)]
    pub height_deviation: f32,

    #[lua(default)]
    pub vertical_speed: f32,

    #[lua(default)]
    pub vertical_speed_deviation: f32,

    #[lua(default)]
    pub center: Vector2<f32>,

    #[lua(default)]
    pub creation_distance: f64,

    #[lua(default)]
    pub creation_distance_orientation: f64,

    #[lua(default)]
    pub use_source_position: bool,
}


#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct AmmoType {
    pub category: String,

    pub action: Option<Trigger>,

    #[lua(default)]
    pub clamp_position: bool, // forced `false` if `target_type == TargetType::Entity`.

    pub energy_consumption: Option<Energy>,

    #[lua(default_with = "1.")]
    pub range_modifier: f64,

    #[lua(default_with = "1.")]
    pub cooldown_modifier: f64,

    #[lua(default_with = "1.")]
    pub consumption_modifier: f64,

    #[lua(default)]
    pub target_type: TargetType,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaValue)]
pub enum TargetType {
    Entity,
    Position,
    Direction,
}

impl Default for TargetType {
    fn default() -> Self {
        TargetType::Entity
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, From, Into, AsRef, AsMut, rustorio_data_derive::FromLuaValue)]
pub struct AttackReaction(Vec<AttackReactionItem>);

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct AttackReactionItem {
    pub range: f32,

    pub action: Option<Trigger>,

    #[lua(default)]
    pub reaction_modifier: f32,

    pub damage_type: Option<PrototypeRef<DamageType>>,
}

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct AutoplaceSpecification {
    pub control: Option<PrototypeRef<AutoplaceControl>>,

    #[lua(default_with = "true")]
    pub default_enabled: bool,

    #[lua(default)]
    pub force: Force,

    #[lua(default)]
    pub order: Order,

    #[lua(default_with = "1")]
    pub placement_density: u32,

    #[lua(default)]
    pub tile_restrictions: Vec<TileRestriction>,

    pub probability_expression: NoiseExpression,

    pub richness_expression: NoiseExpression,
    // TODO: peak-based
}

/// Name of tiles an entity is allowed to be placed on
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TileRestriction {
    /// Entity is allowed to be places on this tile.
    Single(PrototypeRef<Tile>),

    /// Entity is allowed to be places on a transition between the two tiles
    ///
    /// # TODO
    ///
    ///  - Since this is reflexive, we should normalize this by sorting the two tile names.
    ///
    Transition([PrototypeRef<Tile>; 2]),
}

impl FromLuaValue for TileRestriction {
    fn from_lua_value(value: Value) -> Result<Self, Error> {
        let err = || Error::other("Expected single string or array with 2 strings.");
        match value {
            Value::String(s) => Ok(Self::Single(PrototypeRef::Name(s.to_str()?.to_owned()))),
            Value::Table(table) => {
                let a = rustorio_data::to_result(table.get::<_, Value>(1)?, err)?;
                let b = rustorio_data::to_result(table.get::<_, Value>(1)?, err)?;
                Ok(Self::Transition([a, b]))
            }
            _ => Err(err())
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(from = "String", into = "String")]
pub enum Force {
    Enemy,
    Player,
    Neutral,
    Other(String),
}

impl Force {
    const ENEMY: &'static str = "enemy";
    const PLAYER: &'static str = "player";
    const NEUTRAL: &'static str = "neutral";
}

impl Default for Force {
    fn default() -> Self {
        Force::Neutral
    }
}

impl From<String> for Force {
    fn from(s: String) -> Self {
        match s.as_str() {
            Self::ENEMY => Self::Enemy,
            Self::PLAYER => Self::Player,
            Self::NEUTRAL => Self::Neutral,
            _ => Self::Other(s),
        }
    }
}

impl From<Force> for String {
    fn from(force: Force) -> Self {
        match force {
            Force::Enemy => Force::ENEMY.to_owned(),
            Force::Player => Force::PLAYER.to_owned(),
            Force::Neutral => Force::NEUTRAL.to_owned(),
            Force::Other(s) => s,
        }
    }
}

impl FromLuaValue for Force {
    fn from_lua_value(value: Value) -> Result<Self, Error> {
        let s = String::from_lua_value(value)?;
        Ok(Force::from(s))
    }
}

impl AsRef<str> for Force {
    fn as_ref(&self) -> &str {
        match self {
            Force::Enemy => Force::ENEMY,
            Force::Player => Force::PLAYER,
            Force::Neutral => Force::NEUTRAL,
            Force::Other(s) => s,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct BeaconGraphicsSet {
    #[lua(default_with = "true")]
    pub draw_animation_when_idle: bool,

    #[lua(default)]
    pub draw_light_when_idle: bool,

    #[lua(default)]
    pub random_animation_offset: bool,

    #[lua(default)]
    pub module_icons_suppressed: bool,

    #[lua(default)]
    pub base_layer: RenderLayer,

    #[lua(default)]
    pub animation_layer: RenderLayer,

    #[lua(default)]
    pub top_layer: RenderLayer,

    #[lua(default_with = "1.")]
    pub animation_progress: f32,

    #[lua(default)]
    pub min_animation_progress: f32,

    #[lua(default_with = "1000.")]
    pub max_animation_progress: f32,

    #[lua(default)]
    pub apply_beacon_tint: ApplyTint,

    #[lua(default)]
    pub apply_module_tint_to_light: ApplyTint,

    #[lua(default)]
    pub no_modules_tint: Color,

    pub animation_list: Vec<AnimationElement>,

    pub light: LightDefinition,

    pub module_visualisation: Vec<BeaconModuleVisualizations>,

    #[lua(default)]
    pub module_tint_mode: ModuleTintMode,
}


#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct BeaconModuleVisualizations {
    art_style: String,

    #[lua(default)]
    use_for_empty_slots: bool,

    #[lua(default)]
    tier_offset: i32,

    /// The outer array contains the different slots, the inner array contains the different layers for those slots
    /// (with different tints etc).
    #[lua(default)]
    slots: Vec<Vec<BeaconModuleVisualization>>,
}

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct BeaconModuleVisualization {
    #[lua(default)]
    pub has_empty_slots: bool,

    #[lua(default)]
    draw_as_light: bool,

    #[lua(default_with = "true")]
    draw_as_sprite: bool,

    #[lua(default)]
    secondary_draw_order: i8,

    apply_module_tint: ApplyTint,

    render_layer: RenderLayer,

    pictures: SpriteVariations,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaValue)]
pub enum ApplyTint {
    Primary,
    Secondary,
    Tertiary,
    Quaternary,
    None,
}

impl Default for ApplyTint {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaValue)]
pub enum ModuleTintMode {
    SingleModule,
    Mix,
}

impl Default for ModuleTintMode {
    fn default() -> Self {
        Self::SingleModule
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaValue)]
pub enum BlendMode {
    Normal,
    Additive,
    AdditiveSoft,
    Multiplicative,
    Overwrite,
}

impl Default for BlendMode {
    fn default() -> Self {
        Self::Normal
    }
}

impl BlendMode {
    fn apply(&self, _active_rgb: Color, _active_alpha: f32, _background_rgb: Color) -> Color {
        /*match self {
            BlendMode::Normal => active_rgb * active_alpha + background_rgb * (1. - active_alpha),
            BlendMode::Additive => active_rgb + background_rgb,
            BlendMode::AdditiveSoft => active_rgb * (color_white() - background_rgb) + background_rgb,
            BlendMode::Multiplicative => active_rgb * background_rgb,
            BlendMode::Overwrite => todo!(),
        }*/
        todo!()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct ThrowCapsuleAction {
    attack_parameters: AttackParameters,

    #[lua(default_with = "true")]
    uses_stack: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct EquipmentRemoteCapsuleAction {
    equipment: PrototypeRef<Equipment>,
}

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct UseOnSelfCapsuleAction {
    attack_parameters: AttackParameters,

    #[lua(default_with = "true")]
    uses_stack: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct ArtilleryRemoteCapsuleAction {
    flare: PrototypeRef<ArtilleryFlare>,

    #[lua(default_with = "true")]
    play_sound_on_failure: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct DestroyCliffsCapsuleAction {
    attack_parameters: AttackParameters,

    radius: f32,

    #[lua(default_with = "3600")]
    timeout: u32,

    #[lua(default_with = "true")]
    play_sound_on_failure: bool,

    #[lua(default_with = "true")]
    uses_stack: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CapsuleAction {
    Throw(ThrowCapsuleAction),
    EquipmentRemote(EquipmentRemoteCapsuleAction),
    UseOnSelf(UseOnSelfCapsuleAction),
    ArtilleryRemote(ArtilleryRemoteCapsuleAction),
    DestroyCliffs(DestroyCliffsCapsuleAction),
}

impl FromLuaTable for CapsuleAction {
    fn from_lua_table(table: Table) -> Result<Self, Error> {
        let ty: String = rustorio_data::to_result(table.get::<_, Value>("type")?, || Error::missing_field("type"))?;

        Ok(match ty.as_str() {
            "throw" => CapsuleAction::Throw(ThrowCapsuleAction::from_lua_table(table)?),
            "equipment-remote" => CapsuleAction::EquipmentRemote(EquipmentRemoteCapsuleAction::from_lua_table(table)?),
            "use-on-self" => CapsuleAction::UseOnSelf(UseOnSelfCapsuleAction::from_lua_table(table)?),
            "artillery-remote" => CapsuleAction::ArtilleryRemote(ArtilleryRemoteCapsuleAction::from_lua_table(table)?),
            "destroy-cliffs" => CapsuleAction::DestroyCliffs(DestroyCliffsCapsuleAction::from_lua_table(table)?),
            _ => return Err(Error::other(format!("Invalid type for CapsuleAction: {}", ty))),
        })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct CircuitConnectorSprites {
    led_red: Sprite,

    led_green: Sprite,

    led_blue: Sprite,

    led_light: LightDefinition,

    connector_main: Option<Sprite>,

    connector_shadow: Option<Sprite>,

    wire_pins: Option<Sprite>,

    wire_pins_shadow: Option<Sprite>,

    led_blue_off: Option<Sprite>,

    blue_led_light_offset: Option<Vector2<f32>>,

    red_green_led_light_offset: Option<Vector2<f32>>,
}

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct ConnectableEntityGraphics();

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct ConsumingType();

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct CyclicSound();

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct CreateTrivialSmokeEffectItem();

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct DamagePrototype();

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct DaytimeColorLookupTable();

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct Effect();

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct EffectTypeLimitation();

#[derive(Clone, Debug, Serialize, Deserialize, From, Into, AsRef, AsMut, PartialEq, Eq, PartialOrd, Ord)]
pub struct Energy(u64);

#[derive(Debug, Error)]
#[error("Can't parse energy value: {0}")]
pub struct EnergyParseError(String);

impl FromStr for Energy {
    type Err = EnergyParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let err = || EnergyParseError(s.to_owned());
        let energy_unit_conversion = match &s[s.len() - 1..] {
            "J" => 1,
            "W" => 60,
            _ => return Err(err()),
        };

        let si_prefix = match &s[s.len() - 2 .. s.len() - 1] {
            "K" | "k" => 1000,
            "M" => 1000000,
            "G" => 1000000000,
            "T" => 1000000000000,
            "P" => 1000000000000000,
            "E" => 1000000000000000000,
            //"Z" => 1000000000000000000000,
            //"Y" => 1000000000000000000000000,
            _ => 1,
        };

        let n = s.len() - if si_prefix == 1 { 1 } else { 2 };
        let n: u64 = s[0..n].parse().map_err(|_| err())?;

        Ok(Energy(n * si_prefix / energy_unit_conversion))
    }
}

impl FromLuaValue for Energy {
    fn from_lua_value(value: Value) -> Result<Self, Error> {
        let s = String::from_lua_value(value)?;
        s.parse().map_err(Error::other)
    }
}



#[derive(Copy, Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaValue)]
pub enum EnergySourceType {
    Electric,
    Burner,
    Heat,
    Fluid,
    Void,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EnergySource {
    r#type: EnergySourceType,

    emissions_per_minute: f64,

    render_no_power_icon: bool,

    render_no_network_icon: bool,

    specialized: SpecializedEnergySource,
}

impl FromLuaTable for EnergySource {
    fn from_lua_table(table: Table) -> Result<Self, Error> {
        log::debug!("{:?}", rustorio_data::value::Table::from_lua_table(table.clone()));

        let r#type = to_result(table.get::<_, Value>("type")?, || Error::missing_field("type"))?;
        let emissions_per_minute = to_option(table.get::<_, Value>("emissions_per_minute")?)?.unwrap_or_default();
        let render_no_power_icon = to_option(table.get::<_, Value>("render_no_power_icon")?)?.unwrap_or(true);
        let render_no_network_icon = to_option(table.get::<_, Value>("render_no_network_icon")?)?.unwrap_or(true);

        let specialized = match r#type {
            EnergySourceType::Electric => SpecializedEnergySource::Electric(ElectricEnergySource::from_lua_table(table.clone())?),
            EnergySourceType::Burner => SpecializedEnergySource::Burner(BurnerEnergySource::from_lua_table(table.clone())?),
            EnergySourceType::Heat => SpecializedEnergySource::Heat(HeatEnergySource::from_lua_table(table.clone())?),
            EnergySourceType::Void => SpecializedEnergySource::Void,
            EnergySourceType::Fluid => SpecializedEnergySource::Fluid(FluidEnergySource::from_lua_table(table.clone())?),
        };

        Ok(EnergySource {
            r#type,
            emissions_per_minute,
            render_no_power_icon,
            render_no_network_icon,
            specialized,
        })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SpecializedEnergySource {
    Electric(ElectricEnergySource),
    Burner(BurnerEnergySource),
    Heat(HeatEnergySource),
    Void,
    Fluid(FluidEnergySource),
}

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct ElectricEnergySource {
    #[lua(default)]
    buffer_capacity: Option<Energy>,

    usage_priority: ElectricUsagePriority,

    #[lua(default)]
    input_flow_limit: Option<Energy>,

    #[lua(default)]
    output_flow_limit: Option<Energy>,

    #[lua(default)]
    drain: Option<Energy>,
}

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaValue)]
pub enum ElectricUsagePriority {
    PrimaryInput,
    PrimaryOutput,
    SecondaryInput,
    SecondaryOutput,
    Tertiary,
    Solar,
    Lamp,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BurnerEnergySource {
    fuel_inventory_size: ItemStackIndex,
    burnt_inventory_size: ItemStackIndex,
    smoke: Vec<SmokeSource>,
    light_flicker: Option<LightFlickeringDefinition>,
    effectivity: f64,
    fuel_categories: Vec<PrototypeRef<FuelCategory>>,
}

impl FromLuaTable for BurnerEnergySource {
    fn from_lua_table(table: Table) -> Result<Self, Error> {
        let fuel_categories = if let Some(fuel_category) = to_option(table.get("fuel_category")?)? {
            vec![fuel_category]
        }
        else {
            to_result(table.get("fuel_categories")?, || Error::missing_field("fuel_categories"))?
        };

        Ok(Self {
            fuel_inventory_size: to_result(table.get::<_, Value>("fuel_inventory_size")?, || Error::missing_field("fuel_inventory_size"))?,
            burnt_inventory_size: to_option(table.get::<_, Value>("burnt_inventory_size")?)?.unwrap_or_default(),
            smoke: to_option(table.get::<_, Value>("smoke")?)?.unwrap_or_default(),
            light_flicker: to_option(table.get::<_, Value>("light_flicker")?)?,
            effectivity: to_option(table.get::<_, Value>("effectivity")?)?.unwrap_or(1.0),
            fuel_categories,
        })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct LightFlickeringDefinition {
    #[lua(default_with="0.2")]
    minimum_intensity: f32,

    #[lua(default_with="0.8")]
    maximum_intensity: f32,

    #[lua(default_with="0.3")]
    derivation_change_frequency: f32,

    #[lua(default_with="0.06")]
    derivation_change_deviation: f32,

    #[lua(default_with="0.2")]
    border_fix_speed: f32,

    #[lua(default_with="0.5")]
    minimum_light_size: f32,

    #[lua(default_with="0.5")]
    light_intensity_to_size_coefficient: f32,

    #[lua(default_with="Color::new(1., 1., 1., 1.)")]
    color: Color,
}

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct HeatEnergySource {
    max_temperature: f64,

    #[lua(default_with="15.0")]
    default_temperature: f64,

    specific_heat: Energy,

    max_transfer: Energy,

    #[lua(default_with="1.0")]
    min_temperature_gradient: f64,

    #[lua(default_with="15.0")]
    min_working_temperature: f64,

    #[lua(default_with="1.0")]
    minimum_glow_temperature: f64,

    #[lua(default)]
    pipe_covers: Option<Sprite4Way>,

    #[lua(default)]
    heat_pipe_covers: Option<Sprite4Way>,

    #[lua(default)]
    heat_picture: Option<Sprite4Way>,

    #[lua(default)]
    heat_glow: Option<Sprite4Way>,

    #[lua(default)]
    connections: Vec<HeatConnection>,
}

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct HeatConnection {
    position: Point2<i32>,

    direction: Direction,
}

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct FluidEnergySource {
    fluid_box: FluidBox,

    #[lua(default)]
    smoke: Vec<SmokeSource>,

    #[lua(default)]
    light_flicker: Option<LightFlickeringDefinition>,

    #[lua(default_with="1.0")]
    effectivity: f64,

    #[lua(default_with="true")]
    burns_fluid: bool,

    #[lua(default)]
    scale_fluid_usage: bool,

    #[lua(default)]
    fluid_usage_per_tick: f64,

    #[lua(default)]
    maximum_temperature: Option<f64>,
}

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaValue)]
pub enum EntityPrototypeFlag {
    NotRotatable,
    PlaceablePlayer,
    PlaceableNeutral,
    PlaceableEnemy,
    PlaceableOffGrid,
    PlayerCreation,
    BuildingDirection8Way,
    FilterDirections,
    FastReplaceableNoBuildWhileMoving,
    BreathsAir,
    NotRepairable,
    NotOnMap,
    NotBlueprintable,
    NotDeconstructable,
    Hidden,
    HideAltInfo,
    FastReplaceableNoCrossTypeWhileMoving,
    NoGapFillWhileBuilding,
    NotFlammable,
    NoAutomatedItemRemoval,
    NoAutomatedItemInsertion,
    NoCopyPaste,
    NotSelectableInGame,
    NotUpgradable,
    NotInKillStatistic,
}

pub type EntityPrototypeFlags = Vec<EntityPrototypeFlag>;

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct EquipmentShape {
    width: u32,
    height: u32,
    r#type: EquipmentShapeType,
    points: Vec<Point2<u32>>,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaValue)]
pub enum EquipmentShapeType {
    Full,
    Manual,
}

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaValue, From, Into, AsRef, AsMut, PartialEq, Eq, Hash, PartialOrd, Ord, Display)]
pub struct FileName(String);

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct FluidBox {
    pipe_connections: Vec<PipeConnectionDefinition>,

    #[lua(default_with="1.0")]
    base_area: f64,

    #[lua(default)]
    base_level: f64,

    #[lua(default_with="1.0")]
    height: f64,

    #[lua(default)]
    filter: Option<PrototypeRef<Fluid>>,

    #[lua(default)]
    render_layer: RenderLayer,

    #[lua(default)]
    pipe_covers: Option<Sprite4Way>,

    #[lua(default)]
    pipe_picture: Option<Sprite4Way>,

    minimum_temperature: f64,

    maximum_temperature: f64,

    #[lua(default)]
    production_type: Option<ProductionType>,

    #[lua(default_with="1")]
    secondary_draw_order: i8,

    #[lua(default_with="FourWay::new(1, 1, 1, 1)")]
    secondary_draw_orders: FourWay<i8>,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaValue)]
pub enum ProductionType {
    Input,
    InputOutput,
    Output,
}

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct PipeConnectionDefinition {
    #[lua(default)]
    position: Option<Vector2<f32>>,
    #[lua(default)]
    positions: Vec<Vector2<f32>>,

    #[lua(default)]
    max_underground_distance: u32,

    r#type: Option<ProductionType>,
}

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct FootstepTriggerEffectList();

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct ForceCondition();

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct HeatBuffer();

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum IconSpecification {
    Multiple {
        icons: Vec<IconData>,
        icon_size: Option<SpriteSizeType>,
        icon_mipmaps: u8,
    },
    Single {
        icon: FileName,
        icon_size: SpriteSizeType,
        icon_mipmaps: u8,
    }

}

impl FromLuaTable for IconSpecification {
    fn from_lua_table(table: Table) -> Result<Self, Error> {
        if let Some(icon) = to_option(table.get("icon")?)? {
            let icon_size = to_result(table.get("icon_size")?, || Error::missing_field("icon_size"))?;
            let icon_mipmaps = to_option(table.get("icon_mipmaps")?)?.unwrap_or_default();
            Ok(Self::Single {
                icon,
                icon_size,
                icon_mipmaps,
            })
        }
        else {
            let icons: Vec<IconData> = to_result(table.get("icons")?, || Error::missing_field("icons"))?;
            if icons.is_empty() {
                return Err(Error::other("At least one icon must be present"));
            }
            let icon_size = to_option(table.get("icon_size")?)?;
            if !icons.iter().all(|icon| icon.icon_size.is_some()) && icon_size.is_none() {
                return Err(Error::other("`icon_size` must be set in the `IconSpecification` if it's not set in all `IconData`s"));
            }
            let icon_mipmaps = to_option(table.get("icon_mipmaps")?)?.unwrap_or_default();
            Ok(Self::Multiple {
                icons,
                icon_size,
                icon_mipmaps,
            })
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct IconData {
    icon: FileName,

    #[lua(default)]
    icon_size: Option<SpriteSizeType>,

    #[lua(default_with="Color::new(0., 0., 0., 1.)")]
    tint: Color,

    #[lua(default)]
    shift: Vector2<f32>,

    #[lua(default_with="1.")]
    scale: f64,

    #[lua(default)]
    icon_mipmaps: u8,
}

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct InterruptibleSound();

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct ItemProductPrototype();

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct ItemPrototypeFlags();

pub type ItemStackIndex = u16;
pub type ItemCountType = u32;
pub type UnitNumber = u32;

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct LayeredSound();

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct LightDefinition {
    #[lua(default)]
    r#type: LightDefinitionType,

    #[lua(default)]
    picture: Option<Sprite>,

    #[lua(default)]
    rotation_shift: Option<RealOrientation>,

    intensity: f32,

    size: f32,

    #[lua(default)]
    source_orientation_offset: RealOrientation,

    #[lua(default)]
    add_perspective: bool,

    #[lua(default)]
    shift: Vector2<f32>,

    #[lua(default)]
    color: Color,

    #[lua(default)]
    minimum_darkness: f32,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaValue)]
pub enum LightDefinitionType {
    Basic,
    Oriented
}

impl Default for LightDefinitionType {
    fn default() -> Self {
        Self::Basic
    }
}

pub type RealOrientation = f32;

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct LocalisedString();

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct Loot();

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct MaterialAmountType();

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct MinableProperties();

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct MiningDrillGraphicsSet();

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct ModuleSpecification();

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
// TODO: Build expressions based on a trait and type parameters. Then only box the outer one. ??
pub struct NoiseExpression();

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Into, From, AsRef, AsMut, Display, rustorio_data_derive::FromLuaValue)]
#[serde(transparent)]
pub struct Order(String);

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct PlaceAsTile();

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct RadiusVisualisationSpecification();

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, rustorio_data_derive::FromLuaValue)]
pub enum RenderLayer {
    WaterTile,
    GroundTile,
    TileTransition,
    Decals,
    LowerRadiusVisualization,
    RadiusVisualization,
    TransportBeltIntegration,
    Resource,
    BuildingSmoke,
    Decorative,
    GroundPatch,
    GroundPatchHigher,
    GroundPatchHigher2,
    Remnants,
    Floor,
    TransportBelt,
    TransportBeltEndings,
    FloorMechanicsUnderCorpse,
    Corpse,
    FloorMechanics,
    Item,
    LowerObject,
    TransportBeltCircuitConnector,
    LowerObjectAboveShadow,
    Object,
    HigherObjectUnder,
    HigherObjectAbove,
    ItemInInserterHand,
    Wires,
    WiresAbove,
    EntityInfoIcon,
    EntityInfoIconAbove,
    Explosion,
    Projectile,
    Smoke,
    AirObject,
    AirEntityInfoIcon,
    LightEffect,
    SelectionBox,
    HigherSelectionBox,
    CollisionSelectionBox,
    Arrow,
    Cursor,
}

impl Default for RenderLayer {
    fn default() -> Self {
        RenderLayer::Object
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct Resistances();

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct RotatedAnimation();

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct RotatedAnimation4Way();

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct RotatedAnimationVariations();

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct RotatedSprite();

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct SignalIDConnector {
    r#type: SignalType,
    name: String,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaValue)]
pub enum SignalType {
    Virtual,
    Item,
    Fluid
}

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct Sound();

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct Sprite {
    #[lua(default)]
    filename: Option<FileName>,

    /// If this property is present, all Sprite definitions have to be placed as entries in the array, and they will all be loaded from there. Each item (Sprite definition) in the array may also have the layers property.
    ///
    /// If this property is present, all other properties are ignored and the mandatory properties do not have to be defined.
    ///
    /// Layers may not be an empty table.
    #[lua(default)]
    layers: Option<Vec<Sprite>>,

    #[lua(with="Sprite::load_hr_version")]
    hr_version: Option<Box<Sprite>>,

    #[lua(default)]
    pub priority: SpritePriority,

    #[lua(default)]
    pub flags: SpriteFlags,

    #[lua(with_context="size_from_fields")]
    pub size: Vector2<SpriteSizeType>,

    #[lua(with_context="position_from_fields", default)]
    pub position: Vector2<SpriteSizeType>,

    #[lua(default)]
    pub shift: Vector2<f32>,

    #[lua(default)]
    pub scale: f64,

    #[lua(default)]
    pub draw_as_shadow: bool,

    #[lua(default)]
    pub mipmap_count: u8,

    #[lua(default)]
    pub apply_runtime_tint: bool,

    #[lua(default_with="Color::new(1., 1., 1., 1.)")]
    pub tint: Color,

    #[lua(default)]
    pub blend_mode: BlendMode,

    #[lua(default)]
    pub load_in_minimal_mode: bool,

    #[lua(default_with = "true")]
    pub premul_alpha: bool,

    #[lua(default)]
    pub generate_sdf: bool,

    #[lua(with_context="dice_from_fields")]
    pub dice: Vector2<SpriteSizeType>,
}

impl Sprite {
    fn load_hr_version(value: Value) -> Result<Option<Box<Sprite>>, Error> {
        let sprite: Option<Sprite> = to_option(value)?;
        Ok(sprite.map(Box::new))
    }
}

pub type Sprite4Way = FourWay<Sprite>;

#[derive(Clone, Debug, Error, Deserialize)]
#[error("Error while parsing SpriteFlag: {0}")]
pub struct SpriteFlagParseError(String);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SpriteFlag {
    NoCrop,
    NotCompressed,
    AlwaysCompressed,
    Mipmap,
    LinearMinification,
    LinearMagnification,
    LinearMipLevel,
    AlphaMask,
    NoScale,
    Mask,
    Icon,
    Gui,
    GuiIcon,
    Light,
    Terrain,
    TerrainEffectMap,
    Shadow,
    Smoke,
    Decal,
    LowObject,
    TrilinearFiltering,
    Group(SpriteGroup),
    Compressed,
}

impl FromLuaValue for SpriteFlag {
    fn from_lua_value(value: Value) -> Result<Self, Error> {
        let s = String::from_lua_value(value)?;
        Self::try_from(s).map_err(|_| Error::other("Expected a SpriteFlag string"))
    }
}

impl TryFrom<String> for SpriteFlag {
    type Error = SpriteFlagParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl From<SpriteFlag> for String {
    fn from(flag: SpriteFlag) -> Self {
        flag.to_string()
    }
}

impl FromStr for SpriteFlag {
    type Err = SpriteFlagParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "no-crop" => SpriteFlag::NoCrop,
            "not-compressed" => SpriteFlag::NotCompressed,
            "always-compressed" => SpriteFlag::AlwaysCompressed,
            "mipmap" => SpriteFlag::Mipmap,
            "linear-minification" => SpriteFlag::LinearMinification,
            "linear-magnification" => SpriteFlag::LinearMagnification,
            "linear-mip-level" => SpriteFlag::LinearMipLevel,
            "alpha-mask" => SpriteFlag::AlphaMask,
            "no-scale" => SpriteFlag::NoScale,
            "mask" => SpriteFlag::Mask,
            "icon" => SpriteFlag::Icon,
            "gui" => SpriteFlag::Gui,
            "gui-icon" => SpriteFlag::GuiIcon,
            "light" => SpriteFlag::Light,
            "terrain" => SpriteFlag::Terrain,
            "terrain-effect-map" => SpriteFlag::TerrainEffectMap,
            "shadow" => SpriteFlag::Shadow,
            "smoke" => SpriteFlag::Smoke,
            "decal" => SpriteFlag::Decal,
            "low-object" => SpriteFlag::LowObject,
            "trilinear-filtering" => SpriteFlag::TrilinearFiltering,
            "compressed" => SpriteFlag::Compressed,
            s if s.starts_with("group=") => SpriteFlag::Group(match &s[6..] {
                "none" => SpriteGroup::None,
                "terrain" => SpriteGroup::Terrain,
                "shadow" => SpriteGroup::Shadow,
                "smoke" => SpriteGroup::Smoke,
                "decal" => SpriteGroup::Decal,
                "low-object" => SpriteGroup::LowObject,
                "gui" => SpriteGroup::Gui,
                "icon" => SpriteGroup::Icon,
                "icon-background" => SpriteGroup::IconBackground,
                _ => return Err(SpriteFlagParseError(s.to_owned())),
            }),
            _ => return Err(SpriteFlagParseError(s.to_owned())),
        })
    }
}

impl Display for SpriteFlag {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            SpriteFlag::NoCrop => write!(f, "no-crop"),
            SpriteFlag::NotCompressed => write!(f, "not-compressed"),
            SpriteFlag::AlwaysCompressed => write!(f, "always-compressed"),
            SpriteFlag::Mipmap => write!(f, "mipmap"),
            SpriteFlag::LinearMinification => write!(f, "linear-minification"),
            SpriteFlag::LinearMagnification => write!(f, "linear-magnification"),
            SpriteFlag::LinearMipLevel => write!(f, "linear-mip-level"),
            SpriteFlag::AlphaMask => write!(f, "always-mask"),
            SpriteFlag::NoScale => write!(f, "no-scale"),
            SpriteFlag::Mask => write!(f, "mask"),
            SpriteFlag::Icon => write!(f, "icon"),
            SpriteFlag::Gui => write!(f, "gui"),
            SpriteFlag::GuiIcon => write!(f, "gui-icon"),
            SpriteFlag::Light => write!(f, "light"),
            SpriteFlag::Terrain => write!(f, "terrain"),
            SpriteFlag::TerrainEffectMap => write!(f, "terrain-effect-map"),
            SpriteFlag::Shadow => write!(f, "shadow"),
            SpriteFlag::Smoke => write!(f, "smoke"),
            SpriteFlag::Decal => write!(f, "decal"),
            SpriteFlag::LowObject => write!(f, "low-object"),
            SpriteFlag::TrilinearFiltering => write!(f, "trilinear-filtering"),
            SpriteFlag::Compressed => write!(f, "compressed"),
            SpriteFlag::Group(group) => write!(
                f,
                "group={}",
                match group {
                    SpriteGroup::None => "none",
                    SpriteGroup::Terrain => "terrain",
                    SpriteGroup::Shadow => "shadow",
                    SpriteGroup::Smoke => "smoke",
                    SpriteGroup::Decal => "decal",
                    SpriteGroup::LowObject => "low-object",
                    SpriteGroup::Gui => "gui",
                    SpriteGroup::Icon => "icon",
                    SpriteGroup::IconBackground => "icon-background",
                }
            ),
        }
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaValue)]
pub enum SpriteGroup {
    None,
    Terrain,
    Shadow,
    Smoke,
    Decal,
    LowObject,
    Gui,
    Icon,
    IconBackground,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, rustorio_data_derive::FromLuaValue)]
pub struct SpriteFlags(Vec<SpriteFlag>);

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaValue)]
pub enum SpritePriority {
    ExtraHighNoScale,
    ExtraHigh,
    High,
    Medium,
    Low,
    VeryLow,
    NoAtlas,
}

impl Default for SpritePriority {
    fn default() -> Self {
        SpritePriority::Medium
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct SpriteVariations();

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct Stripe();

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct TileTransitions();

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct TransportBeltConnectorFrame();

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct Trigger();

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct TriggerEffect();

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct TriggerTargetMask();

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct UnitAISettings();

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct WaterReflectionDefinition();

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct WireConnectionPoint {
    wire: WirePosition,
    shadow: WirePosition,
}

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct WirePosition {
    #[lua(default)]
    copper: Option<Vector2<f32>>,
    #[lua(default)]
    red: Option<Vector2<f32>>,
    #[lua(default)]
    green: Option<Vector2<f32>>,
}

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct WorkingSound();

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct CharacterArmorAnimation();

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct FootprintParticle();

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct OrientedCliffPrototype();

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct WorkingVisualisation();

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct UnitSpawnDefinition();

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct SmokeSource();

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct SignalColorMapping();

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct PumpConnectorGraphics();

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct IngredientPrototype();

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct ProductPrototype();

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct Modifier();

#[derive(Clone, Debug, Serialize, Deserialize, rustorio_data_derive::FromLuaTable)]
pub struct TreePrototypeVariation();

// Fix usages
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Todo();

impl FromLuaTable for Todo {
    fn from_lua_table(_table: Table) -> Result<Self, Error> {
        todo!()
    }
}
