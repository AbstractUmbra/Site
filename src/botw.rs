use rocket::{form::Form, serde::Deserialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, FromForm)]
#[serde(crate = "rocket::serde")]
struct Headgear {
    tier_one: Option<HashMap<String, u8>>,
    tier_two: Option<HashMap<String, u8>>,
    tier_three: Option<HashMap<String, u8>>,
    tier_four: Option<HashMap<String, u8>>,
    totals: Option<HashMap<String, u8>>,
}
#[derive(Debug, Deserialize, FromForm)]
#[serde(crate = "rocket::serde")]
struct Chestpiece {
    tier_one: Option<HashMap<String, u8>>,
    tier_two: Option<HashMap<String, u8>>,
    tier_three: Option<HashMap<String, u8>>,
    tier_four: Option<HashMap<String, u8>>,
    totals: Option<HashMap<String, u8>>,
}
#[derive(Debug, Deserialize, FromForm)]
#[serde(crate = "rocket::serde")]
struct Greaves {
    tier_one: Option<HashMap<String, u8>>,
    tier_two: Option<HashMap<String, u8>>,
    tier_three: Option<HashMap<String, u8>>,
    tier_four: Option<HashMap<String, u8>>,
    totals: Option<HashMap<String, u8>>,
}
#[derive(Debug, Deserialize, FromForm)]
#[serde(crate = "rocket::serde")]
struct Boots {
    tier_one: Option<HashMap<String, u8>>,
    tier_two: Option<HashMap<String, u8>>,
    tier_three: Option<HashMap<String, u8>>,
    tier_four: Option<HashMap<String, u8>>,
    totals: Option<HashMap<String, u8>>,
}

#[derive(FromForm)]
struct ArmorSet<'r> {
    head: Headgear,
    body: Chestpiece,
    legs: Greaves,
    feet: Boots,
}

#[post("/armor", data = "<armor>")]
async fn return_armor_recipe(armor: Form<ArmorSet<'_>>) {
    todo!()
}
