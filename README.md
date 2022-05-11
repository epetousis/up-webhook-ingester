# UpBank Webhook Ingester

An AWS Lambda function to ingest UpBank payloads and forwards them to Discord, ~~so that my friends can laugh at me~~

Feel free to use this yourself or reuse the type definitions.

![A screenshot of the bot in action in Discord](docs/screenshot.png)

## Important clarification

This was hacked together in like an hour or two for fun, please don't judge _too_ hard.

## How to install

Warning: this uses AWS Lambda.

1. Install [Serverless](https://www.serverless.com/framework/docs/getting-started/).
2. Clone this repo.
3. Run `npm install && npm run deploy`. You may need to update the profile and region in the `serverless.yml` config or add `--region [aws region] --aws-profile [profile]` to the end of the deploy script in `package.json`.
4. POST to https://api.up.com.au/api/v1/webhooks (you can use a tool like [Insomnia](https://insomnia.rest/) or [Postman](https://www.postman.com/)) [with a Bearer Auth header using your Personal Access Token as a value](https://developer.up.com.au/#post_webhooks) (e.g. `Authorization: Bearer up:demo:uFHtxi6JnFtqrx1o`), and with this body:
```json
{
	"data": {
		"attributes": {
			"url": "https://my-shitty-host.com"
		}
	}
}
```
Where `url` is the endpoint for your Lambda function. This can be found in the triggers section of the Lambda function - you're looking for the API endpoint for the API Gateway trigger.

The response from this request will provide a bunch of data relating to the webhook resource that was created. Save the value of `data.attributes.secretKey`, as we will need this later.

5. You'll need to create a webhook in Discord for each channel you want the bot to post in, In your desired Discord channel, create a webhook by going to Edit Channel -> Integrations -> View Webhooks, and clicking New Webhook, and then take the second-last and last path component of the webhook URL and store it in `ID:TOKEN` format. You can have multiple webhooks comma-separated, such as `ID:TOKEN,ID:TOKEN,ID:TOKEN`, and these webhooks can be from multiple different servers.

6. Create a config.json in the root code directory and replace the comments with your appropriate values:

```json
{
    "DISCORD_WEBHOOKS": "/* Webhook IDs and tokens for channel specific webhooks */",
    "UP_ACCOUNT_HOLDER": "/* The name to use for Discord messages */",
    "UP_AUTH_TOKEN": "/* Up Bank API Personal Token */",
    "UP_WEBHOOK_SECRET": "/* Up Bank Webhook Secret retrieved from the API's `data.attributes.secretKey` prop upon webhook creation */"
}
```

7. Run `npm run deploy` again.
8. Test with a webhook ping.
9. Lose all self esteem
