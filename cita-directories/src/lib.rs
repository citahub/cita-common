// CITA
// Copyright 2016-2019 Cryptape Technologies LLC.

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

//! Wrapper all data paths for cita.
//!
//! Fix "Open Error: IO error Invalid argument" when using vagrant.
//! More about above error. See <https://github.com/Level/levelup/issues/222>.

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
