export interface IConnections {
  create ( recipientInfo: string ): number
  connect (): number
  get_data (): string
  get_state (): number
  release (): number
}
