use handle_errors::Error;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub struct Pagination {
    pub start: usize,
    pub end: usize,
}

pub fn extract_pagination(params: HashMap<String, String>) -> Result<Pagination, Error> {
    if params.contains_key("start") && params.contains_key("end") {
        return Ok(Pagination {
            start: params
                .get("start")
                .unwrap()
                .parse::<usize>()
                .map_err(Error::ParseError)?,
            end: params
                .get("end")
                .unwrap()
                .parse::<usize>()
                .map_err(Error::ParseError)?,
        });
    }

    Err(Error::MissingParameters)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_positive_pagination() {
        let params = HashMap::from([
            ("start".to_string(), 1.to_string()),
            ("end".to_string(), 2.to_string()),
        ]);
        assert_eq!(
            extract_pagination(params).unwrap(),
            Pagination { start: 1, end: 2 }
        );
    }

    #[test]
    fn test_doesnt_have_start() {
        let params = HashMap::from([("end".to_string(), 2.to_string())]);
        assert_eq!(
            extract_pagination(params).unwrap_err(),
            Error::MissingParameters
        );
    }
    #[test]
    fn test_doesnt_have_end() {
        let params = HashMap::from([("start".to_string(), 2.to_string())]);
        assert_eq!(
            extract_pagination(params).unwrap_err(),
            Error::MissingParameters
        );
    }
}
