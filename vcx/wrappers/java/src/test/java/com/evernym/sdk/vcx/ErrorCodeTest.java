package com.evernym.sdk.vcx;


import com.evernym.sdk.vcx.connection.ConnectionApi;
import java.util.concurrent.CompletableFuture;
import org.awaitility.Awaitility;
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

    @Test
    public void testGetErrorDetails(){
        try {
            CompletableFuture<String> future = ConnectionApi.connectionSerialize(0);
            Awaitility.await().until(future::isDone);
        } catch (VcxException e){
            assert(!e.getMessage().isEmpty());
            assert(!e.getSdkMessage().isEmpty());
            assert(!e.getSdkFullMessage().isEmpty());
            assert(!e.getSdkCause().isEmpty());
        }
    }
}
