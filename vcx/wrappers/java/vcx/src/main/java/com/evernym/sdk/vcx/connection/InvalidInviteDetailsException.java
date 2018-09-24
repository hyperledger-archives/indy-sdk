package com.evernym.sdk.vcx.connection;

import com.evernym.sdk.vcx.ErrorCode;
import com.evernym.sdk.vcx.VcxException;

/**
 * Created by abdussami on 13/06/18.
 */

public class InvalidInviteDetailsException extends VcxException
{
    private static final long serialVersionUID = 3294831240096535507L;
    private final static String message = "Invalid invite details";


    public InvalidInviteDetailsException()
    {
        super(message, ErrorCode.INVALID_INVITE_DETAILS.value());
    }
}
