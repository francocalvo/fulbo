//! This module creates the structs that represent the domain of the
//! application. The idea is to have a struct for each entity of the domain.

/// This struct represents a result from a data source.
/// For example, if I want to get the leagues from a data source, I will
/// get a vector of ItemResult structs.
/// ## Example
/// todo
pub trait ItemResult {
    /// Getter for name of ItemResult.
    fn get_name(&self) -> &String;

    /// Getter for ID of ItemResult.
    fn get_id(&self) -> &String;
}

pub mod league;
pub mod team;

pub use league::League;
pub use team::Team;
