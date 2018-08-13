extern crate mpd;

mod helpers;
use helpers::connect;
use std::borrow::Cow;

#[test]
fn search() {
    let mut mpd = connect();
    // XX: fix this cow business
    let entities = mpd.lsinfo(Cow::Borrowed(""));
    println!("ents: {:#?}", entities);
    assert!(entities.is_ok());

    let entities = entities.unwrap();
    let dir = entities.get(2).unwrap();
    let in_dir = mpd.lsinfo(dir);
    assert!(in_dir.is_ok());
    println!("dir: {:#?}", in_dir);

    assert!(false);
}
