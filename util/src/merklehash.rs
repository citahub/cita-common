// CITA
// Copyright 2016-2017 Cryptape Technologies LLC.

// This program is free software: you can redistribute it
// and/or modify it under the terms of the GNU General Public
// License as published by the Free Software Foundation,
// either version 3 of the License, or (at your option) any
// later version.

// This program is distributed in the hope that it will be
// useful, but WITHOUT ANY WARRANTY; without even the implied
// warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR
// PURPOSE. See the GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

//! Generetes complete merkle tree root.
//!
//! This module should be used to generate complete merkle tree root hash.

use H256;
use hashable::{Hashable, HASH_NULL_RLP};
use rlp::RlpStream;

#[derive(Debug, Clone)]
pub struct MerkleTree {
    nodes: Vec<H256>,
    input_size: usize,
}

#[derive(Debug, Clone)]
pub struct MerkleProofNode {
    pub is_right: bool,
    pub hash: H256,
}

pub type MerkleProof = Vec<MerkleProofNode>;

impl MerkleTree {
    pub fn from_hashes(input: Vec<H256>) -> Self {
        generate_merkle_tree_from_hash(input)
    }

    pub fn from_bytes<I>(input: I) -> Self
    where
        I: IntoIterator<Item = Vec<u8>>,
    {
        MerkleTree::from_hashes(input.into_iter().map(|v| v.crypt_hash()).collect())
    }

    pub fn get_root_hash(&self) -> H256 {
        self.nodes[0]
    }

    pub fn get_proof_by_input_index(&self, input_index: usize) -> MerkleProof {
        get_proof_indexes(input_index, self.input_size)
            .into_iter()
            .map(|i| MerkleProofNode {
                is_right: (i & 1) == 0,
                hash: self.nodes[i],
            })
            .collect()
    }
}

pub fn verify_proof(root_hash: H256, proof: &MerkleProof, data_hash: H256) -> bool {
    proof.iter().fold(data_hash, |h, ref x| {
        if x.is_right {
            merge(&h, &x.hash)
        } else {
            merge(&x.hash, &h)
        }
    }) == root_hash
}

// For example, there is 11 hashes [A..K] are used to generate a merkle tree:
//
//      A_B C_D E_F
//       |   |   |
//       7___8   9___G   H___I   J___K
//         |       |       |       |
//         3_______4       5_______6
//             |               |
//             1_______________2
//                     |
//                     0
//
// The algorithm is:
//
//      1. Create a vec:    [_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _]
//      2. Insert A..F:     [_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, A, B, C, D, E, F]
//      3. Insert G..K:     [_, _, _, _, _, _, _, _, _, _, G, H, I, J, K, A, B, C, D, E, F]
//      4. Update for 7..8: [_, _, _, _, _, _, _, 7, 8, 9, G, H, I, J, K, A, B, C, D, E, F]
//      5. Update for 3..6: [_, _, _, 3, 4, 5, 6, 7, 8, 9, G, H, I, J, K, A, B, C, D, E, F]
//      6. Update for 1..2: [_, 1, 2, 3, 4, 5, 6, 7, 8, 9, G, H, I, J, K, A, B, C, D, E, F]
//      7. Update for 0:    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, G, H, I, J, K, A, B, C, D, E, F]
#[inline]
fn generate_merkle_tree_from_hash(input: Vec<H256>) -> MerkleTree {
    let input_size = input.len();

    let nodes = match input_size {
        // in case of empty slice, just return HASH_NULL_RLP
        0 => vec![HASH_NULL_RLP],

        // If only one input.
        1 => input,

        _ => {
            let nodes_size = get_size_of_nodes(input_size);
            let mut nodes = vec![H256::default(); nodes_size];

            let lowest_row = get_lowest_row(input_size);

            let first_input_node_index = nodes_size - input_size;
            let first_index_in_lowest_row = (1 << (lowest_row - 1)) - 1;
            let nodes_size_of_lowest_row = nodes_size - first_index_in_lowest_row;

            // Insert the input into the merkle tree.
            for i in 0..nodes_size_of_lowest_row {
                nodes[first_index_in_lowest_row + i] = input[i]
            }
            for i in 0..input_size - nodes_size_of_lowest_row {
                nodes[first_input_node_index + i] = input[nodes_size_of_lowest_row + i];
            }

            let max_nodes_size_of_lowest_row = 1 << (lowest_row - 1);
            // Calc hash for the lowest row
            if max_nodes_size_of_lowest_row == input_size {
                // The lowest row is full.
                calc_merkle_tree_at_row(&mut nodes, lowest_row - 1, 0)
            } else {
                calc_merkle_tree_at_row(&mut nodes, lowest_row - 1, nodes_size_of_lowest_row >> 1);
            }

            // The first row do not have to calc.
            for i in 1..lowest_row - 1 {
                let row = lowest_row - i - 1;
                calc_merkle_tree_at_row(&mut nodes, row, 0);
            }

            nodes
        }
    };

    MerkleTree {
        nodes: nodes,
        input_size: input_size,
    }
}

// Calc hash for one row in merkle tree.
// If break_cnt > 0, just calc break_cnt hash for that row.
fn calc_merkle_tree_at_row(nodes: &mut Vec<H256>, row: usize, break_cnt: usize) {
    let index_update = (1 << (row - 1)) - 1;
    let size_update = if break_cnt > 0 {
        break_cnt
    } else {
        1 << (row - 1)
    };
    for i in 0..size_update {
        let j = index_update + i;
        nodes[j] = merge(&nodes[j * 2 + 1], &nodes[j * 2 + 2]);
    }
}

fn merge(left: &H256, right: &H256) -> H256 {
    let mut stream = RlpStream::new();
    stream.append(left);
    stream.append(right);
    stream.out().crypt_hash()
}

#[inline]
fn get_lowest_row(m: usize) -> usize {
    let mut x: usize = 1;
    let mut y: usize = 0;
    while x < m {
        x <<= 1;
        y += 1;
    }
    y + 1
}

#[inline]
fn get_size_of_nodes(m: usize) -> usize {
    // The lowest row for m:
    //      x = get_lowest_row(m);
    // First row above the lowest row:
    //      y = x - 1;
    // Size of nodes for the first row above the lowest row:
    //      z = 2 ^ (y - 1);
    // Size of nodes above the lowest row:
    //      n1 = 2 ^ y - 1;
    // Size of nodes for the lowest row:
    //      n2 = (m - z) * 2;
    // Returns:
    //      n1 + n2
    //      = (2 ^ y - 1) + ((m - z) * 2)
    //      = m * 2  - 1
    m * 2 - 1
}

// Use math index (start from 1).
#[inline]
fn get_math_index_of_brother_node(mi: usize) -> usize {
    (mi & ((!0) - 1)) + ((!mi) & 1)
}

#[inline]
fn get_proof_indexes(input_index: usize, input_size: usize) -> Vec<usize> {
    let mut ret = Vec::new();
    let nodes_size = get_size_of_nodes(input_size);
    let lowest_row = get_lowest_row(input_size);
    let first_input_node_index = nodes_size - input_size;
    let first_index_in_lowest_row = (1 << (lowest_row - 1)) - 1;
    let nodes_size_of_lowest_row = nodes_size - first_index_in_lowest_row;
    let index = if input_index + 1 > nodes_size_of_lowest_row {
        first_input_node_index + (input_index - nodes_size_of_lowest_row)
    } else {
        first_index_in_lowest_row + input_index
    };
    // Convert computer index (start from 0) to math index (start from 1).
    let mut midx = index + 1;
    while midx > 1 {
        ret.push(get_math_index_of_brother_node(midx) - 1);
        midx >>= 1;
    }
    ret
}

#[cfg(test)]
mod tests {
    use hashable::Hashable;

    #[test]
    fn test_lowest_row() {
        let check = vec![
            (1, 1),
            (2, 2),
            (3, 3),
            (4, 3),
            (5, 4),
            (8, 4),
            (9, 5),
            (16, 5),
            (17, 6),
        ];
        for (x, y) in check {
            assert_eq!(y, super::get_lowest_row(x));
        }
    }

    #[test]
    fn test_size_of_nodes() {
        let check = vec![
            (1, 1),
            (2, 3),
            (3, 5),
            (4, 7),
            (5, 9),
            (8, 15),
            (9, 17),
            (16, 31),
            (20, 39),
        ];
        for (x, y) in check {
            assert_eq!(y, super::get_size_of_nodes(x));
        }
    }

    #[test]
    fn test_math_index_of_brother_node() {
        let check = vec![
            (2, 3),
            (3, 2),
            (12, 13),
            (13, 12),
            (22, 23),
            (23, 22),
            (32, 33),
            (33, 32),
        ];
        for (x, y) in check {
            assert_eq!(y, super::get_math_index_of_brother_node(x));
        }
    }

    #[test]
    fn test_proof_indexes() {
        let check = vec![
            ((0, 20), vec![32, 16, 8, 4, 2]),
            ((11, 20), vec![21, 9, 3, 2]),
            ((15, 20), vec![25, 11, 6, 1]),
            ((0, 19), vec![32, 16, 8, 4, 2]),
            ((11, 19), vec![24, 12, 6, 1]),
            ((15, 19), vec![28, 14, 5, 1]),
        ];
        for ((x1, x2), y) in check {
            assert_eq!(y, super::get_proof_indexes(x1, x2));
        }
    }

    #[test]
    fn test_proof() {
        let input = vec![
            b"a".to_vec(),
            b"b".to_vec(),
            b"c".to_vec(),
            b"d".to_vec(),
            b"e".to_vec(),
            b"f".to_vec(),
            b"g".to_vec(),
            b"h".to_vec(),
            b"i".to_vec(),
            b"j".to_vec(),
            b"k".to_vec(),
            b"l".to_vec(),
            b"m".to_vec(),
            b"n".to_vec(),
            b"o".to_vec(),
            b"p".to_vec(),
            b"q".to_vec(),
            b"r".to_vec(),
            b"s".to_vec(),
            b"t".to_vec(),
            b"u".to_vec(),
            b"v".to_vec(),
            b"w".to_vec(),
            b"x".to_vec(),
            b"y".to_vec(),
            b"z".to_vec(),
        ];
        let tree = super::MerkleTree::from_bytes(input.clone());
        let root_hash = tree.get_root_hash();
        let input_size = input.len();
        for index in 0..input_size {
            let data_hash = input[index].crypt_hash();
            let proof = tree.get_proof_by_input_index(index);
            assert!(super::verify_proof(root_hash, &proof, data_hash));
        }
    }
}

#[cfg(test)]
#[cfg(feature = "sha3hash")]
mod tests_for_sha3hash {
    use super::MerkleTree;
    use H256;
    use std::str::FromStr;

    #[test]
    fn test_from_bytes() {
        let check = vec![
            (
                vec![
                    b"A".to_vec(),
                    b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_vec(),
                ],
                "9bd41e0d43f4ec7a703edc2eb9fbb4106e1bc2a845e9ee1d4f3f4cf99b8549e6",
            ),
            (
                vec![
                    b"A".to_vec(),
                    b"aaaa".to_vec(),
                    b"abaa".to_vec(),
                    b"aaba".to_vec(),
                    b"aaab".to_vec(),
                ],
                "8e827ab731f2416f6057b9c7f241b1841e345ffeabb4274e35995a45f4d42a1a",
            ),
            (
                vec![],
                "56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421",
            ),
            (
                vec![
                    b"a".to_vec(),
                    b"b".to_vec(),
                    b"c".to_vec(),
                    b"d".to_vec(),
                    b"e".to_vec(),
                    b"f".to_vec(),
                    b"g".to_vec(),
                ],
                "768dfb4ca3311fa3bf4d696dde334e30edf3542e8ea114a4f9d18fb34365f1d1",
            ),
        ];
        for (x, y) in check {
            assert_eq!(
                MerkleTree::from_bytes(x).get_root_hash(),
                H256::from_str(y).unwrap()
            );
        }
    }

    #[test]
    fn test_from_hashes() {
        let check = vec![
            (
                vec![
                    "8e827ab731f2416f6057b9c7f241b1841e345ffeabb4274e35995a45f4d42a1a",
                    "768dfb4ca3311fa3bf4d696dde334e30edf3542e8ea114a4f9d18fb34365f1d1",
                ],
                "42d89ed38fbef98df15d80913bd54e5964dee662e41936b9e061e3cb3211a6be",
            ),
            (
                vec![
                    "42d89ed38fbef98df15d80913bd54e5964dee662e41936b9e061e3cb3211a6be",
                    "e68dfb4ca3311fa3bf4d696dde334e30edf3542e8ea114a4f9d18fb34365f1d1",
                ],
                "e73b5eff121d65174af0517361b1ecc01efeb869f9f79c27dd02f5d0dec42d11",
            ),
            (
                vec![
                    "f68dfb4ca3311fa3bf4d696dde334e30edf3542e8ea114a4f9d18fb34365f1d1",
                    "968dfb4ca3311fa3bf4d696dde334e30edf3542e8ea114a4f9d18fb34365f1d1",
                ],
                "9c436eab724269a0e75d2d2feeda33b7c0e5febec118f9bdf52383ada0911cf3",
            ),
            (
                vec![
                    "e73b5eff121d65174af0517361b1ecc01efeb869f9f79c27dd02f5d0dec42d11",
                    "9c436eab724269a0e75d2d2feeda33b7c0e5febec118f9bdf52383ada0911cf3",
                ],
                "e30a149e738cfaf89fb3a2267d7109a1bda978320426c2ff8b3a2d77aa103a6a",
            ),
            (
                vec![
                    "8e827ab731f2416f6057b9c7f241b1841e345ffeabb4274e35995a45f4d42a1a",
                    "768dfb4ca3311fa3bf4d696dde334e30edf3542e8ea114a4f9d18fb34365f1d1",
                    "e68dfb4ca3311fa3bf4d696dde334e30edf3542e8ea114a4f9d18fb34365f1d1",
                    "f68dfb4ca3311fa3bf4d696dde334e30edf3542e8ea114a4f9d18fb34365f1d1",
                    "968dfb4ca3311fa3bf4d696dde334e30edf3542e8ea114a4f9d18fb34365f1d1",
                ],
                "e30a149e738cfaf89fb3a2267d7109a1bda978320426c2ff8b3a2d77aa103a6a",
            ),
        ];
        for (x, y) in check {
            assert_eq!(
                MerkleTree::from_hashes(x.into_iter().map(|x| H256::from_str(x).unwrap()).collect()).get_root_hash(),
                H256::from_str(y).unwrap()
            );
        }
    }
}
