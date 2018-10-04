package com.evernym.sdk.vcx;


import org.junit.jupiter.api.Test;

import static com.evernym.sdk.vcx.ErrorCode.CONNECTION_ERROR;
import static com.evernym.sdk.vcx.ErrorCode.UNIDENTIFIED_ERROR_CODE;

import static org.junit.jupiter.api.Assertions.assertEquals;

public class ErrorCodeTest {

    @Test
    public void assertUnknownErrorCodeisHandled(){
        ErrorCode errorCode = ErrorCode.valueOf(1);
        assertEquals(UNIDENTIFIED_ERROR_CODE,errorCode);
    }

    @Test
    public void assertKnownErrorCodeisHandled(){
        ErrorCode errorCode = ErrorCode.valueOf(1002);
        assertEquals(CONNECTION_ERROR,errorCode);
    }
}
