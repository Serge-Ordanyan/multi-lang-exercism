pub struct Allergies {
    score: u32,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl Allergen {
    // Return the bitmask associated with each allergen
    fn to_mask(self) -> u32 {
        match self {
            Allergen::Eggs => 1,
            Allergen::Peanuts => 2,
            Allergen::Shellfish => 4,
            Allergen::Strawberries => 8,
            Allergen::Tomatoes => 16,
            Allergen::Chocolate => 32,
            Allergen::Pollen => 64,
            Allergen::Cats => 128,
        }
    }

    // List of all allergens in order of valuation
    fn all_allergens() -> &'static [Allergen] {
        use Allergen::*;
        &[
            Eggs,
            Peanuts,
            Shellfish,
            Strawberries,
            Tomatoes,
            Chocolate,
            Pollen,
            Cats,
        ]
    }
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies { score }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        (self.score & allergen.to_mask()) != 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        Allergen::all_allergens()
            .iter()
            .filter(|&allergen| self.is_allergic_to(allergen))
            .cloned()
            .collect()
    }
}
