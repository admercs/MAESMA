//! Agent registry — dynamic registration and lookup of agent instances.

use std::collections::HashMap;
use std::sync::Arc;

use crate::traits::{Agent, AgentId, AgentRole};

/// Registry holding all active agent instances.
pub struct AgentRegistry {
    agents: HashMap<AgentId, Arc<dyn Agent>>,
}

impl AgentRegistry {
    pub fn new() -> Self {
        Self {
            agents: HashMap::new(),
        }
    }

    /// Register an agent.
    pub fn register(&mut self, agent: Arc<dyn Agent>) {
        self.agents.insert(agent.id().clone(), agent);
    }

    /// Get an agent by ID.
    pub fn get(&self, id: &AgentId) -> Option<&Arc<dyn Agent>> {
        self.agents.get(id)
    }

    /// Get all agents with a given role.
    pub fn by_role(&self, role: AgentRole) -> Vec<&Arc<dyn Agent>> {
        self.agents
            .values()
            .filter(|a| a.role() == role)
            .collect()
    }

    /// List all registered agent IDs.
    pub fn list(&self) -> Vec<&AgentId> {
        self.agents.keys().collect()
    }

    /// Number of registered agents.
    pub fn len(&self) -> usize {
        self.agents.len()
    }

    pub fn is_empty(&self) -> bool {
        self.agents.is_empty()
    }
}

impl Default for AgentRegistry {
    fn default() -> Self {
        Self::new()
    }
}
