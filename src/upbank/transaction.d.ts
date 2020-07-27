export interface TransactionResponse {
    data?: TransactionResource;
    errors?: Record<string, unknown>[];
}

export interface TransactionResource {
    id:            string;
    type:          'transactions';
    attributes:    Attributes;
    links?:        {
        self: string
    };
    relationships: {
        account: {
            links?: {
                related: string
            };
            data: {
                type: string;
                id: string
            }
        }
    };
}

export interface Attributes {
    status:        TransactionStatusEnum;
    rawText?:      string;
    description:   string;
    message?:      null;
    holdInfo?:     HoldInfoObject;
    roundUp:       RoundUpObject;
    cashback?:     CashbackObject;
    amount:        MoneyObject;
    foreignAmount?: MoneyObject;
    settledAt?:     Date;
    createdAt:     Date;
}

export interface CashbackObject {
    description: string;
    amount:      MoneyObject;
}

export enum TransactionStatusEnum {
    held = 'HELD',
    settled = 'SETTLED'
}

export interface MoneyObject {
    currencyCode:     string;
    value:            string;
    valueInBaseUnits: number;
}

export interface HoldInfoObject {
    amount:        MoneyObject;
    foreignAmount: MoneyObject;
}

export interface RoundUpObject {
    amount:        MoneyObject;
    boostPortion?: MoneyObject;
}
