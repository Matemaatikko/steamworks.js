use napi::bindgen_prelude::BigInt;
use napi_derive::napi;
use steamworks::LeaderboardScoreUploaded;

//////////////////////////////////////////////////

#[napi(object)]
#[derive(Clone)]
pub struct UploadResponse0 {
    pub score: i32,
    pub was_changed: bool,
    pub global_rank_new: i32,
    pub global_rank_previous: i32,
}
#[napi(object)]
pub struct UploadResponse {
    pub res: Option<UploadResponse0>,
    pub msg: String,
}

impl From<LeaderboardScoreUploaded> for UploadResponse0 {
    fn from(elem: LeaderboardScoreUploaded) -> Self {
        UploadResponse0 {
            score: elem.score,
            was_changed: elem.was_changed,
            global_rank_new: elem.global_rank_new,
            global_rank_previous: elem.global_rank_previous,
        }
    }
}

//////////////////////////////////////////////////

#[napi(object)]
pub struct LeaderboardEntry0 {
    pub user_steam_id: BigInt,
    pub user_name: String,
    pub global_rank: i32,
    pub score: i32,
    pub details: Vec<i32>,
}

#[napi(object)]
pub struct LeaderboardResponse {
    pub entries: Option<Vec<LeaderboardEntry0>>,
    pub msg: String,
}

#[napi(object)]
pub struct RequestLeaderboard {
    pub name: String,
    pub ensure_created: Option<bool>,
    pub sort_method: Option<u32>, // 0 Ascending, 1 Descending, (Default Ascending)
    pub display_type: Option<u32>, // 0 Numeric, 1 TimeSeconds, 2 TimeMilliSeconds, (Default Numeric)
}

//////////////////////////////////////////////////

#[napi]
pub mod leaderboard {

    use napi::bindgen_prelude::BigInt;
    use steamworks::{
        Leaderboard, LeaderboardDataRequest, LeaderboardDisplayType, LeaderboardSortMethod,
        UploadScoreMethod,
    };
    use tokio::sync::oneshot;

    use super::*;

    #[napi]
    pub async fn upload(
        leaderboard: RequestLeaderboard,
        score: i32,
        details: Vec<i32>,
        upload_score_method: u32, // 0 KeepBest, 1 ForceUpdate
    ) -> UploadResponse {
        let _method = upload_score_method_from(upload_score_method);

        let _leaderboard = match resolve_leaderboard(leaderboard).await {
            Ok(Some(l)) => l,
            Ok(None) => {
                return UploadResponse {
                    res: None,
                    msg: "Leaderboard does not exist".to_string(),
                }
            }
            Err(e) => return UploadResponse { res: None, msg: e },
        };

        let client = crate::client::get_client();

        let (tx, rx) = oneshot::channel();

        client.user_stats().upload_leaderboard_score(
            &_leaderboard,
            _method,
            score,
            &details,
            |result| {
                tx.send(result).unwrap();
            },
        );

        let result = rx.await.unwrap();

        match result {
            Ok(Some(r)) => UploadResponse {
                res: Some(UploadResponse0::from(r)),
                msg: "Successful upload".to_string(),
            },
            Ok(None) => UploadResponse {
                res: None,
                msg: ("Upload response None. ".to_string() + &_leaderboard.raw().to_string()),
            },
            Err(e) => UploadResponse {
                res: None,
                msg: e.to_string(),
            },
        }
    }

    #[napi]
    pub async fn get_user_leaderboard_data(
        leaderboard: RequestLeaderboard,
        max_details_len: u32,
    ) -> LeaderboardResponse {
        get_leaderboard_data(leaderboard, 0, 0, max_details_len, 1).await
    }

    #[napi]
    pub async fn get_leaderboard_data(
        leaderboard: RequestLeaderboard,
        start: u32,
        end: u32,
        max_details_len: u32,
        leaderboard_data_request: u32, //0 Global, 1 GlobalAroundUser, 2 Friends
    ) -> LeaderboardResponse {
        let request_type = leaderboard_data_request_from(leaderboard_data_request);

        let _leaderboard = match resolve_leaderboard(leaderboard).await {
            Ok(Some(l)) => l,
            Ok(None) => {
                return LeaderboardResponse {
                    entries: None,
                    msg: "Leaderboard does not exist".to_string(),
                }
            }
            Err(e) => {
                return LeaderboardResponse {
                    entries: None,
                    msg: e,
                }
            }
        };

        let client = crate::client::get_client();

        let (tx, rx) = oneshot::channel();
        client.user_stats().download_leaderboard_entries(
            &_leaderboard,
            request_type,
            start as usize,
            end as usize,
            max_details_len as usize,
            |result| {
                tx.send(result).unwrap();
            },
        );

        let result = rx.await.unwrap();
        match result {
            Ok(leaderboard) => {
                let entries = leaderboard
                    .iter()
                    .map(|l| LeaderboardEntry0 {
                        user_steam_id: BigInt::from(l.user.raw()),
                        user_name: client.friends().get_friend(l.user).name(),
                        global_rank: l.global_rank,
                        score: l.score,
                        details: l.details.clone(),
                    })
                    .collect::<Vec<LeaderboardEntry0>>();

                LeaderboardResponse {
                    entries: Some(entries),
                    msg: "Succesful Leaderboard retrieval".to_string(),
                }
            }
            Err(e) => LeaderboardResponse {
                entries: None,
                msg: e.to_string(),
            },
        }
    }

    ////////////////////////////////////////////

    fn upload_score_method_from(v: u32) -> UploadScoreMethod {
        match v {
            0 => UploadScoreMethod::KeepBest,
            1 => UploadScoreMethod::ForceUpdate,
            _ => UploadScoreMethod::KeepBest,
        }
    }

    fn leaderboard_data_request_from(v: u32) -> LeaderboardDataRequest {
        match v {
            0 => LeaderboardDataRequest::Global,
            1 => LeaderboardDataRequest::GlobalAroundUser,
            2 => LeaderboardDataRequest::Friends,
            _ => LeaderboardDataRequest::Global,
        }
    }

    async fn resolve_leaderboard(
        leaderboard: RequestLeaderboard,
    ) -> Result<Option<Leaderboard>, String> {
        if leaderboard.ensure_created.unwrap_or(false) {
            find_or_create_leaderboard(leaderboard).await
        } else {
            find_leaderboard(leaderboard.name).await
        }
    }

    async fn find_leaderboard(name: String) -> Result<Option<Leaderboard>, String> {
        let client = crate::client::get_client();

        let (tx, rx) = oneshot::channel();

        client.user_stats().find_leaderboard(&name, |result| {
            tx.send(result).unwrap();
        });

        let result = rx.await.unwrap();

        match result {
            Ok(l) => Ok(l),
            Err(e) => Err(e.to_string()),
        }
    }

    async fn find_or_create_leaderboard(
        leaderboard: RequestLeaderboard,
    ) -> Result<Option<Leaderboard>, String> {
        let client = crate::client::get_client();

        let (tx, rx) = oneshot::channel();

        let sort_method = match leaderboard.sort_method {
            Some(0) => LeaderboardSortMethod::Ascending,
            Some(1) => LeaderboardSortMethod::Descending,
            _ => LeaderboardSortMethod::Ascending,
        };

        let display_type = match leaderboard.display_type {
            Some(0) => LeaderboardDisplayType::Numeric,
            Some(1) => LeaderboardDisplayType::TimeSeconds,
            Some(2) => LeaderboardDisplayType::TimeMilliSeconds,
            _ => LeaderboardDisplayType::Numeric,
        };

        client.user_stats().find_or_create_leaderboard(
            &leaderboard.name,
            sort_method,
            display_type,
            |result| {
                tx.send(result).unwrap();
            },
        );

        let result = rx.await.unwrap();

        match result {
            Ok(l) => Ok(l),
            Err(e) => Err(e.to_string()),
        }
    }
}
