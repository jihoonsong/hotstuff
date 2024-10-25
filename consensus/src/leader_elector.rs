use std::net::SocketAddr;

use crate::Round;

pub trait LeaderElector {
    // TODO: Use cryptographic public key instead of SocketAddr as an identifier.
    fn leader(&self, round: Round) -> SocketAddr;
}

pub struct RoundRobinLeaderElector {
    committee: Vec<SocketAddr>,
}

impl RoundRobinLeaderElector {
    pub fn new(committee: Vec<SocketAddr>) -> Self {
        Self { committee }
    }
}

impl LeaderElector for RoundRobinLeaderElector {
    fn leader(&self, round: Round) -> SocketAddr {
        self.committee[round as usize % self.committee.len()]
    }
}
