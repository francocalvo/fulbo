//! This module contains the struct for a team and its implementation.
use super::ItemResult;

/// This struct represents a team.
pub struct Team {
    name: String,
    id: String,
}

impl Team {
    /// Constructor for Team.
    pub fn new(name: String, id: String) -> Team {
        Team {
            name,
            id,
        }
    }
}

impl ItemResult for Team {
    /// Getter for name of Team.
    fn get_name(&self) -> &String {
        &self.name
    }

    /// Getter for ID of Team.
    fn get_id(&self) -> &String {
        &self.id
    }
}
