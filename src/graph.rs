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
use crate::item::cluster::GraphVizCluster;
use crate::item::item::GraphVizGraphItem;
use crate::item::node::node::GraphVizNode;
use crate::item::node::style::GraphvizNodeStyle;
use crate::traits::{DotTranslatable, RenameableWithPrefix, DotBuildable};


pub struct GraphVizDiGraph {
    clusters : Vec<GraphVizCluster>,
    nodes : Vec<Box<GraphVizGraphItem>>,
    edges : Vec<GraphVizEdge>
}

impl GraphVizDiGraph {
    pub fn new() -> GraphVizDiGraph {
        return GraphVizDiGraph{clusters:vec![],nodes:vec![],edges:vec![]};
    }

    pub fn get_specific_cluster(&mut self, cluster_id : usize) -> Option<&mut GraphVizCluster> {
        return self.clusters.get_mut(cluster_id);
    }

    pub fn as_cluster(&self,
                      new_name : String,
                      style : GraphvizNodeStyle,
                      prefix : Option<String>) -> GraphVizCluster {
        match prefix {
            None => {
                let mut new_items : Vec<Box<GraphVizGraphItem>> = self.nodes.iter().cloned().collect();
                for cluster in &self.clusters {
                    new_items.push(Box::new(GraphVizGraphItem::Cluster(cluster.clone())));
                }
                // ***
                return GraphVizCluster::new(new_name,style,new_items,self.edges.clone());
            },
            Some(prefix) => {
                let mut new_items : Vec<Box<GraphVizGraphItem>> = self.nodes.iter().map(
                    |item| Box::new(item.rename_with_prefix(&prefix))).collect();
                for cluster in &self.clusters {
                    new_items.push(Box::new(GraphVizGraphItem::Cluster(cluster.rename_with_prefix(&prefix))));
                }
                let new_edges : Vec<GraphVizEdge> = self.edges.iter().map(
                    |edge| edge.rename_with_prefix(&prefix)).collect();
                // ***
                return GraphVizCluster::new(new_name,style,new_items,new_edges);
            }
        }
    }
}

impl DotTranslatable for GraphVizDiGraph {
    fn to_dot_string(&self) -> String {
        let mut res = String::new();
        res.push_str("digraph G {");
        res.push_str("\ncompound=true;" );
        for cluster in &self.clusters {
            res.push_str("\n\t");
            res.push_str(& cluster.to_dot_string() );
        }
        for node in &self.nodes {
            res.push_str("\n\t");
            res.push_str(& node.to_dot_string() );
        }
        for edge in &self.edges {
            res.push_str("\n\t");
            res.push_str(& edge.to_dot_string() );
        }
        res.push_str("\n}");
        return res;
    }
}



impl DotBuildable for GraphVizDiGraph {

    fn add_node(&mut self, node : GraphVizNode) {
        self.nodes.push(Box::new(GraphVizGraphItem::Node(node)));
    }

    fn add_cluster(&mut self, cluster : GraphVizCluster) {
        self.clusters.push(cluster);
    }

    fn add_edge(&mut self, edge : GraphVizEdge) {
        self.edges.push(edge);
    }

}

