package com.evernym.sdk.vcx.proof;

import com.evernym.sdk.vcx.ErrorCode;
import com.evernym.sdk.vcx.VcxException;

/**
 * Created by abdussami on 13/06/18.
 */

public class FailedProofComplianceException extends VcxException
{
    private static final long serialVersionUID = 3294831240096535507L;
    private final static String message = "VCX Exception";


    public FailedProofComplianceException()
    {
        super(message, ErrorCode.FAILED_PROOF_COMPLIANCE.value());
    }
}