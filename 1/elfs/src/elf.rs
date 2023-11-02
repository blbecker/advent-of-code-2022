use std::clone;

#[derive(Default, Copy, Clone)]
pub(crate) struct FoodItem {
    calories: u64,
}

impl FoodItem {
    pub fn new(calories: u64) -> Self {
        Self { calories: calories }
    }
}

#[derive(Default)]
pub(crate) struct Elf {
    food: Vec<FoodItem>,
}

impl Elf {
    pub fn total_calories(&self) -> u64 {
        let mut total_calories = 0;
        self.food
            .clone()
            .into_iter()
            .enumerate()
            .into_iter()
            .for_each(|(food_index, food)| {
                total_calories += food.calories;
            });
        return total_calories;
    }

    pub fn add_food(&mut self, item: FoodItem) {
        let mut new_food = self.food.clone();
        new_food.push(item);
        self.set_food(new_food);
        // self.food.push(item);
    }

    fn set_food(&mut self, new_food: Vec<FoodItem>) {
        self.food = new_food;
    }

    pub fn new() -> Self {
        Self { food: Vec::new() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total_calories() {
        let mut this_elf = Elf { food: Vec::new() };
        assert_eq!(this_elf.total_calories(), 0);
        this_elf.add_food(FoodItem { calories: 10 });
        assert_eq!(this_elf.total_calories(), 10);
    }
}
