use crate::card::Card;
use fake::faker::company::en::*;
use fake::faker::internet::en::*;
use fake::faker::name::en::*;
use fake::faker::phone_number::en::*;
use fake::Fake;
use rand::rngs::SmallRng;
use rand::SeedableRng;

/// Generate a vector of fake cards for testing
pub fn generate_fake_cards(count: usize) -> Vec<Card> {
    let mut rng = SmallRng::from_entropy();

    (0..count)
        .map(|_| {
            let name: String = Name().fake_with_rng(&mut rng);
            let email: String = SafeEmail().fake_with_rng(&mut rng);
            let phone: String = PhoneNumber().fake_with_rng(&mut rng);
            let company: String = CompanyName().fake_with_rng(&mut rng);
            let notes: String = format!(
                "{}. {}",
                CatchPhrase().fake_with_rng::<String, _>(&mut rng),
                Bs().fake_with_rng::<String, _>(&mut rng)
            );

            Card::new(name, email, phone, company, notes)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_generate_fake_cards() {
        let cards = generate_fake_cards(10);
        assert_eq!(cards.len(), 10);

        for card in &cards {
            assert!(!card.name.is_empty());
            assert!(!card.email.is_empty());
            assert!(card.email.contains('@'));
        }
    }

    #[wasm_bindgen_test]
    fn test_generate_100_cards() {
        let cards = generate_fake_cards(100);
        assert_eq!(cards.len(), 100);

        // Verify uniqueness of IDs
        let mut ids: Vec<_> = cards.iter().map(|c| c.id.clone()).collect();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), 100);
    }
}
