mod torrent_info;
mod tracker;
use std::env::current_dir;

fn main() {
    let path = current_dir().unwrap().join("debian.torrent");
    let file_url_str = format!("file://{}", path.to_str().unwrap());

    let t = torrent_info::get_torrent_info(&file_url_str).unwrap();

    tracker::get_tracker_info(&t, &tracker::TrackerEvent::Started);
}
