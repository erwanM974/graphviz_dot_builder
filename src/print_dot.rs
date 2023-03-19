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

use core::fmt;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use strum_macros::IntoStaticStr;

use crate::graph::GraphVizDiGraph;
use crate::traits::DotTranslatable;


#[allow(non_camel_case_types)]
#[derive(IntoStaticStr, Clone, PartialEq, Debug, Eq, Hash)]
pub enum GraphVizOutputFormat {
    svg,
    png
}

impl fmt::Display for GraphVizOutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let as_static_str : &'static str = self.into();
        write!(f, "{}", as_static_str)
    }
}

pub fn print_dot(output_file_name : String,
                 output_file_format : GraphVizOutputFormat,
                 dot_graph : GraphVizDiGraph) -> std::io::Result<std::process::Output> {
    let dot_file_name = format!("{:}.dot", output_file_name);
    let dot_file_path : PathBuf = [&dot_file_name].iter().collect();
    {
        // ***
        let mut dot_file = File::create(dot_file_path.as_path()).unwrap();
        // ***
        dot_file.write(dot_graph.to_dot_string().as_bytes());
    }
    // ***
    let output_file_name_with_extension : String;
    let command_format_argument : &str;
    match output_file_format {
        GraphVizOutputFormat::svg => {
            output_file_name_with_extension = format!("{:}.svg", output_file_name);
            command_format_argument = "-Tsvg:cairo";
        },
        GraphVizOutputFormat::png => {
            output_file_name_with_extension = format!("{:}.png", output_file_name);
            command_format_argument = "-Tpng";
        }
    }
    let output_file_path : PathBuf = [&output_file_name_with_extension].iter().collect();
    // ***
    return std::process::Command::new("dot")
        .arg(command_format_argument)
        .arg(dot_file_path.as_path())
        .arg("-o")
        .arg(output_file_path.as_path())
        .output();
}