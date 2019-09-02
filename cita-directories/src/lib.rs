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

use std::env;
use uuid::Uuid;

pub const DATA_PATH: &str = "DATA_PATH";
const VAGRANT_DATA_PATH: &str = "VAGRANT_DATA_PATH";

pub struct DataPath;

impl DataPath {
    /// root data path
    fn root_data_path() -> String {
        let is_using_vagrant = DataPath::is_using_vagrant();
        if is_using_vagrant {
            env::var(VAGRANT_DATA_PATH).unwrap_or_else(|_| panic!("{} must be set", DATA_PATH))
        } else {
            env::var(DATA_PATH).unwrap_or_else(|_| panic!("{} must be set", DATA_PATH))
        }
    }

    /// nosql path
    pub fn nosql_path() -> String {
        DataPath::root_node_path() + "/nosql"
    }

    pub fn state_path() -> String {
        DataPath::root_node_path() + "/statedb"
    }

    /// proof.bin path
    pub fn proof_bin_path() -> String {
        DataPath::root_node_path() + "/proof.bin"
    }

    /// wal log path
    pub fn wal_path() -> String {
        DataPath::root_node_path() + "/wal"
    }

    /// node path
    pub fn root_node_path() -> String {
        let node_component = match env::current_dir() {
            Ok(pathbuf) => match pathbuf.file_name() {
                Some(name) => String::from(name.to_str().unwrap()),
                None => Uuid::new_v4().to_simple().to_string(),
            },
            Err(_) => Uuid::new_v4().to_simple().to_string(),
        };

        let is_using_vagrant = DataPath::is_using_vagrant();
        if is_using_vagrant {
            DataPath::root_data_path() + "/" + &node_component
        } else {
            DataPath::root_data_path()
        }
    }
}

trait VagrantHelper {
    fn is_using_vagrant() -> bool;
}

impl VagrantHelper for DataPath {
    fn is_using_vagrant() -> bool {
        env::var("USING_VAGRANT").unwrap_or_else(|_| "0".to_string()) == "1"
    }
}
