/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
	rare: bool,
	vegetables: usize,
	sugars: usize,
	checkbox_states: [bool; 15],

	affinity: moonlighter::Affinity,
	bear_meal_affinity: moonlighter::Affinity,
	recipe: Option<moonlighter::Recipe>,
}

impl Default for TemplateApp {
	fn default() -> Self {
		Self {
			bear_meal_affinity: moonlighter::Affinity::AggressiveFighting,
			affinity: moonlighter::Affinity::AggressiveFighting,
			recipe: None,
			rare: false,
			vegetables: 12,
			sugars: 50,
			checkbox_states: [false; 15],
		}
	}
}

impl TemplateApp {
	/// Called once before the first frame.
	pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
		// This is also where you can customize the look and feel of egui using
		// `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

		// Load previous app state (if any).
		// Note that you must enable the `persistence` feature for this to work.
		if let Some(storage) = cc.storage {
			eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
		} else {
			Default::default()
		}
	}
}

impl eframe::App for TemplateApp {
	/// Called by the framework to save state before shutdown.
	fn save(&mut self, storage: &mut dyn eframe::Storage) {
		eframe::set_value(storage, eframe::APP_KEY, self);
	}

	/// Called each time the UI needs repainting, which may be many times per second.
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		// Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
		// For inspiration and more examples, go to https://emilk.github.io/egui

		egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
			// The top panel is often a good place for a menu bar:

			egui::MenuBar::new().ui(ui, |ui| {
				// NOTE: no File->Quit on web pages!
				let is_web = cfg!(target_arch = "wasm32");
				if !is_web {
					ui.menu_button("File", |ui| {
						if ui.button("Quit").clicked() {
							ctx.send_viewport_cmd(egui::ViewportCommand::Close);
						}
					});
					ui.add_space(16.0);
				}

				egui::widgets::global_theme_preference_buttons(ui);
			});
		});

		egui::CentralPanel::default().show(ctx, |ui| {
			// The central panel the region left after adding TopPanel's and SidePanel's
			ui.heading("V12: 12 vegetable moonshine generator");

			ui.checkbox(&mut self.rare, "My Oven Is Rare");

			let selected = &mut self.bear_meal_affinity;

			let combo = egui::ComboBox::from_label("<- Bear + Corn + Pan + Oven gives me").selected_text(format!("{:?}", selected));

			ui.add(egui::Slider::new(&mut self.vegetables, 1..=12).text("Unique vegetables to use"));
			ui.add(egui::Slider::new(&mut self.sugars, 2..=80).text("Maximum sugars to add"));

			combo.show_ui(ui, |ui| {
				ui.selectable_value(selected, moonlighter::Affinity::AggressiveFighting, "AggressiveFighting");
				ui.selectable_value(selected, moonlighter::Affinity::Alchemy, "Alchemy");
				ui.selectable_value(selected, moonlighter::Affinity::AnimalHusbandry, "AnimalHusbandry");
				ui.selectable_value(selected, moonlighter::Affinity::AnimalTaming, "AnimalTaming");
				ui.selectable_value(selected, moonlighter::Affinity::Archaeology, "Archaeology");
				ui.selectable_value(selected, moonlighter::Affinity::Archery, "Archery");
				ui.selectable_value(selected, moonlighter::Affinity::ArmourSmithing, "ArmourSmithing");
				ui.selectable_value(selected, moonlighter::Affinity::Axes, "Axes");
				ui.selectable_value(selected, moonlighter::Affinity::Baking, "Baking");
				ui.selectable_value(selected, moonlighter::Affinity::Ballistae, "Ballistae");
				ui.selectable_value(selected, moonlighter::Affinity::Beverages, "Beverages");
				ui.selectable_value(selected, moonlighter::Affinity::Blacksmithing, "Blacksmithing");
				ui.selectable_value(selected, moonlighter::Affinity::BladesSmithing, "BladesSmithing");
				ui.selectable_value(selected, moonlighter::Affinity::Body, "Body");
				ui.selectable_value(selected, moonlighter::Affinity::BodyControl, "BodyControl");
				ui.selectable_value(selected, moonlighter::Affinity::BodyStamina, "BodyStamina");
				ui.selectable_value(selected, moonlighter::Affinity::BodyStrength, "BodyStrength");
				ui.selectable_value(selected, moonlighter::Affinity::Botanizing, "Botanizing");
				ui.selectable_value(selected, moonlighter::Affinity::Bowyery, "Bowyery");
				ui.selectable_value(selected, moonlighter::Affinity::Butchering, "Butchering");
				ui.selectable_value(selected, moonlighter::Affinity::ButcheringKnife, "ButcheringKnife");
				ui.selectable_value(selected, moonlighter::Affinity::Carpentry, "Carpentry");
				ui.selectable_value(selected, moonlighter::Affinity::CarvingKnife, "CarvingKnife");
				ui.selectable_value(selected, moonlighter::Affinity::Catapults, "Catapults");
				ui.selectable_value(selected, moonlighter::Affinity::ChainArmourSmithing, "ChainArmourSmithing");
				ui.selectable_value(selected, moonlighter::Affinity::Channeling, "Channeling");
				ui.selectable_value(selected, moonlighter::Affinity::Climbing, "Climbing");
				ui.selectable_value(selected, moonlighter::Affinity::ClothTailoring, "ClothTailoring");
				ui.selectable_value(selected, moonlighter::Affinity::Clubs, "Clubs");
				ui.selectable_value(selected, moonlighter::Affinity::CoalMaking, "CoalMaking");
				ui.selectable_value(selected, moonlighter::Affinity::Cooking, "Cooking");
				ui.selectable_value(selected, moonlighter::Affinity::DairyFoodMaking, "DairyFoodMaking");
				ui.selectable_value(selected, moonlighter::Affinity::DefensiveFighting, "DefensiveFighting");
				ui.selectable_value(selected, moonlighter::Affinity::Digging, "Digging");
				ui.selectable_value(selected, moonlighter::Affinity::Exorcism, "Exorcism");
				ui.selectable_value(selected, moonlighter::Affinity::Farming, "Farming");
				ui.selectable_value(selected, moonlighter::Affinity::Fighting, "Fighting");
				ui.selectable_value(selected, moonlighter::Affinity::FineCarpentry, "FineCarpentry");
				ui.selectable_value(selected, moonlighter::Affinity::Firemaking, "Firemaking");
				ui.selectable_value(selected, moonlighter::Affinity::FirstAid, "FirstAid");
				ui.selectable_value(selected, moonlighter::Affinity::Fishing, "Fishing");
				ui.selectable_value(selected, moonlighter::Affinity::Fletching, "Fletching");
				ui.selectable_value(selected, moonlighter::Affinity::Foraging, "Foraging");
				ui.selectable_value(selected, moonlighter::Affinity::Forestry, "Forestry");
				ui.selectable_value(selected, moonlighter::Affinity::Gardening, "Gardening");
				ui.selectable_value(selected, moonlighter::Affinity::Halberd, "Halberd");
				ui.selectable_value(selected, moonlighter::Affinity::Hammer, "Hammer");
				ui.selectable_value(selected, moonlighter::Affinity::Hammers, "Hammers");
				ui.selectable_value(selected, moonlighter::Affinity::Hatchet, "Hatchet");
				ui.selectable_value(selected, moonlighter::Affinity::Healing, "Healing");
				ui.selectable_value(selected, moonlighter::Affinity::HotFoodCooking, "HotFoodCooking");
				ui.selectable_value(selected, moonlighter::Affinity::HugeAxe, "HugeAxe");
				ui.selectable_value(selected, moonlighter::Affinity::HugeClub, "HugeClub");
				ui.selectable_value(selected, moonlighter::Affinity::JewelrySmithing, "JewelrySmithing");
				ui.selectable_value(selected, moonlighter::Affinity::Knives, "Knives");
				ui.selectable_value(selected, moonlighter::Affinity::LargeAxe, "LargeAxe");
				ui.selectable_value(selected, moonlighter::Affinity::LargeMaul, "LargeMaul");
				ui.selectable_value(selected, moonlighter::Affinity::LargeMetalShield, "LargeMetalShield");
				ui.selectable_value(selected, moonlighter::Affinity::LargeWoodenShield, "LargeWoodenShield");
				ui.selectable_value(selected, moonlighter::Affinity::Leatherworking, "Leatherworking");
				ui.selectable_value(selected, moonlighter::Affinity::LockPicking, "LockPicking");
				ui.selectable_value(selected, moonlighter::Affinity::Locksmithing, "Locksmithing");
				ui.selectable_value(selected, moonlighter::Affinity::LongBow, "LongBow");
				ui.selectable_value(selected, moonlighter::Affinity::LongSpear, "LongSpear");
				ui.selectable_value(selected, moonlighter::Affinity::Longsword, "Longsword");
				ui.selectable_value(selected, moonlighter::Affinity::Masonry, "Masonry");
				ui.selectable_value(selected, moonlighter::Affinity::Mauls, "Mauls");
				ui.selectable_value(selected, moonlighter::Affinity::Meditating, "Meditating");
				ui.selectable_value(selected, moonlighter::Affinity::MediumBow, "MediumBow");
				ui.selectable_value(selected, moonlighter::Affinity::MediumMaul, "MediumMaul");
				ui.selectable_value(selected, moonlighter::Affinity::MediumMetalShield, "MediumMetalShield");
				ui.selectable_value(selected, moonlighter::Affinity::MediumWoodenShield, "MediumWoodenShield");
				ui.selectable_value(selected, moonlighter::Affinity::Metallurgy, "Metallurgy");
				ui.selectable_value(selected, moonlighter::Affinity::Milking, "Milking");
				ui.selectable_value(selected, moonlighter::Affinity::Milling, "Milling");
				ui.selectable_value(selected, moonlighter::Affinity::Mind, "Mind");
				ui.selectable_value(selected, moonlighter::Affinity::MindLogic, "MindLogic");
				ui.selectable_value(selected, moonlighter::Affinity::MindSpeed, "MindSpeed");
				ui.selectable_value(selected, moonlighter::Affinity::Mining, "Mining");
				ui.selectable_value(selected, moonlighter::Affinity::MiscItems, "MiscItems");
				ui.selectable_value(selected, moonlighter::Affinity::NaturalSubstances, "NaturalSubstances");
				ui.selectable_value(selected, moonlighter::Affinity::Nature, "Nature");
				ui.selectable_value(selected, moonlighter::Affinity::NormalFighting, "NormalFighting");
				ui.selectable_value(selected, moonlighter::Affinity::Papyrusmaking, "Papyrusmaking");
				ui.selectable_value(selected, moonlighter::Affinity::Paving, "Paving");
				ui.selectable_value(selected, moonlighter::Affinity::Pickaxe, "Pickaxe");
				ui.selectable_value(selected, moonlighter::Affinity::PlateArmourSmithing, "PlateArmourSmithing");
				ui.selectable_value(selected, moonlighter::Affinity::Polearms, "Polearms");
				ui.selectable_value(selected, moonlighter::Affinity::Pottery, "Pottery");
				ui.selectable_value(selected, moonlighter::Affinity::Praying, "Praying");
				ui.selectable_value(selected, moonlighter::Affinity::Preaching, "Preaching");
				ui.selectable_value(selected, moonlighter::Affinity::Prospecting, "Prospecting");
				ui.selectable_value(selected, moonlighter::Affinity::Puppeteering, "Puppeteering");
				ui.selectable_value(selected, moonlighter::Affinity::Rake, "Rake");
				ui.selectable_value(selected, moonlighter::Affinity::Religion, "Religion");
				ui.selectable_value(selected, moonlighter::Affinity::Repairing, "Repairing");
				ui.selectable_value(selected, moonlighter::Affinity::Restoration, "Restoration");
				ui.selectable_value(selected, moonlighter::Affinity::Ropemaking, "Ropemaking");
				ui.selectable_value(selected, moonlighter::Affinity::Saw, "Saw");
				ui.selectable_value(selected, moonlighter::Affinity::Scythe, "Scythe");
				ui.selectable_value(selected, moonlighter::Affinity::ShieldBashing, "ShieldBashing");
				ui.selectable_value(selected, moonlighter::Affinity::ShieldSmithing, "ShieldSmithing");
				ui.selectable_value(selected, moonlighter::Affinity::Shields, "Shields");
				ui.selectable_value(selected, moonlighter::Affinity::ShipBuilding, "ShipBuilding");
				ui.selectable_value(selected, moonlighter::Affinity::ShortBow, "ShortBow");
				ui.selectable_value(selected, moonlighter::Affinity::Shortsword, "Shortsword");
				ui.selectable_value(selected, moonlighter::Affinity::Shovel, "Shovel");
				ui.selectable_value(selected, moonlighter::Affinity::Sickle, "Sickle");
				ui.selectable_value(selected, moonlighter::Affinity::SmallAxe, "SmallAxe");
				ui.selectable_value(selected, moonlighter::Affinity::SmallMaul, "SmallMaul");
				ui.selectable_value(selected, moonlighter::Affinity::SmallMetalShield, "SmallMetalShield");
				ui.selectable_value(selected, moonlighter::Affinity::SmallWoodenShield, "SmallWoodenShield");
				ui.selectable_value(selected, moonlighter::Affinity::Smithing, "Smithing");
				ui.selectable_value(selected, moonlighter::Affinity::Soul, "Soul");
				ui.selectable_value(selected, moonlighter::Affinity::SoulDepth, "SoulDepth");
				ui.selectable_value(selected, moonlighter::Affinity::SoulStrength, "SoulStrength");
				ui.selectable_value(selected, moonlighter::Affinity::Staff, "Staff");
				ui.selectable_value(selected, moonlighter::Affinity::Stealing, "Stealing");
				ui.selectable_value(selected, moonlighter::Affinity::StoneChisel, "StoneChisel");
				ui.selectable_value(selected, moonlighter::Affinity::StoneCutting, "StoneCutting");
				ui.selectable_value(selected, moonlighter::Affinity::Swords, "Swords");
				ui.selectable_value(selected, moonlighter::Affinity::Tailoring, "Tailoring");
				ui.selectable_value(selected, moonlighter::Affinity::Taunting, "Taunting");
				ui.selectable_value(selected, moonlighter::Affinity::Thatching, "Thatching");
				ui.selectable_value(selected, moonlighter::Affinity::Thievery, "Thievery");
				ui.selectable_value(selected, moonlighter::Affinity::ToyMaking, "ToyMaking");
				ui.selectable_value(selected, moonlighter::Affinity::Toys, "Toys");
				ui.selectable_value(selected, moonlighter::Affinity::Tracking, "Tracking");
				ui.selectable_value(selected, moonlighter::Affinity::Traps, "Traps");
				ui.selectable_value(selected, moonlighter::Affinity::Trebuchets, "Trebuchets");
				ui.selectable_value(selected, moonlighter::Affinity::TwoHandedSword, "TwoHandedSword");
				ui.selectable_value(selected, moonlighter::Affinity::WarMachines, "WarMachines");
				ui.selectable_value(selected, moonlighter::Affinity::Warhammer, "Warhammer");
				ui.selectable_value(selected, moonlighter::Affinity::WeaponHeadsSmithing, "WeaponHeadsSmithing");
				ui.selectable_value(selected, moonlighter::Affinity::WeaponSmithing, "WeaponSmithing");
				ui.selectable_value(selected, moonlighter::Affinity::WeaponlessFighting, "WeaponlessFighting");
				ui.selectable_value(selected, moonlighter::Affinity::Woodcutting, "Woodcutting");
				ui.selectable_value(selected, moonlighter::Affinity::Yoyo, "Yoyo");
			});

			let selected = &mut self.affinity;

			let combo = egui::ComboBox::from_label("<- Desired affinity").selected_text(format!("{:?}", selected));
			combo.show_ui(ui, |ui| {
				ui.selectable_value(selected, moonlighter::Affinity::AggressiveFighting, "AggressiveFighting");
				ui.selectable_value(selected, moonlighter::Affinity::Alchemy, "Alchemy");
				ui.selectable_value(selected, moonlighter::Affinity::AnimalHusbandry, "AnimalHusbandry");
				ui.selectable_value(selected, moonlighter::Affinity::AnimalTaming, "AnimalTaming");
				ui.selectable_value(selected, moonlighter::Affinity::Archaeology, "Archaeology");
				ui.selectable_value(selected, moonlighter::Affinity::Archery, "Archery");
				ui.selectable_value(selected, moonlighter::Affinity::ArmourSmithing, "ArmourSmithing");
				ui.selectable_value(selected, moonlighter::Affinity::Axes, "Axes");
				ui.selectable_value(selected, moonlighter::Affinity::Baking, "Baking");
				ui.selectable_value(selected, moonlighter::Affinity::Ballistae, "Ballistae");
				ui.selectable_value(selected, moonlighter::Affinity::Beverages, "Beverages");
				ui.selectable_value(selected, moonlighter::Affinity::Blacksmithing, "Blacksmithing");
				ui.selectable_value(selected, moonlighter::Affinity::BladesSmithing, "BladesSmithing");
				ui.selectable_value(selected, moonlighter::Affinity::Body, "Body");
				ui.selectable_value(selected, moonlighter::Affinity::BodyControl, "BodyControl");
				ui.selectable_value(selected, moonlighter::Affinity::BodyStamina, "BodyStamina");
				ui.selectable_value(selected, moonlighter::Affinity::BodyStrength, "BodyStrength");
				ui.selectable_value(selected, moonlighter::Affinity::Botanizing, "Botanizing");
				ui.selectable_value(selected, moonlighter::Affinity::Bowyery, "Bowyery");
				ui.selectable_value(selected, moonlighter::Affinity::Butchering, "Butchering");
				ui.selectable_value(selected, moonlighter::Affinity::ButcheringKnife, "ButcheringKnife");
				ui.selectable_value(selected, moonlighter::Affinity::Carpentry, "Carpentry");
				ui.selectable_value(selected, moonlighter::Affinity::CarvingKnife, "CarvingKnife");
				ui.selectable_value(selected, moonlighter::Affinity::Catapults, "Catapults");
				ui.selectable_value(selected, moonlighter::Affinity::ChainArmourSmithing, "ChainArmourSmithing");
				ui.selectable_value(selected, moonlighter::Affinity::Channeling, "Channeling");
				ui.selectable_value(selected, moonlighter::Affinity::Climbing, "Climbing");
				ui.selectable_value(selected, moonlighter::Affinity::ClothTailoring, "ClothTailoring");
				ui.selectable_value(selected, moonlighter::Affinity::Clubs, "Clubs");
				ui.selectable_value(selected, moonlighter::Affinity::CoalMaking, "CoalMaking");
				ui.selectable_value(selected, moonlighter::Affinity::Cooking, "Cooking");
				ui.selectable_value(selected, moonlighter::Affinity::DairyFoodMaking, "DairyFoodMaking");
				ui.selectable_value(selected, moonlighter::Affinity::DefensiveFighting, "DefensiveFighting");
				ui.selectable_value(selected, moonlighter::Affinity::Digging, "Digging");
				ui.selectable_value(selected, moonlighter::Affinity::Exorcism, "Exorcism");
				ui.selectable_value(selected, moonlighter::Affinity::Farming, "Farming");
				ui.selectable_value(selected, moonlighter::Affinity::Fighting, "Fighting");
				ui.selectable_value(selected, moonlighter::Affinity::FineCarpentry, "FineCarpentry");
				ui.selectable_value(selected, moonlighter::Affinity::Firemaking, "Firemaking");
				ui.selectable_value(selected, moonlighter::Affinity::FirstAid, "FirstAid");
				ui.selectable_value(selected, moonlighter::Affinity::Fishing, "Fishing");
				ui.selectable_value(selected, moonlighter::Affinity::Fletching, "Fletching");
				ui.selectable_value(selected, moonlighter::Affinity::Foraging, "Foraging");
				ui.selectable_value(selected, moonlighter::Affinity::Forestry, "Forestry");
				ui.selectable_value(selected, moonlighter::Affinity::Gardening, "Gardening");
				ui.selectable_value(selected, moonlighter::Affinity::Halberd, "Halberd");
				ui.selectable_value(selected, moonlighter::Affinity::Hammer, "Hammer");
				ui.selectable_value(selected, moonlighter::Affinity::Hammers, "Hammers");
				ui.selectable_value(selected, moonlighter::Affinity::Hatchet, "Hatchet");
				ui.selectable_value(selected, moonlighter::Affinity::Healing, "Healing");
				ui.selectable_value(selected, moonlighter::Affinity::HotFoodCooking, "HotFoodCooking");
				ui.selectable_value(selected, moonlighter::Affinity::HugeAxe, "HugeAxe");
				ui.selectable_value(selected, moonlighter::Affinity::HugeClub, "HugeClub");
				ui.selectable_value(selected, moonlighter::Affinity::JewelrySmithing, "JewelrySmithing");
				ui.selectable_value(selected, moonlighter::Affinity::Knives, "Knives");
				ui.selectable_value(selected, moonlighter::Affinity::LargeAxe, "LargeAxe");
				ui.selectable_value(selected, moonlighter::Affinity::LargeMaul, "LargeMaul");
				ui.selectable_value(selected, moonlighter::Affinity::LargeMetalShield, "LargeMetalShield");
				ui.selectable_value(selected, moonlighter::Affinity::LargeWoodenShield, "LargeWoodenShield");
				ui.selectable_value(selected, moonlighter::Affinity::Leatherworking, "Leatherworking");
				ui.selectable_value(selected, moonlighter::Affinity::LockPicking, "LockPicking");
				ui.selectable_value(selected, moonlighter::Affinity::Locksmithing, "Locksmithing");
				ui.selectable_value(selected, moonlighter::Affinity::LongBow, "LongBow");
				ui.selectable_value(selected, moonlighter::Affinity::LongSpear, "LongSpear");
				ui.selectable_value(selected, moonlighter::Affinity::Longsword, "Longsword");
				ui.selectable_value(selected, moonlighter::Affinity::Masonry, "Masonry");
				ui.selectable_value(selected, moonlighter::Affinity::Mauls, "Mauls");
				ui.selectable_value(selected, moonlighter::Affinity::Meditating, "Meditating");
				ui.selectable_value(selected, moonlighter::Affinity::MediumBow, "MediumBow");
				ui.selectable_value(selected, moonlighter::Affinity::MediumMaul, "MediumMaul");
				ui.selectable_value(selected, moonlighter::Affinity::MediumMetalShield, "MediumMetalShield");
				ui.selectable_value(selected, moonlighter::Affinity::MediumWoodenShield, "MediumWoodenShield");
				ui.selectable_value(selected, moonlighter::Affinity::Metallurgy, "Metallurgy");
				ui.selectable_value(selected, moonlighter::Affinity::Milking, "Milking");
				ui.selectable_value(selected, moonlighter::Affinity::Milling, "Milling");
				ui.selectable_value(selected, moonlighter::Affinity::Mind, "Mind");
				ui.selectable_value(selected, moonlighter::Affinity::MindLogic, "MindLogic");
				ui.selectable_value(selected, moonlighter::Affinity::MindSpeed, "MindSpeed");
				ui.selectable_value(selected, moonlighter::Affinity::Mining, "Mining");
				ui.selectable_value(selected, moonlighter::Affinity::MiscItems, "MiscItems");
				ui.selectable_value(selected, moonlighter::Affinity::NaturalSubstances, "NaturalSubstances");
				ui.selectable_value(selected, moonlighter::Affinity::Nature, "Nature");
				ui.selectable_value(selected, moonlighter::Affinity::NormalFighting, "NormalFighting");
				ui.selectable_value(selected, moonlighter::Affinity::Papyrusmaking, "Papyrusmaking");
				ui.selectable_value(selected, moonlighter::Affinity::Paving, "Paving");
				ui.selectable_value(selected, moonlighter::Affinity::Pickaxe, "Pickaxe");
				ui.selectable_value(selected, moonlighter::Affinity::PlateArmourSmithing, "PlateArmourSmithing");
				ui.selectable_value(selected, moonlighter::Affinity::Polearms, "Polearms");
				ui.selectable_value(selected, moonlighter::Affinity::Pottery, "Pottery");
				ui.selectable_value(selected, moonlighter::Affinity::Praying, "Praying");
				ui.selectable_value(selected, moonlighter::Affinity::Preaching, "Preaching");
				ui.selectable_value(selected, moonlighter::Affinity::Prospecting, "Prospecting");
				ui.selectable_value(selected, moonlighter::Affinity::Puppeteering, "Puppeteering");
				ui.selectable_value(selected, moonlighter::Affinity::Rake, "Rake");
				ui.selectable_value(selected, moonlighter::Affinity::Religion, "Religion");
				ui.selectable_value(selected, moonlighter::Affinity::Repairing, "Repairing");
				ui.selectable_value(selected, moonlighter::Affinity::Restoration, "Restoration");
				ui.selectable_value(selected, moonlighter::Affinity::Ropemaking, "Ropemaking");
				ui.selectable_value(selected, moonlighter::Affinity::Saw, "Saw");
				ui.selectable_value(selected, moonlighter::Affinity::Scythe, "Scythe");
				ui.selectable_value(selected, moonlighter::Affinity::ShieldBashing, "ShieldBashing");
				ui.selectable_value(selected, moonlighter::Affinity::ShieldSmithing, "ShieldSmithing");
				ui.selectable_value(selected, moonlighter::Affinity::Shields, "Shields");
				ui.selectable_value(selected, moonlighter::Affinity::ShipBuilding, "ShipBuilding");
				ui.selectable_value(selected, moonlighter::Affinity::ShortBow, "ShortBow");
				ui.selectable_value(selected, moonlighter::Affinity::Shortsword, "Shortsword");
				ui.selectable_value(selected, moonlighter::Affinity::Shovel, "Shovel");
				ui.selectable_value(selected, moonlighter::Affinity::Sickle, "Sickle");
				ui.selectable_value(selected, moonlighter::Affinity::SmallAxe, "SmallAxe");
				ui.selectable_value(selected, moonlighter::Affinity::SmallMaul, "SmallMaul");
				ui.selectable_value(selected, moonlighter::Affinity::SmallMetalShield, "SmallMetalShield");
				ui.selectable_value(selected, moonlighter::Affinity::SmallWoodenShield, "SmallWoodenShield");
				ui.selectable_value(selected, moonlighter::Affinity::Smithing, "Smithing");
				ui.selectable_value(selected, moonlighter::Affinity::Soul, "Soul");
				ui.selectable_value(selected, moonlighter::Affinity::SoulDepth, "SoulDepth");
				ui.selectable_value(selected, moonlighter::Affinity::SoulStrength, "SoulStrength");
				ui.selectable_value(selected, moonlighter::Affinity::Staff, "Staff");
				ui.selectable_value(selected, moonlighter::Affinity::Stealing, "Stealing");
				ui.selectable_value(selected, moonlighter::Affinity::StoneChisel, "StoneChisel");
				ui.selectable_value(selected, moonlighter::Affinity::StoneCutting, "StoneCutting");
				ui.selectable_value(selected, moonlighter::Affinity::Swords, "Swords");
				ui.selectable_value(selected, moonlighter::Affinity::Tailoring, "Tailoring");
				ui.selectable_value(selected, moonlighter::Affinity::Taunting, "Taunting");
				ui.selectable_value(selected, moonlighter::Affinity::Thatching, "Thatching");
				ui.selectable_value(selected, moonlighter::Affinity::Thievery, "Thievery");
				ui.selectable_value(selected, moonlighter::Affinity::ToyMaking, "ToyMaking");
				ui.selectable_value(selected, moonlighter::Affinity::Toys, "Toys");
				ui.selectable_value(selected, moonlighter::Affinity::Tracking, "Tracking");
				ui.selectable_value(selected, moonlighter::Affinity::Traps, "Traps");
				ui.selectable_value(selected, moonlighter::Affinity::Trebuchets, "Trebuchets");
				ui.selectable_value(selected, moonlighter::Affinity::TwoHandedSword, "TwoHandedSword");
				ui.selectable_value(selected, moonlighter::Affinity::WarMachines, "WarMachines");
				ui.selectable_value(selected, moonlighter::Affinity::Warhammer, "Warhammer");
				ui.selectable_value(selected, moonlighter::Affinity::WeaponHeadsSmithing, "WeaponHeadsSmithing");
				ui.selectable_value(selected, moonlighter::Affinity::WeaponSmithing, "WeaponSmithing");
				ui.selectable_value(selected, moonlighter::Affinity::WeaponlessFighting, "WeaponlessFighting");
				ui.selectable_value(selected, moonlighter::Affinity::Woodcutting, "Woodcutting");
				ui.selectable_value(selected, moonlighter::Affinity::Yoyo, "Yoyo");
			});

			if ui.button("Generate").clicked() {
				self.checkbox_states = [false; 15];
				self.recipe = moonlighter::find_recipe(&moonlighter::Options {
					affinity: self.affinity.clone(),
					max_length: self.vegetables,
					max_sugars: self.sugars,
					player_number: (138 + 57 + self.bear_meal_affinity.offset() - moonlighter::Affinity::CoalMaking.offset()) % 138,
					rare: self.rare,
				});
			};

			if let Some(recipe) = &self.recipe {
				ui.label(format!("Best recipe found with {} vegetables!", recipe.unique_vegs.len()));
				ui.checkbox(&mut self.checkbox_states[0], format!("{:?}", recipe.cereal));
				ui.checkbox(&mut self.checkbox_states[1], "water");
				ui.checkbox(&mut self.checkbox_states[2], format!("{} sugars", recipe.filler_sugars));
				for (idx, (veg, processing)) in recipe.unique_vegs.iter().enumerate() {
					ui.checkbox(&mut self.checkbox_states[3 + idx], format!("{:?} {:?}", veg, processing));
				}
			}

			ui.separator();

			ui.horizontal(|ui| {
				ui.label("Grimmaz a Zink Stake. Source code:");
				ui.add(egui::github_link_file!("https://github.com/zink-stake/v12", "V12"));
				ui.label("and");
				ui.add(egui::github_link_file!("https://github.com/zink-stake/moonlighter", "moonlighter"));
			});

			ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
				powered_by_egui_and_eframe(ui);
				egui::warn_if_debug_build(ui);
			});
		});
	}
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
	ui.horizontal(|ui| {
		ui.spacing_mut().item_spacing.x = 0.0;
		ui.label("Powered by ");
		ui.hyperlink_to("egui", "https://github.com/emilk/egui");
		ui.label(" and ");
		ui.hyperlink_to("eframe", "https://github.com/emilk/egui/tree/master/crates/eframe");
		ui.label(".");
	});
}
