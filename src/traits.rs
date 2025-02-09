/*
Copyright 2020 Erwan Mahe (github.com/erwanM974)

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

use std::fmt;
use strum_macros::IntoStaticStr;


use crate::item::item::GraphVizGraphItem;
use crate::item::node::node::GraphVizNode;
use crate::item::cluster::GraphVizCluster;
use crate::edge::edge::GraphVizEdge;

#[allow(non_camel_case_types)]
#[derive(IntoStaticStr, Clone, PartialEq, Debug, Eq, Hash)]
pub enum GraphVizOutputFormat {
    svg,
    png
}

impl fmt::Display for GraphVizOutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let as_static_str : &'static str = self.into();
        write!(f, "{}", as_static_str)
    }
}

pub trait DotTranslatable {
    fn to_dot_string(&self) -> String;
}

pub trait RenameableWithPrefix {
    fn rename_with_prefix(&self, prefix : &str) -> Self;
}

pub trait DotBuildable {
    fn add_item(&mut self, item : GraphVizGraphItem);
    fn add_edge(&mut self, edge : GraphVizEdge);
    // ***
    fn add_node(&mut self, node : GraphVizNode) {
        self.add_item(GraphVizGraphItem::Node(node));
    }
    fn add_cluster(&mut self, cluster : GraphVizCluster) {
        self.add_item(GraphVizGraphItem::Cluster(cluster));
    }
}

pub trait DotPrintable {

    fn print_dot(&self,parent_folder_path : &[String],
                 output_file_name : &str,
                 output_file_format : &GraphVizOutputFormat) -> std::io::Result<std::process::Output>;

}


