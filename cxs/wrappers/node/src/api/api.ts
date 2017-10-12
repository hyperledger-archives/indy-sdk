export interface IConnections {
  create ( recipientInfo: string ): number
  connect (): number
  getData (): string
  getState (): number
  release (): number
}

export enum StateType{
    None = 0,
    Initialized = 1,
    OfferSent = 2,
    RequestReceived = 3,
    Accepted = 4,
    Unfulfilled = 5,
    Expired = 6,
    Revoked = 7,
}