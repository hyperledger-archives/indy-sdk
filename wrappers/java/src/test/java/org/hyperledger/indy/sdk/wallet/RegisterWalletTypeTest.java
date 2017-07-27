package org.hyperledger.indy.sdk.wallet;


import org.hyperledger.indy.sdk.ErrorCode;
import org.hyperledger.indy.sdk.ErrorCodeMatcher;
import org.hyperledger.indy.sdk.IndyIntegrationTest;

import org.junit.Test;

import java.util.concurrent.ExecutionException;


public class RegisterWalletTypeTest extends IndyIntegrationTest {

	@Test
	public void testRegisterWalletTypeWorks() throws Exception {
		WalletTypeInmem.getInstance().clear();

		Wallet.registerWalletType("inmem", WalletTypeInmem.getInstance(), false).get();

		WalletTypeInmem.getInstance().clear();
	}

	@Test
	public void testRegisterWalletTypeDoesNotWorkForTwiceWithSameName() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.WalletTypeAlreadyRegisteredError));

		String type = "inmem";

		Wallet.registerWalletType(type, WalletTypeInmem.getInstance(), false).get();
		Wallet.registerWalletType(type, WalletTypeInmem.getInstance(), true).get();
	}
}
