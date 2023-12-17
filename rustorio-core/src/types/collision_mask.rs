use serde::{
    Deserialize,
    Serialize,
};

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

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct CollisionMask();
