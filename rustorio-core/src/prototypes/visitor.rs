use super::*;


pub trait Visitor {
    type Error;

    fn visit_accumulator(&mut self, _accumulator: &accumulator::Accumulator) -> Result<(), Self::Error> { Ok(()) }
    fn visit_achievement(&mut self, _achievement: &achievement::Achievement) -> Result<(), Self::Error> { Ok(()) }
    fn visit_active_defense_equipment(&mut self, _active_defense_equipment: &active_defense_equipment::ActiveDefenseEquipment) -> Result<(), Self::Error> { Ok(()) }
    fn visit_ambient_sound(&mut self, _ambient_sound: &ambient_sound::AmbientSound) -> Result<(), Self::Error> { Ok(()) }
    fn visit_ammo_category(&mut self, _ammo_category: &ammo_category::AmmoCategory) -> Result<(), Self::Error> { Ok(()) }
    fn visit_ammo_item(&mut self, _ammo_item: &ammo_item::AmmoItem) -> Result<(), Self::Error> { Ok(()) }
    fn visit_ammo_turret(&mut self, _ammo_turret: &ammo_turret::AmmoTurret) -> Result<(), Self::Error> { Ok(()) }
    fn visit_animation(&mut self, _animation: &animation::Animation) -> Result<(), Self::Error> { Ok(()) }
    fn visit_arithmetic_combinator(&mut self, _arithmetic_combinator: &arithmetic_combinator::ArithmeticCombinator) -> Result<(), Self::Error> { Ok(()) }
    fn visit_armor(&mut self, _armor: &armor::Armor) -> Result<(), Self::Error> { Ok(()) }
    fn visit_arrow(&mut self, _arrow: &arrow::Arrow) -> Result<(), Self::Error> { Ok(()) }
    fn visit_artillery_flare(&mut self, _artillery_flare: &artillery_flare::ArtilleryFlare) -> Result<(), Self::Error> { Ok(()) }
    fn visit_artillery_projectile(&mut self, _artillery_projectile: &artillery_projectile::ArtilleryProjectile) -> Result<(), Self::Error> { Ok(()) }
    fn visit_artillery_turret(&mut self, _artillery_turret: &artillery_turret::ArtilleryTurret) -> Result<(), Self::Error> { Ok(()) }
    fn visit_artillery_wagon(&mut self, _artillery_wagon: &artillery_wagon::ArtilleryWagon) -> Result<(), Self::Error> { Ok(()) }
    fn visit_assembling_machine(&mut self, _assembling_machine: &assembling_machine::AssemblingMachine) -> Result<(), Self::Error> { Ok(()) }
    fn visit_autoplace_control(&mut self, _autoplace_control: &autoplace_control::AutoplaceControl) -> Result<(), Self::Error> { Ok(()) }
    fn visit_battery_equipment(&mut self, _battery_equipment: &battery_equipment::BatteryEquipment) -> Result<(), Self::Error> { Ok(()) }
    fn visit_beacon(&mut self, _beacon: &beacon::Beacon) -> Result<(), Self::Error> { Ok(()) }
    fn visit_beam(&mut self, _beam: &beam::Beam) -> Result<(), Self::Error> { Ok(()) }
    fn visit_belt_immunity_equipment(&mut self, _belt_immunity_equipment: &belt_immunity_equipment::BeltImmunityEquipment) -> Result<(), Self::Error> { Ok(()) }
    fn visit_blueprint_book(&mut self, _blueprint_book: &blueprint_book::BlueprintBook) -> Result<(), Self::Error> { Ok(()) }
    fn visit_blueprint_item(&mut self, _blueprint_item: &blueprint_item::BlueprintItem) -> Result<(), Self::Error> { Ok(()) }
    fn visit_boiler(&mut self, _boiler: &boiler::Boiler) -> Result<(), Self::Error> { Ok(()) }
    fn visit_build_entity_achievement(&mut self, _build_entity_achievement: &build_entity_achievement::BuildEntityAchievement) -> Result<(), Self::Error> { Ok(()) }
    fn visit_burner_generator(&mut self, _burner_generator: &burner_generator::BurnerGenerator) -> Result<(), Self::Error> { Ok(()) }
    fn visit_capsule(&mut self, _capsule: &capsule::Capsule) -> Result<(), Self::Error> { Ok(()) }
    fn visit_car(&mut self, _car: &car::Car) -> Result<(), Self::Error> { Ok(()) }
    fn visit_cargo_wagon(&mut self, _cargo_wagon: &cargo_wagon::CargoWagon) -> Result<(), Self::Error> { Ok(()) }
    fn visit_character(&mut self, _character: &character::Character) -> Result<(), Self::Error> { Ok(()) }
    fn visit_character_corpse(&mut self, _character_corpse: &character_corpse::CharacterCorpse) -> Result<(), Self::Error> { Ok(()) }
    fn visit_cliff(&mut self, _cliff: &cliff::Cliff) -> Result<(), Self::Error> { Ok(()) }
    fn visit_combat_robot(&mut self, _combat_robot: &combat_robot::CombatRobot) -> Result<(), Self::Error> { Ok(()) }
    fn visit_combat_robot_count_achievement(&mut self, _combat_robot_count_achievement: &combat_robot_count_achievement::CombatRobotCountAchievement) -> Result<(), Self::Error> { Ok(()) }
    fn visit_combinator(&mut self, _combinator: &combinator::Combinator) -> Result<(), Self::Error> { Ok(()) }
    fn visit_constant_combinator(&mut self, _constant_combinator: &constant_combinator::ConstantCombinator) -> Result<(), Self::Error> { Ok(()) }
    fn visit_construct_with_robots_achievement(&mut self, _construct_with_robots_achievement: &construct_with_robots_achievement::ConstructWithRobotsAchievement) -> Result<(), Self::Error> { Ok(()) }
    fn visit_construction_robot(&mut self, _construction_robot: &construction_robot::ConstructionRobot) -> Result<(), Self::Error> { Ok(()) }
    fn visit_container(&mut self, _container: &container::Container) -> Result<(), Self::Error> { Ok(()) }
    fn visit_copy_paste_tool(&mut self, _copy_paste_tool: &copy_paste_tool::CopyPasteTool) -> Result<(), Self::Error> { Ok(()) }
    fn visit_corpse(&mut self, _corpse: &corpse::Corpse) -> Result<(), Self::Error> { Ok(()) }
    fn visit_crafting_machine(&mut self, _crafting_machine: &crafting_machine::CraftingMachine) -> Result<(), Self::Error> { Ok(()) }
    fn visit_curved_rail(&mut self, _curved_rail: &curved_rail::CurvedRail) -> Result<(), Self::Error> { Ok(()) }
    fn visit_custom_input(&mut self, _custom_input: &custom_input::CustomInput) -> Result<(), Self::Error> { Ok(()) }
    fn visit_damage_type(&mut self, _damage_type: &damage_type::DamageType) -> Result<(), Self::Error> { Ok(()) }
    fn visit_decider_combinator(&mut self, _decider_combinator: &decider_combinator::DeciderCombinator) -> Result<(), Self::Error> { Ok(()) }
    fn visit_deconstruct_with_robots_achievement(&mut self, _deconstruct_with_robots_achievement: &deconstruct_with_robots_achievement::DeconstructWithRobotsAchievement) -> Result<(), Self::Error> { Ok(()) }
    fn visit_deconstructible_tile_proxy(&mut self, _deconstructible_tile_proxy: &deconstructible_tile_proxy::DeconstructibleTileProxy) -> Result<(), Self::Error> { Ok(()) }
    fn visit_deconstruction_item(&mut self, _deconstruction_item: &deconstruction_item::DeconstructionItem) -> Result<(), Self::Error> { Ok(()) }
    fn visit_decorative(&mut self, _decorative: &decorative::Decorative) -> Result<(), Self::Error> { Ok(()) }
    fn visit_deliver_by_robots_achievement(&mut self, _deliver_by_robots_achievement: &deliver_by_robots_achievement::DeliverByRobotsAchievement) -> Result<(), Self::Error> { Ok(()) }
    fn visit_dont_build_entity_achievement(&mut self, _dont_build_entity_achievement: &dont_build_entity_achievement::DontBuildEntityAchievement) -> Result<(), Self::Error> { Ok(()) }
    fn visit_dont_craft_manually_achievement(&mut self, _dont_craft_manually_achievement: &dont_craft_manually_achievement::DontCraftManuallyAchievement) -> Result<(), Self::Error> { Ok(()) }
    fn visit_dont_use_entity_in_energy_production_achievement(&mut self, _dont_use_entity_in_energy_production_achievement: &dont_use_entity_in_energy_production_achievement::DontUseEntityInEnergyProductionAchievement) -> Result<(), Self::Error> { Ok(()) }
    fn visit_editor_controller(&mut self, _editor_controller: &editor_controller::EditorController) -> Result<(), Self::Error> { Ok(()) }
    fn visit_electric_energy_interface(&mut self, _electric_energy_interface: &electric_energy_interface::ElectricEnergyInterface) -> Result<(), Self::Error> { Ok(()) }
    fn visit_electric_pole(&mut self, _electric_pole: &electric_pole::ElectricPole) -> Result<(), Self::Error> { Ok(()) }
    fn visit_electric_turret(&mut self, _electric_turret: &electric_turret::ElectricTurret) -> Result<(), Self::Error> { Ok(()) }
    fn visit_enemy_spawner(&mut self, _enemy_spawner: &enemy_spawner::EnemySpawner) -> Result<(), Self::Error> { Ok(()) }
    fn visit_energy_shield_equipment(&mut self, _energy_shield_equipment: &energy_shield_equipment::EnergyShieldEquipment) -> Result<(), Self::Error> { Ok(()) }
    fn visit_entity(&mut self, _entity: &entity::Entity) -> Result<(), Self::Error> { Ok(()) }
    fn visit_entity_ghost(&mut self, _entity_ghost: &entity_ghost::EntityGhost) -> Result<(), Self::Error> { Ok(()) }
    fn visit_entity_particle(&mut self, _entity_particle: &entity_particle::EntityParticle) -> Result<(), Self::Error> { Ok(()) }
    fn visit_entity_with_health(&mut self, _entity_with_health: &entity_with_health::EntityWithHealth) -> Result<(), Self::Error> { Ok(()) }
    fn visit_equipment(&mut self, _equipment: &equipment::Equipment) -> Result<(), Self::Error> { Ok(()) }
    fn visit_equipment_category(&mut self, _equipment_category: &equipment_category::EquipmentCategory) -> Result<(), Self::Error> { Ok(()) }
    fn visit_equipment_grid(&mut self, _equipment_grid: &equipment_grid::EquipmentGrid) -> Result<(), Self::Error> { Ok(()) }
    fn visit_explosion(&mut self, _explosion: &explosion::Explosion) -> Result<(), Self::Error> { Ok(()) }
    fn visit_finish_the_game_achievement(&mut self, _finish_the_game_achievement: &finish_the_game_achievement::FinishTheGameAchievement) -> Result<(), Self::Error> { Ok(()) }
    fn visit_fire_flame(&mut self, _fire_flame: &fire_flame::FireFlame) -> Result<(), Self::Error> { Ok(()) }
    fn visit_fish(&mut self, _fish: &fish::Fish) -> Result<(), Self::Error> { Ok(()) }
    fn visit_flame_thrower_explosion(&mut self, _flame_thrower_explosion: &flame_thrower_explosion::FlameThrowerExplosion) -> Result<(), Self::Error> { Ok(()) }
    fn visit_fluid(&mut self, _fluid: &fluid::Fluid) -> Result<(), Self::Error> { Ok(()) }
    fn visit_fluid_stream(&mut self, _fluid_stream: &fluid_stream::FluidStream) -> Result<(), Self::Error> { Ok(()) }
    fn visit_fluid_turret(&mut self, _fluid_turret: &fluid_turret::FluidTurret) -> Result<(), Self::Error> { Ok(()) }
    fn visit_fluid_wagon(&mut self, _fluid_wagon: &fluid_wagon::FluidWagon) -> Result<(), Self::Error> { Ok(()) }
    fn visit_flying_robot(&mut self, _flying_robot: &flying_robot::FlyingRobot) -> Result<(), Self::Error> { Ok(()) }
    fn visit_flying_text(&mut self, _flying_text: &flying_text::FlyingText) -> Result<(), Self::Error> { Ok(()) }
    fn visit_font(&mut self, _font: &font::Font) -> Result<(), Self::Error> { Ok(()) }
    fn visit_fuel_category(&mut self, _fuel_category: &fuel_category::FuelCategory) -> Result<(), Self::Error> { Ok(()) }
    fn visit_furnace(&mut self, _furnace: &furnace::Furnace) -> Result<(), Self::Error> { Ok(()) }
    fn visit_gate(&mut self, _gate: &gate::Gate) -> Result<(), Self::Error> { Ok(()) }
    fn visit_generator(&mut self, _generator: &generator::Generator) -> Result<(), Self::Error> { Ok(()) }
    fn visit_generator_equipment(&mut self, _generator_equipment: &generator_equipment::GeneratorEquipment) -> Result<(), Self::Error> { Ok(()) }
    fn visit_god_controller(&mut self, _god_controller: &god_controller::GodController) -> Result<(), Self::Error> { Ok(()) }
    fn visit_group_attack_achievement(&mut self, _group_attack_achievement: &group_attack_achievement::GroupAttackAchievement) -> Result<(), Self::Error> { Ok(()) }
    fn visit_gui_style(&mut self, _gui_style: &gui_style::GuiStyle) -> Result<(), Self::Error> { Ok(()) }
    fn visit_gun(&mut self, _gun: &gun::Gun) -> Result<(), Self::Error> { Ok(()) }
    fn visit_heat_interface(&mut self, _heat_interface: &heat_interface::HeatInterface) -> Result<(), Self::Error> { Ok(()) }
    fn visit_heat_pipe(&mut self, _heat_pipe: &heat_pipe::HeatPipe) -> Result<(), Self::Error> { Ok(()) }
    fn visit_highlight_box_entity(&mut self, _highlight_box_entity: &highlight_box_entity::HighlightBoxEntity) -> Result<(), Self::Error> { Ok(()) }
    fn visit_infinity_container(&mut self, _infinity_container: &infinity_container::InfinityContainer) -> Result<(), Self::Error> { Ok(()) }
    fn visit_infinity_pipe(&mut self, _infinity_pipe: &infinity_pipe::InfinityPipe) -> Result<(), Self::Error> { Ok(()) }
    fn visit_inserter(&mut self, _inserter: &inserter::Inserter) -> Result<(), Self::Error> { Ok(()) }
    fn visit_item(&mut self, _item: &item::Item) -> Result<(), Self::Error> { Ok(()) }
    fn visit_item_entity(&mut self, _item_entity: &item_entity::ItemEntity) -> Result<(), Self::Error> { Ok(()) }
    fn visit_item_group(&mut self, _item_group: &item_group::ItemGroup) -> Result<(), Self::Error> { Ok(()) }
    fn visit_item_request_proxy(&mut self, _item_request_proxy: &item_request_proxy::ItemRequestProxy) -> Result<(), Self::Error> { Ok(()) }
    fn visit_item_sub_group(&mut self, _item_sub_group: &item_sub_group::ItemSubGroup) -> Result<(), Self::Error> { Ok(()) }
    fn visit_item_with_entity_data(&mut self, _item_with_entity_data: &item_with_entity_data::ItemWithEntityData) -> Result<(), Self::Error> { Ok(()) }
    fn visit_item_with_inventory(&mut self, _item_with_inventory: &item_with_inventory::ItemWithInventory) -> Result<(), Self::Error> { Ok(()) }
    fn visit_item_with_label(&mut self, _item_with_label: &item_with_label::ItemWithLabel) -> Result<(), Self::Error> { Ok(()) }
    fn visit_item_with_tags(&mut self, _item_with_tags: &item_with_tags::ItemWithTags) -> Result<(), Self::Error> { Ok(()) }
    fn visit_kill_achievement(&mut self, _kill_achievement: &kill_achievement::KillAchievement) -> Result<(), Self::Error> { Ok(()) }
    fn visit_lab(&mut self, _lab: &lab::Lab) -> Result<(), Self::Error> { Ok(()) }
    fn visit_lamp(&mut self, _lamp: &lamp::Lamp) -> Result<(), Self::Error> { Ok(()) }
    fn visit_land_mine(&mut self, _land_mine: &land_mine::LandMine) -> Result<(), Self::Error> { Ok(()) }
    fn visit_leaf_particle(&mut self, _leaf_particle: &leaf_particle::LeafParticle) -> Result<(), Self::Error> { Ok(()) }
    fn visit_legacy_decorative(&mut self, _legacy_decorative: &legacy_decorative::LegacyDecorative) -> Result<(), Self::Error> { Ok(()) }
    fn visit_loader1x1(&mut self, _loader1x1: &loader1x1::Loader1x1) -> Result<(), Self::Error> { Ok(()) }
    fn visit_loader1x2(&mut self, _loader1x2: &loader1x2::Loader1x2) -> Result<(), Self::Error> { Ok(()) }
    fn visit_locomotive(&mut self, _locomotive: &locomotive::Locomotive) -> Result<(), Self::Error> { Ok(()) }
    fn visit_logistic_container(&mut self, _logistic_container: &logistic_container::LogisticContainer) -> Result<(), Self::Error> { Ok(()) }
    fn visit_logistic_robot(&mut self, _logistic_robot: &logistic_robot::LogisticRobot) -> Result<(), Self::Error> { Ok(()) }
    fn visit_map_gen_presets(&mut self, _map_gen_presets: &map_gen_presets::MapGenPresets) -> Result<(), Self::Error> { Ok(()) }
    fn visit_map_settings(&mut self, _map_settings: &map_settings::MapSettings) -> Result<(), Self::Error> { Ok(()) }
    fn visit_market(&mut self, _market: &market::Market) -> Result<(), Self::Error> { Ok(()) }
    fn visit_mining_drill(&mut self, _mining_drill: &mining_drill::MiningDrill) -> Result<(), Self::Error> { Ok(()) }
    fn visit_mining_tool(&mut self, _mining_tool: &mining_tool::MiningTool) -> Result<(), Self::Error> { Ok(()) }
    fn visit_module(&mut self, _module: &module::Module) -> Result<(), Self::Error> { Ok(()) }
    fn visit_module_category(&mut self, _module_category: &module_category::ModuleCategory) -> Result<(), Self::Error> { Ok(()) }
    fn visit_mouse_cursor(&mut self, _mouse_cursor: &mouse_cursor::MouseCursor) -> Result<(), Self::Error> { Ok(()) }
    fn visit_movement_bonus_equipment(&mut self, _movement_bonus_equipment: &movement_bonus_equipment::MovementBonusEquipment) -> Result<(), Self::Error> { Ok(()) }
    fn visit_named_noise_expression(&mut self, _named_noise_expression: &named_noise_expression::NamedNoiseExpression) -> Result<(), Self::Error> { Ok(()) }
    fn visit_night_vision_equipment(&mut self, _night_vision_equipment: &night_vision_equipment::NightVisionEquipment) -> Result<(), Self::Error> { Ok(()) }
    fn visit_noise_layer(&mut self, _noise_layer: &noise_layer::NoiseLayer) -> Result<(), Self::Error> { Ok(()) }
    fn visit_offshore_pump(&mut self, _offshore_pump: &offshore_pump::OffshorePump) -> Result<(), Self::Error> { Ok(()) }
    fn visit_particle(&mut self, _particle: &particle::Particle) -> Result<(), Self::Error> { Ok(()) }
    fn visit_particle_source(&mut self, _particle_source: &particle_source::ParticleSource) -> Result<(), Self::Error> { Ok(()) }
    fn visit_pipe(&mut self, _pipe: &pipe::Pipe) -> Result<(), Self::Error> { Ok(()) }
    fn visit_pipe_to_ground(&mut self, _pipe_to_ground: &pipe_to_ground::PipeToGround) -> Result<(), Self::Error> { Ok(()) }
    fn visit_player_damaged_achievement(&mut self, _player_damaged_achievement: &player_damaged_achievement::PlayerDamagedAchievement) -> Result<(), Self::Error> { Ok(()) }
    fn visit_player_port(&mut self, _player_port: &player_port::PlayerPort) -> Result<(), Self::Error> { Ok(()) }
    fn visit_power_switch(&mut self, _power_switch: &power_switch::PowerSwitch) -> Result<(), Self::Error> { Ok(()) }
    fn visit_produce_achievement(&mut self, _produce_achievement: &produce_achievement::ProduceAchievement) -> Result<(), Self::Error> { Ok(()) }
    fn visit_produce_per_hour_achievement(&mut self, _produce_per_hour_achievement: &produce_per_hour_achievement::ProducePerHourAchievement) -> Result<(), Self::Error> { Ok(()) }
    fn visit_programmable_speaker(&mut self, _programmable_speaker: &programmable_speaker::ProgrammableSpeaker) -> Result<(), Self::Error> { Ok(()) }
    fn visit_projectile(&mut self, _projectile: &projectile::Projectile) -> Result<(), Self::Error> { Ok(()) }
    fn visit_pump(&mut self, _pump: &pump::Pump) -> Result<(), Self::Error> { Ok(()) }
    fn visit_radar(&mut self, _radar: &radar::Radar) -> Result<(), Self::Error> { Ok(()) }
    fn visit_rail(&mut self, _rail: &rail::Rail) -> Result<(), Self::Error> { Ok(()) }
    fn visit_rail_chain_signal(&mut self, _rail_chain_signal: &rail_chain_signal::RailChainSignal) -> Result<(), Self::Error> { Ok(()) }
    fn visit_rail_planner(&mut self, _rail_planner: &rail_planner::RailPlanner) -> Result<(), Self::Error> { Ok(()) }
    fn visit_rail_remnants(&mut self, _rail_remnants: &rail_remnants::RailRemnants) -> Result<(), Self::Error> { Ok(()) }
    fn visit_rail_signal(&mut self, _rail_signal: &rail_signal::RailSignal) -> Result<(), Self::Error> { Ok(()) }
    fn visit_rail_signal_base(&mut self, _rail_signal_base: &rail_signal_base::RailSignalBase) -> Result<(), Self::Error> { Ok(()) }
    fn visit_reactor(&mut self, _reactor: &reactor::Reactor) -> Result<(), Self::Error> { Ok(()) }
    fn visit_recipe(&mut self, _recipe: &recipe::Recipe) -> Result<(), Self::Error> { Ok(()) }
    fn visit_recipe_category(&mut self, _recipe_category: &recipe_category::RecipeCategory) -> Result<(), Self::Error> { Ok(()) }
    fn visit_repair_tool(&mut self, _repair_tool: &repair_tool::RepairTool) -> Result<(), Self::Error> { Ok(()) }
    fn visit_research_achievement(&mut self, _research_achievement: &research_achievement::ResearchAchievement) -> Result<(), Self::Error> { Ok(()) }
    fn visit_resource_category(&mut self, _resource_category: &resource_category::ResourceCategory) -> Result<(), Self::Error> { Ok(()) }
    fn visit_resource_entity(&mut self, _resource_entity: &resource_entity::ResourceEntity) -> Result<(), Self::Error> { Ok(()) }
    fn visit_roboport(&mut self, _roboport: &roboport::Roboport) -> Result<(), Self::Error> { Ok(()) }
    fn visit_roboport_equipment(&mut self, _roboport_equipment: &roboport_equipment::RoboportEquipment) -> Result<(), Self::Error> { Ok(()) }
    fn visit_robot_with_logistic_interface(&mut self, _robot_with_logistic_interface: &robot_with_logistic_interface::RobotWithLogisticInterface) -> Result<(), Self::Error> { Ok(()) }
    fn visit_rocket_silo(&mut self, _rocket_silo: &rocket_silo::RocketSilo) -> Result<(), Self::Error> { Ok(()) }
    fn visit_rocket_silo_rocket(&mut self, _rocket_silo_rocket: &rocket_silo_rocket::RocketSiloRocket) -> Result<(), Self::Error> { Ok(()) }
    fn visit_rocket_silo_rocket_shadow(&mut self, _rocket_silo_rocket_shadow: &rocket_silo_rocket_shadow::RocketSiloRocketShadow) -> Result<(), Self::Error> { Ok(()) }
    fn visit_rolling_stock(&mut self, _rolling_stock: &rolling_stock::RollingStock) -> Result<(), Self::Error> { Ok(()) }
    fn visit_selection_tool(&mut self, _selection_tool: &selection_tool::SelectionTool) -> Result<(), Self::Error> { Ok(()) }
    fn visit_shortcut(&mut self, _shortcut: &shortcut::Shortcut) -> Result<(), Self::Error> { Ok(()) }
    fn visit_simple_entity(&mut self, _simple_entity: &simple_entity::SimpleEntity) -> Result<(), Self::Error> { Ok(()) }
    fn visit_simple_entity_with_force(&mut self, _simple_entity_with_force: &simple_entity_with_force::SimpleEntityWithForce) -> Result<(), Self::Error> { Ok(()) }
    fn visit_simple_entity_with_owner(&mut self, _simple_entity_with_owner: &simple_entity_with_owner::SimpleEntityWithOwner) -> Result<(), Self::Error> { Ok(()) }
    fn visit_simple_smoke(&mut self, _simple_smoke: &simple_smoke::SimpleSmoke) -> Result<(), Self::Error> { Ok(()) }
    fn visit_smoke(&mut self, _smoke: &smoke::Smoke) -> Result<(), Self::Error> { Ok(()) }
    fn visit_smoke_with_trigger(&mut self, _smoke_with_trigger: &smoke_with_trigger::SmokeWithTrigger) -> Result<(), Self::Error> { Ok(()) }
    fn visit_solar_panel(&mut self, _solar_panel: &solar_panel::SolarPanel) -> Result<(), Self::Error> { Ok(()) }
    fn visit_solar_panel_equipment(&mut self, _solar_panel_equipment: &solar_panel_equipment::SolarPanelEquipment) -> Result<(), Self::Error> { Ok(()) }
    fn visit_sound(&mut self, _sound: &sound::Sound) -> Result<(), Self::Error> { Ok(()) }
    fn visit_spectator_controller(&mut self, _spectator_controller: &spectator_controller::SpectatorController) -> Result<(), Self::Error> { Ok(()) }
    fn visit_speech_bubble(&mut self, _speech_bubble: &speech_bubble::SpeechBubble) -> Result<(), Self::Error> { Ok(()) }
    fn visit_splitter(&mut self, _splitter: &splitter::Splitter) -> Result<(), Self::Error> { Ok(()) }
    fn visit_sprite(&mut self, _sprite: &sprite::Sprite) -> Result<(), Self::Error> { Ok(()) }
    fn visit_sticker(&mut self, _sticker: &sticker::Sticker) -> Result<(), Self::Error> { Ok(()) }
    fn visit_storage_tank(&mut self, _storage_tank: &storage_tank::StorageTank) -> Result<(), Self::Error> { Ok(()) }
    fn visit_straight_rail(&mut self, _straight_rail: &straight_rail::StraightRail) -> Result<(), Self::Error> { Ok(()) }
    fn visit_technology(&mut self, _technology: &technology::Technology) -> Result<(), Self::Error> { Ok(()) }
    fn visit_tile(&mut self, _tile: &tile::Tile) -> Result<(), Self::Error> { Ok(()) }
    fn visit_tile_effect(&mut self, _tile_effect: &tile_effect::TileEffect) -> Result<(), Self::Error> { Ok(()) }
    fn visit_tile_ghost(&mut self, _tile_ghost: &tile_ghost::TileGhost) -> Result<(), Self::Error> { Ok(()) }
    fn visit_tool(&mut self, _tool: &tool::Tool) -> Result<(), Self::Error> { Ok(()) }
    fn visit_train_path_achievement(&mut self, _train_path_achievement: &train_path_achievement::TrainPathAchievement) -> Result<(), Self::Error> { Ok(()) }
    fn visit_train_stop(&mut self, _train_stop: &train_stop::TrainStop) -> Result<(), Self::Error> { Ok(()) }
    fn visit_transport_belt(&mut self, _transport_belt: &transport_belt::TransportBelt) -> Result<(), Self::Error> { Ok(()) }
    fn visit_transport_belt_connectable(&mut self, _transport_belt_connectable: &transport_belt_connectable::TransportBeltConnectable) -> Result<(), Self::Error> { Ok(()) }
    fn visit_tree(&mut self, _tree: &tree::Tree) -> Result<(), Self::Error> { Ok(()) }
    fn visit_trigger_target_type(&mut self, _trigger_target_type: &trigger_target_type::TriggerTargetType) -> Result<(), Self::Error> { Ok(()) }
    fn visit_trivial_smoke(&mut self, _trivial_smoke: &trivial_smoke::TrivialSmoke) -> Result<(), Self::Error> { Ok(()) }
    fn visit_turret(&mut self, _turret: &turret::Turret) -> Result<(), Self::Error> { Ok(()) }
    fn visit_tutorial(&mut self, _tutorial: &tutorial::Tutorial) -> Result<(), Self::Error> { Ok(()) }
    fn visit_underground_belt(&mut self, _underground_belt: &underground_belt::UndergroundBelt) -> Result<(), Self::Error> { Ok(()) }
    fn visit_unit(&mut self, _unit: &unit::Unit) -> Result<(), Self::Error> { Ok(()) }
    fn visit_upgrade_item(&mut self, _upgrade_item: &upgrade_item::UpgradeItem) -> Result<(), Self::Error> { Ok(()) }
    fn visit_utility_constants(&mut self, _utility_constants: &utility_constants::UtilityConstants) -> Result<(), Self::Error> { Ok(()) }
    fn visit_utility_sounds(&mut self, _utility_sounds: &utility_sounds::UtilitySounds) -> Result<(), Self::Error> { Ok(()) }
    fn visit_utility_sprites(&mut self, _utility_sprites: &utility_sprites::UtilitySprites) -> Result<(), Self::Error> { Ok(()) }
    fn visit_vehicle(&mut self, _vehicle: &vehicle::Vehicle) -> Result<(), Self::Error> { Ok(()) }
    fn visit_virtual_signal(&mut self, _virtual_signal: &virtual_signal::VirtualSignal) -> Result<(), Self::Error> { Ok(()) }
    fn visit_wall(&mut self, _wall: &wall::Wall) -> Result<(), Self::Error> { Ok(()) }
    fn visit_wind_sound(&mut self, _wind_sound: &wind_sound::WindSound) -> Result<(), Self::Error> { Ok(()) }
    fn visit_prototype_base(&mut self, _prototype_base: &prototype_base::PrototypeBase) -> Result<(), Self::Error> { Ok(()) }
}

