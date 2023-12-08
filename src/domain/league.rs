//! This module contains the struct for a league and its implementation.
use super::ItemResult;

/// This struct represents a league.
pub struct League {
    name: String,
    id: String,
    region: String,
}

impl League {
    /// Constructor for League.
    pub fn new(name: String, id: String, region: String) -> League {
        League {
            name,
            id,
            region,
        }
    }

    /// Getter for region of League.
    pub fn get_region(&self) -> &String {
        &self.region
    }
}

impl ItemResult for League {
    /// Getter for name of League.
    fn get_name(&self) -> &String {
        &self.name
    }

    /// Getter for ID of League.
    fn get_id(&self) -> &String {
        &self.id
    }
}
