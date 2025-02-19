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

use crate::traits::{DotTranslatable, RenameableWithPrefix, DotBuildable};
use crate::edge::edge::GraphVizEdge;
use crate::item::item::GraphVizGraphItem;
use crate::item::node::style::GraphvizNodeStyle;


#[derive(Eq,PartialEq,Clone)]
pub struct GraphVizCluster {
    pub id : String,
    pub style : GraphvizNodeStyle,
    pub items : Vec<Box<GraphVizGraphItem>>,
    pub edges : Vec<GraphVizEdge>
}

impl GraphVizCluster {
    pub fn new(id : String,
               style : GraphvizNodeStyle,
               items : Vec<Box<GraphVizGraphItem>>,
               edges : Vec<GraphVizEdge>) -> GraphVizCluster {
        GraphVizCluster{id,style,items,edges}
    }
}

impl DotTranslatable for GraphVizCluster {
    fn to_dot_string(&self) -> String {
        let mut res = String::new();
        res.push_str(&format!("subgraph cluster_{:} {{\n",self.id));
        // ***
        for item in &self.style {
            res.push_str(&format!("{};\n",item.to_dot_string()) );
        }
        // ***
        for item in &self.items {
            res.push('\t');
            res.push_str(& item.to_dot_string() );
            res.push('\n');
        }
        for edge in &self.edges {
            res.push('\t');
            res.push_str(& edge.to_dot_string() );
            res.push('\n');
        }
        res.push('}');
        // ***
        res
    }
}


impl RenameableWithPrefix for GraphVizCluster {
    fn rename_with_prefix(&self, prefix: &str) -> Self {
        let new_items : Vec<Box<GraphVizGraphItem>> = self.items.iter().map(
            |item| Box::new(item.rename_with_prefix(prefix))).collect();
        let new_edges : Vec<GraphVizEdge> = self.edges.iter().map(
            |edge| edge.rename_with_prefix(prefix)).collect();
        // ***
        GraphVizCluster::new(format!("{}{}",prefix,self.id),
                                    self.style.clone(),
                                    new_items,
                                    new_edges)
    }
}


impl DotBuildable for GraphVizCluster {

    fn add_item(&mut self, item : GraphVizGraphItem) {
        self.items.push(Box::new(item));
    }

    fn add_edge(&mut self, edge : GraphVizEdge) {
        self.edges.push(edge);
    }

}