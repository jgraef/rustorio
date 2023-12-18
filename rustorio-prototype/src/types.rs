#![allow(dead_code)]

use std::{
    convert::TryFrom,
    fmt::{
        self,
        Display,
        Formatter,
    },
    path::Path,
    str::FromStr,
};

use bigdecimal::BigDecimal;
use derive_more::{
    AsMut,
    AsRef,
    Display,
    From,
    Into,
};
use lazy_static::lazy_static;
use nalgebra::{
    Point2,
    Vector2,
};
use num::BigInt;
use regex::Regex;
#[cfg(feature = "lua-api")]
use rustorio_lua_api::{
    to_option,
    to_result,
    Error,
    FromLuaTable,
    FromLuaValue,
    Table,
    Value,
};
#[cfg(feature = "serde")]
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    item::ItemPrototype,
    Id,
};

pub type Color = palette::LinSrgba;

pub type SpriteSizeType = i16;

pub type SpriteSize = Vector2<f32>;

#[cfg(feature = "lua-api")]
fn size_from_fields(table: &Table) -> Result<Vector2<SpriteSizeType>, Error> {
    match table.get::<_, Value>("size")? {
        Value::Nil => {
            log::debug!("size_from_fields: size=nil");
            let width = to_option(table.get("width")?)?.unwrap_or_default();
            let height = to_option(table.get("height")?)?.unwrap_or_default();
            Ok(Vector2::new(width, height))
        }
        Value::Table(table) => {
            log::debug!("size_from_fields: size is table");
            Ok(Vector2::from_lua_table(table)?)
        }
        value => {
            log::debug!("size_from_fields: size is value");
            let x = SpriteSizeType::from_lua_value(value)?;
            let y = x.clone();
            Ok(Vector2::new(x, y))
        }
    }
}

#[cfg(feature = "lua-api")]
fn position_from_fields(table: &Table) -> Result<Vector2<SpriteSizeType>, Error> {
    let x: SpriteSizeType = to_option(table.get::<_, Value>("x")?)?.unwrap_or_default();
    let y: SpriteSizeType = to_option(table.get::<_, Value>("y")?)?.unwrap_or_default();

    if x == 0 && y == 0 {
        Ok(to_option(table.get::<_, Value>("size")?)?.unwrap_or_default())
    }
    else {
        Ok(Vector2::default())
    }
}

#[cfg(feature = "lua-api")]
fn dice_from_fields(_table: &Table) -> Result<Vector2<i16>, Error> {
    // TODO
    Ok(Vector2::default())
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BoundingBox {
    pub top_left: Point2<f32>,
    pub bottom_right: Point2<f32>,
    pub orientation: Option<f32>,
}

#[cfg(feature = "lua-api")]
impl FromLuaTable for BoundingBox {
    fn from_lua_table(table: Table) -> Result<Self, Error> {
        let err = || Error::other("2 points defining a bounding-box");
        let top_left = to_result::<Point2<f32>, _, _>(table.get::<_, Value>(1)?, err)?;
        let bottom_right = to_result::<Point2<f32>, _, _>(table.get::<_, Value>(2)?, err)?;
        let orientation = to_option::<f32>(table.get::<_, Value>(3)?)?;

        Ok(BoundingBox {
            top_left,
            bottom_right,
            orientation,
        })
    }
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CollisionMask {
    /*
    GroundTile,
    WaterTile,
    ResourceLayer,
    DoodadLayer,
    FloorLayer,
    ItemLayer,
    GhostLayer,
    ObjectLayer,
    PlayerLayer,
    TrainLayer,
    Layer11,
    Layer12,
    Layer13,
    Layer14,
    Layer15,
    */
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AnimatedVector {
    pub frames: Vec<Vector2<f32>>,
    pub render_layer: Option<RenderLayer>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaValue))]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "kebab-case")
)]
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

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
// TODO: This contains all fields from Sprite. So we could just use that
// flattened
pub struct Animation {
    #[cfg_attr(feature = "lua-api", lua(flatten))]
    sprite: Sprite,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub run_mode: RunMode,

    #[cfg_attr(feature = "lua-api", lua(default_with = "1"))]
    pub frame_count: u32, // NOTE: Can't be 0

    /// Once the specified number of pictures is loaded, other pictures are
    /// loaded on other line. This is to allow having longer animations in
    /// matrix, to avoid pictures with too big width. The game engine limits the
    /// width of any input picture to 2048px, so it is compatible with most
    /// graphics cards.
    #[cfg_attr(feature = "lua-api", lua(default))]
    pub line_length: u32,

    /// Modifier of the animation playing speed, the default is 1, which means
    /// one animation frame per tick (60 fps). The speed of playing can often
    /// vary depending on the usage (output of steam engine for example). Has to
    /// be greater than 0.
    #[cfg_attr(feature = "lua-api", lua(default_with = "1."))]
    pub animation_speed: f32,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub max_advance: Option<f32>,

    #[cfg_attr(feature = "lua-api", lua(default_with = "1"))]
    pub repeat_count: u8, // NOTE: Can't be 0

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub frame_sequence: Option<AnimationFrameSequence>,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub stripes: Vec<Stripe>,
}

impl Animation {
    #[cfg(feature = "lua-api")]
    fn load_hr_version(value: Value) -> Result<Option<Box<Animation>>, Error> {
        let animation: Option<Animation> = to_option(value)?;
        Ok(animation.map(Box::new))
    }
}

pub type Animation4Way = FourWay<Animation>;

#[derive(Copy, Clone, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "kebab-case")
)]
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

#[derive(Copy, Clone, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "kebab-case")
)]
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

#[cfg(feature = "lua-api")]
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
            _ => {
                Err(Error::other(format!(
                    "Expected direction value between 0 and 7, but got {}",
                    x
                )))
            }
        }
    }
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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

#[cfg(feature = "lua-api")]
impl<T: FromLuaValue> FromLuaTable for FourWay<T> {
    fn from_lua_table(table: Table) -> Result<Self, Error> {
        Ok(Self {
            north: to_result(table.get::<_, Value>("north")?, || {
                Error::missing_field("north")
            })?,
            east: to_result(table.get::<_, Value>("east")?, || {
                Error::missing_field("east")
            })?,
            south: to_result(table.get::<_, Value>("south")?, || {
                Error::missing_field("south")
            })?,
            west: to_result(table.get::<_, Value>("west")?, || {
                Error::missing_field("west")
            })?,
        })
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(untagged))]
pub enum SingleOr4Way<T> {
    Single(T),
    FourWay(FourWay<T>),
}

#[cfg(feature = "lua-api")]
impl<T: FromLuaValue> SingleOr4Way<T> {
    fn get(&self, dir: CardinalDirection) -> &T {
        match self {
            SingleOr4Way::Single(x) => x,
            SingleOr4Way::FourWay(four_way) => four_way.get(dir),
        }
    }
}

#[cfg(feature = "lua-api")]
impl<T: FromLuaValue> FromLuaValue for SingleOr4Way<T> {
    fn from_lua_value(value: Value) -> Result<Self, Error> {
        match value {
            Value::Table(table) => Ok(SingleOr4Way::FourWay(FourWay::from_lua_table(table)?)),
            value => Ok(SingleOr4Way::Single(T::from_lua_value(value)?)),
        }
    }
}

#[derive(Clone, Debug, Default, From, Into, AsRef, AsMut)]
#[cfg_attr(feature = "lua-api", derive(FromLuaValue))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AnimationFrameSequence(Vec<u16>);

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AnimationSheet {
    #[cfg_attr(feature = "lua-api", lua(flatten))]
    pub animation: Animation, // TODO: Does not use `layers` and `filename` is mandatory.

    pub variation_count: u32,

    #[cfg_attr(feature = "lua-api", lua(default_with = "1"))]
    pub frame_count: u32,

    pub line_length: Option<u32>, // Default is `frame_count`
}

#[derive(Clone, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "lowercase")
)]
pub enum AnimationVariations {
    Animation(Animation),
    Animations(Vec<Animation>),
    Sheet(AnimationSheet),
    Sheets(Vec<AnimationSheet>),
}

#[cfg(feature = "lua-api")]
impl FromLuaTable for AnimationVariations {
    fn from_lua_table(_table: Table) -> Result<Self, Error> {
        unimplemented!()
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AnimationElement {
    #[cfg_attr(feature = "lua-api", lua(default))]
    pub render_layer: RenderLayer,

    pub secondary_draw_order: i8,

    #[cfg_attr(feature = "lua-api", lua(default_with = "true"))]
    pub draw_as_sprite: bool,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub draw_as_light: bool,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub apply_tint: bool,

    #[cfg_attr(feature = "lua-api", lua(default_with = "true"))]
    pub always_draw: bool,

    pub animation: Animation,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BaseAttackParameters {
    pub range: f32,

    pub cooldown: f32,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub min_range: f32,

    #[cfg_attr(feature = "lua-api", lua(default_with = "1."))]
    pub turn_range: f32,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub fire_penalty: f32,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub rotate_penalty: f32,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub health_penalty: f32,

    pub min_attack_distance: Option<f32>, // default `range`

    #[cfg_attr(feature = "lua-api", lua(default_with = "1."))]
    pub damage_modifier: f32,

    #[cfg_attr(feature = "lua-api", lua(default_with = "1."))]
    pub ammo_consumption_modifier: f32,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub cooldown_deviation: f32,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub warmup: u32,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub lead_target_for_projectile_speed: f32,

    pub movement_slow_down_cooldown: Option<f32>, // default `cooldown`

    #[cfg_attr(feature = "lua-api", lua(default_with = "1."))]
    pub movement_slow_down_factor: f64,

    pub ammo_type: Option<AmmoType>, // Mandatory if ammo_category is not specified

    pub sound: Option<LayeredSound>,

    pub animation: Option<RotatedAnimation>,

    pub cyclic_sound: Option<CyclicSound>,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub use_shooter_direction: bool,

    // TODO `ammo_categories` and `ammo_category` can be merged like we did with flex_vec.
    #[cfg_attr(feature = "lua-api", lua(default))]
    pub ammo_categories: Vec<Id<AmmoCategory>>,
    pub ammo_category: Option<Id<AmmoCategory>>,
}

pub type AmmoCategory = Todo;

#[derive(Clone, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "lowercase")
)]
pub enum SpecializedAttackParameters {
    Projectile(ProjectileAttackParameters),
    Beam(BeamAttackParameters),
    Stream(StreamAttackParameters),
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AttackParameters {
    pub base: BaseAttackParameters,

    pub specialized: SpecializedAttackParameters,
}

#[cfg(feature = "lua-api")]
impl FromLuaTable for AttackParameters {
    fn from_lua_table(_table: Table) -> Result<Self, Error> {
        unimplemented!()
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ProjectileAttackParameters {
    #[cfg_attr(feature = "lua-api", lua(default))]
    pub projectile_center: Vector2<f32>,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub projectile_creation_distance: f32,

    pub shell_particle: CircularParticleCreationSpecification,

    pub projectile_creation_parameters: CircularParticleCreationSpecification,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub projectile_orientation_offset: f32,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BeamAttackParameters {
    #[cfg_attr(feature = "lua-api", lua(default))]
    pub source_direction_count: u32,

    pub source_offset: Vector2<f32>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct StreamAttackParameters {
    #[cfg_attr(feature = "lua-api", lua(default))]
    pub fluid_consumption: f32,

    pub gun_barrel_length: f32,

    pub projectile_creation_parameters: CircularParticleCreationSpecification,

    pub gun_center_shift: SingleOr4Way<Vector2<f32>>,

    pub fluids: Vec<StreamAttackFluids>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct StreamAttackFluids {
    pub r#type: String,

    #[cfg_attr(feature = "lua-api", lua(default_with = "1."))]
    pub damage_modifier: f64,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CircularParticleCreationSpecification {
    pub name: String,

    pub starting_frame_speed: f32,

    #[cfg_attr(feature = "lua-api", lua(default_with = "0.25"))]
    pub direction: f32,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub direction_deviation: f32,

    #[cfg_attr(feature = "lua-api", lua(default_with = "1.0"))]
    pub speed: f32,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub speed_deviation: f32,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub starting_frame_speed_deviation: f32,

    #[cfg_attr(feature = "lua-api", lua(default_with = "1.0"))]
    pub height: f32,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub height_deviation: f32,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub vertical_speed: f32,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub vertical_speed_deviation: f32,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub center: Vector2<f32>,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub creation_distance: f64,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub creation_distance_orientation: f64,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub use_source_position: bool,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AmmoType {
    pub category: String,

    pub action: Option<Trigger>,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub clamp_position: bool, // forced `false` if `target_type == TargetType::Entity`.

    pub energy_consumption: Option<Energy>,

    #[cfg_attr(feature = "lua-api", lua(default_with = "1."))]
    pub range_modifier: f64,

    #[cfg_attr(feature = "lua-api", lua(default_with = "1."))]
    pub cooldown_modifier: f64,

    #[cfg_attr(feature = "lua-api", lua(default_with = "1."))]
    pub consumption_modifier: f64,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub target_type: TargetType,
}

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaValue))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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

#[derive(Clone, Debug, Default, From, Into, AsRef, AsMut)]
#[cfg_attr(feature = "lua-api", derive(FromLuaValue))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(transparent))]
pub struct AttackReaction(Vec<AttackReactionItem>);

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AttackReactionItem {
    pub range: f32,

    pub action: Option<Trigger>,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub reaction_modifier: f32,

    pub damage_type: Option<Id<DamageType>>,
}

pub type DamageType = Todo;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AutoplaceSpecification {
    pub control: Option<Id<AutoplaceControl>>,

    #[cfg_attr(feature = "lua-api", lua(default_with = "true"))]
    pub default_enabled: bool,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub force: Force,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub order: Order,

    #[cfg_attr(feature = "lua-api", lua(default_with = "1"))]
    pub placement_density: u32,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub tile_restrictions: Vec<TileRestriction>,

    pub probability_expression: NoiseExpression,

    pub richness_expression: NoiseExpression,
    // TODO: peak-based
}

pub type AutoplaceControl = Todo;

/// Name of tiles an entity is allowed to be placed on
#[derive(Clone, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "lowercase")
)]
pub enum TileRestriction {
    /// Entity is allowed to be places on this tile.
    Single(Id<Tile>),

    /// Entity is allowed to be places on a transition between the two tiles
    ///
    /// # TODO
    ///
    ///  - Since this is reflexive, we should normalize this by sorting the two
    ///    tile names.
    Transition([Id<Tile>; 2]),
}

pub type Tile = Todo;

#[cfg(feature = "lua-api")]
impl FromLuaValue for TileRestriction {
    fn from_lua_value(value: Value) -> Result<Self, Error> {
        let err = || Error::other("Expected single string or array with 2 strings.");
        match value {
            Value::String(s) => Ok(Self::Single(s.to_str()?.to_owned().into())),
            Value::Table(table) => {
                let a = to_result(table.get::<_, Value>(1)?, err)?;
                let b = to_result(table.get::<_, Value>(2)?, err)?;
                Ok(Self::Transition([a, b]))
            }
            _ => Err(err()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(from = "String", into = "String")
)]
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

#[cfg(feature = "lua-api")]
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

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BeaconGraphicsSet {
    #[cfg_attr(feature = "lua-api", lua(default_with = "true"))]
    pub draw_animation_when_idle: bool,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub draw_light_when_idle: bool,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub random_animation_offset: bool,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub module_icons_suppressed: bool,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub base_layer: RenderLayer,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub animation_layer: RenderLayer,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub top_layer: RenderLayer,

    #[cfg_attr(feature = "lua-api", lua(default_with = "1."))]
    pub animation_progress: f32,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub min_animation_progress: f32,

    #[cfg_attr(feature = "lua-api", lua(default_with = "1000."))]
    pub max_animation_progress: f32,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub apply_beacon_tint: ApplyTint,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub apply_module_tint_to_light: ApplyTint,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub no_modules_tint: Color,

    pub animation_list: Vec<AnimationElement>,

    pub light: LightDefinition,

    pub module_visualisation: Vec<BeaconModuleVisualizations>,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub module_tint_mode: ModuleTintMode,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BeaconModuleVisualizations {
    pub art_style: String,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub use_for_empty_slots: bool,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub tier_offset: i32,

    /// The outer array contains the different slots, the inner array contains
    /// the different layers for those slots (with different tints etc).
    #[cfg_attr(feature = "lua-api", lua(default))]
    pub slots: Vec<Vec<BeaconModuleVisualization>>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BeaconModuleVisualization {
    #[cfg_attr(feature = "lua-api", lua(default))]
    pub has_empty_slots: bool,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub draw_as_light: bool,

    #[cfg_attr(feature = "lua-api", lua(default_with = "true"))]
    pub draw_as_sprite: bool,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub secondary_draw_order: i8,

    pub apply_module_tint: ApplyTint,

    pub render_layer: RenderLayer,

    pub pictures: SpriteVariations,
}

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BeaconVisualizationTints {
    #[cfg_attr(feature = "lua-api", lua(default))]
    pub primary: Option<Color>,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub secondary: Option<Color>,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub tertiary: Option<Color>,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub quaternary: Option<Color>,
}

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaValue))]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "lowercase")
)]
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

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaValue))]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "kebab-case")
)]
pub enum ModuleTintMode {
    SingleModule,
    Mix,
}

impl Default for ModuleTintMode {
    fn default() -> Self {
        Self::SingleModule
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaValue))]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "kebab-case")
)]
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

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ThrowCapsuleAction {
    pub attack_parameters: AttackParameters,

    #[cfg_attr(feature = "lua-api", lua(default_with = "true"))]
    pub uses_stack: bool,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EquipmentRemoteCapsuleAction {
    pub equipment: Id<Equipment>,
}

pub type Equipment = Todo;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UseOnSelfCapsuleAction {
    pub attack_parameters: AttackParameters,

    #[cfg_attr(feature = "lua-api", lua(default_with = "true"))]
    pub uses_stack: bool,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ArtilleryRemoteCapsuleAction {
    pub flare: Id<ArtilleryFlare>,

    #[cfg_attr(feature = "lua-api", lua(default_with = "true"))]
    pub play_sound_on_failure: bool,
}

pub type ArtilleryFlare = Todo;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DestroyCliffsCapsuleAction {
    pub attack_parameters: AttackParameters,

    pub radius: f32,

    #[cfg_attr(feature = "lua-api", lua(default_with = "3600"))]
    pub timeout: u32,

    #[cfg_attr(feature = "lua-api", lua(default_with = "true"))]
    pub play_sound_on_failure: bool,

    #[cfg_attr(feature = "lua-api", lua(default_with = "true"))]
    pub uses_stack: bool,
}

#[derive(Clone, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "kebab-case")
)]
pub enum CapsuleAction {
    Throw(ThrowCapsuleAction),
    EquipmentRemote(EquipmentRemoteCapsuleAction),
    UseOnSelf(UseOnSelfCapsuleAction),
    ArtilleryRemote(ArtilleryRemoteCapsuleAction),
    DestroyCliffs(DestroyCliffsCapsuleAction),
}

#[cfg(feature = "lua-api")]
impl FromLuaTable for CapsuleAction {
    fn from_lua_table(table: Table) -> Result<Self, Error> {
        let ty: String = to_result(table.get::<_, Value>("type")?, || {
            Error::missing_field("type")
        })?;

        Ok(match ty.as_str() {
            "throw" => CapsuleAction::Throw(ThrowCapsuleAction::from_lua_table(table)?),
            "equipment-remote" => {
                CapsuleAction::EquipmentRemote(EquipmentRemoteCapsuleAction::from_lua_table(table)?)
            }
            "use-on-self" => {
                CapsuleAction::UseOnSelf(UseOnSelfCapsuleAction::from_lua_table(table)?)
            }
            "artillery-remote" => {
                CapsuleAction::ArtilleryRemote(ArtilleryRemoteCapsuleAction::from_lua_table(table)?)
            }
            "destroy-cliffs" => {
                CapsuleAction::DestroyCliffs(DestroyCliffsCapsuleAction::from_lua_table(table)?)
            }
            _ => {
                return Err(Error::other(format!(
                    "Invalid type for CapsuleAction: {}",
                    ty
                )))
            }
        })
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CircuitConnectorSprites {
    pub led_red: Sprite,

    pub led_green: Sprite,

    pub led_blue: Sprite,

    pub led_light: LightDefinition,

    pub connector_main: Option<Sprite>,

    pub connector_shadow: Option<Sprite>,

    pub wire_pins: Option<Sprite>,

    pub wire_pins_shadow: Option<Sprite>,

    pub led_blue_off: Option<Sprite>,

    pub blue_led_light_offset: Option<Vector2<f32>>,

    pub red_green_led_light_offset: Option<Vector2<f32>>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ConnectableEntityGraphics();

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ConsumingType();

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CyclicSound();

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CreateTrivialSmokeEffectItem();

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DamagePrototype();

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DaytimeColorLookupTable();

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Energy {
    pub value: BigDecimal,
    pub unit: EnergyUnit,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "kebab-case")
)]
pub enum EnergyUnit {
    Joule,
    Watt,
}

#[derive(Debug, thiserror::Error)]
#[error("Can't parse energy value: {0}")]
pub struct EnergyParseError(String);

lazy_static! {
    static ref ENERGY_VALUE_REGEX: Regex = r"(\d+(\.\d+)?)([KkMGTPEZY])?([JW])".parse().unwrap();
}

impl FromStr for Energy {
    type Err = EnergyParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let err = || EnergyParseError(s.to_owned());

        let captures = ENERGY_VALUE_REGEX.captures(s).ok_or_else(err)?;

        let value: BigDecimal = captures
            .get(1)
            .ok_or_else(err)?
            .as_str()
            .parse()
            .map_err(|_| err())?;

        let exponent = match captures.get(3).map(|m| m.as_str()) {
            Some("K") | Some("k") => 3,
            Some("M") => 6,
            Some("G") => 9,
            Some("T") => 12,
            Some("P") => 15,
            Some("E") => 18,
            Some("Z") => 21,
            Some("Y") => 24,
            None => 0,
            _ => return Err(err()),
        };
        let factor = BigInt::from(10).pow(exponent);
        let value = value * factor;

        let unit = match captures.get(4).ok_or_else(err)?.as_str() {
            "J" => EnergyUnit::Joule,
            "W" => EnergyUnit::Watt,
            _ => return Err(err()),
        };

        Ok(Energy { value, unit })
    }
}

#[cfg(feature = "lua-api")]
impl FromLuaValue for Energy {
    fn from_lua_value(value: Value) -> Result<Self, Error> {
        let s = String::from_lua_value(value)?;
        s.parse().map_err(Error::other)
    }
}

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaValue))]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "kebab-case")
)]
pub enum EnergySourceType {
    Electric,
    Burner,
    Heat,
    Fluid,
    Void,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EnergySource {
    pub r#type: EnergySourceType,

    pub emissions_per_minute: f64,

    pub render_no_power_icon: bool,

    pub render_no_network_icon: bool,

    pub specialized: SpecializedEnergySource,
}

#[cfg(feature = "lua-api")]
impl FromLuaTable for EnergySource {
    fn from_lua_table(table: Table) -> Result<Self, Error> {
        let r#type = to_result(table.get::<_, Value>("type")?, || {
            Error::missing_field("type")
        })?;
        let emissions_per_minute =
            to_option(table.get::<_, Value>("emissions_per_minute")?)?.unwrap_or_default();
        let render_no_power_icon =
            to_option(table.get::<_, Value>("render_no_power_icon")?)?.unwrap_or(true);
        let render_no_network_icon =
            to_option(table.get::<_, Value>("render_no_network_icon")?)?.unwrap_or(true);

        let specialized = match r#type {
            EnergySourceType::Electric => {
                SpecializedEnergySource::Electric(ElectricEnergySource::from_lua_table(
                    table.clone(),
                )?)
            }
            EnergySourceType::Burner => {
                SpecializedEnergySource::Burner(BurnerEnergySource::from_lua_table(table.clone())?)
            }
            EnergySourceType::Heat => {
                SpecializedEnergySource::Heat(HeatEnergySource::from_lua_table(table.clone())?)
            }
            EnergySourceType::Void => SpecializedEnergySource::Void,
            EnergySourceType::Fluid => {
                SpecializedEnergySource::Fluid(FluidEnergySource::from_lua_table(table.clone())?)
            }
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

#[derive(Clone, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "kebab-case")
)]
pub enum SpecializedEnergySource {
    Electric(ElectricEnergySource),
    Burner(BurnerEnergySource),
    Heat(HeatEnergySource),
    Void,
    Fluid(FluidEnergySource),
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ElectricEnergySource {
    #[cfg_attr(feature = "lua-api", lua(default))]
    pub buffer_capacity: Option<Energy>,

    pub usage_priority: ElectricUsagePriority,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub input_flow_limit: Option<Energy>,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub output_flow_limit: Option<Energy>,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub drain: Option<Energy>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaValue))]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "kebab-case")
)]
pub enum ElectricUsagePriority {
    PrimaryInput,
    PrimaryOutput,
    SecondaryInput,
    SecondaryOutput,
    Tertiary,
    Solar,
    Lamp,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BurnerEnergySource {
    pub fuel_inventory_size: ItemStackIndex,
    pub burnt_inventory_size: ItemStackIndex,
    pub smoke: Vec<SmokeSource>,
    pub light_flicker: Option<LightFlickeringDefinition>,
    pub effectivity: f64,
    pub fuel_categories: Vec<Id<FuelCategory>>,
}

pub type FuelCategory = Todo;

#[cfg(feature = "lua-api")]
impl FromLuaTable for BurnerEnergySource {
    fn from_lua_table(table: Table) -> Result<Self, Error> {
        let fuel_categories = if let Some(fuel_category) = to_option(table.get("fuel_category")?)? {
            vec![fuel_category]
        }
        else {
            to_result(table.get("fuel_categories")?, || {
                Error::missing_field("fuel_categories")
            })?
        };

        Ok(Self {
            fuel_inventory_size: to_result(table.get::<_, Value>("fuel_inventory_size")?, || {
                Error::missing_field("fuel_inventory_size")
            })?,
            burnt_inventory_size: to_option(table.get::<_, Value>("burnt_inventory_size")?)?
                .unwrap_or_default(),
            smoke: to_option(table.get::<_, Value>("smoke")?)?.unwrap_or_default(),
            light_flicker: to_option(table.get::<_, Value>("light_flicker")?)?,
            effectivity: to_option(table.get::<_, Value>("effectivity")?)?.unwrap_or(1.0),
            fuel_categories,
        })
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LightFlickeringDefinition {
    #[cfg_attr(feature = "lua-api", lua(default_with = "0.2"))]
    pub minimum_intensity: f32,

    #[cfg_attr(feature = "lua-api", lua(default_with = "0.8"))]
    pub maximum_intensity: f32,

    #[cfg_attr(feature = "lua-api", lua(default_with = "0.3"))]
    pub derivation_change_frequency: f32,

    #[cfg_attr(feature = "lua-api", lua(default_with = "0.06"))]
    pub derivation_change_deviation: f32,

    #[cfg_attr(feature = "lua-api", lua(default_with = "0.2"))]
    pub border_fix_speed: f32,

    #[cfg_attr(feature = "lua-api", lua(default_with = "0.5"))]
    pub minimum_light_size: f32,

    #[cfg_attr(feature = "lua-api", lua(default_with = "0.5"))]
    pub light_intensity_to_size_coefficient: f32,

    #[cfg_attr(feature = "lua-api", lua(default_with = "Color::new(1., 1., 1., 1.)"))]
    pub color: Color,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct HeatEnergySource {
    pub max_temperature: f64,

    #[cfg_attr(feature = "lua-api", lua(default_with = "15."))]
    pub default_temperature: f64,

    pub specific_heat: Energy,

    pub max_transfer: Energy,

    #[cfg_attr(feature = "lua-api", lua(default_with = "1."))]
    pub min_temperature_gradient: f64,

    #[cfg_attr(feature = "lua-api", lua(default_with = "15."))]
    pub min_working_temperature: f64,

    #[cfg_attr(feature = "lua-api", lua(default_with = "1."))]
    pub minimum_glow_temperature: f64,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub pipe_covers: Option<Sprite4Way>,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub heat_pipe_covers: Option<Sprite4Way>,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub heat_picture: Option<Sprite4Way>,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub heat_glow: Option<Sprite4Way>,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub connections: Vec<HeatConnection>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct HeatConnection {
    pub position: Point2<i32>,

    pub direction: Direction,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FluidEnergySource {
    pub fluid_box: FluidBox,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub smoke: Vec<SmokeSource>,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub light_flicker: Option<LightFlickeringDefinition>,

    #[cfg_attr(feature = "lua-api", lua(default_with = "1."))]
    pub effectivity: f64,

    #[cfg_attr(feature = "lua-api", lua(default_with = "true"))]
    pub burns_fluid: bool,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub scale_fluid_usage: bool,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub fluid_usage_per_tick: f64,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub maximum_temperature: Option<f64>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaValue))]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "kebab-case")
)]
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

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EquipmentShape {
    pub width: u32,
    pub height: u32,
    pub r#type: EquipmentShapeType,
    pub points: Vec<Point2<u32>>,
}

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaValue))]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "kebab-case")
)]
pub enum EquipmentShapeType {
    Full,
    Manual,
}

#[derive(Clone, Debug, From, Into, AsRef, AsMut, PartialEq, Eq, Hash, PartialOrd, Ord, Display)]
#[cfg_attr(feature = "lua-api", derive(FromLuaValue))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(transparent))]
pub struct FileName(String);

impl FileName {
    pub fn as_path(&self) -> &Path {
        Path::new(&self.0)
    }
}

impl AsRef<Path> for FileName {
    fn as_ref(&self) -> &Path {
        self.as_path()
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FluidBox {
    pub pipe_connections: Vec<PipeConnectionDefinition>,

    #[cfg_attr(feature = "lua-api", lua(default_with = "1."))]
    pub base_area: f64,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub base_level: f64,

    #[cfg_attr(feature = "lua-api", lua(default_with = "1."))]
    pub height: f64,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub filter: Option<Id<Fluid>>,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub render_layer: RenderLayer,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub pipe_covers: Option<Sprite4Way>,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub pipe_picture: Option<Sprite4Way>,

    pub minimum_temperature: f64,

    pub maximum_temperature: f64,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub production_type: Option<ProductionType>,

    #[cfg_attr(feature = "lua-api", lua(default_with = "1"))]
    pub secondary_draw_order: i8,

    #[cfg_attr(feature = "lua-api", lua(default_with = "FourWay::new(1, 1, 1, 1)"))]
    pub secondary_draw_orders: FourWay<i8>,
}

pub type Fluid = Todo;

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaValue))]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "kebab-case")
)]
pub enum ProductionType {
    Input,
    InputOutput,
    Output,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PipeConnectionDefinition {
    #[cfg_attr(feature = "lua-api", lua(default))]
    pub position: Option<Vector2<f32>>,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub positions: Vec<Vector2<f32>>,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub max_underground_distance: u32,

    pub r#type: Option<ProductionType>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FootstepTriggerEffectList();

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ForceCondition();

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct HeatBuffer();

#[derive(Clone, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "lowercase")
)]
pub enum IconSpecification {
    Multiple {
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
        icons: Vec<IconData>,
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        icon_size: Option<SpriteSizeType>,
        icon_mipmaps: u8,
    },
    Single {
        icon: FileName,
        icon_size: SpriteSizeType,
        icon_mipmaps: u8,
    },
    None,
}

#[cfg(feature = "lua-api")]
impl FromLuaTable for IconSpecification {
    fn from_lua_table(table: Table) -> Result<Self, Error> {
        let icon: Option<FileName> = to_option(table.get("icon")?)?;
        let icons: Option<Vec<IconData>> = to_option(table.get("icons")?)?;

        if let Some(icon) = icon {
            let icon_size = to_result(table.get("icon_size")?, || {
                Error::missing_field("icon_size")
            })?;
            let icon_mipmaps = to_option(table.get("icon_mipmaps")?)?.unwrap_or_default();
            Ok(Self::Single {
                icon,
                icon_size,
                icon_mipmaps,
            })
        }
        else if let Some(icons) = icons {
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
        else {
            Ok(Self::None)
        }
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct IconData {
    pub icon: FileName,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub icon_size: Option<SpriteSizeType>,

    #[cfg_attr(feature = "lua-api", lua(default_with = "Color::new(0., 0., 0., 1.)"))]
    pub tint: Color,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub shift: Vector2<f32>,

    #[cfg_attr(feature = "lua-api", lua(default_with = "1."))]
    pub scale: f64,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub icon_mipmaps: u8,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct InterruptibleSound();

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ItemPrototypeFlags();

pub type ItemStackIndex = u16;
pub type ItemCountType = u32;
pub type UnitNumber = u32;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LayeredSound();

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LightDefinition {
    #[cfg_attr(feature = "lua-api", lua(default))]
    pub r#type: LightDefinitionType,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub picture: Option<Sprite>,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub rotation_shift: Option<RealOrientation>,

    pub intensity: f32,

    pub size: f32,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub source_orientation_offset: RealOrientation,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub add_perspective: bool,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub shift: Vector2<f32>,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub color: Color,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub minimum_darkness: f32,
}

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaValue))]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "kebab-case")
)]
pub enum LightDefinitionType {
    Basic,
    Oriented,
}

impl Default for LightDefinitionType {
    fn default() -> Self {
        Self::Basic
    }
}

pub type RealOrientation = f32;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LocalisedString();

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Loot();

pub type MaterialAmountType = f64;

#[derive(Clone, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "kebab-case")
)]
pub enum MaterialType {
    Item,
    Fluid,
}

#[cfg(feature = "lua-api")]
impl FromLuaValue for MaterialType {
    fn from_lua_value(value: Value) -> Result<Self, Error> {
        let s = value
            .as_str()
            .ok_or_else(|| rustorio_lua_api::Error::unexpected(value.clone()))?;
        match s {
            "item" => Ok(Self::Item),
            "fluid" => Ok(Self::Fluid),
            _ => Err(rustorio_lua_api::Error::unexpected(value)),
        }
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "kebab-case")
)]
pub enum ItemOrFluid<I, F> {
    Item(I),
    Fluid(F),
}

impl<I, F> ItemOrFluid<I, F> {
    pub fn as_item(&self) -> Option<&I> {
        match self {
            Self::Item(item) => Some(item),
            Self::Fluid(_) => None,
        }
    }

    pub fn as_fluid(&self) -> Option<&F> {
        match self {
            Self::Item(_) => None,
            Self::Fluid(fluid) => Some(fluid),
        }
    }
}

#[cfg(feature = "lua-api")]
impl<I: FromLuaTable + From<(Id<ItemPrototype>, u16)>, F: FromLuaTable> FromLuaTable
    for ItemOrFluid<I, F>
{
    fn from_lua_table(table: Table) -> Result<Self, rustorio_lua_api::Error> {
        // tuple variant. this is just an item and and amount
        match (table.get(1), table.get(2)) {
            (Ok(item_id), Ok(amount)) => {
                let item_id: rustorio_lua_api::Value = item_id;
                let name = FromLuaValue::from_lua_value(item_id)?;
                return Ok(Self::Item(I::from((name, amount))));
            }
            _ => {}
        }

        let ty: Option<MaterialType> = FromLuaValue::from_lua_value(table.get("type")?)?;
        match ty {
            Some(MaterialType::Item) | None => Ok(Self::Item(I::from_lua_table(table)?)),
            Some(MaterialType::Fluid) => Ok(Self::Fluid(F::from_lua_table(table)?)),
        }
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MinableProperties();

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MiningDrillGraphicsSet();

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ModuleSpecification();

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
// TODO: Build expressions based on a trait and type parameters. Then only box
// the outer one. ??
pub struct NoiseExpression();

#[derive(
    Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Into, From, AsRef, AsMut, Display,
)]
#[cfg_attr(feature = "lua-api", derive(FromLuaValue))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(transparent))]
pub struct Order(String);

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PlaceAsTile();

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RadiusVisualisationSpecification();

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "lua-api", derive(FromLuaValue))]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "kebab-case")
)]
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

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Resistances();

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RotatedAnimation();

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RotatedAnimation4Way();

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RotatedAnimationVariations();

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RotatedSprite();

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SignalIDConnector {
    pub r#type: SignalType,
    pub name: String,
}

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaValue))]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "kebab-case")
)]
pub enum SignalType {
    Virtual,
    Item,
    Fluid,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Sound();

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Sprite {
    #[cfg_attr(feature = "lua-api", lua(default))]
    pub filename: Option<FileName>,

    /// If this property is present, all Sprite definitions have to be placed as
    /// entries in the array, and they will all be loaded from there. Each item
    /// (Sprite definition) in the array may also have the layers property.
    ///
    /// If this property is present, all other properties are ignored and the
    /// mandatory properties do not have to be defined.
    ///
    /// Layers may not be an empty table.
    #[cfg_attr(feature = "lua-api", lua(default))]
    pub layers: Option<Vec<Sprite>>,

    #[cfg_attr(feature = "lua-api", lua(with = "Sprite::load_hr_version"))]
    pub hr_version: Option<Box<Sprite>>,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub priority: SpritePriority,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub flags: SpriteFlags,

    #[cfg_attr(feature = "lua-api", lua(with_context = "size_from_fields"))]
    pub size: Vector2<SpriteSizeType>,

    #[cfg_attr(
        feature = "lua-api",
        lua(with_context = "position_from_fields", default)
    )]
    pub position: Vector2<SpriteSizeType>,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub shift: Vector2<f32>,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub scale: f64,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub draw_as_shadow: bool,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub mipmap_count: u8,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub apply_runtime_tint: bool,

    #[cfg_attr(feature = "lua-api", lua(default_with = "Color::new(1., 1., 1., 1.)"))]
    pub tint: Color,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub blend_mode: BlendMode,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub load_in_minimal_mode: bool,

    #[cfg_attr(feature = "lua-api", lua(default_with = "true"))]
    pub premul_alpha: bool,

    #[cfg_attr(feature = "lua-api", lua(default))]
    pub generate_sdf: bool,

    #[cfg_attr(feature = "lua-api", lua(with_context = "dice_from_fields"))]
    pub dice: Vector2<SpriteSizeType>,
}

impl Sprite {
    #[cfg(feature = "lua-api")]
    fn load_hr_version(value: Value) -> Result<Option<Box<Sprite>>, Error> {
        let sprite: Option<Sprite> = to_option(value)?;
        Ok(sprite.map(Box::new))
    }
}

pub type Sprite4Way = FourWay<Sprite>;

#[derive(Clone, Debug, thiserror::Error)]
#[error("Error while parsing SpriteFlag: {0}")]
pub struct SpriteFlagParseError(String);

#[derive(Clone, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "kebab-case")
)]
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

#[cfg(feature = "lua-api")]
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
            s if s.starts_with("group=") => {
                SpriteFlag::Group(match &s[6..] {
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
                })
            }
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
            SpriteFlag::Group(group) => {
                write!(
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
                )
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaValue))]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "kebab-case")
)]
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

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "lua-api", derive(FromLuaValue))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SpriteFlags(Vec<SpriteFlag>);

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaValue))]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "kebab-case")
)]
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

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SpriteVariations();

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Stripe();

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TileTransitions();

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TransportBeltConnectorFrame();

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Trigger();

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TriggerEffect();

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TriggerTargetMask();

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UnitAISettings();

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct WaterReflectionDefinition();

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct WireConnectionPoint {
    pub wire: WirePosition,
    pub shadow: WirePosition,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct WirePosition {
    #[cfg_attr(feature = "lua-api", lua(default))]
    pub copper: Option<Vector2<f32>>,
    #[cfg_attr(feature = "lua-api", lua(default))]
    pub red: Option<Vector2<f32>>,
    #[cfg_attr(feature = "lua-api", lua(default))]
    pub green: Option<Vector2<f32>>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct WorkingSound();

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CharacterArmorAnimation();

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FootprintParticle();

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct OrientedCliffPrototype();

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct WorkingVisualisation();

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UnitSpawnDefinition();

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SmokeSource();

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SignalColorMapping();

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PumpConnectorGraphics();

pub type EntityPrototype = Todo;
pub type EquipmentPrototype = Todo;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "lua-api", derive(FromLuaTable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TreePrototypeVariation();

// Fix usages
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Todo();

#[cfg(feature = "lua-api")]
impl FromLuaTable for Todo {
    fn from_lua_table(_table: Table) -> Result<Self, Error> {
        todo!()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Difficulty {
    Normal,
    Expensive,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DifficultyDependentData<T> {
    pub normal: T,
    pub expensive: Option<T>,
}

impl<T> DifficultyDependentData<T> {
    pub fn for_difficulty(&self, difficulty: Difficulty) -> &T {
        match (difficulty, &self.expensive) {
            (Difficulty::Expensive, Some(expensive)) => expensive,
            _ => &self.normal,
        }
    }

    pub fn normal(&self) -> &T {
        &self.normal
    }

    pub fn expensive(&self) -> &T {
        if let Some(expensive) = &self.expensive {
            expensive
        }
        else {
            &self.normal
        }
    }
}

#[cfg(feature = "lua-api")]
impl<T: FromLuaTable> FromLuaTable for DifficultyDependentData<T> {
    fn from_lua_table(table: Table) -> Result<Self, Error> {
        let normal: Option<T> = to_option(table.get("normal")?)?;
        let expensive: Option<T> = to_option(table.get("expensive")?)?;

        match (normal, expensive) {
            (Some(normal), Some(expensive)) => {
                Ok(Self {
                    normal,
                    expensive: Some(expensive),
                })
            }
            (None, None) => {
                let normal: T = FromLuaTable::from_lua_table(table)?;
                Ok(Self {
                    normal,
                    expensive: None,
                })
            }
            _ => {
                Err(Error::other(
                    "expected either both normal and expensive fields, or neither.",
                ))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_energy_values() {
        assert_eq!(
            Energy::from_str("1.2GW").unwrap(),
            Energy {
                value: "1200000000".parse().unwrap(),
                unit: EnergyUnit::Watt
            }
        );
        assert_eq!(
            Energy::from_str("1.234567kW").unwrap(),
            Energy {
                value: "1234.567".parse().unwrap(),
                unit: EnergyUnit::Watt
            }
        );
        assert_eq!(
            Energy::from_str("1J").unwrap(),
            Energy {
                value: "1".parse().unwrap(),
                unit: EnergyUnit::Joule
            }
        );
    }
}
