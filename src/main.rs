use warp::Filter;

fn routes() -> impl Filter<Extract = (String,), Error = warp::Rejection> + Copy {
    warp::any()
        .and(warp::header::<String>("Authorization"))
        .and_then(|v: String| async move {
            match v.strip_prefix("Bearer ") {
                Some(t) => Ok(t.to_string()),
                _ => Err(warp::reject::not_found()),
            }
        })
        .and_then(|t: String| async move {
            match t.len() {
                1 => Ok(t.to_string()),
                _ => Err(warp::reject::not_found()),
            }
        })
        .map(|auth| auth)
}

#[tokio::main]
async fn main() {
    warp::serve(routes()).run(([127, 0, 0, 1], 3030)).await;
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_missing_header() {
        assert!(warp::test::request().filter(&routes()).await.is_err());
    }

    #[tokio::test]
    async fn test_auth_header() {
        let t = "1";
        let v = warp::test::request().header("Authorization", format!("Bearer {}", t)).filter(&routes()).await.unwrap();
        assert_eq!(v, t);
    }
}
