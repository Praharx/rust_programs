use borsh::{ BorshDeserialize, BorshSerialize };

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct IncrementPageVisits {}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct PageVisits {
    pub page_visits: u32,
    pub bump: u8,
}

impl PageVisits {
    // 8 is for metadata storage and 32 is allocated as account data space out of which only 5 bytes is used currently
    pub const ACCOUNT_SPACE:usize = 8 + 32;

    pub const SEED_PREFIX:&'static str = "page_visits";

    pub fn new(page_visits: u32, bump: u8) -> Self {
        PageVisits {
            page_visits,
            bump
        }
    }

    pub fn increment(&mut self) {
        self.page_visits += 1;
    }
}