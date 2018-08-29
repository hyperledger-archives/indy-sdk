package com.evernym.sdk.vcx;

import com.evernym.sdk.vcx.connection.ConnectionErrorException;

import org.junit.Test;

import static org.junit.Assert.assertEquals;

/**
 * Created by abdussami on 31/07/18.
 */
public class VcxExceptionTest {

    @Test
    public void assertFromSDKErrorThrowsCorrectException(){
        VcxException excpetion = VcxException.fromSdkError(1002);
        assertEquals("Incorrect exception thrown for the error code",excpetion.getClass().getName(), ConnectionErrorException.class.getName());
    }

    @Test
    public void assertFromSDKErrorThrowsVcxExceptionForUnknownErrorCode(){
        VcxException excpetion = VcxException.fromSdkError(1);
        assertEquals("Incorrect exception thrown for the error code",excpetion.getClass().getName(), VcxException.class.getName());
    }

    @Test
    public void assertFromSDKErrorThrowsVcxExceptionWithCorrectCodeForUnknownErrorCode(){
        VcxException excpetion = VcxException.fromSdkError(1);
        assertEquals("Incorrect exception thrown for the error code",excpetion.getSdkErrorCode(), 1);
    }

    @Test
    public void assertFromSDKErrorThrowsVcxExceptionForNegetiveErrorCode(){
        VcxException excpetion = VcxException.fromSdkError(-1);
        assertEquals("Incorrect exception thrown for the error code",excpetion.getClass().getName(), VcxException.class.getName());
    }
}
