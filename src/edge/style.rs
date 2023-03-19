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

use crate::colors::GraphvizColor;
use crate::traits::DotTranslatable;

#[derive(Eq,PartialEq,Clone)]
pub enum GvArrowHeadFill {
    Open,
    Filled,
}

impl DotTranslatable for GvArrowHeadFill {
    fn to_dot_string(&self) -> String {
        match self {
            &GvArrowHeadFill::Open => "o".to_string(),
            &GvArrowHeadFill::Filled => "".to_string(),
        }
    }
}

#[derive(Eq,PartialEq,Clone)]
pub enum GvArrowHeadSide {
    Left,
    Right,
    Both,
}

impl DotTranslatable for GvArrowHeadSide {
    fn to_dot_string(&self) -> String {
        match self {
            &GvArrowHeadSide::Left  => "l".to_string(),
            &GvArrowHeadSide::Right => "r".to_string(),
            &GvArrowHeadSide::Both  => "".to_string(),
        }
    }
}

#[derive(Eq,PartialEq,Clone)]
pub enum GvArrowHeadStyle {
    /// No arrow will be displayed
    NoArrow,
    /// Arrow ending in a triangle
    Normal(GvArrowHeadFill, GvArrowHeadSide),
    /// Arrow ending in a small square box
    Box(GvArrowHeadFill, GvArrowHeadSide),
    /// Arrow ending in a three branching lines also called crow's foot
    Crow(GvArrowHeadSide),
    /// Arrow ending in a curve
    Curve(GvArrowHeadSide),
    /// Arrow ending in an inverted curve
    ICurve(GvArrowHeadFill, GvArrowHeadSide),
    /// Arrow ending in an diamond shaped rectangular shape.
    Diamond(GvArrowHeadFill, GvArrowHeadSide),
    /// Arrow ending in a circle.
    Dot(GvArrowHeadFill),
    /// Arrow ending in an inverted triangle.
    Inv(GvArrowHeadFill, GvArrowHeadSide),
    /// Arrow ending with a T shaped arrow.
    Tee(GvArrowHeadSide),
    /// Arrow ending with a V shaped arrow.
    Vee(GvArrowHeadSide),
}

impl DotTranslatable for GvArrowHeadStyle {

    fn to_dot_string(&self) -> String {
        let mut res = String::new();
        match self {
            GvArrowHeadStyle::Box(fill, side)
            | GvArrowHeadStyle::ICurve(fill, side)
            | GvArrowHeadStyle::Diamond(fill, side)
            | GvArrowHeadStyle::Inv(fill, side)
            | GvArrowHeadStyle::Normal(fill, side)=> {
                res.push_str(&fill.to_dot_string());
                match side {
                    GvArrowHeadSide::Left
                    | GvArrowHeadSide::Right => {
                        res.push_str(&side.to_dot_string())
                    },
                    GvArrowHeadSide::Both => {},
                };
            },
            GvArrowHeadStyle::Dot(fill) => {
                res.push_str(&fill.to_dot_string())
            },
            GvArrowHeadStyle::Crow(side)
            | GvArrowHeadStyle::Curve(side)
            | GvArrowHeadStyle::Tee(side)
            | GvArrowHeadStyle::Vee(side) => {
                match side {
                    GvArrowHeadSide::Left
                    | GvArrowHeadSide::Right => {
                        res.push_str(&side.to_dot_string())
                    },
                    GvArrowHeadSide::Both => {},
                }
            }
            GvArrowHeadStyle::NoArrow => {},
        };
        match self {
            GvArrowHeadStyle::NoArrow         => res.push_str("none"),
            GvArrowHeadStyle::Normal(_, _)    => res.push_str("normal"),
            GvArrowHeadStyle::Box(_, _)       => res.push_str("box"),
            GvArrowHeadStyle::Crow(_)         => res.push_str("crow"),
            GvArrowHeadStyle::Curve(_)        => res.push_str("curve"),
            GvArrowHeadStyle::ICurve(_, _)    => res.push_str("icurve"),
            GvArrowHeadStyle::Diamond(_, _)   => res.push_str("diamond"),
            GvArrowHeadStyle::Dot(_)          => res.push_str("dot"),
            GvArrowHeadStyle::Inv(_, _)       => res.push_str("inv"),
            GvArrowHeadStyle::Tee(_)          => res.push_str("tee"),
            GvArrowHeadStyle::Vee(_)          => res.push_str("vee"),
        };
        return res;
    }
}

#[derive(IntoStaticStr,Eq,PartialEq,Clone)]
pub enum GvEdgeLineStyle {
    Solid,
    Dashed,
    Dotted,
    Bold
}

impl DotTranslatable for GvEdgeLineStyle {
    fn to_dot_string(&self) -> String {
        let as_static_str : &'static str = self.into();
        return as_static_str.to_string().to_lowercase();
    }
}


/**
 Edge style enumeration.
The particular cases of LHead and LTail are not handled here:
indeed, if used, they refer to clusters ids so that the arrow is not drawn
between the origin and target nodes but rather between some clusters that may contain them.
So that we can rename those clusters this is not handled here.
 **/
#[derive(Eq,PartialEq,Clone)]
pub enum GraphvizEdgeStyleItem {
    LineStyle(GvEdgeLineStyle),
    Label(String),
    Head(GvArrowHeadStyle),
    Tail(GvArrowHeadStyle),
    Color(GraphvizColor),
    FontColor(GraphvizColor),
    ArrowSize(u32),
    FontSize(u32)
}

impl DotTranslatable for GraphvizEdgeStyleItem {
    fn to_dot_string(&self) -> String {
        let mut res = String::new();
        match self {
            GraphvizEdgeStyleItem::LineStyle(ref line_style) => {
                res.push_str("style=");
                res.push_str(&(line_style.to_dot_string()));
            },
            GraphvizEdgeStyleItem::Label(ref label) => {
                res.push_str("label=\"");
                res.push_str(&label);
                res.push_str("\"");
            },
            GraphvizEdgeStyleItem::Head(arrow_head_style) => {
                res.push_str("arrowhead=");
                res.push_str(&(arrow_head_style.to_dot_string()));
            },
            GraphvizEdgeStyleItem::Tail(arrow_head_style) => {
                res.push_str("arrowtail=");
                res.push_str(&(arrow_head_style.to_dot_string()));
            },
            GraphvizEdgeStyleItem::Color(graphviz_color) => {
                res.push_str("color=");
                res.push_str(&(graphviz_color.to_dot_string()));
            },
            GraphvizEdgeStyleItem::FontColor(graphviz_color) => {
                res.push_str("fontcolor=");
                res.push_str(&(graphviz_color.to_dot_string()));
            },
            GraphvizEdgeStyleItem::ArrowSize(size) => {
                res.push_str("arrowsize=");
                res.push_str(&(size.to_string()));
            },
            GraphvizEdgeStyleItem::FontSize(size) => {
                res.push_str("fontsize=");
                res.push_str(&(size.to_string()));
            }
        }
        return res;
    }
}

pub type GraphvizEdgeStyle = Vec<GraphvizEdgeStyleItem>;


