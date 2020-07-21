use lazy_static::lazy_static;
use scraper::Selector;

pub const AUR_BASE_URL: &str = "https://aur.archlinux.org";

lazy_static! {
    pub static ref VOTE_SELECTOR: Selector =
        Selector::parse(r#"input[name="do_Vote"]"#).expect("failed to init selector");
    pub static ref UNVOTE_SELECTOR: Selector =
        Selector::parse(r#"input[name="do_UnVote"]"#).expect("failed to init selector");
    pub static ref LOGIN_ERROR_SELECTOR: Selector =
        Selector::parse("ul.errorlist li").expect("failed to init selector");
    pub static ref AUR_TOKEN_SELECTOR: Selector =
        Selector::parse(r#"input[name="token"]"#).expect("failed to init selector");
}
