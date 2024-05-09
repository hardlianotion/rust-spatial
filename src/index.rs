use h3o::error::InvalidCellIndex;
use h3o::CellIndex;

#[derive(Clone, Copy, Debug)]
pub struct Index {
    pub index: u64,
}

// bitmasks
static ALL_ONES: u64 = u64::MAX;
static UNSET: u64 = 0;
static ZERO: u64 = UNSET;
static RESOLUTION_MASK: u64 = Index::binary_ones(4u8).index << RESOLUTION_SHIFT;
//static LOCATION_MASK: u64 = Index::binary_ones(52).index;
static ZERO_RESOLUTION_ONLY: u64 = ALL_ONES & !RESOLUTION_MASK;
static CELL_MODE_MASK: u64 = 1 << CELL_MODE_SHIFT;

// shifts
static BASE_CELL_SHIFT: u8 = 45;
static CELL_MODE_SHIFT: u8 = 59;
static RESOLUTION_SHIFT: u8 = 52;
//static MIN_RESOLUTION: u8 = 0;
static MAX_RESOLUTION: u8 = 15;

static CHILD_CELL_COUNT: u8 = 7;

use rand::distributions::Uniform;
use rand::{thread_rng, Rng};

impl Index {
    pub fn to_cell_index(self) -> Result<CellIndex, InvalidCellIndex> {
        CellIndex::try_from(self.index)
    }

    pub fn is_valid(self) -> bool {
        self.to_cell_index().is_ok()
    }

    fn generate_valid_base(from_resolution: u8, to_resolution: u8) -> Index {
        let mut rng = thread_rng();
        let child_gen = Uniform::new(ZERO, u64::from(CHILD_CELL_COUNT));

        Index {
            index: (from_resolution..to_resolution)
                .fold(UNSET, |agg, rhs| agg | (rng.sample(child_gen) << (3 * rhs)))
                | Index::bit_cell_mask(to_resolution, 3).index,
        }
    }

    pub fn unsafe_random(base_cell: u32, from_resolution: u8, to_resolution: u8) -> Index {
        Index {
            index: CELL_MODE_MASK
                | u64::from(to_resolution) << RESOLUTION_SHIFT
                | u64::from(base_cell) << BASE_CELL_SHIFT
                | Index::generate_valid_base(from_resolution, to_resolution).index,
        }
    }

    pub const fn binary_ones(n: u8) -> Index {
        Index {
            index: (1 << n) - 1,
        }
    }

    pub fn bit_cell_mask(resolution: u8, num_bits: u8) -> Index {
        Index {
            index: Index::binary_ones((MAX_RESOLUTION - resolution) * num_bits).index,
        }
    }

    pub fn truncate_to_resolution(self, resolution: u8) -> Index {
        Index {
            index: (self.index & ZERO_RESOLUTION_ONLY)
                | Index::bit_cell_mask(resolution, 3).index
                | (u64::from(resolution) << RESOLUTION_SHIFT),
        }
    }

    pub fn resolution(self) -> u8 {
        ((self.index & RESOLUTION_MASK) >> RESOLUTION_SHIFT) as u8
    }

    pub fn from_u8_indices(indices: &[u8; 15]) -> u64 {
        let mut u64idx: u64 = 0;
        for i in 0..15 {
            u64idx |= u64::from(indices[i]) << (3 * i);
        }
        u64idx
    }

    pub fn to_u8_indices(u64idx: u64) -> [u8; 15] {
        let mut indices = [0; 15];
        for i in 0..15 {
            indices[i] = ((u64idx & (7 << (3 * i))) >> (3 * i)).try_into().unwrap();
        }
        indices
    }
}

#[cfg(test)]
mod tests {
    use crate::index::{Index, ZERO};

    #[test]
    fn test_root_node_is_not_null() {
        let index = Index { index: 0 };
        assert!(index.index == 0);
    }

    #[test]
    fn generated_h3_should_be_valid() {
        let random_h3 = Index::unsafe_random(28, 0, 13);

        assert!(random_h3.is_valid());
    }

    #[test]
    fn level_15_mask_should_be_0() {
        assert!(Index::bit_cell_mask(15, 3).index == ZERO);
    }

    #[test]
    fn level_14_mask_should_be_7() {
        assert!(Index::bit_cell_mask(14, 3).index == 7);
    }
    #[test]
    fn level_13_mask_should_be_3f() {
        assert!(Index::bit_cell_mask(13, 3).index == 0x3F);
    }

    #[test]
    fn test_return_truncated_h3_local_index_0() {
        let random_h3 = Index::unsafe_random(28, 0, 13);
        let truncated_h3 = random_h3.truncate_to_resolution(5);

        //println!("{}", truncated_h3.index);
        assert!(truncated_h3.is_valid());
        assert!(truncated_h3.resolution() == 5);
    }

    #[test]
    fn test_roundtrip_indices() {
        let u8idxs:[u8;15] = [0, 1, 2, 0, 1, 2, 0, 1, 2, 0, 1, 2, 0, 1, 2];
        let u64idx = Index::from_u8_indices(&u8idxs);
        let roundtrip_indices = Index::to_u8_indices(u64idx);

        assert!(u8idxs == roundtrip_indices);
    }
}
