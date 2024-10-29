use hotstuff_crypto::PublicKey;

use crate::Round;

pub trait LeaderElector {
    fn leader(&self, round: Round) -> PublicKey;
}

pub struct RoundRobinLeaderElector {
    committee: Vec<PublicKey>,
}

impl RoundRobinLeaderElector {
    pub fn new(committee: Vec<PublicKey>) -> Self {
        Self { committee }
    }
}

impl LeaderElector for RoundRobinLeaderElector {
    fn leader(&self, round: Round) -> PublicKey {
        self.committee[round as usize % self.committee.len()].clone()
    }
}
