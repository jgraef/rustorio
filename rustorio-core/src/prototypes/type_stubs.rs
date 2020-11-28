use std::{
    fmt::{Display, Formatter, self},
    str::FromStr,
    convert::TryFrom,
};

use serde::{Serialize, Deserialize};
use thiserror::Error;


#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct AnimatedVector {
    frames: Vec<Vector2<f32>>,
    render_layer: Option<RenderLayer>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Animation {
    layers: Vec<Animation>,
    hr_version: Option<Box<Animation>>,
    filename: FileName,
    #[serde(default)]
    priority: SpritePriority,
    flags: SpriteFlags,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Animation4Way();

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct AnimationFrameSequence();

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct AnimationVariations();

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct AttackParameters();

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct AttackReaction();

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct AutoplaceSpecification();

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct BeaconGraphicsSet();

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct BlendMode();

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct BoundingBox();

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct CapsuleAction();

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct CircuitConnectorSprites();

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct CollisionMask();

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Color();

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ConnectableEntityGraphics();

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ConsumingType();

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct CreateTrivialSmokeEffectItem();

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct DamagePrototype();

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct DaytimeColorLookupTable();

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Effect();

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct EffectTypeLimitation();

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Energy();

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct EnergySource();

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct EntityPrototypeFlags();

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct EquipmentShape();

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct FileName();

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct FluidBox();

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct FootstepTriggerEffectList();

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ForceCondition();

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct HeatBuffer();

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct IconSpecification();

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct InterruptibleSound();

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ItemCountType();

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ItemProductPrototype();

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ItemPrototypeFlags();

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ItemStackIndex();

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct LightDefinition();

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct LocalisedString();

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Loot();

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct MaterialAmountType();

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct MinableProperties();

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct MiningDrillGraphicsSet();

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ModuleSpecification();

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct NoiseExpression();

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Order();

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct PlaceAsTile();

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct RadiusVisualisationSpecification();

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct RenderLayer();

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Resistances();

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct RotatedAnimation();

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct RotatedAnimation4Way();

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct RotatedAnimationVariations();

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct RotatedSprite();

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct SignalIDConnector();

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Sound();

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Sprite();

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Sprite4Way();

#[derive(Clone, Debug, Error, Deserialize)]
#[error("Error while parsing SpriteFlag: {0}")]
pub struct SpriteFlagParseError(String);

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(try_from="String", into="String")]
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
                write!(f, "group={}", match group {
                    SpriteGroup::None => "none",
                    SpriteGroup::Terrain => "terrain",
                    SpriteGroup::Shadow => "shadow",
                    SpriteGroup::Smoke => "smoke",
                    SpriteGroup::Decal => "decal",
                    SpriteGroup::LowObject => "low-object",
                    SpriteGroup::Gui => "gui",
                    SpriteGroup::Icon => "icon",
                    SpriteGroup::IconBackground => "icon-background",
                })
            },
        }
    }
}

#[derive(Copy, Clone, Debug)]
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

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct SpriteFlags(Vec<SpriteFlag>);

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all="kebab-case")]
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

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct SpriteSizeType();

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct SpriteVariations();

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Stripe();

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct TileTransitions();

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct TransportBeltConnectorFrame();

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Trigger();

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct TriggerEffect();

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct TriggerTargetMask();

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct UnitAISettings();

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct WaterReflectionDefinition();

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct WireConnectionPoint();

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct WorkingSound();

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct CharacterArmorAnimation();

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct FootprintParticle();

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct OrientedCliffPrototype;

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct WorkingVisualisation;

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct UnitSpawnDefinition;

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct SmokeSource;

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct SignalColorMapping;

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct PumpConnectorGraphics;

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct IngredientPrototype;

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ProductPrototype;

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Modifier;

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct TreePrototypeVariation;


// Fix usages
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Todo;

// TODO: Use nalgebra

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Vector2<T>([T; 2]);
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Vector3<T>([T; 3]);