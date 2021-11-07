use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    // https://api.matsurihi.me/docs/#mltd-v1-events
    pub static ref EVENT_TYPE_MAP: HashMap<u32, &'static str> = {
        let mut m = HashMap::new();
        m.insert(1 ,"THEATER SHOW TIME☆");
        m.insert(2 ,"ミリコレ！");
        m.insert(3 ,"プラチナスターシアター・トラスト");
        m.insert(4 ,"プラチナスターツアー");
        m.insert(5 ,"周年記念イベント");
        m.insert(6 ,"MILLION LIVE WORKING☆");
        m.insert(7 ,"エイプリルフール");
        m.insert(9 ,"ミリコレ！（ボックスガシャ）");
        m.insert(10,"ツインステージ");
        m.insert(11,"プラチナスターチューン");
        m.insert(12,"ツインステージ 2");
        m.insert(13,"プラチナスターテール");
        m.insert(14,"THEATER TALK PARTY☆");
        m.insert(16,"プラチナスタートレジャー");
        m
    };
}

pub const BOT_TOKEN: &str = "UID:TOKEN";
