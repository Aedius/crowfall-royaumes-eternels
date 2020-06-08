use crate::craft::BaseResource::*;

pub mod cooking;

pub enum Profession {
    Cooking
}


pub enum BaseResource {
    Apple,
    Beeswax,
    Bloodworm,
    Blood,
    Bone,
    Carrot,
    CocoaBean,
    CoffeeBean,
    GroundBlackPepper,
    HotPepper,
    HungerShard,
    MeatAuroch,
    MeatBear,
    MeatBigCat,
    MeatBoar,
    MeatElk,
    MeatSpider,
    MeatWolf,
    MildPepper,
    PineNuts,
    Potato,
    RawMilk,
    SugarCane,
    SeasoningSalt,
    SweetPepper,
    WaterFlask,
    WildRice,
    Onion,
}

pub enum GroupResource {
    AnimalMeat,
    Herb,
    Mushroom,
    MeatOrMushroom,
    NonBasicOre,
    NonBasicWood,
    Produce,
    Seasoning,
    Ore,
    WildRiceOrGnocchi,
}

pub enum CraftedResource {
    AppleJuice,
    ArtisanCheese,
    BakedIceCream,
    BasicRoastedMeat,
    BloodwormStew,
    BoneBroth,
    Bread,
    BiscuitsAndGravy,
    BonTippers,
    Butter,
    Cake,
    CampfireMeatyStew,
    CampfireMushroomStew,
    CarrotJuice,
    ChocolateBar,
    ChocolateMilk,
    ChocolateMilkSpicy,
    Coffee,
    Cookie,
    CookieChocolate,
    CookingFoil,
    CrushedHerb,
    EmptyFlask,
    Gnocchi,
    GrilledCheeseSandwich,
    GrilledSandwichAuroch,
    GrilledSandwichBoar,
    GrilledSandwichMushroom,
    GrilledSandwichBear,
    GrilledSandwichElk,
    GrilledSandwichWolf,
    GrilledSandwichSpider,
    GrilledSandwichBigCat,
    IceCream,
    KebabMushroom,
    KebabAuroch,
    KebabBear,
    KebabBigCat,
    KebabBoar,
    KebabElk,
    KebabSpider,
    KebabWolf,
    LargeCookingPot,
    MarsalaStew,
    MeatBurgundy,
    Mead,
    MushroomStew,
    Paella,
    PasteurizedMilk,
    PestoGnocchi,
    PulverizedPotato,
    PotatoFlour,
    PotRoast,
    RedWine,
    RoastingStick,
    RoastedPig,
    RoastedProduce,
    SeasonedMushroom,
    SumptuousPotPie,
    TrailMix,
    Yeast,
}

struct CraftedData {
    pub name: &'static str,
    pub stat: &'static str,
}


impl CraftedResource {
    fn get_information(&self) -> Option<CraftedData> {
        match self {
            CraftedResource::AppleJuice => {
                Some(
                    CraftedData {
                        name: "Apple Juice",
                        stat: "Find Weak Spot +5%",
                    }
                )
            }
            CraftedResource::ArtisanCheese => {
                Some(
                    CraftedData {
                        name: "Artisan Cheese",
                        stat: "Harvest Critical Chance +5%",
                    }
                )
            }
            CraftedResource::BakedIceCream => {
                Some(
                    CraftedData {
                        name: "Baked Ice Cream",
                        stat: "Chance to do Fire damage 5%",
                    }
                )
            }
            CraftedResource::BasicRoastedMeat => { None }
            CraftedResource::BloodwormStew => {
                Some(
                    CraftedData {
                        name: "Bloodworm Stew",
                        stat: "Food Regen +10, grants Trailblazer/master",
                    }
                )
            }
            CraftedResource::BoneBroth => {
                Some(
                    CraftedData {
                        name: "Bone Broth",
                        stat: "Incoming Healing +3%",
                    }
                )
            }
            CraftedResource::Bread => {
                Some(
                    CraftedData {
                        name: "Bread",
                        stat: "Pathfinding +10%",
                    }
                )
            }
            CraftedResource::BiscuitsAndGravy => {
                Some(
                    CraftedData {
                        name: "Biscuits and Gravy",
                        stat: "Food Regen Rate +20",
                    }
                )
            }
            CraftedResource::BonTippers => {
                Some(
                    CraftedData {
                        name: "Bon Tippers",
                        stat: "Exp Difficulty Mod +15 on next experiment",
                    }
                )
            }
            CraftedResource::Butter => {
                Some(
                    CraftedData {
                        name: "Fresh Butter",
                        stat: "Harvest Chance: Cutting Grit +2%",
                    }
                )
            }
            CraftedResource::Cake => {
                Some(
                    CraftedData {
                        name: "Cake",
                        stat: "Health +150, Stamina -15",
                    }
                )
            }
            CraftedResource::CampfireMeatyStew => {
                Some(
                    CraftedData {
                        name: "Campfire Meaty Stew",
                        stat: "Mounted Movement Speed +5%",
                    }
                )
            }
            CraftedResource::CampfireMushroomStew => {
                Some(
                    CraftedData {
                        name: "Campfire Mushroom Stew",
                        stat: "Harvest Critical Chance +5%",
                    }
                )
            }
            CraftedResource::CarrotJuice => {
                Some(
                    CraftedData {
                        name: "Carrot Juice",
                        stat: "Ranged Distance Bonus +3m",
                    }
                )
            }
            CraftedResource::ChocolateBar => {
                Some(
                    CraftedData {
                        name: "Chocalate Bar",
                        stat: "Combat Movement +5% / Health -150",
                    }
                )
            }
            CraftedResource::ChocolateMilk => {
                Some(
                    CraftedData {
                        name: "Chocolate Milk",
                        stat: "HP +150, Stam -15",
                    }
                )
            }
            CraftedResource::ChocolateMilkSpicy => {
                Some(
                    CraftedData {
                        name: "Spicy Chocolate Milk",
                        stat: "Elemental Damage +3%",
                    }
                )
            }
            CraftedResource::Coffee => {
                Some(
                    CraftedData {
                        name: "Coffee",
                        stat: "Stamina +10",
                    }
                )
            }
            CraftedResource::Cookie => {
                Some(
                    CraftedData {
                        name: "Cookies",
                        stat: "Combat Movement Speed +5%",
                    }
                )
            }
            CraftedResource::CookieChocolate => {
                Some(
                    CraftedData {
                        name: "Chocolate Cookies",
                        stat: "Combat Movement Speed +5%",
                    }
                )
            }
            CraftedResource::CookingFoil => { None }
            CraftedResource::CrushedHerb => { None }
            CraftedResource::EmptyFlask => { None }
            CraftedResource::Gnocchi => {
                Some(
                    CraftedData {
                        name: "Gnocchi",
                        stat: "Pathfinding Speed +10%",
                    }
                )
            }
            CraftedResource::GrilledCheeseSandwich => {
                Some(
                    CraftedData {
                        name: "Grilled Cheese Sandwich",
                        stat: "Harvest Critical Amount +1",
                    }
                )
            }
            CraftedResource::GrilledSandwichAuroch => {
                Some(
                    CraftedData {
                        name: "Auroch Meat Sandwich",
                        stat: "Fire Armor Bonus +3%",
                    }
                )
            }
            CraftedResource::GrilledSandwichBoar => {
                Some(
                    CraftedData {
                        name: "Boar Meat Sandwich",
                        stat: "Ice Armor Bonus +3%",
                    }
                )
            }
            CraftedResource::GrilledSandwichMushroom => {
                Some(
                    CraftedData {
                        name: "Mushroom Sandwich",
                        stat: "Harvest Critical Amount +1",
                    }
                )
            }
            CraftedResource::GrilledSandwichBear => {
                Some(
                    CraftedData {
                        name: "Bear Meat Sandwich",
                        stat: "Disease Armor Bonus +3%",
                    }
                )
            }
            CraftedResource::GrilledSandwichElk => {
                Some(
                    CraftedData {
                        name: "Elk Meat Sandwich",
                        stat: "Ice Armor Bonus +3%",
                    }
                )
            }
            CraftedResource::GrilledSandwichWolf => {
                Some(
                    CraftedData {
                        name: "Wolf Meat Sandwich",
                        stat: "Electric Armor Bonus +3%",
                    }
                )
            }
            CraftedResource::GrilledSandwichSpider => {
                Some(
                    CraftedData {
                        name: "Spider Meat Sandwich",
                        stat: "Poison Armor Bonus +3%",
                    }
                )
            }
            CraftedResource::GrilledSandwichBigCat => {
                Some(
                    CraftedData {
                        name: "Big Cat Meat Sandwidch",
                        stat: "Nature Armor Bonus +3%",
                    }
                )
            }
            CraftedResource::IceCream => {
                Some(
                    CraftedData {
                        name: "Ice Cream",
                        stat: "Chance to do Ice damage 5%",
                    }
                )
            }
            CraftedResource::KebabMushroom => {
                Some(
                    CraftedData {
                        name: "Mushroom Kebab",
                        stat: "Plentiful Harvest Wood +1",
                    }
                )
            }
            CraftedResource::KebabAuroch => {
                Some(
                    CraftedData {
                        name: "Auroch Meat Kebab",
                        stat: "Plentiful Harvest Ore +1",
                    }
                )
            }
            CraftedResource::KebabBear => {
                Some(
                    CraftedData {
                        name: "Bear Meat Kebab",
                        stat: "Plentiful Harvest Wood +1",
                    }
                )
            }
            CraftedResource::KebabBigCat => {
                Some(
                    CraftedData {
                        name: "Big Cat Meat Kebab",
                        stat: "Plentiful Harvest Ore +1",
                    }
                )
            }
            CraftedResource::KebabBoar => {
                Some(
                    CraftedData {
                        name: "Boar Meat Kebab",
                        stat: "Plentiful Harvest Stone +1",
                    }
                )
            }
            CraftedResource::KebabElk => {
                Some(
                    CraftedData {
                        name: "Elk Meat Kebab",
                        stat: "Plentiful Harvest Stone +1",
                    }
                )
            }
            CraftedResource::KebabSpider => {
                Some(
                    CraftedData {
                        name: "Spider Meat Kebab",
                        stat: "Plentiful Harvest Graves +1",
                    }
                )
            }
            CraftedResource::KebabWolf => {
                Some(
                    CraftedData {
                        name: "Wolf Meat Kebab",
                        stat: "Plentiful Harvest Animal +1",
                    }
                )
            }
            CraftedResource::LargeCookingPot => { None }
            CraftedResource::MarsalaStew => {
                Some(
                    CraftedData {
                        name: "Marsala Stew",
                        stat: "Bard Songs +6 seconds",
                    }
                )
            }
            CraftedResource::MeatBurgundy => {
                Some(
                    CraftedData {
                        name: "Meat Burgundy",
                        stat: "Basic Attack Damage +10%",
                    }
                )
            }
            CraftedResource::Mead => { None }
            CraftedResource::MushroomStew => {
                Some(
                    CraftedData {
                        name: "Mushroom Stew",
                        stat: "Harvest Critical Chance: All 5%",
                    }
                )
            }
            CraftedResource::Paella => {
                Some(
                    CraftedData {
                        name: "Paella",
                        stat: "Ranged Power Damage +5%",
                    }
                )
            }
            CraftedResource::PasteurizedMilk => {
                Some(
                    CraftedData {
                        name: "Pasteurized Milk",
                        stat: "Incoming Healing +3%",
                    }
                )
            }
            CraftedResource::PestoGnocchi => {
                Some(
                    CraftedData {
                        name: "Pesto Gnocchi",
                        stat: "Harvest Pips +0.5",
                    }
                )
            }
            CraftedResource::PulverizedPotato => { None }
            CraftedResource::PotatoFlour => { None }
            CraftedResource::PotRoast => {
                Some(
                    CraftedData {
                        name: "Pot Roast",
                        stat: "Healing Bonus +3%",
                    }
                )
            }
            CraftedResource::RedWine => {
                Some(
                    CraftedData {
                        name: "Red Wine",
                        stat: "Stamina +10, Food Regen -20",
                    }
                )
            }
            CraftedResource::RoastingStick => { None }
            CraftedResource::RoastedPig => {
                Some(
                    CraftedData {
                        name: "Roasted Pig",
                        stat: "Healing Bonus +3%",
                    }
                )
            }
            CraftedResource::RoastedProduce => { None }
            CraftedResource::SeasonedMushroom => {
                Some(
                    CraftedData {
                        name: "Seasoned Mushroom",
                        stat: "Harvest Chance: Soulgems +2%",
                    }
                )
            }
            CraftedResource::SumptuousPotPie => {
                Some(
                    CraftedData {
                        name: "Sumptuous Pot Pie",
                        stat: "General Crafting Exp. Points +1",
                    }
                )
            }
            CraftedResource::TrailMix => {
                Some(
                    CraftedData {
                        name: "Trail Mix",
                        stat: "Harvest Chance: Survivalist +3%",
                    }
                )
            }
            CraftedResource::Yeast => { None }
        }
    }

    pub fn has_stats(&self) -> bool {
        match self.get_information() {
            Some(_) => true,
            None => false,
        }
    }

    pub fn html_row(&self) -> String {
        match self.get_information() {
            Some(info) => {
                format!("{} : {}", info.name, info.stat)
            }
            None => "".to_string(),
        }
    }
}


pub enum Item {
    Base(BaseResource),
    Group(GroupResource),
    Crafted(CraftedResource),
}


pub struct Recipe {
    pub name: &'static str,
    pub input: Vec<(Item, i8)>,
    pub output: (CraftedResource, i8),
    pub profession: Profession,
    pub menu: &'static str,
}

impl Recipe {
    pub fn has_stats(&self) -> bool {
        self.output.0.has_stats()
    }

    pub fn output_description(&self) -> String {
        self.output.0.html_row()
    }
}


impl GroupResource {
    fn get_base(self) -> Vec<BaseResource> {
        match self {
            GroupResource::AnimalMeat => {
                vec![
                    MeatBear,
                    MeatBigCat,
                    MeatBoar,
                    MeatElk,
                    MeatSpider,
                    MeatWolf,
                ]
            }
            GroupResource::Herb => {
                vec![
                    Onion
                ]
            }
            GroupResource::Mushroom => {
                vec![
                    //TODO
                ]
            }
            GroupResource::MeatOrMushroom => {
                vec![
                    MeatBear,
                    MeatBigCat,
                    MeatBoar,
                    MeatElk,
                    MeatSpider,
                    MeatWolf,
                ]
            }
            GroupResource::NonBasicOre => {
                vec![
                    //TODO
                ]
            }
            GroupResource::NonBasicWood => {
                vec![
                    //TODO
                ]
            }
            GroupResource::Produce => {
                vec![
                    Apple,
                    Carrot,
                    Potato,
                    // Mushroom,
                    Onion,
                    HotPepper,
                    MildPepper,
                    HotPepper
                ]
            }
            GroupResource::Seasoning => {
                vec![
                    //TODO
                ]
            }
            GroupResource::Ore => {
                vec![
                    //TODO
                ]
            }
            GroupResource::WildRiceOrGnocchi => {
                vec![
                    WildRice,
                    // Gnocchi,
                ]
            }
        }
    }
}


