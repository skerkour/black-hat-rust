use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Response<T: Serialize> {
    data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    errors: Option<Vec<super::Error>>,
}

impl<T: Serialize> Response<T> {
    pub fn ok(data: T) -> Response<T> {
        return Response {
            data: Some(data),
            errors: None,
        };
    }

    pub fn err(err: crate::Error) -> Response<()> {
        return Response::<()> {
            data: None,
            errors: Some(vec![err.into()]),
        };
    }
}
