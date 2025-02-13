use lemmy_api_common::{site::{SearchResponse, Search}, lemmy_db_schema::{SortType, SearchType}};

use crate::components::CLIENT;

pub fn fetch_search(page: i64, query: String, search_type: Option<SearchType>) -> std::result::Result<SearchResponse, reqwest::Error> {
    let params = Search {
        q: query,
        sort: Some(SortType::TopMonth),
        page: Some(page),
        type_: search_type,
        ..Default::default()
    };

    let url = format!("{}/search", super::get_api_url());
    CLIENT.get(&url).query(&params).send()?.json()
}