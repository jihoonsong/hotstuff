use hotstuff_crypto::{PublicKey, ValidatorIndex};
use std::collections::HashMap;

use crate::{LeaderElector, Round};

pub struct Committee<L>
where
    L: LeaderElector,
{
    members: HashMap<PublicKey, ValidatorIndex>,
    leader_elector: L,
}

impl<L> Committee<L>
where
    L: LeaderElector,
{
    pub fn new(members: HashMap<PublicKey, ValidatorIndex>, leader_elector: L) -> Self {
        Self {
            members,
            leader_elector,
        }
    }

    pub fn index(&self, public_key: &PublicKey) -> Option<ValidatorIndex> {
        self.members.get(public_key).copied()
    }

    pub fn leader(&self, round: Round) -> PublicKey {
        self.leader_elector.leader(round)
    }
}
