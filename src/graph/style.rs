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




#[derive(IntoStaticStr,Eq,PartialEq,Clone)]
pub enum GraphvizSplines {
    None, // doesn't draw edges at all
    Line, // draw edges as straight line, can be drawn over nodes
    Curved, // draw edges as curved lines, can be drawn over nodes
    Polyline, // draw edges as polylines, do not draw over nodes
    Spline, // draw edges as straight or curved lines, do not draw over nodes
    Ortho, // draw edges as several horizontal and vertical segments, do not draw over nodes
}


#[derive(Eq,PartialEq,Clone)]
pub enum GraphvizGraphStyleItem {
    Rankdir(GvGraphRankDir),
    NodeSep(u32,u32),
    Concentrate(bool),
    Splines(GraphvizSplines),
}



impl DotTranslatable for GraphvizGraphStyleItem {
    fn to_dot_string(&self) -> String {
        match self {
            GraphvizGraphStyleItem::Rankdir(ref rd) => {
                format!("rankdir={}", rd.to_dot_string())
            },
            GraphvizGraphStyleItem::NodeSep(unit,decimal) => {
                format!("nodesep={}.{}", unit, decimal)
            },
            GraphvizGraphStyleItem::Concentrate(cnc) => {
                format!("concentrate={}", cnc)
            },
            GraphvizGraphStyleItem::Splines(spline) => {
                let spline_as_static_str : &'static str = spline.into();
                format!("splines={}", spline_as_static_str.to_lowercase())
            }
        }
    }
}

pub type GraphvizGraphStyle = Vec<GraphvizGraphStyleItem>;



