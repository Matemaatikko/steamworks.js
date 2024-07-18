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

//////////////////////////////////////////////////

#[napi]
pub mod leaderboard {

    use napi::bindgen_prelude::BigInt;
    use steamworks::{Leaderboard, LeaderboardDataRequest};
    use tokio::sync::oneshot;

    use super::*;

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

    #[napi]
    pub async fn upload(name: String, score: i32, details: Vec<i32>) -> UploadResponse {
        let leaderboard = match find_leaderboard(name).await {
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
            &leaderboard,
            steamworks::UploadScoreMethod::KeepBest,
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
                msg: ("Upload response None. ".to_string() + &leaderboard.raw().to_string()),
            },
            Err(e) => UploadResponse {
                res: None,
                msg: e.to_string(),
            },
        }
    }

    //TODO Implement LeaderboardDataRequest -- This can also used to fetch users data (Make separate method for simplicity)
    #[napi]
    pub async fn get_leaderboard(
        name: String,
        start: u32,
        end: u32,
        max_details_len: u32,
    ) -> LeaderboardResponse {
        let leaderboard = match find_leaderboard(name).await {
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
            &leaderboard,
            LeaderboardDataRequest::Global,
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

    //TODO Try to Create Leaderboard -- find_or_create_leaderboard
}
