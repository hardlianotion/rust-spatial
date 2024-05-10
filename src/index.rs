use h3o::error::InvalidCellIndex;
use h3o::CellIndex;

#[derive(Clone, Copy, Debug)]
pub struct Index {
    pub index: u64,
}

pub struct IndexStructure {
    pub base: u8,
    pub address: Vec<u8>,
}

// bitmasks
static ALL_ONES: u64 = u64::MAX;
static UNSET: u64 = 0;
static ZERO: u64 = UNSET;
static BASE_CELL_MASK: u64 = Index::binary_ones(CHILD_CELL_COUNT).index << BASE_CELL_SHIFT;
static RESOLUTION_MASK: u64 = Index::binary_ones(4u8).index << RESOLUTION_SHIFT;
static LOCATION_MASK: u64 = Index::binary_ones(52).index;
static ZERO_RESOLUTION_ONLY: u64 = ALL_ONES & !RESOLUTION_MASK;
static CELL_MODE_MASK: u64 = 1 << CELL_MODE_SHIFT;
//static IN_BASE_MASK: u64 = Index::binary_ones(45).index;

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
    pub fn unsafe_index(value: u64) -> Index {
        Index { index: value }
    }

    pub fn to_cell_index(self) -> Result<CellIndex, InvalidCellIndex> {
        CellIndex::try_from(self.index)
    }

    pub fn is_valid(self) -> bool {
        self.to_cell_index().is_ok()
    }

    pub fn base_hex(self) -> u8 {
        ((self.index & BASE_CELL_MASK) >> BASE_CELL_SHIFT) as u8
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

    pub fn local_index(self, i: u8) -> u8 {
        ((self.index & Index::local_index_mask(i)) >> ((MAX_RESOLUTION - i) * 3)) as u8
    }

    pub fn contains(self, location: Index) -> bool {
        (self.index & LOCATION_MASK)
            == ((location.index & LOCATION_MASK) | Index::bit_cell_mask(self.resolution(), 3).index)
            && self.base_hex() == location.base_hex()
    }

    pub fn local_index_mask(resolution: u8) -> u64 {
        Index::unsafe_index((CHILD_CELL_COUNT as u64) << (3 * (MAX_RESOLUTION - resolution))).index
    }

    pub fn global_index_part(local_index: u8, resolution: u8) -> u64 {
        Index::unsafe_index(Index::local_index_mask(MAX_RESOLUTION) & (local_index as u64)).index
            << (3 * (MAX_RESOLUTION - resolution))
    }

    pub fn to_index_structure(self) -> IndexStructure {
        let base_data: u8 = (self.index | BASE_CELL_MASK >> BASE_CELL_SHIFT) as u8;
        let address_array: Vec<u8> = (0u8..15u8).map(|i| self.local_index(i)).collect();

        IndexStructure {
            base: base_data,
            address: address_array,
        }
    }

    pub fn from_index_data(resolution: u8, base_cell: u8, address: u64) -> Index {
        Index {
            index: CELL_MODE_MASK
                | (resolution as u64) << RESOLUTION_SHIFT
                | ((base_cell as u64) << BASE_CELL_SHIFT)
                | address,
        }
    }

    pub fn from_address_map(
        resolution: u8,
        base_cell: u8,
        resolution_to_local: &[(u8, u8)],
    ) -> Index {
        let index = resolution_to_local
            .iter()
            .fold(UNSET, |agg, (res, local_index)| {
                agg | Index::global_index_part(*local_index, *res)
            });

        Index::from_index_data(resolution, base_cell, index)
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
    fn index_8e754e64992d6df_should_contain_index_8f754e64992d6d8() {
        let smaller_cell: u64 = 0x8f754e64992d6d8;
        let larger_cell: u64 = 0x8e754e64992d6df;
        assert!(Index::unsafe_index(larger_cell).contains(Index::unsafe_index(smaller_cell)));
    }

    #[test]
    fn index_15_should_contain_index_12() {
        assert!(Index::unsafe_index((14u64 << 52) + 15u64).contains(Index::unsafe_index(12u64)));
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
    fn test_index_to_index_data_conversion() {
        let test_data = vec![
            0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 5u8, 0u8, 4u8, 2u8, 6u8, 0u8, 0u8, 0u8,
        ];
        let index_data = vec![(7u8, 5u8), (9u8, 4u8), (10u8, 2u8), (11u8, 6u8)];
        let index = Index::from_address_map(28, 0, &index_data);
        let index_structure = index.to_index_structure();
        assert!(test_data == index_structure.address)
    }

    #[test]
    fn test_return_truncated_h3_local_index_0() {
        let random_h3 = Index::unsafe_random(28, 0, 13);
        let truncated_h3 = random_h3.truncate_to_resolution(5);

        assert!(truncated_h3.is_valid());
        assert!(truncated_h3.resolution() == 5);
    }
}
