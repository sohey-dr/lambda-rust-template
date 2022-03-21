use std::error::Error as StdError;

use lambda_runtime::{error::HandlerError, lambda, Context};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct RequestBody {
    name: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ResponseBody {
    message: String,
}

fn main() -> Result<(), Box<dyn StdError>> {
    lambda!(handler);
    Ok(())
}

fn handler(event: RequestBody, _context: Context) -> Result<ResponseBody, HandlerError> {
    Ok(ResponseBody {
        message: format!("Hello, {}!", event.name),
    })
}
