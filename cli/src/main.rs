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
#[structopt(name="flower", about="A simple CLI for doing common tasks for Flower")]
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
        let delimiter = match title.contains('-') {
            true => '-',
            false => ' '
        };

        let title: Vec<String> = title
            .split(delimiter)
            .map(|word| capitalize(word))
            .collect();

        let title = title.join(" ");
        title
    }
}

#[cfg(test)]
mod tests {
    use crate::utils;
    #[test]
    fn prettify() {
        // All permutations of a hyphenated entry
        let title = utils::prettify("distributed-systems");
        assert_eq!(title, "Distributed Systems");

        let title = utils::prettify("Distributed-Systems");
        assert_eq!(title, "Distributed Systems");

        let title = utils::prettify("Distributed-systems");
        assert_eq!(title, "Distributed Systems");

        let title = utils::prettify("distributed-Systems");
        assert_eq!(title, "Distributed Systems");

        // All permutations of a non-hyphenated entry
        let title = utils::prettify("distributed systems");
        assert_eq!(title, "Distributed Systems");

        let title = utils::prettify("Distributed Systems");
        assert_eq!(title, "Distributed Systems");

        let title = utils::prettify("Distributed systems");
        assert_eq!(title, "Distributed Systems");

        let title = utils::prettify("distributed Systems");
        assert_eq!(title, "Distributed Systems");
    }
}