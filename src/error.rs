pub mod error {
    use serde::Deserialize;
    use crate::log;
    use crate::log::LogLevel::*;

    #[derive(Clone, Debug, Deserialize, PartialEq)]
    pub enum ApiError {
        ApiGetResponseError,
        ApiRequestSendError,
        ApiGetReposJsonError,
        ApiKeyNotFoundError,
        ApiUsernameNotFoundError,
        NotFoundError
    }

    #[derive(Clone, Debug, Deserialize, PartialEq)]
    pub enum FileError {
        FileOpenError,
        FileReadError,
        FileWriteError,
        FileUploadError
    }

    #[derive(Clone, Debug, Deserialize, PartialEq)]
    pub enum JsonError {
        JsonSerializeError,
        JsonDeserializeError
    }

    #[derive(Clone, Debug, Deserialize, PartialEq)]
    pub enum ApplicationError {
        ApiError(ApiError),
        FileError(FileError),
        JsonError(JsonError)
    }

    #[derive(Debug, Deserialize)]
    pub struct Error {
        pub error: ApplicationError,
        pub error_text: String
    }

    impl Error {
        pub fn new(err: ApplicationError, txt: String) -> Error {
            log!(ERROR, &txt[..]);
            Error {
                error: err,
                error_text: txt
            }
        }

        pub fn to_str(&self) -> &'static str {
            match &self.error {
                ApplicationError::ApiError(api_err) => match api_err.clone()
                {
                    ApiError::ApiGetReposJsonError => "Error getting repositories json from GitHub",
                    ApiError::ApiGetResponseError => "Error getting response from GitHub",
                    ApiError::ApiRequestSendError => "Error sending request to GitHub",
                    ApiError::NotFoundError => "Page not Found",
                    ApiError::ApiKeyNotFoundError => "No github key found",
                    ApiError::ApiUsernameNotFoundError => "No github username found",
                },
                ApplicationError::FileError(file_err) => match file_err
                {
                    FileError::FileOpenError => "Error opening file",
                    FileError::FileReadError => "Error reading file",
                    FileError::FileWriteError => "Error writing file",
                    FileError::FileUploadError => "Error uploading file"
                },
                ApplicationError::JsonError(json_err) => match json_err {
                    JsonError::JsonSerializeError => "Error serializing json file",
                    JsonError::JsonDeserializeError => "Error deserializing json file"
                }
            }
        }
    }
}