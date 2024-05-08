#[derive(Clone, Copy, Debug)]
pub struct Index {
    pub index: i64,
}

// bitmasks
static ALL_ONES: i64 = -1;
//static UNSET: i64 = 0;
static RESOLUTION_MASK: i64 = Index::binary_ones(4u8).index << RESOLUTION_SHIFT;
//static LOCATION_MASK: i64 = Index::binary_ones(52).index;
static ZERO_RESOLUTION_ONLY: i64 = ALL_ONES & RESOLUTION_MASK;

// shifts
static RESOLUTION_SHIFT: u8 = 52;
//static MIN_RESOLUTION: u8 = 0;
static MAX_RESOLUTION: u8 = 15;

impl Index {
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

    pub fn truncate_to_res(self, resolution: u8) -> Index {
        Index {
            index: (self.index & ZERO_RESOLUTION_ONLY) & Index::bit_cell_mask(resolution, 3).index
                | (i64::from(resolution) << RESOLUTION_SHIFT),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::index::Index;

    #[test]
    fn test_root_node_is_not_null() {
        let index = Index { index: 0 };
        assert!(index.index == 0);
    }
}
