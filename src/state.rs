use super::pool::Pool;
use super::dex::Dex;
use super::checkpoint::{construct_checkpoint, deconstruct_checkpoint};
use ethers::types::BlockNumber::Number;


pub struct LocalState {
    checkpoint: String,
    block_number: u64,
    dexes: Vec<Dex>,
    pools: Vec<Pool>,
}

impl LocalState {
    pub fn new(checkpoint: String) -> Self {
        let (dexes, pools, block_number) = deconstruct_checkpoint(&checkpoint);
        LocalState {
            checkpoint,
            block_number: match block_number {
                Number(v) => v.as_u64(),
                _ => panic!("block_number in checkpoint is not u64"),
            },
            dexes,
            pools,
        }
    }

    pub fn store_checkpoint(&self) {
        construct_checkpoint(self.dexes.clone(), &self.pools, self.block_number, &self.checkpoint);
    }

    // pub fn update_local_state_from_logs();


    // pub fn search_route();
}