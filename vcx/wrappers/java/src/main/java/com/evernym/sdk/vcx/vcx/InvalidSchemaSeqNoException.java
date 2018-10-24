package com.evernym.sdk.vcx.vcx;

import com.evernym.sdk.vcx.ErrorCode;
import com.evernym.sdk.vcx.VcxException;

/**
 * Created by abdussami on 13/06/18.
 */

public class InvalidSchemaSeqNoException extends VcxException
{
    private static final long serialVersionUID = 3294831240096535507L;
    private final static String message = "VCX Exception";


    public InvalidSchemaSeqNoException()
    {
        super(message, ErrorCode.INVALID_SCHEMA_SEQ_NO.value());
    }
}