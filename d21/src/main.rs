use std::collections::HashSet;
use std::collections::HashMap;

struct Food {
    ingredients: HashSet<String>,
    allergens: HashSet<String>
}

impl Food {
    fn new(s: &str) -> Food {
        let mut split = s.split(" (");
        let i_part = split.next().unwrap();
        let a_part = split.next().unwrap();
        let ingredients: HashSet<_> = i_part.split(' ').map(|ss| String::from(ss)).collect();
        let allergens: HashSet<_> = a_part[9..(a_part.len()-1)].split(", ").map(|ss| String::from(ss)).collect();
        Food{ ingredients, allergens}
    }
}

fn main() {
    let foods: Vec<_> = include_str!("../input.dat").split('\n').map(
        |s| Food::new(s)
    ).collect();

    let mut all_allergens: HashSet<&String> = HashSet::new();
    let mut all_ingredients: HashSet<&String> = HashSet::new();
    let mut all_ingredients_list: Vec<&String> = vec!();
    for food in &foods {
        for a in &food.allergens {
            all_allergens.insert(a);
        }
        for a in &food.ingredients {
            all_ingredients.insert(a);
        }
        for a in &food.ingredients {
            all_ingredients_list.push(a);
        }
    }
    let mut allergic_ingredients: HashSet<String> = HashSet::new();
    let mut possible_ingredients_by_allergen: HashMap<String, HashSet<String>> = HashMap::new();
    for a in all_allergens {
        let mut possible_ingredients: HashSet<String> = HashSet::new() ;
        for food in foods.iter().filter(|f| f.allergens.contains(a)) {
            if possible_ingredients.is_empty() {
                for ing in &food.ingredients {
                    possible_ingredients.insert(String::from(ing));
                }
            } else {
                possible_ingredients = possible_ingredients.intersection(&(food.ingredients)).map(|s| s.clone()).collect()
            }
        }
        for ing in &possible_ingredients {
            allergic_ingredients.insert(ing.clone());
        }
        possible_ingredients_by_allergen.insert(a.clone(), possible_ingredients);
    }
    let answer_1 = all_ingredients_list.iter().filter(|ing| !allergic_ingredients.contains(**ing)).count();
    println!("Answer part 1: {}", answer_1);

    let mut allergene_to_ingredient: HashMap<String, String> = HashMap::new();    
    loop {
        let (k_cand, k_val) = possible_ingredients_by_allergen.iter().find(|(_, v)| v.len() == 1).unwrap();
        allergene_to_ingredient.insert(k_cand.clone(), (k_val.iter().next().unwrap()).clone());
        let mapped_ingredients: HashSet<String> = allergene_to_ingredient.values().map(|v| v.clone()).collect();
        possible_ingredients_by_allergen = possible_ingredients_by_allergen.iter().filter(|(k, _)| *k != k_cand).map(
            |(k, v)| (k.clone(), (v.clone()).difference(&mapped_ingredients).map(|s| s.clone()).collect())
        ).collect();
        if possible_ingredients_by_allergen.len() == 0 {
            break
        }
    }
    let mut allergene_to_ingredient: Vec<_> = allergene_to_ingredient.into_iter().collect();
    allergene_to_ingredient.sort();
    for (_, i) in allergene_to_ingredient.iter().take(1) {
        print!("Answer part 2: {}", i)
    }
    for (_, i) in allergene_to_ingredient.iter().skip(1) {
        print!(",{}", i)
    }
    println!();
}
