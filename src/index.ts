import { APIGatewayProxyHandlerV2, APIGatewayProxyResultV2, APIGatewayProxyEventV2 } from 'aws-lambda';
import Discord from 'discord.js';
import fetch, { Headers } from 'node-fetch';
import crypto from 'crypto';
// eslint-disable-next-line import/extensions
import { WebhookResponse, Relationship, TransactionResponse } from './upbank';

async function sendHook(callback: (hook: Discord.WebhookClient) => Promise<Discord.Message>) {
  if (!process.env.DISCORD_WEBHOOKS) {
    return;
  }

  const messageRequests = process.env.DISCORD_WEBHOOKS.split(',').map(async (webhook) => {
    const webhookData = webhook.split(':');
    const webhookId = webhookData[0];
    const webhookToken = webhookData[1];
    const hook = new Discord.WebhookClient(
      webhookId,
      webhookToken,
    );

    await callback(hook);
  });

  await Promise.all(messageRequests);
}

async function ratOutTransaction(transaction: Relationship) {
  if (!process.env.DISCORD_WEBHOOKS
    || !process.env.UP_AUTH_TOKEN) {
    return;
  }

  const rawResponse = await fetch(transaction.links.related, {
    headers: new Headers({
      Authorization: `Bearer ${process.env.UP_AUTH_TOKEN}`,
    }),
  });

  const res: TransactionResponse = await rawResponse.json();

  if (res.errors || !res.data) {
    throw new Error(`Errors detected while getting transaction. Ensure your API token is correct. ${JSON.stringify(res.errors)}`);
  }

  const { description, cashback, message } = res.data.attributes;
  const isPurchase = res.data.attributes.amount.value.includes('-');
  const money = res.data.attributes.amount.value.replace('-', '');

  const foreignCurrency = res.data.attributes.foreignAmount?.currencyCode;
  const foreignAmount = res.data.attributes.foreignAmount?.value.replace('-', '');

  const isBigMoneyInvolved = !isPurchase && Number(money) > 100;
  const isBlockedAction = /(transfer|round up)/i.test(description);
  // Obfuscate transactions greater than $100 to avoid leaking salaries.
  const publicFacingMoney = isBigMoneyInvolved ? '100+' : money;

  if (isBlockedAction) {
    // Block transfers and round ups from being sent.
    return;
  }

  const status = res.data.attributes.status.toLowerCase();

  const action = !isPurchase ? 'got reimbursed' : 'spent';
  const preposition = !isPurchase ? 'by' : 'on';

  const fields = [
    {
      name: 'Description',
      value: `${description}`,
    },
    {
      name: 'Cost',
      value: `$${publicFacingMoney}`,
    },
  ];

  if (message) {
    fields.push(
      {
        name: 'Message',
        value: `${message}`,
      },
    );
  }

  if (foreignAmount && foreignCurrency) {
    fields.push({
      name: 'Foreign Currency',
      value: `${foreignCurrency} ${foreignAmount}`,
    });
  }

  await sendHook((hook) => hook.send(`${process.env.UP_ACCOUNT_HOLDER ?? 'The bot owner'} just ${action} $${publicFacingMoney} ${preposition} ${description}!`, {
    embeds: [
      {
        title: cashback ? 'New Reimbursement' : 'New Purchase',
        description: `Transaction was ${status}.`,
        color: 16743012,
        fields,
      },
    ],
  }));
}

async function pingPong() {
  await sendHook((hook) => hook.send('Someone just pinged the Up API which, in turn, pinged this bot. Just checking in to say: pong.'));
}

// eslint-disable-next-line import/prefer-default-export, max-len
export const handler: APIGatewayProxyHandlerV2 = async (event: APIGatewayProxyEventV2): Promise<APIGatewayProxyResultV2> => {
  console.log('Received event', event);

  if (!process.env.UP_WEBHOOK_SECRET) {
    throw new Error("No Up Bank API webhook secret provided. No verification can take place, and I don't like that.");
  }

  if (!event.body) {
    return {
      statusCode: 400,
      body: "don't care didn't ask plus you're malformed",
    };
  }

  const hmac = crypto.createHmac('sha256', process.env.UP_WEBHOOK_SECRET);
  hmac.update(event.body);
  const signature = hmac.digest('hex');
  const receivedSignature = event.headers['X-Up-Authenticity-Signature'];

  if (!crypto.timingSafeEqual(Buffer.from(signature), Buffer.from(receivedSignature))) {
    return {
      statusCode: 403,
      body: 'bad signature',
    };
  }

  const payload: WebhookResponse = JSON.parse(event.body);

  switch (payload.data.attributes.eventType) {
    case 'TRANSACTION_CREATED': {
      const { transaction } = payload.data.relationships;
      if (!transaction) {
        return {
          statusCode: 400,
          body: 'transaction will always be true for a transaction webhook but i cbs fixing the typings',
        };
      }
      await ratOutTransaction(transaction);
      return {
        statusCode: 200,
        body: 'handled a-ok',
      };
    }
    case 'PING':
      await pingPong();
      return {
        statusCode: 200,
        body: 'pong',
      };
    default:
      return {
        statusCode: 404,
        body: 'a webhook event of an unsupported type was passed to the server',
      };
  }
};
