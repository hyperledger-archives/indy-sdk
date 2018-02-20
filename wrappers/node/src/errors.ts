// tslint:disable max-classes-per-file
export class ConnectionTimeoutError extends Error {}

export class VCXInternalError {
  private _message: string
  private _code: number

  constructor (code: number, message: string) {
    this._message = message
    this._code = code
  }

  get message () {
    return this._message
  }

  get code () {
    return this._code
  }
}
