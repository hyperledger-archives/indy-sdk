// tslint:disable max-classes-per-file
export class ConnectionTimeoutError extends Error {}

export class VCXInternalError extends Error {
  readonly vcxCode: number

  constructor (code: number, message: string) {
    super(message)
    this.vcxCode = code
  }
}
