pub mod error {

    #[derive(Clone, Debug)]
    pub enum AppError {
        ApiGetResponseError,
        ApiRequestSendError,
        ApiGetReposJsonError,
        ApiKeyNotFoundError,
        ApiUsernameNotFoundError,
        NotFoundError
    }

    #[derive(Clone, Debug)]
    pub struct Error {
        pub error: AppError,
        pub error_text: String
    }

    impl Error {
        pub fn new(err: AppError, txt: String) -> Error {
            Error {
                error: err,
                error_text: txt
            }
        }
    }

    impl std::fmt::Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{}", self.error_text)
        }
    }

    impl std::error::Error for Error {
        fn description(&self) -> &str {
            &self.error_text
        }
    }
}