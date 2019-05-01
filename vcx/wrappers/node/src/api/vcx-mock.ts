
import { rustAPI } from '../rustlib'

export enum VCXMockMessage {
  CreateKey = 1, // create keys response
  UpdateProfile = 2, // update profile response
  GetMessages = 3, // get_message response for connection acceptance
  UpdateIssuerCredential = 4, // get_message response for claim offer
  UpdateProof = 5, // get_message response for updating proof state
  IssuerCredentialReq = 6, // get_message response with claim req
  Proof = 7, // get_message response with proof,
  CredentialResponse = 8, // reply to credential request with an actual credential
  AcceptInvite = 9 // connection invite was accepted
}

export class VCXMock {
  public static setVcxMock (message: VCXMockMessage) {
    rustAPI().vcx_set_next_agency_response(message)
  }

  public static mintTokens (): void {
    rustAPI().vcx_mint_tokens(null, null)
  }
}
