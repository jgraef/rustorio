use serde::{Deserialize, Serialize};

use crate::prototypes::{Prototype, Visitor};
use crate::types::*; // TODO: Import only specific types

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Recipe {
    /// ingredients :: table of IngredientPrototype
    ingredients: Vec<IngredientPrototype>,

    /// allow_as_intermediate :: bool (optional)
    allow_as_intermediate: Option<bool>,

    /// allow_decomposition :: bool (optional)
    allow_decomposition: Option<bool>,

    /// allow_intermediates :: bool (optional)
    allow_intermediates: Option<bool>,

    /// always_show_made_in :: bool (optional)
    always_show_made_in: Option<bool>,

    /// always_show_products :: bool (optional)
    always_show_products: Option<bool>,

    /// category :: string (optional)
    category: Option<String>,

    /// crafting_machine_tint :: table of Color (optional)
    crafting_machine_tint: Option<Vec<Color>>,

    /// emissions_multiplier :: double (optional)
    emissions_multiplier: Option<f64>,

    /// enabled :: bool (optional)
    enabled: Option<bool>,

    /// energy_required :: double (optional)
    energy_required: Option<f64>,

    /// expensive :: Recipe data or bool (optional)
    expensive: Option<Todo>,

    /// hidden :: bool (optional)
    hidden: Option<bool>,

    /// hide_from_player_crafting :: bool (optional)
    hide_from_player_crafting: Option<bool>,

    /// hide_from_stats :: bool (optional)
    hide_from_stats: Option<bool>,

    /// icons, icon,  icon_size (IconSpecification) :: IconSpecification (optional)
    icon_spec: Option<IconSpecification>,

    /// main_product :: string (optional)
    main_product: Option<String>,

    /// normal :: Recipe data or bool (optional)
    normal: Option<Todo>,

    /// overload_multiplier :: uint32 (optional)
    overload_multiplier: Option<u32>,

    /// requester_paste_multiplier :: uint32 (optional)
    requester_paste_multiplier: Option<u32>,

    /// result :: string (optional)
    result: Option<String>,

    /// result_count :: uint32 (optional)
    result_count: Option<u32>,

    /// results :: table of ProductPrototype (optional)
    results: Option<Vec<ProductPrototype>>,

    /// show_amount_in_title :: bool (optional)
    show_amount_in_title: Option<bool>,

    /// subgroup :: string (optional)
    subgroup: Option<String>,

    /// unlock_results :: bool (optional)
    unlock_results: Option<bool>,
}

impl Prototype for Recipe {
    const TYPE: Option<&'static str> = Some("recipe");
}
