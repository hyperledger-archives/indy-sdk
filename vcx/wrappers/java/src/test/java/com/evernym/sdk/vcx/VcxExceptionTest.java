package com.evernym.sdk.vcx;

import com.evernym.sdk.vcx.connection.ConnectionErrorException;
import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;


/**
 * Created by abdussami on 31/07/18.
 */
public class VcxExceptionTest {

    @Test
    public void assertFromSDKErrorThrowsCorrectException(){
        VcxException excpetion = VcxException.fromSdkError(1002);
        assertEquals(excpetion.getClass().getName(), ConnectionErrorException.class.getName());
    }

    @Test
    public void assertFromSDKErrorThrowsVcxExceptionForUnknownErrorCode(){
        VcxException excpetion = VcxException.fromSdkError(1);
        assertEquals(excpetion.getClass().getName(), VcxException.class.getName());
    }

    @Test
    public void assertFromSDKErrorThrowsVcxExceptionWithCorrectCodeForUnknownErrorCode(){
        VcxException excpetion = VcxException.fromSdkError(1);
        assertEquals(excpetion.getSdkErrorCode(), 1);
    }

    @Test
    public void assertFromSDKErrorThrowsVcxExceptionForNegetiveErrorCode(){
        VcxException excpetion = VcxException.fromSdkError(-1);
        assertEquals(excpetion.getClass().getName(), VcxException.class.getName());
    }
}
