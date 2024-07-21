use ureq::{Agent, AgentBuilder};

use super::Auth;

pub struct NewsdataIO {
    pub auth: Auth,
    pub(crate) agent: Agent,
}

impl Clone for NewsdataIO {
    fn clone(&self) -> Self {
        Self {
            auth: self.auth.clone(),
            agent: self.agent.clone(),
        }
    }
}

impl NewsdataIO {
    pub fn new(auth: Auth) -> Self {
        Self {
            auth,
            agent: AgentBuilder::new().build(),
        }
    }
}
