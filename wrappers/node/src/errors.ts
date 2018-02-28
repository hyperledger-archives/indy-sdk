// tslint:disable max-classes-per-file
export class ConnectionTimeoutError extends Error {}

export class VCXInternalError extends Error {
  readonly vcxCode: number
  readonly vcxFunction: string

  constructor (code: number, message: string, fn: string) {
    super(message)
    this.vcxCode = code
    this.vcxFunction = fn
  }
}
