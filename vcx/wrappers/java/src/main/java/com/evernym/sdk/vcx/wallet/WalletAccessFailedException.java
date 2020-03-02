package com.evernym.sdk.vcx.wallet;

import com.evernym.sdk.vcx.ErrorCode;
import com.evernym.sdk.vcx.VcxException;

public class WalletAccessFailedException extends VcxException
{
    private static final long serialVersionUID = 3294831240096535507L;
    private final static String message = "Attempt to open wallet with invalid credentials";


    public WalletAccessFailedException()
    {
        super(message, ErrorCode.WALLET_ACCESS_FAILED.value());
    }
}