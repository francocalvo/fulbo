//! This module contains the struct for a league and its implementation.
use super::{team::Team, ItemResult};

/// This struct represents a league.
pub struct League<'a> {
    name: String,
    id: String,
    region: String,
    teams: Vec<&'a Team>,
}

impl<'a> League<'a> {
    /// Constructor for League.
    pub fn new(name: String, id: String, region: String) -> League<'a> {
        League {
            name,
            id,
            region,
            teams: Vec::new(),
        }
    }

    /// Add a team to the League.
    pub fn add_team(&mut self, team: &'a Team) {
        self.teams.push(team);
    }

    /// Getter for teams of League.
    pub fn get_teams(&self) -> &Vec<&'a Team> {
        &self.teams
    }

    /// Getter for region of League.
    pub fn get_region(&self) -> &String {
        &self.region
    }
}

impl<'a> ItemResult for League<'a> {
    /// Getter for name of League.
    fn get_name(&self) -> &String {
        &self.name
    }

    /// Getter for ID of League.
    fn get_id(&self) -> &String {
        &self.id
    }
}
