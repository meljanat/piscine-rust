pub struct Food {
    pub name: String,
    pub calories: (String, String),
    pub fats: f64,
    pub carbs: f64,
    pub proteins: f64,
    pub nbr_of_portions: f64,
}

pub fn calculate_macros(foods: &[Food]) -> json::JsonValue {
    let mut total_calories = 0.0;
    let mut total_fats = 0.0;
    let mut total_carbs = 0.0;
    let mut total_proteins = 0.0;

    for food in foods {
        let calories_str = food.calories.1.replace("kcal", "");
        total_calories += calories_str.parse::<f64>().unwrap_or(0.0) * food.nbr_of_portions;
        total_fats += food.fats * food.nbr_of_portions;
        total_carbs += food.carbs * food.nbr_of_portions;
        total_proteins += food.proteins * food.nbr_of_portions;
    }

    json::object! {
        "cals" => format!("{:.2}", total_calories).parse::<f64>().ok(),
        "carbs" => format!("{:.2}", total_carbs).parse::<f64>().ok(),
        "proteins" => format!("{:.2}", total_proteins).parse::<f64>().ok(),
        "fats" => format!("{:.2}", total_fats).parse::<f64>().ok(),
    }
}
