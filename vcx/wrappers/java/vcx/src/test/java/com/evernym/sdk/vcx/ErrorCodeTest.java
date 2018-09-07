package com.evernym.sdk.vcx;

import org.junit.Test;

import static com.evernym.sdk.vcx.ErrorCode.CONNECTION_ERROR;
import static com.evernym.sdk.vcx.ErrorCode.UNIDENTIFIED_ERROR_CODE;
import static org.junit.Assert.assertEquals;

public class ErrorCodeTest {

    @Test
    public void assertUnknownErrorCodeisHandled(){
        ErrorCode errorCode = ErrorCode.valueOf(1);
        assertEquals("Unable to handle unkown error code",errorCode,UNIDENTIFIED_ERROR_CODE);
    }

    @Test
    public void assertKnownErrorCodeisHandled(){
        ErrorCode errorCode = ErrorCode.valueOf(1002);
        assertEquals("Unable to find known error code",errorCode,CONNECTION_ERROR);
    }
}
