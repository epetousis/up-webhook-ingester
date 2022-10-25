mod webhook;
mod transaction;
mod user;

use user::User;
use lambda_http::RequestExt;

use std::env;
use hex::FromHex;

// Ideally status would also be defined as just an Into<StatusCode> trait type but I'm too tired to figure out why it's not working when I try
fn simple_request<S: Into<String>>(status: u16, body: S) -> Result<lambda_http::Response<String>, &'static str> {
    let body = body.into();
    if status < 200 || status >= 300 {
	println!("ERROR: {}", body);
    }
    let status = match lambda_http::http::StatusCode::from_u16(status) {
	Ok(x) => x,
	Err(_) => return Err("Invalid status code provided"),
    };
    lambda_http::Response::builder()
        .status(status)
        .body(body)
        .or(Err("Response error"))
}

async fn handler(request: lambda_http::Request) -> Result<lambda_http::Response<String>, &'static str> {
    let body = request.body();
    if body.is_empty() {
	return simple_request(400, "Missing body. Is this a POST request?");
    }

    let query_params = request.query_string_parameters();
    let user_name = match query_params.first("user") {
	Some(x) => x,
	None => return simple_request(400, "Desired user was not given"),
    };

    let users_details = match env::var("USER_DETAILS") {
	Ok(x) => x,
	Err(_) => return simple_request(400, "User details not defined"),
    };

    let users: Vec<User> = serde_json::from_str(users_details.as_str()).unwrap();
    // The created webhooks should point to a /{name} subpage.
    let user = match users.iter().find(|user| user.name == user_name) {
	Some(x) => x,
	None => return simple_request(500, format!("User {} was not found.", user_name)),
    };

    let signature = match request.headers().get("X-Up-Authenticity-Signature") {
	Some(x) => x,
	None => return simple_request(400, "No authenticity signature provided. Make sure X-Up-Authenticity-Signature is provided as a header."),
    };

    let signature = match Vec::from_hex(signature) {
	Ok(x) => x,
	Err(_) => return simple_request(400, "Invalid auth signature. Must be hex string."),
    };

    if let Err(_) = user.verify_webhook_signature(body, signature.as_slice()) {
	return simple_request(500, "Invalid signature");
    }

    let body: webhook::WebhookResponse = match serde_json::from_slice(body) {
	Ok(x) => x,
	Err(_) => return simple_request(500, "Invalid body provided"),
    };

    match body.data.attributes.event_type.as_str() {
	// FIXME: gracefully handle lack of transaction relationship just in case
	"TRANSACTION_CREATED" => user.rat_out_transaction(body.data.relationships.transaction.expect("transaction webhooks should have relationship")).await.or(simple_request(500, "Transaction pull request failed.")),
	"PING" => user.ping_pong().await.or(simple_request(500, "Ping pong request failed.")),
	_ => simple_request(400, "Unsupported action provided - no action taken."),
    }
}

#[tokio::main]
async fn main() -> Result<(), lambda_http::Error> {
    lambda_http::run(lambda_http::service_fn(handler)).await
}
