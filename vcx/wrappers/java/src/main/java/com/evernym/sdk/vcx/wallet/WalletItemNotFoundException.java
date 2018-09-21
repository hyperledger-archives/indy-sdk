package com.evernym.sdk.vcx.wallet;

import com.evernym.sdk.vcx.ErrorCode;
import com.evernym.sdk.vcx.VcxException;

/**
 * Created by naga on 17/07/18.
 */

public class WalletItemNotFoundException extends VcxException
{
    private static final long serialVersionUID = 3294831430096535507L;
    private final static String message = "The wallet record with this id not found.";


    public WalletItemNotFoundException()
    {
        super(message, ErrorCode.WALLET_ITEM_NOT_FOUND.value());
    }
}