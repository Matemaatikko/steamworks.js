use napi_derive::napi;

#[napi]
pub mod leaderboard {

    fn from(i: i64) -> steamworks::Leaderboard {
        let mut str = "{ 0: ".to_owned();
        str.push_str(&(i.to_string()));
        str.push_str("}");
        let ld: steamworks::Leaderboard = serde_json::from_str(&str).unwrap();
        ld
    }

    #[napi]
    pub fn upload(leaderboard_id: i64, score: i32, details: Vec<i32>) -> () {
        let client = crate::client::get_client();

        client.user_stats().upload_leaderboard_score(
            &from(leaderboard_id),
            steamworks::UploadScoreMethod::ForceUpdate,
            score,
            &details,
            |_| {},
        );
    }
}
