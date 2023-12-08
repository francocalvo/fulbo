//! This module implements the general structure and traits of the data sources.
//! The idea is to abstract away how I get the data from the different sources
//! or one API.

use crate::domain;

/// This trait represents the general structure of a data source.
pub trait Datasource {
    /// This function returns a vector of ItemResult structs with the
    /// leagues from the data source.
    fn get_leagues() -> Vec<domain::League>;

    /// This function returns a vector of ItemResult structs with the
    /// teams from the data source.
    /// It takes as an argument the ID of the league.
    fn get_teams(league: &domain::League) -> Vec<domain::Team>;

    /// This function returns a team with information from the data source.
    fn get_team<'a>(team_id: i32) -> domain::Team<'a>;
}
