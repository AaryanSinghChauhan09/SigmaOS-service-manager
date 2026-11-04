//! Service manager engine coordinating service lifecycle and dependencies

use crate::error::{Result, ServiceError};
use crate::unit::{ServiceUnit, UnitState};
use log::info;
use std::collections::HashMap;

#[derive(Default)]
pub struct ServiceManager {
    units: HashMap<String, ServiceUnit>,
}

impl ServiceManager {
    pub fn new() -> Self {
        Self {
            units: HashMap::new(),
        }
    }

    pub fn register(&mut self, unit: ServiceUnit) {
        self.units.insert(unit.name.clone(), unit);
    }

    pub fn get(&self, name: &str) -> Option<&ServiceUnit> {
        self.units.get(name)
    }

    pub fn start(&mut self, name: &str) -> Result<()> {
        let (requires, current_state) = {
            let unit = self
                .units
                .get(name)
                .ok_or_else(|| ServiceError::NotFound(name.to_string()))?;
            (unit.requires.clone(), unit.state.clone())
        };

        if current_state == UnitState::Running {
            return Err(ServiceError::AlreadyRunning(name.to_string()));
        }

        // Start prerequisites first
        for req in requires {
            let req_state = self
                .units
                .get(&req)
                .map(|u| u.state.clone())
                .unwrap_or(UnitState::Stopped);

            if req_state != UnitState::Running {
                self.start(&req).map_err(|_| ServiceError::DependencyFailed(req))?;
            }
        }

        let unit = self.units.get_mut(name).unwrap();
        info!("Starting service '{}' -> {}", unit.name, unit.exec_start);
        unit.state = UnitState::Running;
        Ok(())
    }

    pub fn stop(&mut self, name: &str) -> Result<()> {
        let current_state = {
            let unit = self
                .units
                .get(name)
                .ok_or_else(|| ServiceError::NotFound(name.to_string()))?;
            unit.state.clone()
        };

        if current_state == UnitState::Stopped {
            return Err(ServiceError::NotRunning(name.to_string()));
        }

        let unit = self.units.get_mut(name).unwrap();
        info!("Stopping service '{}'", unit.name);
        unit.state = UnitState::Stopped;
        Ok(())
    }

    pub fn restart(&mut self, name: &str) -> Result<()> {
        let _ = self.stop(name);
        self.start(name)
    }

    pub fn count(&self) -> usize {
        self.units.len()
    }

    pub fn running_count(&self) -> usize {
        self.units
            .values()
            .filter(|u| u.state == UnitState::Running)
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_manager_dependency_start_aaa() {
        // Arrange
        let mut sm = ServiceManager::new();
        let db = ServiceUnit::new("db.service", "Database", "/usr/bin/db");
        let api = ServiceUnit::new("api.service", "API Server", "/usr/bin/api")
            .with_dependency("db.service");

        sm.register(db);
        sm.register(api);

        // Act
        sm.start("api.service").unwrap();

        // Assert
        assert_eq!(sm.get("db.service").unwrap().state, UnitState::Running);
        assert_eq!(sm.get("api.service").unwrap().state, UnitState::Running);
    }
}
