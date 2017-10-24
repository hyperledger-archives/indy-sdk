export interface IConnections {
  create ( IRecipientInfo ): number
  getData (): IConnectionData
  connect ( IConnectOptions ): Promise<void>
  getState (): number
  release (): number
}

export enum StateType {
    None = 0,
    Initialized = 1,
    OfferSent = 2,
    RequestReceived = 3,
    Accepted = 4,
    Unfulfilled = 5,
    Expired = 6,
    Revoked = 7
}

export interface IRecipientInfo {
  id: string,
  DIDself?: string,
  DIDremote?: string
}

export interface IConnectOptions {
  phone?: string,
  timeout?: number
}

export interface IConnectionData {
  source_id: string
  invite_detail: string,
  handle: number,
  pw_did: string,
  pw_verkey: string,
  did_endpoint: string,
  endpoint: string,
  uuid: string,
  wallet: string,
  state: string
}
