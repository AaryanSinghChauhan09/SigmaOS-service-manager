//! Service unit specification and state definitions

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UnitState {
    Stopped,
    Starting,
    Running,
    Stopping,
    Failed(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ServiceType {
    Simple,
    Oneshot,
    Forking,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceUnit {
    pub name: String,
    pub description: String,
    pub service_type: ServiceType,
    pub exec_start: String,
    pub requires: Vec<String>,
    pub after: Vec<String>,
    pub state: UnitState,
}

impl ServiceUnit {
    pub fn new(name: &str, description: &str, exec_start: &str) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            service_type: ServiceType::Simple,
            exec_start: exec_start.to_string(),
            requires: Vec::new(),
            after: Vec::new(),
            state: UnitState::Stopped,
        }
    }

    pub fn with_dependency(mut self, required_service: &str) -> Self {
        self.requires.push(required_service.to_string());
        self.after.push(required_service.to_string());
        self
    }
}
