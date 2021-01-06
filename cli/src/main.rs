use std::{
    fs,
    path::Path,
    collections::HashMap
};
use anyhow::{
    Context,
    Result
};
use structopt::StructOpt;
use handlebars::Handlebars;

const TEMPLATE: &str = include_str!("../template");

#[derive(Debug, StructOpt)]
#[structopt(name="athenaeum", about="A simple CLI for doing common tasks for Athenaeum")]
enum Opt {
    /// Make a new section in the library
    New {
        title: String
    }
}

fn main() -> Result<()> {
    let opt = Opt::from_args();
    match opt {
        Opt::New { title } => new(&title)
    }
}

fn new(title: &str) -> Result<()> {
    fs::create_dir(title)
        .with_context(|| format!("Could not create directory `{}`", title))?;

    let handlebars = Handlebars::new();

    let mut map = HashMap::new();
    // A prettified title for the template
    let template_title = utils::prettify(title);
    map.insert("title", &template_title);

    let render = handlebars
        .render_template(&TEMPLATE, &map)
        .unwrap();

    // Save the template
    let save_path = format!("{}/README.md", &title); 
    let save_path = Path::new(&save_path);
    fs::write(save_path, render)
        .with_context(|| format!("Could not save file to path `{}`", save_path.display()))?;

    Ok(())
}


mod utils {
    // A function to prettify the title
    fn capitalize(word: &str) -> String {
        let mut v: Vec<char> = word.chars().collect();
        v[0] = v[0].to_uppercase().nth(0).unwrap();
        let s2: String = v.into_iter().collect();
        s2
    }

    pub fn prettify(title: & str) -> String {
        match title.contains("-") {
            true => {
                let title: Vec<String> = title
                    .split('-')
                    .map(|word| capitalize(word))
                    .collect();

                let title = title.join(" ");
                title
            },
            false => {
                let title: Vec<String> = title
                    .split_whitespace()
                    .map(|word| capitalize(word))
                    .collect();

                let title = title.join(" ");
                title
            }
        }
    }
}