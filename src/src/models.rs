#![allow(unused_qualifications)]

use crate::models;
#[cfg(any(feature = "client", feature = "server"))]
use crate::header;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct AddTokenResponseBody {
    #[serde(rename = "AddedCell")]
    pub added_cell: i64,

    #[serde(rename = "Column")]
    pub column: i64,

    #[serde(rename = "CurrentPlayer")]
    pub current_player: i64,

    #[serde(rename = "IsGridFull")]
    pub is_grid_full: bool,

    #[serde(rename = "Line")]
    pub line: i64,

    #[serde(rename = "NextPlayer")]
    pub next_player: i64,

    #[serde(rename = "PlayerWon")]
    pub player_won: bool,

}

impl AddTokenResponseBody {
    #[allow(clippy::new_without_default)]
    pub fn new(added_cell: i64, column: i64, current_player: i64, is_grid_full: bool, line: i64, next_player: i64, player_won: bool, ) -> AddTokenResponseBody {
        AddTokenResponseBody {
            added_cell,
            column,
            current_player,
            is_grid_full,
            line,
            next_player,
            player_won,
        }
    }
}

/// Converts the AddTokenResponseBody value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for AddTokenResponseBody {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("AddedCell".to_string()),
            Some(self.added_cell.to_string()),


            Some("Column".to_string()),
            Some(self.column.to_string()),


            Some("CurrentPlayer".to_string()),
            Some(self.current_player.to_string()),


            Some("IsGridFull".to_string()),
            Some(self.is_grid_full.to_string()),


            Some("Line".to_string()),
            Some(self.line.to_string()),


            Some("NextPlayer".to_string()),
            Some(self.next_player.to_string()),


            Some("PlayerWon".to_string()),
            Some(self.player_won.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a AddTokenResponseBody value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for AddTokenResponseBody {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub added_cell: Vec<i64>,
            pub column: Vec<i64>,
            pub current_player: Vec<i64>,
            pub is_grid_full: Vec<bool>,
            pub line: Vec<i64>,
            pub next_player: Vec<i64>,
            pub player_won: Vec<bool>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing AddTokenResponseBody".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "AddedCell" => intermediate_rep.added_cell.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "Column" => intermediate_rep.column.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "CurrentPlayer" => intermediate_rep.current_player.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "IsGridFull" => intermediate_rep.is_grid_full.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "Line" => intermediate_rep.line.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "NextPlayer" => intermediate_rep.next_player.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "PlayerWon" => intermediate_rep.player_won.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing AddTokenResponseBody".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(AddTokenResponseBody {
            added_cell: intermediate_rep.added_cell.into_iter().next().ok_or_else(|| "AddedCell missing in AddTokenResponseBody".to_string())?,
            column: intermediate_rep.column.into_iter().next().ok_or_else(|| "Column missing in AddTokenResponseBody".to_string())?,
            current_player: intermediate_rep.current_player.into_iter().next().ok_or_else(|| "CurrentPlayer missing in AddTokenResponseBody".to_string())?,
            is_grid_full: intermediate_rep.is_grid_full.into_iter().next().ok_or_else(|| "IsGridFull missing in AddTokenResponseBody".to_string())?,
            line: intermediate_rep.line.into_iter().next().ok_or_else(|| "Line missing in AddTokenResponseBody".to_string())?,
            next_player: intermediate_rep.next_player.into_iter().next().ok_or_else(|| "NextPlayer missing in AddTokenResponseBody".to_string())?,
            player_won: intermediate_rep.player_won.into_iter().next().ok_or_else(|| "PlayerWon missing in AddTokenResponseBody".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<AddTokenResponseBody> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<AddTokenResponseBody>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<AddTokenResponseBody>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for AddTokenResponseBody - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<AddTokenResponseBody> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <AddTokenResponseBody as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into AddTokenResponseBody - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct BadRequestErrorBody {
    #[serde(rename = "Reason")]
    pub reason: String,

}

impl BadRequestErrorBody {
    #[allow(clippy::new_without_default)]
    pub fn new(reason: String, ) -> BadRequestErrorBody {
        BadRequestErrorBody {
            reason,
        }
    }
}

/// Converts the BadRequestErrorBody value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for BadRequestErrorBody {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("Reason".to_string()),
            Some(self.reason.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a BadRequestErrorBody value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for BadRequestErrorBody {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub reason: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing BadRequestErrorBody".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "Reason" => intermediate_rep.reason.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing BadRequestErrorBody".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(BadRequestErrorBody {
            reason: intermediate_rep.reason.into_iter().next().ok_or_else(|| "Reason missing in BadRequestErrorBody".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<BadRequestErrorBody> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<BadRequestErrorBody>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<BadRequestErrorBody>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for BadRequestErrorBody - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<BadRequestErrorBody> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <BadRequestErrorBody as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into BadRequestErrorBody - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Cell(i64);

impl std::convert::From<i64> for Cell {
    fn from(x: i64) -> Self {
        Cell(x)
    }
}

impl std::convert::From<Cell> for i64 {
    fn from(x: Cell) -> Self {
        x.0
    }
}

impl std::ops::Deref for Cell {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}

impl std::ops::DerefMut for Cell {
    fn deref_mut(&mut self) -> &mut i64 {
        &mut self.0
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetGridResponseBody {
    #[serde(rename = "CurrentPlayerColor")]
    pub current_player_color: i64,

    #[serde(rename = "Grid")]
    pub grid: Vec<Vec<models::Cell>>,

}

impl GetGridResponseBody {
    #[allow(clippy::new_without_default)]
    pub fn new(current_player_color: i64, grid: Vec<Vec<models::Cell>>, ) -> GetGridResponseBody {
        GetGridResponseBody {
            current_player_color,
            grid,
        }
    }
}

/// Converts the GetGridResponseBody value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for GetGridResponseBody {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("CurrentPlayerColor".to_string()),
            Some(self.current_player_color.to_string()),

            // Skipping Grid in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a GetGridResponseBody value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for GetGridResponseBody {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub current_player_color: Vec<i64>,
            pub grid: Vec<Vec<Vec<models::Cell>>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing GetGridResponseBody".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "CurrentPlayerColor" => intermediate_rep.current_player_color.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "Grid" => return std::result::Result::Err("Parsing a container in this style is not supported in GetGridResponseBody".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing GetGridResponseBody".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(GetGridResponseBody {
            current_player_color: intermediate_rep.current_player_color.into_iter().next().ok_or_else(|| "CurrentPlayerColor missing in GetGridResponseBody".to_string())?,
            grid: intermediate_rep.grid.into_iter().next().ok_or_else(|| "Grid missing in GetGridResponseBody".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<GetGridResponseBody> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<GetGridResponseBody>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<GetGridResponseBody>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for GetGridResponseBody - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<GetGridResponseBody> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <GetGridResponseBody as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into GetGridResponseBody - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct MiniMaxiResponseBody {
    #[serde(rename = "BestMove")]
    pub best_move: i64,

    #[serde(rename = "Scores")]
    pub scores: Vec<i64>,

}

impl MiniMaxiResponseBody {
    #[allow(clippy::new_without_default)]
    pub fn new(best_move: i64, scores: Vec<i64>, ) -> MiniMaxiResponseBody {
        MiniMaxiResponseBody {
            best_move,
            scores,
        }
    }
}

/// Converts the MiniMaxiResponseBody value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for MiniMaxiResponseBody {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("BestMove".to_string()),
            Some(self.best_move.to_string()),


            Some("Scores".to_string()),
            Some(self.scores.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",")),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a MiniMaxiResponseBody value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for MiniMaxiResponseBody {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub best_move: Vec<i64>,
            pub scores: Vec<Vec<i64>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing MiniMaxiResponseBody".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "BestMove" => intermediate_rep.best_move.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "Scores" => return std::result::Result::Err("Parsing a container in this style is not supported in MiniMaxiResponseBody".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing MiniMaxiResponseBody".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(MiniMaxiResponseBody {
            best_move: intermediate_rep.best_move.into_iter().next().ok_or_else(|| "BestMove missing in MiniMaxiResponseBody".to_string())?,
            scores: intermediate_rep.scores.into_iter().next().ok_or_else(|| "Scores missing in MiniMaxiResponseBody".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<MiniMaxiResponseBody> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<MiniMaxiResponseBody>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<MiniMaxiResponseBody>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for MiniMaxiResponseBody - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<MiniMaxiResponseBody> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <MiniMaxiResponseBody as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into MiniMaxiResponseBody - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Player(i64);

impl std::convert::From<i64> for Player {
    fn from(x: i64) -> Self {
        Player(x)
    }
}

impl std::convert::From<Player> for i64 {
    fn from(x: Player) -> Self {
        x.0
    }
}

impl std::ops::Deref for Player {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}

impl std::ops::DerefMut for Player {
    fn deref_mut(&mut self) -> &mut i64 {
        &mut self.0
    }
}

