export interface WebhookResponse {
    data: WebhookResource;
}

export interface WebhookResource {
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
    webhook:     Relationship;
    transaction?: Relationship;
}

export interface Relationship {
    links: RelationshipLinks;
    data:  RelationshipData;
}

export interface RelationshipData {
    type: string;
    id:   string;
}

export interface RelationshipLinks {
    related: string;
}
