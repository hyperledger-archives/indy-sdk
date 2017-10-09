export interface IConnections {
  create ( recipientInfo: string ): number
  connect (): number
  getData (): string
  getState (): number
  release (): number
}
