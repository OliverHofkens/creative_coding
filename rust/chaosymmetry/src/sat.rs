// Implementation of a summed area table.
// Ref: https://en.wikipedia.org/wiki/Summed-area_table

pub struct SummedAreaTable {
    table: Vec<u64>,
    width: usize,
    height: usize,
}

impl SummedAreaTable {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            // Code becomes a ton simpler if using "padded"
            // vecs, since we don't have to care about the edge
            // case of the first row and column.
            table: vec![0; (width + 1) * (height + 1)],
            width,
            height,
        }
    }

    pub fn init_from_map(&mut self, map: &[u64]) {
        // We again use the 1-offset to make the edge
        // case at row and col 0 easy.

        // For readability, otherwise I kept using self.width.
        let stride = self.width + 1;

        for row in 1..self.height + 1 {
            let map_row_offset = (row - 1) * self.width;
            let tbl_row_offset = row * stride;

            for col in 1..self.width + 1 {
                let map_idx = map_row_offset + col - 1;
                let tbl_idx = tbl_row_offset + col;

                self.table[tbl_idx] = map[map_idx]
                    .wrapping_add(self.table[tbl_idx - 1])
                    .wrapping_add(self.table[tbl_idx - stride])
                    .wrapping_sub(self.table[tbl_idx - stride - 1]);
            }
        }
    }

    pub fn query(&self, x0: usize, y0: usize, x1: usize, y1: usize) -> u64 {
        debug_assert!(x1 < self.width && y1 < self.height && x0 <= x1 && y0 <= y1);

        // The SAT query formula for an inclusive rectangle (x0,y0)..(x1,y1)
        // using a 1-padded table is:
        // SAT[y1+1][x1+1] - SAT[y0][x1+1] - SAT[y1+1][x0] + SAT[y0][x0]

        let stride = self.width + 1;
        // Calculate in our 1-offset in the outermost points:
        let x1 = x1 + 1;
        let y1 = y1 + 1;

        let idx_d = stride * y1 + x1;
        let idx_b = stride * y0 + x1;
        let idx_c = stride * y1 + x0;
        let idx_a = stride * y0 + x0;

        self.table[idx_d]
            .wrapping_sub(self.table[idx_b])
            .wrapping_sub(self.table[idx_c])
            .wrapping_add(self.table[idx_a])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zeros() {
        let sat = SummedAreaTable::new(3, 3);

        assert_eq!(sat.query(0, 0, 0, 0), 0);
        assert_eq!(sat.query(0, 0, 2, 2), 0);
    }

    #[test]
    fn test_single_nonzero_cell() {
        let mut sat = SummedAreaTable::new(3, 3);
        #[rustfmt::skip]
        sat.init_from_map(&[
            0, 0, 0,
            0, 5, 0,
            0, 0, 0,
        ]);

        // The cell itself:
        assert_eq!(sat.query(1, 1, 1, 1), 5);
        // Rectangles that exclude it:
        assert_eq!(sat.query(0, 0, 0, 0), 0);
        assert_eq!(sat.query(2, 0, 2, 2), 0);
        // Rectangle that contains it:
        assert_eq!(sat.query(0, 0, 2, 2), 5);
    }

    #[test]
    fn test_non_uniform() {
        let mut sat = SummedAreaTable::new(3, 3);
        #[rustfmt::skip]
        sat.init_from_map(&[
            1, 2, 3,
            4, 5, 6,
            7, 8, 9,
        ]);

        // Bottom-right 2×2: 5+6+8+9 = 28
        assert_eq!(sat.query(1, 1, 2, 2), 28);
        // First two columns of first row: 1+2 = 3
        assert_eq!(sat.query(0, 0, 1, 0), 3);
        // Middle cell only:
        assert_eq!(sat.query(1, 1, 1, 1), 5);
        // Full grid: 1+2+...+9 = 45
        assert_eq!(sat.query(0, 0, 2, 2), 45);
    }

    #[test]
    fn test_rebuild() {
        let mut sat = SummedAreaTable::new(3, 3);
        sat.init_from_map(&[1; 9]);
        assert_eq!(sat.query(0, 0, 2, 2), 9);

        // Rebuild with different data — table must not accumulate.
        sat.init_from_map(&[2; 9]);
        assert_eq!(sat.query(0, 0, 2, 2), 18);
        assert_eq!(sat.query(0, 0, 0, 0), 2);
    }

    #[test]
    fn test_uniform() {
        let mut sat = SummedAreaTable::new(3, 3);
        sat.init_from_map(&[1; 9]);

        // Top left cell is just 1:
        assert_eq!(sat.query(0, 0, 0, 0), 1);

        // Full sum is 9:
        assert_eq!(sat.query(0, 0, 2, 2), 9);

        // First row is 3:
        assert_eq!(sat.query(0, 0, 2, 0), 3);

        // First column is 3:
        assert_eq!(sat.query(0, 0, 0, 2), 3);

        // Bottom right square is 4:
        assert_eq!(sat.query(1, 1, 2, 2), 4);
    }
}
