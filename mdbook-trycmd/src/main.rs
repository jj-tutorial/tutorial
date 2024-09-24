use std::{
 collections::HashMap, fs, io, process::{Command, Stdio}, sync::{LazyLock, Mutex}
};

use mdbook::{BookItem, errors::Result, preprocess::CmdPreprocessor};
use regex::Regex;
use tempfile::TempDir;

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    match args.next().as_deref() {
        Some("supports") => {
            // Supports all renderers.
            return Ok(());
        }
        Some(arg) => {
            eprintln!("unknown argument: {arg}");
            std::process::exit(1);
        }
        None => {}
    }

    let (_ctx, mut book) = CmdPreprocessor::parse_input(io::stdin().lock())?;
    book.for_each_mut(|item| {
        let BookItem::Chapter(chapter) = item else {
            return;
        };
        match run_examples(&chapter.content) {
            Ok(new_content) => chapter.content = new_content,
            Err(e) => eprintln!("could not process chapter: {e}"),
        }
    });

    serde_json::to_writer(io::stdout().lock(), &book)?;

    Ok(())
}

struct Cache {
    inner: LazyLock<Mutex<HashMap<String, String>>>,
}

static CACHE: Cache = Cache { inner: LazyLock::new(|| Mutex::new(HashMap::new())) };

impl Cache {
    fn render(&self, key: &str) -> String {
        let mut map = self.inner.lock().unwrap();

        map.entry(key.to_string()).or_insert_with(|| {
            eprintln!("===rendering {key}");
            let contents = fs::read_to_string(key).unwrap();
            let contents: String = contents.lines().filter(|line| line.starts_with("$ ")).collect::<Vec<&str>>().join("\n");

            let mut rendered = String::new();

            let dir = TempDir::new().unwrap();

            for command in contents.lines() {
                // getting real hard-coded with it. we want to set this to never for
                // reproducibility in trycmd, but we also want it to be on here becuase
                // that's the whole dang point!
                let command = if command == "$ jj config set --repo ui.color never" {
                    "jj config set --repo ui.color always"
                } else {
                    command.strip_prefix('$').unwrap()
                };
                eprintln!("about to run {command}");

                let output = Command::new("bash")
                    .current_dir(&dir)
                    .arg("-c")
                    .arg(command)
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .output()
                    .unwrap();

                let render = |s|
                     ansi_to_html::convert(&String::from_utf8(s).unwrap()).unwrap();
                    
                let stdout = render(output.stdout);
                let stderr = render(output.stderr);

                let stdout = stdout.replace("git_head()", "HEAD@git");
                
                rendered.push('$');
                rendered.push_str(command);
                rendered.push('\n');
                rendered.push_str(&stdout);
                rendered.push_str(&stderr);
                if !stdout.is_empty() || !stderr.is_empty() {
                    rendered.push('\n');
                }

            }

            eprintln!("===output: {rendered}");

            rendered
        }).to_string()
    }
}

fn run_examples(content: &str) -> Result<String> {
    let mut buf = content.to_string();
    let regex = Regex::new(r#"\{\{#trycmdinclude ([\w\/.\-]+):(\d+)(?::(\d+))?\}\}"#).unwrap();

    let mut matches: Vec<_> = regex.captures_iter(content).map(|cap| {
        let m = cap.get(0).unwrap();

        let path = cap.get(1).unwrap();

        Match {
            contents: CACHE.render(path.as_str()),
            pos_start: m.start(),
            pos_end: m.end(),
            start: cap.get(2).map(|c| c.as_str().parse().unwrap()),
            end: cap.get(3).map(|c| c.as_str().parse().unwrap()),
        }
    }).collect();

    replace_matches(&mut buf, &mut matches)?;

    Ok(buf)
}

#[derive(Debug)]
struct Match {
    contents: String,
    pos_start: usize,
    pos_end: usize,
    start: Option<u64>,
    end: Option<u64>,
}

impl Match {
    fn get_replacement(&self) -> io::Result<String> {
        let extracted = self.contents
            .lines()
            .enumerate()
            .filter_map(|(i, line)| {
                let line_num = i as u64 + 1;
                if let Some(start) = self.start {
                    if let Some(end) = self.end {
                        if line_num < start || line_num > end {
                            return None;
                        }
                    } else if line_num != start {
                        return None;
                    }
                }
                Some(line)
            })
            .collect::<Vec<_>>()
            .join("\n");

        Ok(extracted)
    }
}

fn replace_matches(input: &mut String, matches: &mut Vec<Match>) -> io::Result<()> {
    // Sort matches by `pos_start` in descending order to avoid index shifts
    matches.sort_by(|a, b| b.pos_start.cmp(&a.pos_start));

    for m in matches {
        let replacement = m.get_replacement()?;
        let replacement = format!("<pre><code class='language-bash hljs'>{replacement}</code></pre>");
        input.replace_range(m.pos_start..m.pos_end, &replacement);
    }

    Ok(())
}