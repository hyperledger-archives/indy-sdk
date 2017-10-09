export interface IConnections {
  create ( recipientInfo: string ): number
  connect (): number
  get_data (): string
  get_state (): number
  // static release (): number
  list_state (): number
}
