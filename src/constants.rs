/// Constants for the bot.
///
/// They are mostly about Mirishita and THE IDOLM@STER and probably no
/// practical use for whoever only care about the Rust side.
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    /// This is a mapping between internal ID and idol name. The names are
    /// all canonical Japanese names with no space between first and last
    /// names.
    pub static ref IDOL_ID_MAP: HashMap<u32, &'static str> = {
        let mut m = HashMap::new();
        m.insert(1, "天海春香");
        m.insert(2, "如月千早");
        m.insert(3, "星井美希");
        m.insert(4, "萩原雪歩");
        m.insert(5, "高槻やよい");
        m.insert(6, "菊地真");
        m.insert(7, "水瀬伊織");
        m.insert(8, "四条貴音");
        m.insert(9, "秋月律子");
        m.insert(10, "三浦あずさ");
        m.insert(11, "双海亜美");
        m.insert(12, "双海真美");
        m.insert(13, "我那覇響");
        m.insert(14, "春日未来");
        m.insert(15, "最上静香");
        m.insert(16, "伊吹翼");
        m.insert(17, "田中琴葉");
        m.insert(18, "島原エレナ");
        m.insert(19, "佐竹美奈子");
        m.insert(20, "所恵美");
        m.insert(21, "徳川まつり");
        m.insert(22, "箱崎星梨花");
        m.insert(23, "野々原茜");
        m.insert(24, "望月杏奈");
        m.insert(25, "ロコ");
        m.insert(26, "七尾百合子");
        m.insert(27, "高山紗代子");
        m.insert(28, "松田亜利沙");
        m.insert(29, "高坂海美");
        m.insert(30, "中谷育");
        m.insert(31, "天空橋朋花");
        m.insert(32, "エミリー");
        m.insert(33, "北沢志保");
        m.insert(34, "舞浜歩");
        m.insert(35, "木下ひなた");
        m.insert(36, "矢吹可奈");
        m.insert(37, "横山奈緒");
        m.insert(38, "二階堂千鶴");
        m.insert(39, "馬場このみ");
        m.insert(40, "大神環");
        m.insert(41, "豊川風花");
        m.insert(42, "宮尾美也");
        m.insert(43, "福田のり子");
        m.insert(44, "真壁瑞希");
        m.insert(45, "篠宮可憐");
        m.insert(46, "百瀬莉緒");
        m.insert(47, "永吉昴");
        m.insert(48, "北上麗花");
        m.insert(49, "周防桃子");
        m.insert(50, "ジュリア");
        m.insert(51, "白石紬");
        m.insert(52, "桜守歌織");
        m
    };
}

lazy_static! {
    /// Event types.
    ///
    /// Check <https://api.matsurihi.me/docs/#mltd-v1-events>.
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

pub const BOT_TOKEN: &str = include_str!("../conf/BOT_TOKEN");
