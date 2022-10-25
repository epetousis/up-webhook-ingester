# UpBank Webhook Ingester

An AWS Lambda function to ingest UpBank payloads and forwards them to Discord, ~~so that my friends can laugh at me~~

Feel free to use this yourself or reuse the type definitions.

![A screenshot of the bot in action in Discord](docs/screenshot.png)

> **Note**
> Looking for the original TypeScript version? Check out the [js-legacy branch!](/epetousis/up-webhook-ingester/tree/js-legacy).

## Important clarification

This was hacked together in like a day or two for fun with near-zero knowledge of Rust (and lots of potential panic points that need to be fixed), please don't judge _too_ hard.

## How to install

Warning: this uses AWS Lambda.

1. Install [cargo-lambda](https://github.com/cargo-lambda/cargo-lambda).
2. Clone this repo.
3. Run `cargo lambda build --release` to build a release version (as we want those extra optimisations for our production server - we don't need debug information.)
4. Run `cargo lambda deploy --enable-function-url`. You may need to add `--region [aws region] --profile [profile]` to the end of the deploy script in `package.json` if you use the AWS CLI with more than one account or region.
5. POST to https://api.up.com.au/api/v1/webhooks (you can use a tool like [Insomnia](https://insomnia.rest/) or [Postman](https://www.postman.com/)) [with a Bearer Auth header using your Personal Access Token as a value](https://developer.up.com.au/#post_webhooks) (e.g. `Authorization: Bearer up:demo:uFHtxi6JnFtqrx1o`), and with this body:
```json
{
	"data": {
		"attributes": {
			"url": "https://my-shitty-host.com/?name={name}"
		}
	}
}
```
Where `url` is the endpoint for your Lambda function, as returned by cargo. The `name` parameter should be changed to the `name` value that is in the env.conf.

The response from this request will provide a bunch of data relating to the webhook resource that was created. Save the value of `data.attributes.secretKey`, as we will need this later.

6. You'll need to create a webhook in Discord for each channel you want the bot to post in, In your desired Discord channel, create a webhook by going to Edit Channel -> Integrations -> View Webhooks, and clicking New Webhook, and then save the URL. You can have multiple webhooks in the discord_webhooks array inside your config, and these webhooks can be from multiple different servers.

7. Create a env.conf file in the root code directory and replace the comments with your appropriate values:

```
USER_DETAILS=[{
    "discord_webhooks": ["/* Webhook URLs for channel specific webhooks */"],
    "name": "/* The name to use for Discord messages */",
    "auth_token": "/* Up Bank API Personal Token */",
    "webhook_secret": "/* Up Bank Webhook Secret retrieved from the API's `data.attributes.secretKey` prop upon webhook creation */"
}]
```

8. Run `cargo lambda deploy` again (with your relevant flags).
9. Test with a webhook ping. If you don't see a ping, check the CloudWatch logs on AWS.
10. Lose all self esteem
