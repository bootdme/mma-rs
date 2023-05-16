use lazy_static::lazy_static;
use scraper::Selector;

lazy_static! {
    pub static ref SELECTOR: Selector = Selector::parse("a").unwrap();
    pub static ref INFO_SELECTOR: Selector = Selector::parse(".fighter-info").unwrap();
    pub static ref NAME_SELECTOR: Selector = Selector::parse("[itemprop='name'] > .fn").unwrap();
    pub static ref NICKNAME_SELECTOR: Selector =
        Selector::parse("[itemprop='name'] > .nickname").unwrap();
    pub static ref IMAGE_URL_SELECTOR: Selector =
        Selector::parse("img.profile-image.photo").unwrap();
    pub static ref AGE_SELECTOR: Selector = Selector::parse("[itemprop='birthDate']").unwrap();
    pub static ref LOCALITY_SELECTOR: Selector =
        Selector::parse("[itemprop='addressLocality']").unwrap();
    pub static ref NATIONALITY_SELECTOR: Selector =
        Selector::parse("strong[itemprop='nationality']").unwrap();
    pub static ref HEIGHT_SELECTOR: Selector = Selector::parse("[itemprop='height']").unwrap();
    pub static ref WEIGHT_SELECTOR: Selector = Selector::parse("[itemprop='weight']").unwrap();
    pub static ref ASSOCIATION_SELECTOR: Selector =
        Selector::parse(".association > [itemprop='name']").unwrap();
    pub static ref WEIGHT_CLASS_SELECTOR: Selector =
        Selector::parse(".association-class > a").unwrap();
    pub static ref WINS_SELECTOR: Selector = Selector::parse(".wins").unwrap();
    pub static ref WINS_TOTAL_SELECTOR: Selector =
        Selector::parse(".win span:nth-child(2)").unwrap();
    pub static ref WINS_BY_SELECTOR: Selector = Selector::parse(".pl").unwrap();
    pub static ref LOSSES_SELECTOR: Selector = Selector::parse(".loses").unwrap();
    pub static ref LOSSES_TOTAL_SELECTOR: Selector =
        Selector::parse(".lose span:nth-child(2)").unwrap();
    pub static ref LOSSES_BY_SELECTOR: Selector = Selector::parse(".pl").unwrap();
    pub static ref NO_CONTESTS_SELECTOR: Selector =
        Selector::parse(".nc span:nth-child(2)").unwrap();
    pub static ref FIGHT_HISTORY_SELECTOR: Selector =
        Selector::parse(".module.fight_history tr:not(.table_head)").unwrap();
    pub static ref RESULT_SELECTOR: Selector =
        Selector::parse("td:nth-child(1) .final_result").unwrap();
    pub static ref OPPONENT_NAME_SELECTOR: Selector = Selector::parse("td:nth-child(2) a").unwrap();
    pub static ref EVENT_SELECTOR: Selector = Selector::parse("td:nth-child(3) a").unwrap();
    pub static ref EVENT_DATE_SELECTOR: Selector =
        Selector::parse("td:nth-child(3) .sub_line").unwrap();
    pub static ref METHOD_SELECTOR: Selector = Selector::parse("td:nth-child(4)").unwrap();
    pub static ref REFEREE_SELECTOR: Selector =
        Selector::parse("td:nth-child(4) .sub_line").unwrap();
    pub static ref ROUND_SELECTOR: Selector = Selector::parse("td:nth-child(5)").unwrap();
    pub static ref TIME_SELECTOR: Selector = Selector::parse("td:nth-child(6)").unwrap();
}
