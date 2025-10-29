use color_eyre::eyre::Result;
use indicatif::{MultiProgress, ProgressStyle};

pub fn new_multi_progress() -> Result<(MultiProgress, ProgressStyle)> {
    let multi_progress = MultiProgress::new();
    let sty = ProgressStyle::default_bar()
        .template(
            "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta}) {msg}",
        )?
        .progress_chars("#>-");

    Ok((multi_progress, sty))
}
