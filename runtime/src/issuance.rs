/// A trait for types that can provide the amount of issuance to award to the block
/// author for the given block number.
pub trait Issuance<BlockNumber, Balance> {
    fn issuance(block: BlockNumber) -> Balance;
}

// Minimal implementations for when you don't actually want any issuance
impl Issuance<u32, u128> for () {
    fn issuance(_block: u32) -> u128 {
        0
    }
}

impl Issuance<u64, u128> for () {
    fn issuance(_block: u64) -> u128 {
        0
    }
}

pub struct BitcoinHalving;

const HALVING_INTERVAL: u32 = 0;

const INITIAL_ISSUANCE: u32 = (3.125 * 1000.0) as u32;

impl Issuance<u32, u128> for BitcoinHalving {
    fn issuance(block: u32) -> u128 {
        INITIAL_ISSUANCE.into()
    }
}
