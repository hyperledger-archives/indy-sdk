package org.hyperledger.indy.sdk.wallet;


import org.hyperledger.indy.sdk.ErrorCode;
import org.hyperledger.indy.sdk.ErrorCodeMatcher;
import org.hyperledger.indy.sdk.IndyIntegrationTest;

import org.junit.Test;

import java.util.concurrent.ExecutionException;


public class RegisterWalletTypeTest extends IndyIntegrationTest {

	@Test
	public void testRegisterWalletTypeWorks() throws Exception {

		Wallet.registerWalletType("inmem", new InMemWalletType(), false).get();
	}

	@Test
	public void testRegisterWalletTypeDoesNotWorkForTwiceWithSameName() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.WalletTypeAlreadyRegisteredError));

		String type = "inmem";

		Wallet.registerWalletType(type, new InMemWalletType(), false).get();
		Wallet.registerWalletType(type, new InMemWalletType(), true).get();
	}
}
