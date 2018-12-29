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

use std::cmp::PartialEq;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct Tree<T, M> {
    nodes: Vec<T>,
    input_size: usize,
    phanton: PhantomData<M>,
}

#[derive(Debug, Clone)]
pub struct ProofNode<T> {
    pub is_right: bool,
    pub hash: T,
}

#[derive(Debug, Clone)]
pub struct Proof<T>(pub Vec<ProofNode<T>>);

impl<T, M> Tree<T, M>
where
    T: Default + Clone + PartialEq,
    M: Fn(&T, &T) -> T,
{
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
    pub fn from_hashes(input: Vec<T>, merge: M) -> Self {
        let input_size = input.len();

        let nodes = match input_size {
            0 => vec![],

            // If only one input.
            1 => input,

            _ => {
                let nodes_size = get_size_of_nodes(input_size);
                let mut nodes = vec![T::default(); nodes_size];

                let rows_size = get_size_of_rows(input_size);
                let lowest_row_index = rows_size - 1;

                let first_input_node_index = nodes_size - input_size;
                let first_index_in_lowest_row = (1 << lowest_row_index) - 1;
                let nodes_size_of_lowest_row = nodes_size - first_index_in_lowest_row;

                // Insert the input into the merkle tree.
                for i in 0..nodes_size_of_lowest_row {
                    nodes[first_index_in_lowest_row + i] = input[i].clone()
                }
                for i in 0..input_size - nodes_size_of_lowest_row {
                    nodes[first_input_node_index + i] = input[nodes_size_of_lowest_row + i].clone();
                }

                let max_nodes_size_of_lowest_row = 1 << lowest_row_index;
                // Calc hash for the lowest row
                if max_nodes_size_of_lowest_row == input_size {
                    // The lowest row is full.
                    calc_tree_at_row(&mut nodes, lowest_row_index, 0, &merge)
                } else {
                    calc_tree_at_row(
                        &mut nodes,
                        lowest_row_index,
                        nodes_size_of_lowest_row >> 1,
                        &merge,
                    );
                }

                // From the second row from the bottom to the second row from the top.
                for i in 1..lowest_row_index {
                    let row_index = lowest_row_index - i;
                    calc_tree_at_row(&mut nodes, row_index, 0, &merge);
                }

                nodes
            }
        };

        Self {
            nodes,
            input_size,
            phanton: PhantomData,
        }
    }

    pub fn get_root_hash(&self) -> Option<&T> {
        self.nodes.get(0)
    }

    pub fn get_proof_by_input_index(&self, input_index: usize) -> Option<Proof<T>> {
        get_proof_indexes(input_index, self.input_size).map(|indexes| {
            Proof::<T>(
                indexes
                    .into_iter()
                    .map(|i| ProofNode::<T> {
                        is_right: (i & 1) == 0,
                        hash: self.nodes[i].clone(),
                    })
                    .collect(),
            )
        })
    }
}

impl<T> Proof<T>
where
    T: Default + Clone + PartialEq,
{
    pub fn verify<M>(&self, root: T, data: T, merge: M) -> bool
    where
        M: Fn(&T, &T) -> T,
    {
        self.0.iter().fold(data, |h, ref x| {
            if x.is_right {
                merge(&h, &x.hash)
            } else {
                merge(&x.hash, &h)
            }
        }) == root
    }
}

// Calc hash for one row in merkle tree.
// If break_cnt > 0, just calc break_cnt hash for that row.
fn calc_tree_at_row<T, M>(nodes: &mut Vec<T>, row_index: usize, break_cnt: usize, merge: M)
where
    M: Fn(&T, &T) -> T,
{
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
    #[derive(Default, Clone, PartialEq, Debug)]
    struct Node(Vec<u32>);

    fn merge(left: &Node, right: &Node) -> Node {
        let mut root: Vec<u32> = vec![];
        root.extend_from_slice(&left.0);
        root.extend_from_slice(&right.0);
        Node(root)
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
            vec![Node(vec![1u32])],
            (1u32..26u32).map(|i| Node(vec![i])).collect(),
        ];
        for input in inputs {
            let tree = super::Tree::from_hashes(input.clone(), merge);
            let root_hash = tree.get_root_hash().unwrap().clone();
            let input_size = input.len();
            let loop_size = if input_size == 0 { 1 } else { input_size };
            for index in 0..loop_size {
                let data_hash = input[index].clone();
                let proof = tree
                    .get_proof_by_input_index(index)
                    .expect("proof is not none");
                assert!(proof.verify(root_hash.clone(), data_hash, merge));
            }
        }
    }

    #[test]
    fn test_root() {
        let inputs = (0u32..12u32).map(|i| Node(vec![i])).collect();
        let tree = super::Tree::from_hashes(inputs, merge);
        let root = tree.get_root_hash().unwrap();
        assert_eq!(root, &Node((0u32..12u32).collect::<Vec<u32>>()));
    }
}
