package com.evernym.sdk.vcx.proof;

import com.evernym.sdk.vcx.ErrorCode;
import com.evernym.sdk.vcx.VcxException;

/**
 * Created by abdussami on 05/06/18.
 */

public class InvalidProofCredentialDataException extends VcxException
{
    private static final long serialVersionUID = 3294831240096535507L;
    private final static String message = "VCX Exception";


    public InvalidProofCredentialDataException()
    {
        super(message, ErrorCode.INVALID_PROOF_CREDENTIAL_DATA.value());

    }
}