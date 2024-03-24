use percent_encoding::{AsciiSet, NON_ALPHANUMERIC};
use serde_derive::{Serialize, Deserialize};
use std::error::Error;

use crate::torrent_info::TorrentInfo;

pub enum TrackerEvent {
    Started,
    Stopped,
}

impl TrackerEvent {
    fn as_str(&self) -> &'static str {
        match self {
            TrackerEvent::Started => "started",
            TrackerEvent::Stopped => "stopped",
        }
    }
}

#[derive(Serialize)]
struct TrackerQueryParams {
    peer_id: String,
    uploaded: u64,
    downloaded: u64,
    event: String,
    compact: u8,
    left: u64,
}

const URL_ENCODE_RESERVED: &AsciiSet = &NON_ALPHANUMERIC
    .remove(b'-')
    .remove(b'_')
    .remove(b'~')
    .remove(b'.');

pub fn build_tracker_url(t: &TorrentInfo, e: &TrackerEvent) -> String {
    let info_hash = percent_encoding::percent_encode(&t.info_hash, URL_ENCODE_RESERVED).collect::<String>();

    let query = serde_urlencoded::to_string(TrackerQueryParams {
        peer_id: t.tracker_info.peer_id.clone(),
        uploaded: t.tracker_info.uploaded,
        downloaded: t.tracker_info.downloaded,
        left: t.tracker_info.left,
        event: e.as_str().to_string(),
        compact: 1,
    })
    .unwrap();

    let mut u2 = t.announce.to_owned();
    u2.set_query(Some(query.as_str()));

    // bad for the reasons described here:
    // https://github.com/vimpunk/cratetorrent/blob/34aa13835872a14f00d4a334483afff79181999f/cratetorrent/src/tracker.rs#L188
    format!("{u2}&info_hash={info_hash}")
}

#[derive(Deserialize)]
struct TrackerResponse {
    failure_reason: String,
    warning_message: String,
    interval: u64,
    min_interval: u64,
    tracker_id: String,
    complete: u64,   // seeders
    incomplete: u64, // leechers
    //peers: Vec<u8>,
}

pub fn get_tracker_info(t: &TorrentInfo, e: &TrackerEvent) -> Result<TrackerResponse, Box<dyn Error>> {
    let url = build_tracker_url(t, e);
    println!("{}", url);

    let client = reqwest::blocking::Client::new();
    let result = client.get(url).send()?;
    let response_body = result.bytes()?;
    serde_bencode::from_bytes(&response_body)
}

