use crate::bits::read_data;
use std::collections::{HashSet, HashMap};
use itertools::Itertools;

pub fn day21a() -> String {
    let foods = read_ingredients();
    let allergen_free = find_allergen_free_ingredients(&foods);
    let answer = foods.into_iter()
        .map(|f| f.ingredients)
        .flatten()
        .filter(|ingred| allergen_free.contains(ingred))
        .count();
    format!("{}", answer)
}

pub fn day21b() -> String {
    let foods = read_ingredients();
    let allergen_free = find_allergen_free_ingredients(&foods);
    let foods = eliminate_allergen_free(foods, &allergen_free);
    let answer = identify_allergens_one_pass(&foods);
    let answer = answer.into_iter()
        .sorted_by(|(a, _), (b, _)| a.cmp(b))
        .map(|(_, ingred)| ingred).join(",");
    format!("{}", answer)
}

#[derive(Debug, Clone)]
pub struct Food {
    ingredients: Vec<String>,
    allergens: Vec<String>,
}

impl Food {
    pub fn from_definition(line: &str) -> Self {
        let raw = line.split(" (contains ").map(String::from).collect::<Vec<String>>();
        let ingredients = raw[0].split(' ').map(String::from).collect::<Vec<String>>();
        let allergens = raw[1][0..raw[1].len() - 1].split(", ").map(String::from).collect::<Vec<String>>();
        Self { ingredients, allergens }
    }
}

pub fn read_ingredients() -> Vec<Food> {
    let data = read_data("assets/day21.txt");
    data.iter()
        .filter(|s| !s.is_empty())
        .map(|s| Food::from_definition(s.as_str()))
        .collect()
}

fn eliminate_allergen_free(foods: Vec<Food>, allergen_free: &[String]) -> Vec<Food> {
    foods.into_iter().map(|f| {
        let ingredients = f.ingredients.into_iter().filter(|i| !allergen_free.contains(i)).collect();
        Food { ingredients, allergens: f.allergens }
    }).collect()
}

fn find_allergen_free_ingredients(foods: &[Food]) -> Vec<String> {
    let mut foods = foods.to_vec();
    let mut res = Vec::new();
    loop {
        let mut new_foods = find_allergen_free_foods_one_pass(&foods);
        println!("New allergen free foods: {}", new_foods.len());
        if new_foods.is_empty() {
            return res;
        }
        foods = eliminate_allergen_free(foods, &new_foods);
        res.append(&mut new_foods);
    }
}

fn find_allergen_free_foods_one_pass(foods: &[Food]) -> Vec<String> {
    let allergens = get_allergens(foods);
    let ingredients = get_ingredients(&foods);
    println!("Ingredients: {}, Allergens: {}", ingredients.len(), allergens.len());
    let mut possible_allergens = HashMap::new();
    ingredients.iter().for_each(|i| {
        possible_allergens.insert(i.clone(), allergens.clone());
    });
    for ingredient in &ingredients {
        foods.iter().filter(|f| !f.ingredients.contains(ingredient))
            .for_each(|f| {
                let possibles = possible_allergens.get_mut(ingredient).unwrap();
                f.allergens.iter().for_each(|a| { possibles.remove(a); });
            })
    }
    possible_allergens.into_iter()
        .filter(|(_, possibles)| possibles.is_empty())
        .map(|(ingredient, _)| ingredient)
        .collect()
}

fn get_allergens(foods: &[Food]) -> HashSet<String> {
    let mut allergens = HashSet::new();
    // Get a list of all the allergens
    for food in foods.iter() {
        food.allergens.iter()
            .for_each(|a| { allergens.insert(a.clone()); });
    }
    allergens
}

fn get_ingredients(foods: &[Food]) -> Vec<String> {
    let mut ingredients = HashSet::new();
    for food in foods.iter() {
        food.ingredients.iter()
            .for_each(|a| { ingredients.insert(a.clone()); });
    }
    ingredients.into_iter().collect()
}

fn identify_allergens_one_pass(foods: &[Food]) -> HashMap<String, String> {
    let mut res: HashMap<String, HashSet<String>> = HashMap::new();
    let allergens = get_allergens(foods);
    // Figure out which ingredients have the allergen
    // Take an allergen, and scan the food list, finding the ingredient that is in EVERY food with this allergen
    for allergen in allergens.iter() {
        let (counts, total) = foods.iter()
            .filter(|f| f.allergens.contains(allergen))
            .fold((HashMap::<String, usize>::new(), 0usize), |(mut counts, tot), f| {
                f.ingredients.iter().for_each(|ing| {
                    counts.entry(ing.clone())
                        .and_modify(|c| { *c += 1 })
                        .or_insert(1);
                });
                (counts, tot + 1)
            });
        // println!("\nTotal for allergen {}: {}, Counts: {:?}", allergen, total, counts);
        let ingreds_for_allergen = counts.iter()
            .filter(|(_, &count)| count == total)
            .map(|(i, _)| i.clone())
            .fold(HashSet::new(), |mut hs, i| {
                hs.insert(i);
                hs
            });
        res.insert(allergen.clone(), ingreds_for_allergen);
    }
    println!("{} allergens unidentified. {} allergens found", allergens.len(), res.len());
    remove_duplicates(&mut res);

    // Extract the Ingredients from the hashmap
    res.into_iter().fold(HashMap::new(), |mut result, (allergen, ingredients)| {
        let item = ingredients.iter().take(1).cloned().collect();
        result.insert(allergen, item);
        result
    })
}

fn remove_duplicates(allergen_map: &mut HashMap<String, HashSet<String>>) {
    let to_remove = allergen_map.iter()
        .filter(|(_, ingredients)| ingredients.len() == 1)
        .map(|(allergen, ingredients)|
            (allergen.clone(), ingredients.iter().take(1).cloned().collect())
        ).collect::<Vec<(String, String)>>();
    let mut changes = false;
    for (allergen, ingredient) in to_remove {
        allergen_map.iter_mut()
            .filter(|(a, _)| a.as_str() != allergen.as_str())
            .for_each(|(_a, v)| {
                changes = changes || v.remove(&ingredient);
            });
    }
    if changes { remove_duplicates(allergen_map); }
}