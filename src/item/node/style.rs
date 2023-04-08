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


#[derive(IntoStaticStr,Eq,PartialEq,Clone)]
pub enum GvNodeStyleKind {
    Solid,
    Dashed,
    Dotted,
    Bold,
    Rounded,
    Diagonals,
    Filled,
    Striped,
    Wedged,
    Invis
}

impl DotTranslatable for GvNodeStyleKind {
    fn to_dot_string(&self) -> String {
        let as_static_str : &'static str = self.into();
        as_static_str.to_string().to_lowercase()
    }
}

pub type GvNodeStyle = Vec<GvNodeStyleKind>;

impl DotTranslatable for GvNodeStyle {
    fn to_dot_string(&self) -> String {
        let elements : Vec<String> = self.iter().map(
            |item| item.to_dot_string()).collect();
        format!("\"{}\"", elements.join(","))
    }
}


#[derive(IntoStaticStr,Eq,PartialEq,Clone)]
pub enum GvNodeShape {
    Ellipse,
    Circle,
    DoubleCircle,
    Triangle,
    Diamond,
    Trapezium,
    Parallelogram,
    House,
    Pentagon,
    Hexagon,
    Septagon,
    Octagon,
    Rectangle,
    Square,
    InvTriangle,
    InvTrapezium,
    InvHouse,
    Star,
    PlainText,
    Point
}

impl DotTranslatable for GvNodeShape {
    fn to_dot_string(&self) -> String {
        let as_static_str : &'static str = self.into();
        as_static_str.to_string().to_lowercase()
    }
}




#[derive(Eq,PartialEq,Clone)]
pub enum GraphvizNodeStyleItem {
    Style(GvNodeStyle),
    Shape(GvNodeShape),
    Label(String),
    Image(String),
    Color(GraphvizColor),
    FillColor(GraphvizColor),
    FontColor(GraphvizColor),
    FontSize(u32),
    FontName(String),
    Height(u32),
    Width(u32),
    Peripheries(u32),
    PenWidth(u32)
}

impl DotTranslatable for GraphvizNodeStyleItem {
    fn to_dot_string(&self) -> String {
        match self {
            GraphvizNodeStyleItem::PenWidth(pw) => {
                format!("penwidth={:}",pw)
            },
            GraphvizNodeStyleItem::Height(height) => {
                format!("height={:}",height)
            },
            GraphvizNodeStyleItem::Width(width) => {
                format!("width={:}",width)
            },
            GraphvizNodeStyleItem::Peripheries(per) => {
                format!("peripheries={:}",per)
            },
            GraphvizNodeStyleItem::Style(node_style) => {
                format!("style={:}",node_style.to_dot_string())
            },
            GraphvizNodeStyleItem::Shape(node_shape) => {
                format!("shape={:}",node_shape.to_dot_string())
            },
            GraphvizNodeStyleItem::Label(label) => {
                format!("label=\"{}\"",label)
            },
            GraphvizNodeStyleItem::Image(imgpath) => {
                format!("imagescale=true;image=\"{}\"",imgpath)
            },
            GraphvizNodeStyleItem::Color(graphviz_color) => {
                format!("color={:}",graphviz_color.to_dot_string())
            },
            GraphvizNodeStyleItem::FillColor(graphviz_color) => {
                format!("style=filled;fillcolor={:}",graphviz_color.to_dot_string())
            },
            GraphvizNodeStyleItem::FontColor(graphviz_color) => {
                format!("fontcolor={:}",graphviz_color.to_dot_string())
            },
            GraphvizNodeStyleItem::FontSize(size) => {
                format!("fontsize={:}",size)
            },GraphvizNodeStyleItem::FontName(fname) => {
                format!("fontname=\"{}\"",fname)
            }
        }
    }
}

pub type GraphvizNodeStyle = Vec<GraphvizNodeStyleItem>;


