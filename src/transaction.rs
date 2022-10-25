use serde::{Serialize, Deserialize};

/*#[derive(Serialize, Deserialize)]
pub struct Relationships {
    // FIXME: switch to generic response type
    //pub account: Box<TransactionResponse>,
    // #[serde(rename = "transferAccount")]
    //pub transfer_account: Box<TransactionResponse>,
    pub category: Category,
    //#[serde(rename = "parentCategory")]
    //pub parent_category: Box<TransactionResponse>,
    pub tags: Category,
}*/

#[derive(Serialize, Deserialize)]
pub struct TransactionResponseData {
    #[serde(rename = "type")]
    pub data_type: String,
    pub id: String,
    pub attributes: TransactionAttributes,
    // FIXME: relationships types are broken
    // pub relationships: Relationships,
    pub links: DataLinks,
}

#[derive(Serialize, Deserialize)]
pub struct TransactionResponse {
    pub data: Option<TransactionResponseData>,
    pub errors: Option<Vec<Error>>
}

#[derive(Serialize, Deserialize)]
pub struct Error {
    pub status: String,
    pub title: String,
    pub detail: String,
}

#[derive(Serialize, Deserialize)]
pub struct Category {
    pub data: Option<Vec<Option<serde_json::Value>>>,
    pub links: DataLinks,
}

#[derive(Serialize, Deserialize)]
pub struct DataLinks {
    #[serde(rename = "self")]
    pub links_self: String,
    pub related: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct TransactionAttributes {
    pub status: String,
    #[serde(rename = "rawText")]
    pub raw_text: Option<String>,
    pub description: String,
    pub message: Option<String>,
    #[serde(rename = "isCategorizable")]
    pub is_categorizable: bool,
    #[serde(rename = "holdInfo")]
    pub hold_info: Option<HoldInfoObject>,
    #[serde(rename = "roundUp")]
    pub round_up: Option<RoundUpObject>,
    pub cashback: Option<CashbackObject>,
    pub amount: MoneyObject,
    #[serde(rename = "foreignAmount")]
    pub foreign_amount: Option<MoneyObject>,
    #[serde(rename = "cardPurchaseMethod")]
    pub card_purchase_method: Option<CardPurchaseMethod>,
    #[serde(rename = "settledAt")]
    pub settled_at: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct MoneyObject {
    #[serde(rename = "currencyCode")]
    pub currency_code: String,
    pub value: String,
    #[serde(rename = "valueInBaseUnits")]
    pub value_in_base_units: i64,
}

#[derive(Serialize, Deserialize)]
pub struct CardPurchaseMethod {
    pub method: String,
    #[serde(rename = "cardNumberSuffix")]
    pub card_number_suffix: String,
}

#[derive(Serialize, Deserialize)]
pub struct HoldInfoObject {
    pub amount: MoneyObject,
    #[serde(rename = "foreignAmount")]
    pub foreign_amount: Option<MoneyObject>,
}

#[derive(Serialize, Deserialize)]
pub struct CashbackObject {
    pub description: String,
    pub amount: MoneyObject,
}

#[derive(Serialize, Deserialize)]
pub struct RoundUpObject {
    pub amount: MoneyObject,
    #[serde(rename = "boostPortion")]
    pub boost_portion: Option<serde_json::Value>,
}
