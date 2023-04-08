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



use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::Output;
use crate::graph::graph::GraphVizDiGraph;
use crate::traits::{DotPrintable, DotTranslatable, GraphVizOutputFormat};

impl DotPrintable for GraphVizDiGraph {

    fn print_dot(&self,
                 parent_folder_path: &[String],
                 output_file_name: &str,
                 output_file_format: &GraphVizOutputFormat) -> std::io::Result<Output> {

        let dot_file_path : Vec<String>;
        {
            let mut fp : Vec<String> = parent_folder_path.to_vec();
            fp.push( format!("{:}.dot", output_file_name) );
            dot_file_path = fp;
        }
        // ***
        let image_file_path : Vec<String>;
        let command_format_argument : &str;
        match output_file_format {
            GraphVizOutputFormat::svg => {
                let mut fp : Vec<String> = parent_folder_path.to_vec();
                fp.push( format!("{:}.svg", output_file_name) );
                // ***
                image_file_path = fp;
                command_format_argument = "-Tsvg:cairo";
            },
            GraphVizOutputFormat::png => {
                let mut fp : Vec<String> = parent_folder_path.to_vec();
                fp.push( format!("{:}.png", output_file_name) );
                // ***
                image_file_path = fp;
                command_format_argument = "-Tpng";
            }
        }
        // ***
        let dot_buf : PathBuf = dot_file_path.iter().collect();
        let mut dot_file = File::create(dot_buf.as_path()).unwrap();
        dot_file.write(self.to_dot_string().as_bytes());

        let img_buf: PathBuf = image_file_path.iter().collect();

        std::process::Command::new("dot")
            .arg(command_format_argument)
            .arg(dot_buf.as_path())
            .arg("-o")
            .arg(img_buf.as_path())
            .output()
    }
}