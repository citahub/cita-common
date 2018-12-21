// CITA
// Copyright 2016-2018 Cryptape Technologies LLC.

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

extern crate cita_types;
extern crate hashable;
extern crate rlp;
#[macro_use]
extern crate rlp_derive;

use cita_types::H256;
use hashable::{Hashable, HASH_NULL_RLP};
use rlp::RlpStream;

#[derive(Debug, Clone)]
pub struct Tree {
    nodes: Vec<H256>,
    input_size: usize,
}

#[derive(Debug, Clone, RlpEncodable, RlpDecodable)]
pub struct ProofNode {
    is_right: bool,
    hash: H256,
}

#[derive(Debug, Clone, RlpEncodableWrapper, RlpDecodableWrapper)]
pub struct Proof(Vec<ProofNode>);

impl Tree {
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
    pub fn from_hashes(input: Vec<H256>) -> Self {
        let input_size = input.len();

        let nodes = match input_size {
            // in case of empty slice, just return HASH_NULL_RLP
            0 => vec![HASH_NULL_RLP],

            // If only one input.
            1 => input,

            _ => {
                let nodes_size = get_size_of_nodes(input_size);
                let mut nodes = vec![H256::default(); nodes_size];

                let rows_size = get_size_of_rows(input_size);
                let lowest_row_index = rows_size - 1;

                let first_input_node_index = nodes_size - input_size;
                let first_index_in_lowest_row = (1 << lowest_row_index) - 1;
                let nodes_size_of_lowest_row = nodes_size - first_index_in_lowest_row;

                // Insert the input into the merkle tree.
                for i in 0..nodes_size_of_lowest_row {
                    nodes[first_index_in_lowest_row + i] = input[i]
                }
                for i in 0..input_size - nodes_size_of_lowest_row {
                    nodes[first_input_node_index + i] = input[nodes_size_of_lowest_row + i];
                }

                let max_nodes_size_of_lowest_row = 1 << lowest_row_index;
                // Calc hash for the lowest row
                if max_nodes_size_of_lowest_row == input_size {
                    // The lowest row is full.
                    calc_tree_at_row(&mut nodes, lowest_row_index, 0)
                } else {
                    calc_tree_at_row(&mut nodes, lowest_row_index, nodes_size_of_lowest_row >> 1);
                }

                // From the second row from the bottom to the second row from the top.
                for i in 1..lowest_row_index {
                    let row_index = lowest_row_index - i;
                    calc_tree_at_row(&mut nodes, row_index, 0);
                }

                nodes
            }
        };

        Self {
            nodes: nodes,
            input_size: input_size,
        }
    }

    pub fn from_bytes<I>(input: I) -> Self
    where
        I: IntoIterator<Item = Vec<u8>>,
    {
        Self::from_hashes(input.into_iter().map(|v| v.crypt_hash()).collect())
    }

    pub fn get_root_hash(&self) -> H256 {
        self.nodes[0]
    }

    pub fn get_proof_by_input_index(&self, input_index: usize) -> Option<Proof> {
        get_proof_indexes(input_index, self.input_size).map(|indexes| {
            Proof(
                indexes
                    .into_iter()
                    .map(|i| ProofNode {
                        is_right: (i & 1) == 0,
                        hash: self.nodes[i],
                    })
                    .collect(),
            )
        })
    }
}

impl Proof {
    pub fn verify(&self, root_hash: H256, data_hash: H256) -> bool {
        self.0.iter().fold(data_hash, |h, ref x| {
            if x.is_right {
                merge(&h, &x.hash)
            } else {
                merge(&x.hash, &h)
            }
        }) == root_hash
    }
}

// Calc hash for one row in merkle tree.
// If break_cnt > 0, just calc break_cnt hash for that row.
fn calc_tree_at_row(nodes: &mut Vec<H256>, row_index: usize, break_cnt: usize) {
    // The first index in the row which above the row_index row.
    let index_update = (1 << (row_index - 1)) - 1;
    let size_max = 1 << (row_index - 1);
    let size_update = if break_cnt > 0 && break_cnt < size_max {
        break_cnt
    } else {
        size_max
    };
    for i in 0..size_update {
        let j = index_update + i;
        nodes[j] = merge(&nodes[j * 2 + 1], &nodes[j * 2 + 2]);
    }
}

#[inline]
fn get_size_of_rows(m: usize) -> usize {
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
    // The rows size for m:
    //      x = get_size_of_rows(m);
    // The second row from the bottom (math index):
    //      y = x - 1;
    // Size of nodes for the second row from the bottom:
    //      z = 2 ^ (y - 1);
    // Size of nodes above the lowest row:
    //      n1 = 2 ^ y - 1;
    // Size of nodes in the lowest row:
    //      n2 = (m - z) * 2;
    // Returns:
    //      n1 + n2
    //      = (2 ^ y - 1) + ((m - z) * 2)
    //      = m * 2  - 1
    if m == 0 {
        1
    } else {
        m * 2 - 1
    }
}

#[inline]
fn get_index_of_brother_and_father(index: usize) -> (usize, usize) {
    // Convert computer index (start from 0) to math index (start from 1).
    let math_index = index + 1;
    // The only one difference between brother math indexes in binary tree is the last bit.
    let math_index_for_brother = (math_index & ((!0) - 1)) + ((!math_index) & 1);
    let math_index_for_father = math_index >> 1;
    // Convert back to computer index.
    (math_index_for_brother - 1, math_index_for_father - 1)
}

#[inline]
fn get_proof_indexes(input_index: usize, input_size: usize) -> Option<Vec<usize>> {
    if input_index == 0 && input_size < 2 {
        Some(vec![])
    } else if input_size != 0 && input_index < input_size {
        let mut ret = Vec::new();
        let nodes_size = get_size_of_nodes(input_size);
        let rows_size = get_size_of_rows(input_size);
        let lowest_row_index = rows_size - 1;
        let first_input_node_index = nodes_size - input_size;
        let first_index_in_lowest_row = (1 << lowest_row_index) - 1;
        let nodes_size_of_lowest_row = nodes_size - first_index_in_lowest_row;
        let mut index = if input_index + 1 > nodes_size_of_lowest_row {
            first_input_node_index + (input_index - nodes_size_of_lowest_row)
        } else {
            first_index_in_lowest_row + input_index
        };
        while index > 0 {
            let (brother_index, parent_index) = get_index_of_brother_and_father(index);
            ret.push(brother_index);
            index = parent_index;
        }
        Some(ret)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    extern crate rlp;
    extern crate cita_types;
    extern crate hashable;

    use super::Merge;
    use self::hashable::{Hashable, HASH_NULL_RLP};
    use self::rlp::RlpStream;
    use self::cita_types::H256;

    struct RlpMerge;

    impl Merge<H256> for RlpMerge {
        fn merge(&self, left: &H256, right: &H256) -> H256 {
            let mut stream = RlpStream::new();
            stream.append(left);
            stream.append(right);
            stream.out().crypt_hash()
        }

        fn null(&self) -> H256 {
            HASH_NULL_RLP
        }
    }

    impl RlpMerge {
        fn new() -> Self {
            RlpMerge{}
        }
    }

    #[test]
    fn test_size_of_rows() {
        let check = vec![
            (0, 1),
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
            assert_eq!(y, super::get_size_of_rows(x));
        }
    }

    #[test]
    fn test_size_of_nodes() {
        let check = vec![
            (0, 1),
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
    fn test_index_of_brother_and_father() {
        let check = vec![
            (1, (2, 0)),
            (2, (1, 0)),
            (11, (12, 5)),
            (12, (11, 5)),
            (21, (22, 10)),
            (22, (21, 10)),
            (31, (32, 15)),
            (32, (31, 15)),
        ];
        for (x, y) in check {
            assert_eq!(y, super::get_index_of_brother_and_father(x));
        }
    }

    #[test]
    fn test_proof_indexes() {
        let check = vec![
            ((1, 0), None),
            ((1, 1), None),
            ((2, 1), None),
            ((2, 2), None),
            ((0, 0), Some(vec![])),
            ((0, 1), Some(vec![])),
            ((0, 20), Some(vec![32, 16, 8, 4, 2])),
            ((11, 20), Some(vec![21, 9, 3, 2])),
            ((15, 20), Some(vec![25, 11, 6, 1])),
            ((0, 19), Some(vec![32, 16, 8, 4, 2])),
            ((11, 19), Some(vec![24, 12, 6, 1])),
            ((15, 19), Some(vec![28, 14, 5, 1])),
        ];
        for ((x1, x2), y) in check {
            assert_eq!(y, super::get_proof_indexes(x1, x2));
        }
    }

    #[test]
    fn test_proof() {
        let inputs = vec![
            vec![],
            vec![b"".to_vec()],
            vec![
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
            ],
        ];
        for input in inputs {
            let tree = super::Tree::from_hashes(input.clone().into_iter().map(|v| v.crypt_hash()).collect(), RlpMerge::new());
            let root_hash = tree.get_root_hash();
            let input_size = input.len();
            let loop_size = if input_size == 0 { 1 } else { input_size };
            for index in 0..loop_size {
                let data_hash = if input_size == 0 {
                    HASH_NULL_RLP
                } else {
                    input[index].crypt_hash()
                };
                let proof = tree
                    .get_proof_by_input_index(index)
                    .expect("proof is not none");
                assert!(proof.verify(*root_hash, data_hash, RlpMerge::new()));
            }
        }
    }

    #[cfg(feature = "sha3hash")]
    mod tests_for_sha3hash {
        use super::super::Tree;
        use super::cita_types::H256;
        use std::str::FromStr;
        use super::hashable::Hashable;
        use super::RlpMerge;

        #[test]
        fn test_from_bytes() {
            let check = vec![
                (
                    vec![],
                    "56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421",
                ),
                (
                    vec![b"".to_vec()],
                    "c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470",
                ),
                (
                    vec![b"A".to_vec()],
                    "03783fac2efed8fbc9ad443e592ee30e61d65f471140c10ca155e937b435b760",
                ),
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
                    Tree::from_hashes(x.into_iter().map(|v| v.crypt_hash()).collect(), RlpMerge::new()).get_root_hash(),
                    &H256::from_str(y).unwrap()
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
                    Tree::from_hashes(x.into_iter().map(|x| H256::from_str(x).unwrap()).collect(), RlpMerge::new())
                        .get_root_hash(),
                    &H256::from_str(y).unwrap()
                );
            }
        }
    }
}
