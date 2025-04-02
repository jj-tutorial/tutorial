use std::io;

use mdbook::errors::Result;
use mdbook::preprocess::CmdPreprocessor;
use mdbook::BookItem;
use regex::Regex;

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
        match insert_trycmd_output(&chapter.content) {
            Ok(new_content) => chapter.content = new_content,
            Err(e) => eprintln!("could not process chapter: {e}"),
        }
    });

    serde_json::to_writer(io::stdout().lock(), &book)?;

    Ok(())
}

fn insert_trycmd_output(content: &str) -> Result<String> {
    let mut buf = content.to_string();
    let regex = Regex::new(r#"\{\{#trycmdinclude ([\w\/.\-]+):(\d+)(?::(\d+))?\}\}"#).unwrap();

    let mut matches: Vec<_> = regex
        .captures_iter(content)
        .map(|cap| {
            let m = cap.get(0).unwrap();

            let path = cap.get(1).unwrap();

            let contents = std::fs::read_to_string(path.as_str())?;
            let contents = replace_colors(contents);
            let contents = ansi_to_html::Converter::new()
                .four_bit_var_prefix(Some("jj-".to_string()))
                .convert(&contents)?;

            Ok(Match {
                contents,
                pos_start: m.start(),
                pos_end: m.end(),
                start: cap.get(2).map(|c| c.as_str().parse().unwrap()),
                end: cap.get(3).map(|c| c.as_str().parse().unwrap()),
            })
        })
        .collect::<Result<Vec<Match>>>()?;

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
        let extracted = self
            .contents
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
        let replacement = format!("<pre><code class='nohighlight'>{replacement}</code></pre>");
        input.replace_range(m.pos_start..m.pos_end, &replacement);
    }

    Ok(())
}

/// replace 256 color output with 16 color output
fn replace_colors(input: String) -> String {
    let re = Regex::new(r"\x1b\[38;5;([0-9]+)m").unwrap();

    re.replace_all(&input, |caps: &regex::Captures| {
        if let Ok(num) = caps[1].parse::<u8>() {
            let replacement = match num {
                0..=7 => 30 + num,        // Standard foreground colors
                8..=15 => 90 + (num - 8), // Bright foreground colors
                code => {
                    eprintln!("non-16 color found: {code}");
                    return caps[0].to_string(); // Keep unchanged if out of
                                                // range
                }
            };
            format!("\x1b[{}m", replacement)
        } else {
            caps[0].to_string()
        }
    })
    .to_string()
}
