use hmac::Mac;
use serenity::http::Http;
use serenity::model::webhook::Webhook;
use serenity::model::channel::Embed;
use futures::future::join_all;
use crate::webhook;
use crate::transaction;

#[derive(serde::Deserialize)]
pub struct User {
    pub discord_webhooks: Vec<String>,
    pub name: String,
    pub auth_token: String,
    pub webhook_secret: String,
}

impl User {
    pub fn verify_webhook_signature(&self, body: &lambda_http::Body, signature: &[u8]) -> Result<(), hmac::digest::MacError> {
	let mut hmac = hmac::Hmac::<sha2::Sha256>::new_from_slice(self.webhook_secret.as_bytes())
	    .expect("new_from_slice should accept string of any size");
	hmac.update(body);

	hmac.verify_slice(signature)
    }

    /// Gets details of a provided transaction and sends those details to all the user's provided webhooks.
    pub async fn rat_out_transaction(&self, transaction: webhook::RelationshipResource) -> Result<lambda_http::Response<String>, ()> {
	let client = reqwest::Client::new();
	let transaction_request = match client.get(transaction.links.related)
	    .bearer_auth(&self.auth_token)
	    .send()
	    .await {
		Ok(x) => x,
		Err(_) => return Ok(lambda_http::Response::builder()
               .status(500)
               .body("Error getting transaction details.".to_string())
               .expect("Response should accept static inputs")),
	    };

	let response = match transaction_request.json::<transaction::TransactionResponse>()
	    .await {
		Ok(x) => x,
		Err(e) => {
		    println!("ERROR: parsing failed with error: {}", e);
		    return Ok(lambda_http::Response::builder()
			      .status(500)
			      .body("Failed to parse transaction response.".to_string())
			      .expect("Response should accept static inputs"))
		},
	    };

	let attributes = match response.data {
	    Some(x) => x.attributes,
	    None => return Ok(lambda_http::Response::builder()
			      .status(500)
			      // FIXME: show more than the one error
			      .body(response.errors.unwrap()[0].detail.clone())
			      .expect("Response should accept static inputs")),
	};

	let is_purchase = attributes.amount.value.contains("-");

	let lowercase_description = attributes.description.to_lowercase();
	let is_blocked_action = lowercase_description.contains("transfer") || lowercase_description.contains("round up");
	if is_blocked_action {
	    return Ok(lambda_http::Response::builder()
               .status(200)
               .body("Ignored action.".to_string())
               .expect("Response should accept static inputs"));
	}

	let money_amount = attributes.amount.value_in_base_units;
	let positive_value = attributes.amount.value.replace("-", "");
	let public_facing_money_amount = match money_amount > 10000 {
	    true => "100+",
	    false => positive_value.as_str(),
	};

	let status = attributes.status.to_lowercase();

	let action = match is_purchase {
	    true => "spent",
	    false => "got reimbursed",
	};
	let preposition = match is_purchase {
	    true => "on",
	    false => "by",
	};

	let mut fields = vec![
	    ("Description", attributes.description.clone(), false),
	    ("Cost", format!("${}", public_facing_money_amount.to_string()), false),
	];

	if let Some(message) = attributes.message {
	    fields.push(
		("Message", message, false)
	    );
	}

	if let Some(foreign_amount) = attributes.foreign_amount {
	    fields.push(
		("Foreign Currency", format!("{} {}", foreign_amount.currency_code, foreign_amount.value), false)
	    );
	}

	let embed_title = match attributes.cashback.is_some() {
	    true => "New Reimbursement",
	    false => "New Purchase",
	};

	let embed = Embed::fake(|e| {
	    e.title(embed_title)
		.description(format!("Transaction was {}", status))
		.colour(16743012)
		.fields(fields)
	});

	let webhook_calls = self.discord_webhooks.iter().map(|webhook_url| { async {
	    let http = Http::new("");
	    let webhook = Webhook::from_url(&http, webhook_url).await.expect("valid webhook URL should be provided");
	    webhook.execute(&http, false, |hook| {
		hook.content(format!(
		    "{} just {} ${} {} {}!",
		    self.name,
		    action,
		    public_facing_money_amount,
		    preposition,
		    attributes.description,
		)).embeds(vec![embed.clone()]) // Make sure to clone our embed for each closure call
	    }).await
	} });
	join_all(webhook_calls).await;

	Ok(lambda_http::Response::builder()
            .status(200)
            .body("Success".to_string())
            .expect("Response should accept static inputs"))
    }

    pub async fn ping_pong(&self) -> Result<lambda_http::Response<String>, ()> {
	// TODO: make webhook calling into a method
	let webhook_calls = self.discord_webhooks.iter().map(|webhook_url| { async {
	    let http = Http::new("");
	    let webhook = Webhook::from_url(&http, webhook_url).await.expect("valid webhook URL should be provided");
	    webhook.execute(&http, false, |hook| hook.content(format!("Pinging the bot as {}.", self.name))).await
	} });
	join_all(webhook_calls).await;
	Ok(lambda_http::Response::builder()
            .status(200)
            .body("Success".to_string())
            .expect("Response should accept static inputs"))
    }
}
