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



use crate::edge::style::GraphvizEdgeStyle;
use crate::traits::DotTranslatable;

#[derive(Eq,PartialEq,Clone)]
pub struct GraphVizEdge {
    pub origin_node_id : String,
    pub origin_cluster : Option<String>,
    pub target_node_id : String,
    pub target_cluster : Option<String>,
    pub style : GraphvizEdgeStyle
}

impl GraphVizEdge {
    pub fn new(
            origin_node_id : String,
            origin_cluster : Option<String>,
            target_node_id : String,
            target_cluster : Option<String>,
               style : GraphvizEdgeStyle) -> GraphVizEdge {
        return GraphVizEdge{origin_node_id,
            origin_cluster,
            target_node_id,
            target_cluster,
            style};
    }

    pub fn rename_with_prefix(&self, prefix: &String) -> Self {
        let new_origin_id = format!("{}{}",prefix,self.origin_node_id);
        let new_origin_cluster = match &self.origin_cluster {
            None => None,
            Some(cluster_id) => Some(format!("{}{}",prefix,cluster_id))
        };
        let new_target_id = format!("{}{}",prefix,self.target_node_id);
        let new_target_cluster = match &self.target_cluster {
            None => None,
            Some(cluster_id) => Some(format!("{}{}",prefix,cluster_id))
        };
        return GraphVizEdge::new(new_origin_id,
                                 new_origin_cluster,
                                 new_target_id,
                                 new_target_cluster,
                                 self.style.clone());
    }
}

impl DotTranslatable for GraphVizEdge {
    fn to_dot_string(&self) -> String {
        let mut style : Vec<String> = self.style.iter().map(
            |item| item.to_dot_string()).collect();
        match &self.origin_cluster {
            None => {},
            Some(cluster_id) => {
                style.push(format!("ltail=cluster_{}",cluster_id));
            }
        }
        match &self.target_cluster {
            None => {},
            Some(cluster_id) => {
                style.push(format!("lhead=cluster_{}",cluster_id));
            }
        }
        if style.len() > 0 {
            return format!("{}->{} [{}];", self.origin_node_id, self.target_node_id, style.join(","));
        } else {
            return format!("{}->{};", self.origin_node_id, self.target_node_id);
        }
    }
}
