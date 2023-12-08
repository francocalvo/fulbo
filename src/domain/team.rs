//! This module contains the struct for a team and its implementation.
use super::{ItemResult, League};

/// This struct represents a league.
pub struct Team<'a> {
    name: String,
    id: String,
    league: Option<&'a League>,
}

impl<'a> Team<'a> {
    /// Constructor for Team.
    pub fn new(
        name: String,
        id: String,
        league: Option<&'a League>,
    ) -> Team<'a> {
        Team {
            name,
            id,
            league,
        }
    }

    /// Getter for league of Team.
    pub fn get_league(&self) -> &Option<&'a League> {
        &self.league
    }
}

impl<'a> ItemResult for Team<'a> {
    /// Getter for name of Team.
    fn get_name(&self) -> &String {
        &self.name
    }

    /// Getter for ID of Team.
    fn get_id(&self) -> &String {
        &self.id
    }
}
