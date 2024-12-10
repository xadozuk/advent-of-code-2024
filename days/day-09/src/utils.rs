use core::panic;
use std::collections::BTreeSet;
use std::fmt::Debug;

pub struct Drive {
    data: Vec<Block>,
    free_space_index: BTreeSet<FreeSpaceDescriptor>,
}

#[derive(Clone, PartialEq, Eq)]
struct FreeSpaceDescriptor {
    size: usize,
    start_at: usize,
}

#[derive(Clone)]
pub enum Block {
    File(u32),
    Free,
}

impl Drive {
    pub fn new(layout: &str) -> Self {
        let mut current_file_id = 0;

        let data = layout
            .chars()
            .enumerate()
            .flat_map(|(i, c)| {
                let size = c.to_digit(10).unwrap();

                if i % 2 == 0 {
                    let result = vec![Block::File(current_file_id); size as usize];
                    current_file_id += 1;

                    result
                } else {
                    vec![Block::Free; size as usize]
                }
            })
            .collect();

        let mut drive = Drive {
            data,
            free_space_index: BTreeSet::new(),
        };

        drive.refresh_free_space_index();

        drive
    }

    fn refresh_free_space_index(&mut self) {
        self.free_space_index.clear();

        let mut current_pos = 0;

        while current_pos < self.data.len() {
            let free_space_start = self
                .data
                .iter()
                .skip(current_pos)
                .position(|b| matches!(b, Block::Free));

            if let Some(f_start) = free_space_start {
                let f_size = self
                    .data
                    .iter()
                    .skip(current_pos + f_start)
                    .take_while(|b| matches!(b, Block::Free))
                    .count();

                self.free_space_index.insert(FreeSpaceDescriptor {
                    start_at: current_pos + f_start,
                    size: f_size,
                });

                current_pos += f_start + f_size;
            } else {
                break;
            }
        }
    }

    pub fn checksum(&self) -> u64 {
        self.data
            .iter()
            .enumerate()
            .map(|(i, b)| match b {
                Block::File(id) => *id as u64 * i as u64,
                Block::Free => 0,
            })
            .sum()
    }

    pub fn dumb_compact(&mut self) {
        self.refresh_free_space_index();

        let mut last_free_space_used_index = 0;

        for i in (1..self.data.len()).rev() {
            if let Block::File(_) = self.data[i] {
                if let Some(fsp_desc) = self.find_free_space(i, 1) {
                    last_free_space_used_index = fsp_desc.start_at;
                    self.move_at_free_space(i, 1, fsp_desc);
                }
            }

            if i < last_free_space_used_index {
                break;
            }
        }
    }

    pub fn smart_compact(&mut self) {
        self.refresh_free_space_index();

        let mut first_free_space_block_index = 0;
        let mut current_pos = self.data.len();

        while let Some((block_index, block_size)) =
            self.rposition_full_file_block_before(current_pos)
        {
            if let Some(fsp_desc) = self.find_free_space(block_index, block_size) {
                self.move_at_free_space(block_index, block_size, fsp_desc);

                first_free_space_block_index = self
                    .find_free_space(self.data.len(), 1)
                    .map_or(0, |fsd| fsd.start_at);
            }

            if block_index < first_free_space_block_index {
                break;
            }

            current_pos = block_index;
        }
    }

    fn find_free_space(&self, before: usize, size: usize) -> Option<FreeSpaceDescriptor> {
        self.free_space_index
            .iter()
            .find(|fsd| fsd.start_at < before && fsd.size >= size)
            .cloned()
    }

    fn rposition_full_file_block_before(&self, before: usize) -> Option<(usize, usize)> {
        let (block_end, block) = self
            .data
            .iter()
            .enumerate()
            .rev()
            .skip(self.data.len() - before)
            .rev()
            .rfind(|(_, b)| matches!(b, Block::File(_)))?;

        if let Block::File(expected_id) = block {
            let block_size = self.data[..block_end]
                .iter()
                .rev()
                .position(|b| match b {
                    Block::File(id) => expected_id != id,
                    _ => true,
                })
                .or(Some(block_end))
                // +1 because block_end is excluded
                .map(|i| i + 1)
                .unwrap();

            Some((block_end - (block_size - 1), block_size))
        } else {
            None
        }
    }

    fn move_at_free_space(
        &mut self,
        block_index: usize,
        block_size: usize,
        free_space_descriptor: FreeSpaceDescriptor,
    ) {
        if free_space_descriptor.size < block_size {
            panic!("Free space is to small for the move");
        }

        for i in 0..block_size {
            self.data
                .swap(block_index + i, free_space_descriptor.start_at + i);
        }

        self.free_space_index.remove(&free_space_descriptor);

        if free_space_descriptor.size > block_size {
            let remaining_free_space_size = free_space_descriptor.size - block_size;

            self.free_space_index.insert(FreeSpaceDescriptor {
                size: remaining_free_space_size,
                start_at: free_space_descriptor.start_at + block_size,
            });
        }

        self.free_space_index.insert(FreeSpaceDescriptor {
            size: block_size,
            start_at: block_index,
        });
    }
}

impl Clone for Drive {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            free_space_index: BTreeSet::new(),
        }
    }
}

impl Ord for FreeSpaceDescriptor {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.start_at.cmp(&other.start_at)
    }
}

impl PartialOrd for FreeSpaceDescriptor {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Debug for Drive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for block in &self.data {
            match block {
                Block::File(id) => write!(f, "{}", id)?,
                Block::Free => write!(f, ".")?,
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rposition_full_file_block_before() {
        // 1..333
        let drive = Drive::new("123");

        assert_eq!(
            drive.rposition_full_file_block_before(drive.data.len()),
            Some((3, 3))
        );

        assert_eq!(drive.rposition_full_file_block_before(2), Some((0, 1)));
        assert_eq!(drive.rposition_full_file_block_before(0), None);
    }
}
