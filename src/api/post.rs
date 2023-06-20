use lemmy_api_common::{post::{GetPost, GetPostResponse, PostResponse, CreatePost}, lemmy_db_schema::{newtypes::{PostId, CommunityId}, CommentSortType, ListingType}, comment::{GetComments, GetCommentsResponse}, lemmy_db_views::structs::CommentView};

use crate::util;

pub fn get_post(id: PostId) -> std::result::Result<GetPostResponse, reqwest::Error> {
    let params = GetPost {
        id: Some(id),
        ..Default::default()
    };

    super::get("/post", &params)
}

pub fn get_comments(post_id: PostId) -> std::result::Result<Vec<CommentView>, reqwest::Error> {
    let params = GetComments {
        post_id: Some(post_id),
        sort: Some(CommentSortType::Hot),
        type_: Some(ListingType::All),
        ..Default::default()
    };

    let mut comments = super::get::<GetCommentsResponse, _>("/comment/list", &params)?.comments;

    // hide removed and deleted comments
    comments.retain(|c| !c.comment.deleted && !c.comment.removed);

    Ok(comments)
}

pub fn default_post() -> GetPostResponse {
    serde_json::from_str(include_str!("../examples/post.json")).unwrap()
}

pub fn create_post(
    name: String,
    body: String,
    community_id: i32,
) -> Result<PostResponse, reqwest::Error> {
    let params = CreatePost {
        name,
        body: Some(body),
        community_id: CommunityId(community_id),
        auth: util::get_auth_token().unwrap(),
        ..Default::default()
    };
    super::post("/post", &params)
}
