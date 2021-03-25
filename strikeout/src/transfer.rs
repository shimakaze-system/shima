use crate::error::{Error, Result};
use regex::Regex;
use std::{fmt, lazy::SyncLazy, path::Path};

static EPI_PAT: SyncLazy<Regex> = SyncLazy::new(|| Regex::new(r#"^(\d+)([Vv]\d+)?$"#).unwrap());

pub fn trans(input: &Path) -> Result<String> {
    let file_name = input
        .file_stem()
        .map(|s| s.to_str())
        .flatten()
        .ok_or(Error::InvalidUnicodeFilename)?;
    let episode = find_episode_num(file_name)?;
    let mut name = Name::new(episode, 1).to_string();
    if let Some(extension) = input.extension().map(|s| s.to_str()).flatten() {
        name.push('.');
        name.push_str(extension);
    }
    Ok(name)
}

fn find_episode_num(input: &str) -> Result<u16> {
    let blocks: Vec<&str> = input
        .split(|c| c == '[' || c == ']' || c == ' ' || c == '【' || c == '】')
        .filter(|s| !s.is_empty())
        .collect();
    for block in blocks {
        // deal with /d+v\d+
        if let Some(caps) = EPI_PAT.captures(block) {
            // should never panic
            return Ok(caps.get(1).unwrap().as_str().parse().unwrap());
        }
    }
    Err(Error::EpisodeNotFound)
}

struct Name {
    episode: u16,
    season: u16,
    episode_bit: usize,
}

impl Name {
    fn new(episode: u16, season: u16) -> Self {
        Self {
            episode,
            season,
            episode_bit: 2,
        }
    }
}

impl fmt::Display for Name {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut episode_formatted = self.episode.to_string();
        if episode_formatted.len() < self.episode_bit {
            let zeros = self.episode_bit - episode_formatted.len();
            let zeros: String = (0..zeros).into_iter().map(|_| "0").collect();
            episode_formatted.insert_str(0, &zeros);
        }
        let season_formatted = if self.season > 10 {
            self.season.to_string()
        } else {
            format!("0{}", self.season)
        };
        let name = ["S", &season_formatted, "E", &episode_formatted].concat();
        formatter.write_str(&name)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn trans_1() {
        let input = Path::new("[Munou na Nana][06][BIG5][1080P].mp4");
        assert_eq!(trans(input).unwrap(), "S01E06.mp4".to_owned());
    }

    #[test]
    fn trans_2() {
        let input = Path::new("[SweetSub&LoliHouse] Akudama Drive - 05 [WebRip 1080p HEVC-10bit AAC ASSx2].mkv");
        assert_eq!(trans(input).unwrap(), "S01E05.mkv".to_owned());
    }

    #[test]
    fn trans_3() {
        let input = Path::new("1.text");
        assert_eq!(trans(input).unwrap(), "S01E01.text".to_owned());
    }

    #[test]
    fn trans_4() {
        let input = Path::new("[SweetSub&LoliHouse] Akudama Drive - 05v2 [WebRip 1080p HEVC-10bit AAC ASSx2].mkv");
        assert_eq!(trans(input).unwrap(), "S01E05.mkv".to_owned());
    }
}
