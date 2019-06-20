use std::path::PathBuf;
use std::env;

use inotify::{
    WatchMask,
    Inotify,
};

fn main()
{
    let mut inotify = Inotify::init()
        .expect("Failed to initialize inotify");

    // TODO: get the path from commandline, add some help under --help and -h
    // TODO: update scripts
    
    let path = PathBuf::from("/nix/var/nix/profiles/per-user/allgreed/profile");

    inotify
        .add_watch(
            path,
            WatchMask::DELETE_SELF | WatchMask::DONT_FOLLOW,
        )
        .expect("inotify watch added sucessfully");

    let mut buffer = [0u8; 4096];

    inotify
        .read_events_blocking(&mut buffer)
        .expect("inotify events read sucessfully");
}
