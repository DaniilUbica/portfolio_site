pub mod api {
    use reqwest::Client;
    use serde::{Deserialize, Serialize};
    use serde_json::json;
    use std::env;

    use crate::error::error::{Error, ApplicationError};
    use crate::error::error::ApiError::*;
    use crate::log;
    use crate::log::LogLevel::*;

    #[derive(Deserialize)]
    struct PinnedRepositoriesResponse {
        data: Option<Data>,
        errors: Option<Vec<String>>,
    }

    #[derive(Deserialize)]
    struct Data {
        user: User,
    }

    #[allow(non_snake_case)]
    #[derive(Deserialize)]
    struct User {
        pinnedItems: PinnedItems,
    }

    #[derive(Deserialize)]
    struct PinnedItems {
        nodes: Vec<Repository>,
    }

    #[derive(Deserialize, Serialize)]
    struct LanguagesResponse {
        nodes: Vec<Language>,
    }

    #[derive(Deserialize, Serialize)]
    struct Language {
        name: String,
    }

    #[allow(non_snake_case)]
    #[derive(Deserialize, Serialize)]
    pub struct Repository {
        pub name: String,
        pub url: String,
        pub description: Option<String>,
        pub stargazerCount: u32,
        pub updatedAt: String,
        languages: LanguagesResponse,
    }

    pub async fn get_repos() -> Result<Vec<Repository>, Error> {
        log!(INFO, "Going to get repos from github");
        let github_token = match env::var("GITHUB_TOKEN") {
            Ok(token) => token,
            Err(err_text) => return Err(Error::new(ApplicationError::ApiError(ApiKeyNotFoundError), err_text.to_string()))
        };
        let github_username = match env::var("GITHUB_USERNAME") {
            Ok(username) => username,
            Err(err_text) => return Err(Error::new(ApplicationError::ApiError(ApiUsernameNotFoundError), err_text.to_string()))
        };

        let query_string = format!(r#"
        query {{
            user(login: "{}") {{
                pinnedItems(first: 6, types: REPOSITORY) {{
                    nodes {{
                        ... on Repository {{
                            name
                            description
                            url
                            stargazerCount
                            updatedAt
                            languages(first: 5) {{
                                nodes {{
                                    name
                                }}
                            }}
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
                return Err(Error::new(ApplicationError::ApiError(ApiRequestSendError), err.to_string()))
            }
        };

        if res.status().is_success() {
            let response_body: PinnedRepositoriesResponse = match res.json().await {
                Ok(res) => res,
                Err(err) => {
                    return Err(Error::new(ApplicationError::ApiError(ApiGetReposJsonError), err.to_string()))
                }
            };

            if let Some(data) = response_body.data {
                let repos = data.user.pinnedItems.nodes;
                return Ok(repos);
            }
            else {
                let err_text = response_body.errors.unwrap().join("\n");
                Err(Error::new(ApplicationError::ApiError(ApiGetResponseError), err_text))
            }
        }
        else {
            Err(Error::new(ApplicationError::ApiError(ApiGetResponseError), String::from("Error getting the response")))
        }
    }
}