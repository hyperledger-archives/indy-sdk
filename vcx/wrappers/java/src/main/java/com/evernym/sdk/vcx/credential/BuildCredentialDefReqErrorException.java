package com.evernym.sdk.vcx.credential;

import com.evernym.sdk.vcx.ErrorCode;
import com.evernym.sdk.vcx.VcxException;

/**
 * Created by abdussami on 05/06/18.
 */

public class BuildCredentialDefReqErrorException extends VcxException
{
    private static final long serialVersionUID = 3294831240096535507L;
    private final static String message = "BuildCredentialDefReqErrorException";


    public BuildCredentialDefReqErrorException()
    {
        super(message, ErrorCode.BUILD_CREDENTIAL_DEF_REQ_ERR.value());
    }
}
