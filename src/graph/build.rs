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
use crate::graph::graph::GraphVizDiGraph;
use crate::item::item::GraphVizGraphItem;
use crate::traits::{DotBuildable, DotTranslatable};

impl DotTranslatable for GraphVizDiGraph {
    fn to_dot_string(&self) -> String {
        let mut res = String::new();
        res.push_str("digraph G {");
        res.push_str("\ncompound=true;" );
        for item in &self.style {
            res.push_str(&format!("\n{};",item.to_dot_string()) );
        }
        for item in &self.items {
            res.push_str("\n\t");
            res.push_str(& item.to_dot_string() );
        }
        for edge in &self.edges {
            res.push_str("\n\t");
            res.push_str(& edge.to_dot_string() );
        }
        res.push_str("\n}");
        // ***
        res
    }
}



impl DotBuildable for GraphVizDiGraph {
    
    fn add_item(&mut self, item : GraphVizGraphItem) {
        self.items.push(item);
    }

    fn add_edge(&mut self, edge : GraphVizEdge) {
        self.edges.push(edge);
    }

}

