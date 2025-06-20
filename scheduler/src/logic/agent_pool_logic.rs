use crate::errors::Error;

use std::cmp::Ordering;

/// Top secret algorithm used to mathematically compute the freeness score of an Agent. Do not leak!
pub(crate) fn compute_score(cpu_avail: u32, memory_avail: u64) -> u64 {
    (0.5 * cpu_avail as f64 + 0.5 * memory_avail as f64) as u64
}

/// A struct representing an Agent in the Pool.
/// The Agent has an ID and a score.
#[derive(Eq, PartialEq, Debug)]
pub(crate) struct Agent {
    id: u32,
    hostname: Hostname,
    score: u64,
}

impl Agent {
    /// Constructor
    pub(crate) fn new(id: u32, hostname: Hostname, score: u64) -> Self {
        Self {
            id: id,
            hostname: hostname,
            score: score,
        }
    }
    /// ID getter
    pub(crate) fn get_id(&self) -> u32 {
        self.id
    }

    /// Score getter
    pub(crate) fn _get_score(&self) -> u64 {
        self.score
    }

    /// ID setter
    pub(crate) fn _set_id(&mut self, id: u32) {
        self.id = id;
    }

    /// Score setter
    pub(crate) fn set_score(&mut self, score: u64) {
        self.score = score;
    }

    /// Returns the Agent's IP address in the format "host:port"
    /// If the hostname is empty, returns an error.
    pub(crate) fn get_ip_address(&self) -> Result<String, Error> {
        if self.hostname.get_host().is_empty() {
            return Err(Error::InvalidAgentHostError(String::from("Agent hostname is empty. Cannot resolve IP address.")));
        }
        Ok(format!("{}:{}", self.hostname.get_host(), self.hostname.get_port()))
    }
}

/// Implement `Ord` to order/compare by `score`.
impl Ord for Agent {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.cmp(&other.score)
    }
}

/// Implement `PartialOrd` to order/compare by `score`.
impl PartialOrd for Agent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// A struct representing the IP address of an Agent.
#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) struct Hostname {
    host: String,
    port: u32,
}

impl Hostname {
    /// Constructor
    pub(crate) fn new(host: String, port: u32) -> Self {
        Self { host, port }
    }

    /// Host getter
    pub(crate) fn get_host(&self) -> &str {
        &self.host
    }

    /// Port getter
    pub(crate) fn get_port(&self) -> u32 {
        self.port
    }

    /// Host setter
    pub(crate) fn _set_host(&mut self, host: String) {
        self.host = host;
    }

    /// Port setter
    pub(crate) fn _set_port(&mut self, port: u32) {
        self.port = port;
    }
}

/// AgentPool is a collection of Agents stored in a vector.
/// The vector is sorted whenever necessary to maintain order.
pub struct AgentPool {
    agents: Vec<Agent>,
}

impl AgentPool {
    /// Constructor
    pub fn new() -> Self {
        Self {
            agents: Vec::new(),
        }
    }

    /// Insert an Agent into the Agent Pool and sort the Pool by score.
    pub(crate) fn push(&mut self, item: Agent) {
        self.agents.push(item);
        self.sort();  // Keep the vector sorted after each insertion of a new Agent
    }

    /// Remove and return the Agent with the lowest score (that is, the first Agent), or return None if the Pool is empty.
    pub(crate) fn _pop(&mut self) -> Option<Agent> {
        if self.agents.is_empty() {
            None
        } else {
            Some(self.agents.remove(0))
        }
    }

    /// Peek at the Agent with the lowest score without removing it, or return None if the Pool is empty.
    pub(crate) fn peek(&self) -> Option<&Agent> {
        if self.agents.is_empty() {
            None
        } else {
            self.agents.first()  // The first element has the lowest score
        }
    }

    /// Return the number of Agents in the Pool
    pub(crate) fn _len(&self) -> usize {
        self.agents.len()
    }

    /// Check if the Agent Pool is empty
    pub(crate) fn _is_empty(&self) -> bool {
        self.agents.is_empty()
    }

    /// Sort the Agents by score (ascending)
    /// Uses Rust's built-in sorting algorithm to sort the Agents by score. It is a Timsort.
    pub(crate) fn sort(&mut self) {
        self.agents.sort_by_key(|agent| agent.score);
    }

    /// Return a *mutable* reference to the Agent of the given ID, or None if the Agent is not found.
    pub(crate) fn find_agent_mut(&mut self, id: u32) -> Option<&mut Agent> {
        self.agents.iter_mut().find(|agent| agent.id == id)
    }
    

    /// Check if the Agent with the given ID is out of order compared to its neighbors
    /// ALWAYS make sure the Agent exists before using this method! Or else it will panic, as index will be None.
    pub(crate) fn check_agent_neighbors(&self, id: u32) -> bool {
        // ALWAYS make sure the Agent exists before using this method! Or else it will panic, as index will be None.
        let index = self.agents.iter().position(|agent| agent.id == id).unwrap();
        if index > 0 && self.agents[index].score < self.agents[index - 1].score {
            return true;  // Agent is out of order (lower score than previous)
        }
        if index < self.agents.len() - 1 && self.agents[index].score > self.agents[index + 1].score {
            return true;  // Agent is out of order (higher score than next)
        }
        return false;  // Agent is in correct order
    }

    /// Generate a unique ID by finding the maximum existing ID and incrementing it by 1. This ensures that the new ID is *always* unique among the Agent Pool.
    pub(crate) fn generate_unique_id(&self) -> u32 {
        self.agents.iter().map(|agent| agent.id).max().unwrap_or(0) + 1  // unwrap_or(0) is used to handle the case when the Agent Pool is empty
    }
}
