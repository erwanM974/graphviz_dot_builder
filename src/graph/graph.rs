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

use crate::edge::edge::GraphVizEdge;
use crate::graph::style::GraphvizGraphStyle;
use crate::item::cluster::GraphVizCluster;
use crate::item::item::GraphVizGraphItem;
use crate::item::node::style::GraphvizNodeStyle;
use crate::traits::RenameableWithPrefix;


pub struct GraphVizDiGraph {
    pub(crate) style : GraphvizGraphStyle,
    pub(crate) items : Vec<GraphVizGraphItem>,
    pub(crate) edges : Vec<GraphVizEdge>
}

impl GraphVizDiGraph {

    pub fn new(style:GraphvizGraphStyle) -> GraphVizDiGraph {
        GraphVizDiGraph{style,items:vec![],edges:vec![]}
    }

    pub fn as_cluster(&self,
                      new_name : String,
                      style : GraphvizNodeStyle,
                      prefix : Option<String>) -> GraphVizCluster {
        match prefix {
            None => {
                let new_items : Vec<Box<GraphVizGraphItem>> = self.items.iter().map(
                    |item| Box::new(item.clone())).collect();
                GraphVizCluster::new(new_name,style,new_items,self.edges.clone())
            },
            Some(prefix) => {
                let new_items : Vec<Box<GraphVizGraphItem>> = self.items.iter().map(
                    |item| Box::new(item.rename_with_prefix(&prefix))).collect();
                let new_edges : Vec<GraphVizEdge> = self.edges.iter().map(
                    |edge| edge.rename_with_prefix(&prefix)).collect();
                // ***
                GraphVizCluster::new(new_name,style,new_items,new_edges)
            }
        }
    }
}


