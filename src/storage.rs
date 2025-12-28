use crate::card::Card;
use gloo_storage::{LocalStorage, Storage};

const STORAGE_KEY: &str = "rolodex_cards";

/// Storage manager for rolodex cards using browser local storage
pub struct CardStorage;

impl CardStorage {
    /// Load all cards from local storage
    pub fn load_cards() -> Vec<Card> {
        LocalStorage::get(STORAGE_KEY).unwrap_or_default()
    }

    /// Save all cards to local storage
    pub fn save_cards(cards: &[Card]) -> Result<(), gloo_storage::errors::StorageError> {
        LocalStorage::set(STORAGE_KEY, cards)
    }

    /// Add a new card and save
    pub fn add_card(card: Card) -> Result<Vec<Card>, gloo_storage::errors::StorageError> {
        let mut cards = Self::load_cards();
        cards.push(card);
        Self::save_cards(&cards)?;
        Ok(cards)
    }

    /// Update an existing card by ID
    pub fn update_card(
        updated_card: Card,
    ) -> Result<Vec<Card>, gloo_storage::errors::StorageError> {
        let mut cards = Self::load_cards();
        if let Some(card) = cards.iter_mut().find(|c| c.id == updated_card.id) {
            *card = updated_card;
        }
        Self::save_cards(&cards)?;
        Ok(cards)
    }

    /// Delete a card by ID
    pub fn delete_card(card_id: &str) -> Result<Vec<Card>, gloo_storage::errors::StorageError> {
        let mut cards = Self::load_cards();
        cards.retain(|c| c.id != card_id);
        Self::save_cards(&cards)?;
        Ok(cards)
    }

    /// Clear all cards from storage
    pub fn clear_all() -> Result<(), gloo_storage::errors::StorageError> {
        LocalStorage::delete(STORAGE_KEY);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    fn setup_test() {
        // Clear storage before each test
        let _ = CardStorage::clear_all();
    }

    #[wasm_bindgen_test]
    fn test_load_empty_cards() {
        setup_test();
        let cards = CardStorage::load_cards();
        assert!(cards.is_empty());
    }

    #[wasm_bindgen_test]
    fn test_add_card() {
        setup_test();
        let card = Card::new(
            "Test User".to_string(),
            "test@example.com".to_string(),
            "555-0000".to_string(),
            "Test Co".to_string(),
            "Notes".to_string(),
        );
        let result = CardStorage::add_card(card.clone());
        assert!(result.is_ok());
        let cards = result.unwrap();
        assert_eq!(cards.len(), 1);
        assert_eq!(cards[0].name, "Test User");
    }

    #[wasm_bindgen_test]
    fn test_update_card() {
        setup_test();
        let card = Card::new(
            "Original".to_string(),
            "orig@example.com".to_string(),
            "555-0000".to_string(),
            "Co".to_string(),
            "".to_string(),
        );
        let _ = CardStorage::add_card(card.clone());

        let mut updated = card.clone();
        updated.name = "Updated".to_string();
        let result = CardStorage::update_card(updated);
        assert!(result.is_ok());
        let cards = result.unwrap();
        assert_eq!(cards[0].name, "Updated");
    }

    #[wasm_bindgen_test]
    fn test_delete_card() {
        setup_test();
        let card = Card::new(
            "To Delete".to_string(),
            "del@example.com".to_string(),
            "555-0000".to_string(),
            "Co".to_string(),
            "".to_string(),
        );
        let cards = CardStorage::add_card(card.clone()).unwrap();
        assert_eq!(cards.len(), 1);

        let result = CardStorage::delete_card(&card.id);
        assert!(result.is_ok());
        let cards = result.unwrap();
        assert!(cards.is_empty());
    }
}
