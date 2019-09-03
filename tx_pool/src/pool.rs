// Copyright Cryptape Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crypto::{pubkey_to_address, PubKey};
use libproto::blockchain::{AccountGasLimit, SignedTransaction};
use std::cmp::Ordering;
use std::collections::{BTreeSet, HashMap, HashSet};
use types::traits::LowerHex;
use types::{Address, H256};
use util::BLOCKLIMIT;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Strategy {
    FIFO,
    PRIORITY,
    VIP,
}

#[derive(Clone, Debug)]
struct TxOrder {
    hash: H256,
    order: u64,
}

impl TxOrder {
    fn new(hash: H256, order: u64) -> Self {
        TxOrder { hash, order }
    }
}

impl Eq for TxOrder {}

impl PartialEq for TxOrder {
    fn eq(&self, other: &TxOrder) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}
impl PartialOrd for TxOrder {
    fn partial_cmp(&self, other: &TxOrder) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TxOrder {
    fn cmp(&self, b: &TxOrder) -> Ordering {
        self.order.cmp(&b.order)
    }
}

#[derive(Debug)]
pub struct Pool {
    package_limit: usize,
    order_set: BTreeSet<TxOrder>,
    txs: HashMap<H256, SignedTransaction>,
    strategy: Strategy,
    order: u64,
}

impl Pool {
    pub fn new(package_limit: usize) -> Self {
        Pool {
            package_limit,
            order_set: BTreeSet::new(),
            txs: HashMap::new(),
            strategy: Strategy::FIFO,
            order: 0,
        }
    }

    pub fn new_with_strategy(package_limit: usize, strategy: Strategy) -> Self {
        Pool {
            package_limit,
            order_set: BTreeSet::new(),
            txs: HashMap::new(),
            strategy,
            order: 0,
        }
    }

    fn get_order(&mut self) -> u64 {
        let order = self.order;
        let (new_order, _) = order.overflowing_add(1);
        self.order = new_order;
        order
    }

    fn get_order_by_priority(&mut self, _tx: &SignedTransaction) -> u64 {
        self.get_order()
    }

    fn get_order_by_vip(&mut self, _tx: &SignedTransaction) -> u64 {
        self.get_order()
    }

    pub fn enqueue(&mut self, tx: SignedTransaction) -> bool {
        let hash = H256::from_slice(tx.get_tx_hash());

        let is_ok = !self.txs.contains_key(&hash);
        if is_ok {
            let order = match self.strategy {
                Strategy::FIFO => self.get_order(),
                Strategy::PRIORITY => self.get_order_by_priority(&tx),
                Strategy::VIP => self.get_order_by_vip(&tx),
            };
            let tx_order = TxOrder::new(hash, order);
            self.order_set.insert(tx_order);
            self.txs.insert(hash, tx);
        }
        is_ok
    }

    fn update_order_set(&mut self, hash_list: &HashSet<H256>) {
        self.order_set = self
            .order_set
            .iter()
            .cloned()
            .filter(|order| !hash_list.contains(&order.hash))
            .collect();
    }

    pub fn update(&mut self, txs: &[SignedTransaction]) {
        let mut hash_list = HashSet::with_capacity(txs.len());
        for tx in txs {
            let hash = tx.crypt_hash();
            self.txs.remove(&hash);
            hash_list.insert(hash);
        }
        self.update_order_set(&hash_list);
    }

    pub fn update_with_hash(&mut self, txs: &HashSet<H256>) {
        for tx in txs {
            self.txs.remove(&tx);
        }
        self.update_order_set(txs);
    }

    pub fn get(&self, hash: &H256) -> Option<&SignedTransaction> {
        self.txs.get(hash)
    }

    pub fn package(
        &mut self,
        height: u64,
        block_quota_limit: u64,
        account_quota_limit: AccountGasLimit,
        check_quota: bool,
        admin_address: Option<Address>,
        version: u32,
    ) -> Vec<SignedTransaction> {
        let mut tx_list = Vec::new();
        let mut invalid_tx_list = Vec::new();
        let mut n = block_quota_limit;
        let mut quota_limit = account_quota_limit.get_common_quota_limit();
        let mut specific_quota_limit = account_quota_limit.get_specific_quota_limit().clone();
        let mut account_quota_used: HashMap<Address, u64> = HashMap::new();
        {
            let mut iter = self.order_set.iter();
            loop {
                let order = iter.next();
                if order.is_none() {
                    break;
                }
                let hash = order.unwrap().hash;
                let tx = self.txs.get(&hash);
                let tx_is_valid =
                    |signed_tx: &SignedTransaction, height: u64, address: Address, version: u32| {
                        let valid_until_block = signed_tx.get_transaction().get_valid_until_block();
                        let tx_version = signed_tx.get_transaction().get_version();
                        (height < valid_until_block && valid_until_block <= (height + BLOCKLIMIT))
                            && admin_address
                                .map(|admin| address == admin)
                                .unwrap_or_else(|| true)
                            && (tx_version == version)
                    };
                if let Some(tx) = tx {
                    let address = pubkey_to_address(&PubKey::from(tx.get_signer()));
                    if tx_is_valid(tx, height, address, version) {
                        let quota = tx.get_transaction_with_sig().get_transaction().quota;
                        if n <= quota {
                            if tx_list.is_empty() {
                                tx_list.push(tx.clone());
                            }
                            break;
                        }

                        if check_quota {
                            if let Some(value) = specific_quota_limit.get_mut(&address.lower_hex())
                            {
                                quota_limit = *value;
                            }

                            let mut _remainder = if quota < quota_limit {
                                quota_limit - quota
                            } else {
                                0
                            };
                            account_quota_used.entry(address).or_insert(_remainder);
                            if account_quota_used.contains_key(&address) {
                                let value = account_quota_used.get_mut(&address).unwrap();
                                if *value < quota {
                                    continue;
                                }
                                *value -= quota;
                            }
                        }
                        n -= quota;
                        tx_list.push(tx.clone());
                    } else {
                        invalid_tx_list.push(tx.clone());
                    }
                } else {
                    panic!("invalid tx order {:?}", order);
                }
            }
        }
        self.update(&invalid_tx_list);
        tx_list
    }

    pub fn cull(&self, txs: &[H256]) -> Vec<SignedTransaction> {
        let mut acc = Vec::with_capacity(txs.len());
        for tx in txs.iter() {
            if let Some(tx) = self.txs.get(tx) {
                acc.push(tx.clone())
            }
        }
        acc
    }

    pub fn package_backword_compatible(&mut self, height: u64) -> Vec<SignedTransaction> {
        let mut tx_list = Vec::new();
        let mut invalid_tx_list = Vec::new();
        let mut n = self.package_limit;
        {
            let mut iter = self.order_set.iter();
            loop {
                let order = iter.next();
                if order.is_none() {
                    break;
                }
                let hash = order.unwrap().hash;
                let tx = self.txs.get(&hash);
                if let Some(tx) = tx {
                    if tx
                        .get_transaction_with_sig()
                        .get_transaction()
                        .valid_until_block
                        >= height
                        && tx
                            .get_transaction_with_sig()
                            .get_transaction()
                            .valid_until_block
                            < (height + BLOCKLIMIT)
                    {
                        tx_list.push(tx.clone());
                        n -= 1;
                        if n == 0 {
                            break;
                        }
                    } else {
                        invalid_tx_list.push(tx.clone());
                    }
                } else {
                    panic!("invalid tx order {:?}", order);
                }
            }
        }

        self.update(&invalid_tx_list);
        tx_list
    }

    pub fn len(&self) -> usize {
        self.txs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[cfg(test)]
#[cfg(feature = "secp256k1")]
mod tests {
    use super::*;
    use crypto::{CreateKey, KeyPair, PrivKey};
    use libproto::blockchain::{AccountGasLimit, SignedTransaction, Transaction};

    pub fn generate_tx(
        data: Vec<u8>,
        valid_until_block: u64,
        privkey: &PrivKey,
        version: u32,
    ) -> SignedTransaction {
        let mut tx = Transaction::new();
        tx.set_data(data);
        tx.set_to("1234567".to_string());
        tx.set_nonce("0".to_string());
        tx.set_valid_until_block(valid_until_block);
        tx.set_quota(184467440737095);
        tx.set_version(version);

        tx.sign(*privkey)
    }

    #[test]
    fn basic() {
        let mut p = Pool::new(1);
        let keypair = KeyPair::gen_keypair();
        let privkey = keypair.privkey();

        let tx1 = generate_tx(vec![1], 99, privkey, 0);
        let tx2 = generate_tx(vec![1], 99, privkey, 0);
        let tx3 = generate_tx(vec![2], 99, privkey, 0);
        let tx4 = generate_tx(vec![3], 5, privkey, 0);

        let mut account_quota_limit = AccountGasLimit::new();
        account_quota_limit.set_common_quota_limit(10000);
        account_quota_limit.set_specific_quota_limit(HashMap::new());

        assert_eq!(p.enqueue(tx1.clone()), true);
        assert_eq!(p.enqueue(tx2.clone()), false);
        assert_eq!(p.enqueue(tx3.clone()), true);
        assert_eq!(p.enqueue(tx4.clone()), true);
        assert_eq!(p.len(), 3);
        p.update(&vec![tx1.clone()]);
        assert_eq!(p.len(), 2);
        assert_eq!(
            p.package(5, 30, account_quota_limit.clone(), true, None, 0),
            vec![tx3.clone()]
        );
        p.update(&vec![tx3.clone()]);
        assert_eq!(
            p.package(4, 30, account_quota_limit.clone(), true, None, 0),
            vec![tx4]
        );
        assert_eq!(p.len(), 1);
        assert_eq!(
            p.package(5, 30, account_quota_limit.clone(), true, None, 0),
            vec![]
        );
        assert_eq!(p.len(), 0);
    }

    #[test]
    fn version_test() {
        let mut p = Pool::new(1);
        let keypair = KeyPair::gen_keypair();
        let privkey = keypair.privkey();

        let tx1 = generate_tx(vec![1], 99, privkey, 0);
        let tx2 = generate_tx(vec![2], 99, privkey, 1);

        let mut account_quota_limit = AccountGasLimit::new();
        account_quota_limit.set_common_quota_limit(10000);
        account_quota_limit.set_specific_quota_limit(HashMap::new());

        assert_eq!(p.enqueue(tx1.clone()), true);
        assert_eq!(p.enqueue(tx2.clone()), true);
        assert_eq!(p.len(), 2);

        assert_eq!(
            p.package(5, 30, account_quota_limit.clone(), false, None, 0),
            vec![tx1.clone()]
        );
        p.update(&vec![tx1.clone()]);
        assert_eq!(
            p.package(5, 30, account_quota_limit.clone(), false, None, 0),
            vec![]
        );
        assert_eq!(p.len(), 0);
    }

    #[test]
    fn test_cull() {
        let mut p = Pool::new(1);

        let keypair = KeyPair::gen_keypair();
        let privkey = keypair.privkey();

        let tx1 = generate_tx(vec![1], 99, privkey, 0);
        let tx2 = generate_tx(vec![2], 99, privkey, 1);
        let tx3 = generate_tx(vec![3], 99, privkey, 2);
        let tx4 = generate_tx(vec![4], 99, privkey, 3);

        assert_eq!(p.enqueue(tx1.clone()), true);
        assert_eq!(p.enqueue(tx2.clone()), true);
        assert_eq!(p.enqueue(tx3.clone()), true);
        assert_eq!(p.enqueue(tx4.clone()), true);
        assert_eq!(p.len(), 4);

        let tx_hashes = vec![tx1.crypt_hash(), tx3.crypt_hash(), tx4.crypt_hash()];

        let txs = p.cull(tx_hashes.as_slice());

        assert_eq!(txs, vec![tx1, tx3, tx4]);
    }
}
