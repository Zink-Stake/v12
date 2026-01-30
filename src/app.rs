/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
	// Example stuff:
	player_number: usize,

	#[serde(skip)] // This how you opt-out of serialization of a field
	affinity: moonlighter::Affinity,
	#[serde(skip)]
	recipe: Option<moonlighter::Recipe>,
}

impl Default for TemplateApp {
	fn default() -> Self {
		Self {
			// Example stuff:
			player_number: 0,
			affinity: moonlighter::Affinity::AggressiveFighting,
			recipe: None,
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
			ui.heading("eframe template");

			ui.horizontal(|ui| {
				ui.label("Player number: ");
				ui.add(egui::Slider::new(&mut self.player_number, 0..=138).text("player number"));
			});

			let selected = &mut self.affinity;

			let combo = egui::ComboBox::from_label("Desired affinity").selected_text(format!("{:?}", selected));
			combo.show_ui(ui, |ui| {
				ui.selectable_value(selected, moonlighter::Affinity::AggressiveFighting, "Aggressive Fighting");
				ui.selectable_value(selected, moonlighter::Affinity::Alchemy, "Alchemy");
				ui.selectable_value(selected, moonlighter::Affinity::AnimalHusbandry, "Animal Husbandry");
				ui.selectable_value(selected, moonlighter::Affinity::AnimalTaming, "Animal Taming");
				ui.selectable_value(selected, moonlighter::Affinity::Archaeology, "Archaeology");
				ui.selectable_value(selected, moonlighter::Affinity::Archery, "Archery");
				ui.selectable_value(selected, moonlighter::Affinity::ArmourSmithing, "Armour Smithing");
				ui.selectable_value(selected, moonlighter::Affinity::Axes, "Axes");
				ui.selectable_value(selected, moonlighter::Affinity::Baking, "Baking");
				ui.selectable_value(selected, moonlighter::Affinity::Ballistae, "Ballistae");
				ui.selectable_value(selected, moonlighter::Affinity::Beverages, "Beverages");
				ui.selectable_value(selected, moonlighter::Affinity::Blacksmithing, "Blacksmithing");
				ui.selectable_value(selected, moonlighter::Affinity::BladesSmithing, "Blades Smithing");
				ui.selectable_value(selected, moonlighter::Affinity::Body, "Body");
				ui.selectable_value(selected, moonlighter::Affinity::BodyControl, "Body Control");
				ui.selectable_value(selected, moonlighter::Affinity::BodyStamina, "Body Stamina");
				ui.selectable_value(selected, moonlighter::Affinity::BodyStrength, "Body Strength");
				ui.selectable_value(selected, moonlighter::Affinity::Botanizing, "Botanizing");
				ui.selectable_value(selected, moonlighter::Affinity::Bowyery, "Bowyery");
				ui.selectable_value(selected, moonlighter::Affinity::Butchering, "Butchering");
				ui.selectable_value(selected, moonlighter::Affinity::ButcheringKnife, "Butchering Knife");
				ui.selectable_value(selected, moonlighter::Affinity::Carpentry, "Carpentry");
				ui.selectable_value(selected, moonlighter::Affinity::CarvingKnife, "Carving Knife");
				ui.selectable_value(selected, moonlighter::Affinity::Catapults, "Catapults");
				ui.selectable_value(selected, moonlighter::Affinity::ChainArmourSmithing, "Chain Armour Smithing");
				ui.selectable_value(selected, moonlighter::Affinity::Channeling, "Channeling");
				ui.selectable_value(selected, moonlighter::Affinity::Climbing, "Climbing");
				ui.selectable_value(selected, moonlighter::Affinity::ClothTailoring, "Cloth Tailoring");
				ui.selectable_value(selected, moonlighter::Affinity::Clubs, "Clubs");
				ui.selectable_value(selected, moonlighter::Affinity::CoalMaking, "Coal-Making");
				ui.selectable_value(selected, moonlighter::Affinity::Cooking, "Cooking");
				ui.selectable_value(selected, moonlighter::Affinity::DairyFoodMaking, "Dairy Food Making");
				ui.selectable_value(selected, moonlighter::Affinity::DefensiveFighting, "Defensive Fighting");
				ui.selectable_value(selected, moonlighter::Affinity::Digging, "Digging");
				ui.selectable_value(selected, moonlighter::Affinity::Exorcism, "Exorcism");
				ui.selectable_value(selected, moonlighter::Affinity::Farming, "Farming");
				ui.selectable_value(selected, moonlighter::Affinity::Fighting, "Fighting");
				ui.selectable_value(selected, moonlighter::Affinity::FineCarpentry, "Fine Carpentry");
				ui.selectable_value(selected, moonlighter::Affinity::Firemaking, "Firemaking");
				ui.selectable_value(selected, moonlighter::Affinity::FirstAid, "First Aid");
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
				ui.selectable_value(selected, moonlighter::Affinity::HotFoodCooking, "Hot Food Cooking");
				ui.selectable_value(selected, moonlighter::Affinity::HugeAxe, "Huge Axe");
				ui.selectable_value(selected, moonlighter::Affinity::HugeClub, "Huge Club");
				ui.selectable_value(selected, moonlighter::Affinity::JewelrySmithing, "Jewelry Smithing");
				ui.selectable_value(selected, moonlighter::Affinity::Knives, "Knives");
				ui.selectable_value(selected, moonlighter::Affinity::LargeAxe, "Large Axe");
				ui.selectable_value(selected, moonlighter::Affinity::LargeMaul, "Large Maul");
				ui.selectable_value(selected, moonlighter::Affinity::LargeMetalShield, "Large Metal Shield");
				ui.selectable_value(selected, moonlighter::Affinity::LargeWoodenShield, "Large Wooden Shield");
				ui.selectable_value(selected, moonlighter::Affinity::Leatherworking, "Leather working");
				ui.selectable_value(selected, moonlighter::Affinity::LockPicking, "Lock Picking");
				ui.selectable_value(selected, moonlighter::Affinity::Locksmithing, "Locksmithing");
				ui.selectable_value(selected, moonlighter::Affinity::LongBow, "Long Bow");
				ui.selectable_value(selected, moonlighter::Affinity::LongSpear, "Long Spear");
				ui.selectable_value(selected, moonlighter::Affinity::Longsword, "Longsword");
				ui.selectable_value(selected, moonlighter::Affinity::Masonry, "Masonry");
				ui.selectable_value(selected, moonlighter::Affinity::Mauls, "Mauls");
				ui.selectable_value(selected, moonlighter::Affinity::Meditating, "Meditating");
				ui.selectable_value(selected, moonlighter::Affinity::MediumBow, "Medium Bow");
				ui.selectable_value(selected, moonlighter::Affinity::MediumMaul, "Medium Maul");
				ui.selectable_value(selected, moonlighter::Affinity::MediumMetalShield, "Medium Metal Shield");
				ui.selectable_value(selected, moonlighter::Affinity::MediumWoodenShield, "Medium Wooden Shield");
				ui.selectable_value(selected, moonlighter::Affinity::Metallurgy, "Metallurgy");
				ui.selectable_value(selected, moonlighter::Affinity::Milking, "Milking");
				ui.selectable_value(selected, moonlighter::Affinity::Milling, "Milling");
				ui.selectable_value(selected, moonlighter::Affinity::Mind, "Mind");
				ui.selectable_value(selected, moonlighter::Affinity::MindLogic, "Mind Logic");
				ui.selectable_value(selected, moonlighter::Affinity::MindSpeed, "Mind Speed");
				ui.selectable_value(selected, moonlighter::Affinity::Mining, "Mining");
				ui.selectable_value(selected, moonlighter::Affinity::MiscItems, "Misc Items");
				ui.selectable_value(selected, moonlighter::Affinity::NaturalSubstances, "Natural Substances");
				ui.selectable_value(selected, moonlighter::Affinity::Nature, "Nature");
				ui.selectable_value(selected, moonlighter::Affinity::NormalFighting, "Normal Fighting");
				ui.selectable_value(selected, moonlighter::Affinity::Papyrusmaking, "Papyrusmaking");
				ui.selectable_value(selected, moonlighter::Affinity::Paving, "Paving");
				ui.selectable_value(selected, moonlighter::Affinity::Pickaxe, "Pick axe");
				ui.selectable_value(selected, moonlighter::Affinity::PlateArmourSmithing, "Plate Armour Smithing");
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
				ui.selectable_value(selected, moonlighter::Affinity::ShieldBashing, "Shield Bashing");
				ui.selectable_value(selected, moonlighter::Affinity::ShieldSmithing, "Shield Smithing");
				ui.selectable_value(selected, moonlighter::Affinity::Shields, "Shields");
				ui.selectable_value(selected, moonlighter::Affinity::ShipBuilding, "Ship Building");
				ui.selectable_value(selected, moonlighter::Affinity::ShortBow, "Short Bow");
				ui.selectable_value(selected, moonlighter::Affinity::Shortsword, "Shortsword");
				ui.selectable_value(selected, moonlighter::Affinity::Shovel, "Shovel");
				ui.selectable_value(selected, moonlighter::Affinity::Sickle, "Sickle");
				ui.selectable_value(selected, moonlighter::Affinity::SmallAxe, "Small Axe");
				ui.selectable_value(selected, moonlighter::Affinity::SmallMaul, "Small Maul");
				ui.selectable_value(selected, moonlighter::Affinity::SmallMetalShield, "Small Metal Shield");
				ui.selectable_value(selected, moonlighter::Affinity::SmallWoodenShield, "Small Wooden Shield");
				ui.selectable_value(selected, moonlighter::Affinity::Smithing, "Smithing");
				ui.selectable_value(selected, moonlighter::Affinity::Soul, "Soul");
				ui.selectable_value(selected, moonlighter::Affinity::SoulDepth, "Soul Depth");
				ui.selectable_value(selected, moonlighter::Affinity::SoulStrength, "Soul Strength");
				ui.selectable_value(selected, moonlighter::Affinity::Staff, "Staff");
				ui.selectable_value(selected, moonlighter::Affinity::Stealing, "Stealing");
				ui.selectable_value(selected, moonlighter::Affinity::StoneChisel, "Stone Chisel");
				ui.selectable_value(selected, moonlighter::Affinity::StoneCutting, "Stone Cutting");
				ui.selectable_value(selected, moonlighter::Affinity::Swords, "Swords");
				ui.selectable_value(selected, moonlighter::Affinity::Tailoring, "Tailoring");
				ui.selectable_value(selected, moonlighter::Affinity::Taunting, "Taunting");
				ui.selectable_value(selected, moonlighter::Affinity::Thatching, "Thatching");
				ui.selectable_value(selected, moonlighter::Affinity::Thievery, "Thievery");
				ui.selectable_value(selected, moonlighter::Affinity::ToyMaking, "Toy Making");
				ui.selectable_value(selected, moonlighter::Affinity::Toys, "Toys");
				ui.selectable_value(selected, moonlighter::Affinity::Tracking, "Tracking");
				ui.selectable_value(selected, moonlighter::Affinity::Traps, "Traps");
				ui.selectable_value(selected, moonlighter::Affinity::Trebuchets, "Trebuchets");
				ui.selectable_value(selected, moonlighter::Affinity::TwoHandedSword, "Two Handed Sword");
				ui.selectable_value(selected, moonlighter::Affinity::WarMachines, "War Machines");
				ui.selectable_value(selected, moonlighter::Affinity::Warhammer, "Warhammer");
				ui.selectable_value(selected, moonlighter::Affinity::WeaponHeadsSmithing, "Weapon Heads Smithing");
				ui.selectable_value(selected, moonlighter::Affinity::WeaponSmithing, "Weapon Smithing");
				ui.selectable_value(selected, moonlighter::Affinity::WeaponlessFighting, "Weaponless Fighting");
				ui.selectable_value(selected, moonlighter::Affinity::Woodcutting, "Woodcutting");
				ui.selectable_value(selected, moonlighter::Affinity::Yoyo, "Yoyo");
			});

			if ui.button("Generate").clicked() {
				self.recipe = moonlighter::find_recipe(&moonlighter::Options {
					affinity: self.affinity.clone(),
					max_length: 12,
					max_sugars: 50,
					player_number: self.player_number,
					rare: false,
				});
			};

			if let Some(recipe) = &self.recipe {
				ui.label(format!("Best recipe found with {} vegetables!", recipe.unique_vegs.len()));
				let mut ch = false;
				ui.checkbox(&mut ch, format!("{:?}", recipe.cereal));
				let mut ch = false;
				ui.checkbox(&mut ch, "water");
				let mut ch = false;
				ui.checkbox(&mut ch, format!("{} sugars", recipe.filler_sugars));
				for (veg, processing) in &recipe.unique_vegs {
					let mut ch = false;
					ui.checkbox(&mut ch, format!("{:?} {:?}", veg, processing));
				}
			}

			ui.separator();

			ui.add(egui::github_link_file!("https://github.com/zink-stake/v12", "Source code."));

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
