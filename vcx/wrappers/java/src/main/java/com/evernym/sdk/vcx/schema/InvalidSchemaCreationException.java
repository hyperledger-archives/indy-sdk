package com.evernym.sdk.vcx.schema;

import com.evernym.sdk.vcx.ErrorCode;
import com.evernym.sdk.vcx.VcxException;

/**
 * Created by abdussami on 13/06/18.
 */

public class InvalidSchemaCreationException extends VcxException
{
    private static final long serialVersionUID = 3294831240096535507L;
    private final static String message = "VCX Exception";


    public InvalidSchemaCreationException()
    {
        super(message, ErrorCode.INVALID_SCHEMA_CREATION.value());
    }
}