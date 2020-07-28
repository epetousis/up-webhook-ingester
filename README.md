# UpBank Webhook Ingester

An AWS Lambda function to ingest UpBank payloads and forwards them to Discord, ~~so that my friends can laugh at me~~

Feel free to use this yourself, or steal the TypeScript type definitions idk I don't care

## Important clarification

This was hacked together in like an hour or two for fun, please don't judge

## How to install

Warning: this uses AWS Lambda.

1. Get a Lambda function
2. POST to `https://api.up.com.au/api/v1/webhooks with this body:
```json
{
	"data": {
		"attributes": {
			"url": "https://my-shitty-host.com"
		}
	}
}
```
Where `url` is the endpoint for your Lambda function.

3. Set these environment variables:

```javascript
DISCORD_WEBHOOK_ID /* Webhook ID for a channel specific webhook (second-last path component of the webhook URL) */
DISCORD_WEBHOOK_TOKEN /* Webhook token (last path component of the webhook URL) */
UP_AUTH_TOKEN /* Up Bank API Personal Token */
UP_WEBHOOK_SECRET /* Up Bank Webhook Secret */
UP_ACCOUNT_HOLDER /* The name to use for Discord messages */
```
4. Deploy
5. Test with a ping and by transferring some money from your savings account to spending
6. Lose all self esteem
