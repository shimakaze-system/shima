use crate::error::{Error, Result};
use std::{fmt, path::Path};

pub fn trans(input: &Path) -> Result<String> {
    let file_name = input
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or(Error::InvalidUnicodeFilename)?;
    // use input as anime name
    let anime = input
        .parent()
        .and_then(|p| p.file_name())
        .and_then(|anime| anime.to_str());
    let episode = find_episode_num(file_name)?;
    let mut name = if let Some(anime) = anime {
        format!("{} {}", anime, episode)
    } else {
        // use S01E** as a fallback
        Name::new(episode, 1).to_string()
    };
    if let Some(extension) = input.extension().map(|s| s.to_str()).flatten() {
        name.push('.');
        name.push_str(extension);
    }
    Ok(name)
}

mod group_rule {
    use super::{Error, Result};
    use once_cell::sync::Lazy;
    use regex::{Regex, RegexSet};

    static RSET: Lazy<RegexSet> =
        Lazy::new(|| RegexSet::new(&[r"Lilith-Raws", r"SweetSub", r"NC-Raws"]).expect("Fail to create regex set"));
    static EPI_PAT: Lazy<Regex> = Lazy::new(|| Regex::new(r#"^(\d+)([Vv]\d+)?$"#).unwrap());
    static LILITH_RAWS_PAT: Lazy<Regex> = Lazy::new(|| Regex::new(r"\[.*?\] .*?(\d+)([vV]\d)? (\[.*?\])+").unwrap());

    pub(crate) fn find_episode_num(input: &str) -> Result<u16> {
        let first = RSET.matches(input).into_iter().next();
        match first {
            // lilith_raws
            Some(0) => fallback_to_default(input, lilith_raws),
            // SweetSub has same name pattern with lilith-raws
            Some(1) => fallback_to_default(input, lilith_raws),
            // NC-Raws has same name pattern with lilith-raws
            Some(2) => fallback_to_default(input, lilith_raws),
            _ => default(input),
        }
    }

    fn fallback_to_default<F: Fn(&str) -> Result<u16>>(input: &str, handle: F) -> Result<u16> {
        match handle(input) {
            Ok(epi) => Ok(epi),
            Err(_) => default(input),
        }
    }

    fn lilith_raws(input: &str) -> Result<u16> {
        let caps = LILITH_RAWS_PAT.captures(input).ok_or(Error::EpisodeNotFound)?;
        if let Some(epi) = caps.get(1) {
            return Ok(epi.as_str().parse().expect("should always success"));
        }
        Err(Error::EpisodeNotFound)
    }

    fn default(input: &str) -> Result<u16> {
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
}

use group_rule::find_episode_num;

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

    #[test]
    fn trans_5() {
        let input =
            Path::new("Akudama Drive/[SweetSub&LoliHouse] Akudama Drive - 05 [WebRip 1080p HEVC-10bit AAC ASSx2].mkv");
        assert_eq!(trans(input).unwrap(), "Akudama Drive 5.mkv".to_owned());
    }

    #[test]
    fn trans_6() {
        let input = Path::new(
            "86―エイティシックス/[Lilith-Raws] 86 - Eighty Six - 01 [Baha][WEB-DL][1080p][AVC AAC][CHT][MP4].mp4",
        );
        assert_eq!(trans(input).unwrap(), "86―エイティシックス 1.mp4".to_owned());
    }

    #[test]
    fn trans_7() {
        let input = Path::new(
            "86―エイティシックス/[Lilith-Raws] 86 - Eighty Six - 01v2 [Baha][WEB-DL][1080p][AVC AAC][CHT][MP4].mp4",
        );
        assert_eq!(trans(input).unwrap(), "86―エイティシックス 1.mp4".to_owned());
    }

    #[test]
    fn trans_8() {
        let input = Path::new(
            "86―エイティシックス/[SweetSub&LoliHouse] 86 - Eighty Six - 01 [Baha][WEB-DL][1080p][AVC AAC][CHT][MP4].mp4"
        );
        assert_eq!(trans(input).unwrap(), "86―エイティシックス 1.mp4".to_owned());
    }

    #[test]
    fn trans_9() {
        let input = Path::new(
            "86―エイティシックス/[SweetSub&LoliHouse] 86 - Eighty Six - 01v2 [Baha][WEB-DL][1080p][AVC AAC][CHT][MP4].mp4"
        );
        assert_eq!(trans(input).unwrap(), "86―エイティシックス 1.mp4".to_owned());
    }

    #[test]
    fn trans_10() {
        let input = Path::new(
            "迷宮ブラックカンパニー/[NC-Raws] 异世界迷宫黑心企业 - 01 [B-Global][WEB-DL][2160p][AVC AAC][CHS_CHT_ENG_TH_SRT].mkv"
        );
        assert_eq!(trans(input).unwrap(), "迷宮ブラックカンパニー 1.mkv".to_owned());
    }
}
