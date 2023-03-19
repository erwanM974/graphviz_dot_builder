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



use crate::item::node::style::GraphvizNodeStyle;
use crate::traits::{DotTranslatable, RenameableWithPrefix};

#[derive(Eq,PartialEq,Clone)]
pub struct GraphVizNode {
    pub id : String,
    pub style : GraphvizNodeStyle
}

impl GraphVizNode {
    pub fn new(id : String,
               style : GraphvizNodeStyle) -> GraphVizNode {
        return GraphVizNode{id,style};
    }
}

impl DotTranslatable for GraphVizNode {
    fn to_dot_string(&self) -> String {
        let style : Vec<String> = self.style.iter().map(
            |item| item.to_dot_string()).collect();
        if style.len() > 0 {
            return format!("{} [{}];", self.id, style.join(","));
        } else {
            return format!("{};", self.id);
        }
    }
}

impl RenameableWithPrefix for GraphVizNode {
    fn rename_with_prefix(&self, prefix: &String) -> Self {
        return GraphVizNode::new(format!("{}{}",prefix,self.id),self.style.clone());
    }
}

