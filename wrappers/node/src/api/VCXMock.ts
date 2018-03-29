
import { rustAPI } from '../rustlib'

export enum VCXMockMessage {
  CreateKey = 1, // create keys response
  UpdateProfile = 2, // update profile response
  GetMessages = 3, // get_message response for connection acceptance
  UpdateCredential = 4, // get_message response for claim offer
  UpdateProof = 5, // get_message response for updating proof state
  CredentialReq = 6, // get_message response with claim req
  Proof = 7 // get_message response with proof
}

export class VCXMock {
  static setVcxMock (message: VCXMockMessage) {
    rustAPI().vcx_set_next_agency_response(message)
  }
}
