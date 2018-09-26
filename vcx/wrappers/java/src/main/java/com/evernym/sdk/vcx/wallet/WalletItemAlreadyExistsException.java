package com.evernym.sdk.vcx.wallet;

import com.evernym.sdk.vcx.ErrorCode;
import com.evernym.sdk.vcx.VcxException;

/**
 * Created by naga on 17/07/18.
 */

public class WalletItemAlreadyExistsException extends VcxException
{
    private static final long serialVersionUID = 3294831340096535507L;
    private final static String message = "The wallet record with same id already exists.";


    public WalletItemAlreadyExistsException()
    {
        super(message, ErrorCode.WALLET_ITEM_CANNOT_ADD.value());
    }
}