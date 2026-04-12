use async_trait::async_trait;
use std::collections::HashSet;

#[derive(Default)]
pub struct HashsetBannedTokenStore {
    tokens: HashSet<String>,
}

#[async_trait]
pub trait BannedTokenStore {
    fn add_token(&mut self, token: String) -> ();
    fn is_token_banned(&self, token: &String) -> bool;
}

impl BannedTokenStore for HashsetBannedTokenStore {
    fn add_token(&mut self, token: String) -> () {
        self.tokens.insert(token);
    }

    fn is_token_banned(&self, token: &String) -> bool {
        self.tokens.contains(token)
    }
}
