use bevy::prelude::*;

#[derive(Clone)]
pub struct ShopItemDef {
    pub id: &'static str,
    pub name: &'static str,
    pub desc: &'static str,
    pub cost: u64,
    pub max_purchases: u32,
}

#[derive(Resource)]
pub struct ShopCatalog {
    pub items: Vec<ShopItemDef>,
}

pub fn load_shop_catalog() -> ShopCatalog {
    ShopCatalog {
        items: vec![
            ShopItemDef { id: "hp_up", name: "Vitality Boon", desc: "+20 max HP", cost: 200, max_purchases: 5 },
            ShopItemDef { id: "dmg_up", name: "Soul Render", desc: "+15% damage", cost: 300, max_purchases: 3 },
            ShopItemDef { id: "speed_up", name: "Wind Walker", desc: "+10% move speed", cost: 250, max_purchases: 3 },
            ShopItemDef { id: "magnet_up", name: "Magnetism", desc: "+30% magnet range", cost: 150, max_purchases: 3 },
            ShopItemDef { id: "xp_up", name: "Wisdom", desc: "+20% XP gain", cost: 400, max_purchases: 3 },
            ShopItemDef { id: "extra_proj", name: "Duality", desc: "Fire an extra projectile", cost: 1000, max_purchases: 1 },
        ],
    }
}
