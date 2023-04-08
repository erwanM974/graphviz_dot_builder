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



use strum_macros::IntoStaticStr;

use crate::traits::DotTranslatable;

#[derive(IntoStaticStr,Eq,PartialEq,Clone)]
pub enum GvGraphRankDir {
    TB,
    BT,
    LR,
    RL
}

impl DotTranslatable for GvGraphRankDir {
    fn to_dot_string(&self) -> String {
        let as_static_str : &'static str = self.into();
        as_static_str.to_string().to_lowercase()
    }
}


#[derive(Eq,PartialEq,Clone)]
pub enum GraphvizGraphStyleItem {
    Rankdir(GvGraphRankDir)
}



impl DotTranslatable for GraphvizGraphStyleItem {
    fn to_dot_string(&self) -> String {
        match self {
            GraphvizGraphStyleItem::Rankdir(ref rd) => {
                format!("rankdir={}", rd.to_dot_string())
            }
        }
    }
}

pub type GraphvizGraphStyle = Vec<GraphvizGraphStyleItem>;



