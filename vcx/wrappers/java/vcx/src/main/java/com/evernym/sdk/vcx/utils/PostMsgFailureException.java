package com.evernym.sdk.vcx.utils;

import com.evernym.sdk.vcx.ErrorCode;
import com.evernym.sdk.vcx.VcxException;

/**
 * Created by abdussami on 05/06/18.
 */

public class PostMsgFailureException extends VcxException
{
    private static final long serialVersionUID = 3294831240096535507L;
    private final static String message = "VCX Exception";


    public PostMsgFailureException()
    {
        super(message, ErrorCode.POST_MSG_FAILURE.value());
    }
}