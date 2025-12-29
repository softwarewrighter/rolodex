use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents a single card in the rolodex
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Card {
    pub id: String,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub company: String,
    pub notes: String,
}

impl Card {
    /// Create a new card with a generated UUID
    pub fn new(name: String, email: String, phone: String, company: String, notes: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            email,
            phone,
            company,
            notes,
        }
    }

    /// Create an empty card template
    pub fn empty() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: String::new(),
            email: String::new(),
            phone: String::new(),
            company: String::new(),
            notes: String::new(),
        }
    }

    /// Check if the card name starts with the filter query (case-insensitive)
    pub fn matches_search(&self, query: &str) -> bool {
        if query.is_empty() {
            return true;
        }
        let query = query.to_lowercase();
        self.name.to_lowercase().starts_with(&query)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_card_new() {
        let card = Card::new(
            "John Doe".to_string(),
            "john@example.com".to_string(),
            "555-1234".to_string(),
            "Acme Corp".to_string(),
            "Test notes".to_string(),
        );
        assert_eq!(card.name, "John Doe");
        assert_eq!(card.email, "john@example.com");
        assert!(!card.id.is_empty());
    }

    #[wasm_bindgen_test]
    fn test_card_empty() {
        let card = Card::empty();
        assert!(card.name.is_empty());
        assert!(card.email.is_empty());
        assert!(!card.id.is_empty());
    }

    #[wasm_bindgen_test]
    fn test_card_matches_search_starts_with() {
        let card = Card::new(
            "John Doe".to_string(),
            "john@example.com".to_string(),
            "555-1234".to_string(),
            "Acme Corp".to_string(),
            "Notes".to_string(),
        );
        // Filter matches names starting with query
        assert!(card.matches_search("john"));
        assert!(card.matches_search("JOHN"));
        assert!(card.matches_search("jo"));
        assert!(card.matches_search("j"));
        // Does not match middle of name
        assert!(!card.matches_search("doe"));
        assert!(!card.matches_search("ohn"));
    }

    #[wasm_bindgen_test]
    fn test_card_filter_name_only() {
        let card = Card::new(
            "John Doe".to_string(),
            "john@example.com".to_string(),
            "555-1234".to_string(),
            "Acme Corp".to_string(),
            "Notes".to_string(),
        );
        // Filter only matches name start, not other fields
        assert!(!card.matches_search("example.com"));
        assert!(!card.matches_search("acme"));
        assert!(!card.matches_search("555"));
    }

    #[wasm_bindgen_test]
    fn test_card_matches_search_empty_query() {
        let card = Card::new(
            "John Doe".to_string(),
            "john@example.com".to_string(),
            "555-1234".to_string(),
            "Acme Corp".to_string(),
            "Notes".to_string(),
        );
        assert!(card.matches_search(""));
    }

    #[wasm_bindgen_test]
    fn test_card_matches_search_no_match() {
        let card = Card::new(
            "John Doe".to_string(),
            "john@example.com".to_string(),
            "555-1234".to_string(),
            "Acme Corp".to_string(),
            "Notes".to_string(),
        );
        assert!(!card.matches_search("xyz123"));
    }
}
