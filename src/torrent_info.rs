use serde_derive::{Deserialize, Serialize};
use sha1::{Digest, Sha1};
use std::fs;
use std::{error::Error, fmt};
use std::io::{self, Write};
use url::{ParseError, Url};
use hex;

#[derive(Deserialize)]
struct TorrentFile {
    announce: String,
    info: TorrentFileInfo
}

#[derive(Deserialize, Serialize)]
struct TorrentFileInfo {
    #[serde(with = "serde_bytes")]
    pieces: Vec<u8>,
    #[serde(alias = "piece length", rename = "piece length")]
    piece_length: u64,
    length: u64,
    name: String,
}

#[derive(Debug, Clone)]
struct InvalidSchemeError;

impl fmt::Display for InvalidSchemeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "unsupported scheme. valid schemes are http, https, and file"
        )
    }
}

#[derive(Debug)]
pub struct TorrentInfo {
    pub announce: Url,
    pub info_hash: [u8; 20],
    pub name: String,
    pub pieces: Vec<[u8; 20]>,
    pub piece_length: u64,
    pub length: u64,
    pub tracker_info: TrackerInfo,
}

#[derive(Debug)]
pub struct TrackerInfo {
    pub peer_id: String,
    pub port: u16,
    pub uploaded: u64,
    pub downloaded: u64,
    pub left: u64,
}

pub fn get_torrent_info(url: &str) -> Result<TorrentInfo, Box<dyn Error>> {
    let parsed_url = parse_torrent_url(url)?;
    if parsed_url.scheme() == "file" {
        get_torrent_from_file(parsed_url)
    } else {
        Ok(todo!())
    }
}

fn parse_torrent_url(u: &str) -> Result<Url, ParseError> {
    Url::parse(u)
}

fn get_torrent_from_file(u: Url) -> Result<TorrentInfo, Box<dyn Error>> {
    let path = u.path().to_string();
    let data = fs::read(path)?;
    let re: TorrentFile = serde_bencode::from_bytes(&data)?;

    let announce_url = Url::parse(&re.announce)?;

    let info_bytes = serde_bencode::to_bytes(&re.info)?;

    let mut hasher = Sha1::new();
    hasher.update(&info_bytes);

    let mut info_hash = [0u8; 20];
    let res = hasher.finalize();

    let hexen = hex::encode(res);
    io::stdout().write_all(hexen.as_bytes())?;

    info_hash.copy_from_slice(&res.to_vec());

    let pieces: Vec<[u8; 20]> = re.info
        .pieces
        .chunks_exact(20)
        .map(|f| {
            let mut arr = [0u8; 20];
            arr.copy_from_slice(f);
            arr
        })
        .collect();

    Ok(TorrentInfo {
        announce: announce_url,
        info_hash,
        pieces,
        piece_length: re.info.piece_length,
        length: re.info.length,
        name: re.info.name,
        tracker_info: TrackerInfo {
            peer_id: String::from("ALDPHEAIBNLMLDBTDXIR"),
            port: 80, //todo pass this in
            uploaded: 0,
            downloaded: 0,
            left: re.info.length,
        },
    })
}

#[cfg(test)]
mod tests {
    use url::Url;

    use crate::torrent_info::get_torrent_from_file;

    #[test]
    fn gets_torrent_from_file() {
        let u = Url::parse("file://arch.torrent").unwrap();
        get_torrent_from_file(u).unwrap();
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
