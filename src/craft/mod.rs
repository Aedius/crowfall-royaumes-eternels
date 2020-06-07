use crate::craft::BaseResource::*;

pub mod cooking;

enum Profession {
    Cooking
}


enum BaseResource {
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

enum GroupResource {
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

enum CraftedResource {
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


enum Item {
    Base(BaseResource),
    Group(GroupResource),
    Crafted(CraftedResource),
}


pub struct Recipe {
    name: &'static str,
    input: Vec<(Item, i8)>,
    output: (CraftedResource, i8),
    profession: Profession,
    menu: &'static str,
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
