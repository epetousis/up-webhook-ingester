export interface WebhookPayload {
    data: WebhookPayloadData;
}

export interface WebhookPayloadData {
    id:            string;
    type:          string;
    attributes:    Attributes;
    relationships: Relationships;
}

export interface Attributes {
    eventType: string;
    createdAt: Date;
}

export interface Relationships {
    webhook:     Transaction;
    transaction: Transaction;
}

export interface Transaction {
    links: Links;
    data:  TransactionData;
}

export interface TransactionData {
    type: string;
    id:   string;
}

export interface Links {
    related: string;
}