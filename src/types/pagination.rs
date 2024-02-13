use handle_errors::Error;
use std::collections::HashMap;

#[derive(Default, Debug, PartialEq, Eq)]
pub struct Pagination {
    pub limit: Option<i32>,
    pub offset: i32,
}
pub fn extract_pagination(params: HashMap<String, String>) -> Result<Pagination, Error> {
    if params.contains_key("limit") && params.contains_key("offset") {
        return Ok(Pagination {
            // take the limit paramter in the query
            // and tries to convert it to a number
            limit: Some(
                params
                    .get("limit")
                    .unwrap()
                    .parse::<i32>()
                    .map_err(Error::ParseError)?,
            ),
            // takes the offset paramter in the query
            // and tries to convert it to a number
            offset: params
                .get("offset")
                .unwrap()
                .parse::<i32>()
                .map_err(Error::ParseError)?,
        });
    }

    Err(Error::MissingParameters)
    // }
    // #[cfg(test)]
    // mod tests {
    //     use super::*;
    //
    //     #[test]
    //     fn test_extract_positive_pagination() {
    //         let params = HashMap::from([
    //             ("".to_string(), 1.to_string()),
    //             ("end".to_string(), 2.to_string()),
    //         ]);
    //         assert_eq!(
    //             extract_pagination(params).unwrap(),
    //             Pagination {
    //                 limit: Some(1),
    //                 offset: 2
    //             }
    //         );
    //     }

    // #[test]
    // fn test_doesnt_have_start() {
    //     let params = HashMap::from([("offset".to_string(), 2.to_string())]);
    //     assert_eq!(
    //         extract_pagination(params).unwrap_err(),
    //         Error::MissingParameters
    //     );
    // }
    // #[test]
    // fn test_doesnt_have_end() {
    //     let params = HashMap::from([("limit".to_string(), 2.to_string())]);
    //     assert_eq!(
    //         extract_pagination(params).unwrap_err(),
    //         Error::MissingParameters
    //     );
    // }
}
