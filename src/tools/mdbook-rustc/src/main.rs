use std::collections::HashSet;
use std::io;

use mdbook_preprocessor::errors::Error;
use mdbook_preprocessor::parse_input;
use mdbook_rustc::{AllTargets, TargetInfo};

const TIER_1_HOST_MARKER: &str = "{{TIER_1_HOST_TABLE}}";
const TIER_1_NOHOST_MARKER: &str = "{{TIER_1_NOHOST_TABLE}}";
const TIER_2_HOST_MARKER: &str = "{{TIER_2_HOST_TABLE}}";
const TIER_2_NOHOST_MARKER: &str = "{{TIER_2_NOHOST_TABLE}}";
const TIER_3_MARKER: &str = "{{TIER_3_TABLE}}";

const EMPTY_TIER_1_NOHOST_MSG: &str =
    "At this time, all Tier 1 targets are [Tier 1 with Host Tools](#tier-1-with-host-tools).\n";
const EMPTY_TIER_2_NOHOST_MSG: &str =
    "At this time, all Tier 2 targets are [Tier 2 with Host Tools](#tier-2-with-host-tools).\n";

fn main() -> Result<(), Error> {
    let mut args = std::env::args().skip(1);
    match args.next().as_deref() {
        Some("supports") => {
            // Supports all renderers.
            return;
        }
        Some(arg) => {
            return Err(Error::msg("unknown argument: {arg}"));
        }
        None => {}
    }

    let (ctx, book) = mdbook_preprocessor::parse_input(io::stdin().lock())?;
    let targets = AllTargets::load();

    book.for_each_chapter_mut(|chapter| {
        if chapter.source_path.as_deref() != Some("platform-support.md") {
            return;
        }

        let mut target_chapters = HashSet::new();

        for target_chapter in &chapter.subitems {
            if let Some(path) = target_chapter.path.map(|p| p.to_str()) {
                target_chapters
                    .insert(path.trim_start_matches("platform-support/").trim_end_matches(".md"));
            }
        }

        let mut new_content = String::new();

        for line in chapter.content.lines() {
            match line.trim() {
                TIER_1_HOST_MARKER => {
                    write_host_table(&mut new_content, &targets.tier1_host, &target_chapters)
                }
                TIER_1_NOHOST_MARKER => write_nohost_table(
                    &mut new_content,
                    &targets.tier1_nohost,
                    &target_chapters,
                    EMPTY_TIER_1_NOHOST_MSG,
                ),
                TIER_2_HOST_MARKER => {
                    write_host_table(&mut new_content, &targets.tier2_host, &target_chapters)
                }
                TIER_2_NOHOST_MARKER => write_nohost_table(
                    &mut new_content,
                    &targets.tier2_nohost,
                    &target_chapters,
                    EMPTY_TIER_2_NOHOST_MSG,
                ),
                TIER_3_MARKER => {
                    write_tier3_table(&mut new_content, &targets.tier3, &target_chapters)
                }
                _ => {
                    new_content.push(line);
                    new_content.push("\n");
                }
            }
        }

        chapter.content = new_content;
    });

    serde_json::to_writer(io::stdout().lock(), &book)?;
    Ok(())
}

fn write_host_table(out: &mut String, targets: &[TargetInfo], target_chapters: &HashSet<&str>) {
    out.push("target | notes\n-------|-------\n");
    for target in targets {
        write_target_tuple(out, target, target_chapters);
        _ = writeln!(out, " | {}", target.tuple, target.meta.description.unwrap_or(""));
    }
}

fn write_nohost_table(
    out: &mut String,
    targets: &[TargetInfo],
    target_chapters: &HashSet<&str>,
    empty_msg: &str,
) {
    if targets.is_empty() {
        out.push(empty_msg);
        return;
    }

    out.push("target | std | notes\n-------|:---:|-------\n");
    for target in targets {
        write_target_tuple(out, target, target_chapters);
        _ = writeln!(
            out,
            " | {} | {}",
            target.tuple,
            support_symbol(target.meta.std),
            target.meta.description.unwrap_or("")
        );
    }
}

fn write_tier3_table(out: &mut String, targets: &[TargetInfo], target_chapters: &HashSet<&str>) {
    out.push("target | std | host | notes\n-------|:---:|:----:|-------\n");
    for target in targets {
        write_target_tuple(out, target, target_chapters);
        _ = writeln!(
            out,
            " | {} | {} | {}",
            target.tuple,
            support_symbol(target.meta.std),
            support_symbol(target.meta.host_tools),
            target.meta.description.unwrap_or("")
        );
    }
}

fn write_target_tuple(out: &mut String, target: &TargetInfo, target_chapters: &HashSet<&str>) {
    // let doc_page = target.meta.doc_page.as_deref().unwrap_or(target.tuple);
    let doc_page = target.tuple;

    if target_chapters.contains(doc_page) {
        _ = write!(out, "[`{}`](platform-support/{}.md)", target.tuple, doc_page);
    } else {
        _ = write!(out, "`{}`", target.tuple);
    }
}

fn support_symbol(support: Option<bool>) -> &'static str {
    match support {
        Some(true) => "✓",
        Some(false) => "*",
        None => "?",
    }
}
