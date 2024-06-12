pub mod api {
    use reqwest::Client;
    use serde::Deserialize;
    use serde_json::json;
    use std::env;

    use crate::error::error::{Error, AppError};

    #[derive(Deserialize)]
    struct PinnedRepositoriesResponse {
        data: Option<Data>,
        errors: Option<Vec<String>>,
    }

    #[derive(Deserialize)]
    struct Data {
        user: User,
    }

    #[derive(Deserialize)]
    struct User {
        pinnedItems: PinnedItems,
    }

    #[derive(Deserialize)]
    struct PinnedItems {
        nodes: Vec<Repository>,
    }

    #[derive(Deserialize)]
    pub struct Repository {
        pub name: String,
        pub url: String,
        pub description: Option<String>,
    }

    pub async fn get_repos() -> Result<Vec<Repository>, Error> {
        let github_token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not set");
        let github_username = env::var("GITHUB_USERNAME").expect("GITHUB_USERNAME not set");
        let query_string = format!(r#"
        query {{
            user(login: "{}") {{
                pinnedItems(first: 6, types: REPOSITORY) {{
                    nodes {{
                        ... on Repository {{
                            name
                            description
                            url
                        }}
                    }}
                }}
            }}
        }}
        "#, github_username);

        let query = json!({
            "query": query_string,
        });

        let client = Client::new();
        let res = client
            .post("https://api.github.com/graphql")
            .bearer_auth(github_token)
            .header("User-Agent", "my-rust-app")
            .json(&query)
            .send()
            .await;

        let res = match res {
            Ok(res) => res,
            Err(err) => {
                return Err(Error::new(AppError::ApiRequestSendError, err.to_string()))
            }
        };

        if res.status().is_success() {
            let response_body: PinnedRepositoriesResponse = match res.json().await {
                Ok(res) => res,
                Err(err) => {
                    return Err(Error::new(AppError::ApiGetReposJsonError, err.to_string()))
                }
            };

            if let Some(data) = response_body.data {
                let repos = data.user.pinnedItems.nodes;
                return Ok(repos);
            } 
            else {
                let err_text = response_body.errors.unwrap().join("\n");
                Err(Error::new(AppError::ApiGetResponseError, err_text))
            }
        } 
        else {
            Err(Error::new(AppError::ApiGetResponseError, String::from("Error getting the response")))
        }
    }
}