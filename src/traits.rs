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

use crate::item::node::node::GraphVizNode;
use crate::item::cluster::GraphVizCluster;
use crate::edge::edge::GraphVizEdge;

pub trait DotTranslatable {
    fn to_dot_string(&self) -> String;
}

pub trait RenameableWithPrefix {
    fn rename_with_prefix(&self, prefix : &String) -> Self;
}

pub trait DotBuildable {
    fn add_node(&mut self, node : GraphVizNode);
    fn add_cluster(&mut self, cluster : GraphVizCluster);
    fn add_edge(&mut self, edge : GraphVizEdge);
}



