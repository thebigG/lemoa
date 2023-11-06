use lemmy_api_common::{
    comment::{
        CommentReportResponse, CommentResponse, CreateComment, CreateCommentLike,
        CreateCommentReport, DeleteComment, EditComment, SaveComment,
    },
    lemmy_db_schema::newtypes::{CommentId, PostId},
};

use crate::settings;

pub fn create_comment(
    post_id: PostId,
    content: String,
    parent_id: Option<CommentId>,
) -> Result<CommentResponse, reqwest::Error> {
    let params = CreateComment {
        post_id,
        content,
        parent_id,
        auth: settings::get_current_account().jwt.unwrap(),
        ..Default::default()
    };
    super::post("/comment", &params)
}

// see posts.rs for possible score parameters
pub fn like_comment(comment_id: CommentId, score: i16) -> Result<CommentResponse, reqwest::Error> {
    let params = CreateCommentLike {
        comment_id,
        score,
        auth: settings::get_current_account().jwt.unwrap(),
    };
    super::post("/comment/like", &params)
}

pub fn edit_comment(
    body: String,
    comment_id: CommentId,
) -> Result<CommentResponse, reqwest::Error> {
    let params = EditComment {
        content: Some(body),
        comment_id,
        auth: settings::get_current_account().jwt.unwrap(),
        ..Default::default()
    };
    super::put("/post", &params)
}

pub fn delete_comment(comment_id: CommentId) -> Result<CommentResponse, reqwest::Error> {
    let params = DeleteComment {
        comment_id,
        deleted: true,
        auth: settings::get_current_account().jwt.unwrap(),
    };
    super::post("/comment/delete", &params)
}

pub fn save_comment(comment_id: CommentId, save: bool) -> Result<CommentResponse, reqwest::Error> {
    let params = SaveComment {
        auth: settings::get_current_account().jwt.unwrap(),
        comment_id,
        save,
    };
    super::put("/comment/save", &params)
}

pub fn report_comment(
    comment_id: CommentId,
    reason: String,
) -> Result<CommentReportResponse, reqwest::Error> {
    let params = CreateCommentReport {
        comment_id,
        reason,
        auth: settings::get_current_account().jwt.unwrap(),
    };
    super::post("/comment/report", &params)
}
