pub mod error {
    #[derive(Clone, Debug)]
    pub enum AppError {
        ApiGetResponseError,
        ApiRequestSendError,
        ApiGetReposJsonError,
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
}