package com.evernym.sdk.vcx.wallet;

import com.evernym.sdk.vcx.ErrorCode;
import com.evernym.sdk.vcx.VcxException;

/**
 * Created by abdussami on 13/06/18.
 */

public class WalletAlreadyExistsException extends VcxException
{
    private static final long serialVersionUID = 3294831240096535507L;
    private final static String message = "The wallet already exists.";


    public WalletAlreadyExistsException()
    {
        super(message, ErrorCode.WALLET_ALREADY_EXISTS.value());
    }
}