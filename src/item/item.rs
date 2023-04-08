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

use crate::item::cluster::GraphVizCluster;
use crate::item::node::node::GraphVizNode;
use crate::traits::{DotTranslatable, RenameableWithPrefix};

#[derive(Eq,PartialEq,Clone)]
pub enum GraphVizGraphItem {
    Node(GraphVizNode),
    Cluster(GraphVizCluster)
}

impl DotTranslatable for GraphVizGraphItem {
    fn to_dot_string(&self) -> String {
        match self {
            GraphVizGraphItem::Cluster(cluster) => {
                cluster.to_dot_string()
            },
            GraphVizGraphItem::Node(node) => {
                node.to_dot_string()
            }
        }
    }
}

impl RenameableWithPrefix for GraphVizGraphItem {
    fn rename_with_prefix(&self, prefix: &str) -> Self {
        match self {
            GraphVizGraphItem::Cluster(cluster) => {
                GraphVizGraphItem::Cluster(cluster.rename_with_prefix(prefix))
            },
            GraphVizGraphItem::Node(node) => {
                GraphVizGraphItem::Node(node.rename_with_prefix(prefix))
            }
        }
    }
}
