// Copyright 2017 The Grin Developers
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Main interface for the cuckoo_miner plugin manager, which queries
//! all available plugins in a particular directory and returns their
//! descriptions, parameters, and capabilities

use cuckoo_sys::{get_available_plugins};
use cuckoo_config::{CuckooMinerError, CuckooPluginCapabilities};

pub struct CuckooPluginManager {
    // The directory in which to look for plugins
    plugin_dir: String,

    //
    current_plugin_caps: Option<Vec<CuckooPluginCapabilities>>,
}

impl Default for CuckooPluginManager {
	fn default() -> CuckooPluginManager {
		CuckooPluginManager {
            plugin_dir: String::from("target/debug"),
            current_plugin_caps: None,
		}
	}
}

impl CuckooPluginManager {

    pub fn new()->Result<CuckooPluginManager, CuckooMinerError>{
        Ok(CuckooPluginManager::default())
    }

    pub fn load_plugin_dir (&mut self, plugin_dir:String) 
        -> Result<(), CuckooMinerError> {
        self.plugin_dir = plugin_dir;
        let caps=get_available_plugins(&self.plugin_dir)?;
        self.current_plugin_caps=Some(caps);
        Ok(())
    }

    pub fn get_available_plugins(&mut self) -> 
        Result<&Vec<CuckooPluginCapabilities>, CuckooMinerError>{
        Ok(&self.current_plugin_caps.as_ref().unwrap())
    }
}