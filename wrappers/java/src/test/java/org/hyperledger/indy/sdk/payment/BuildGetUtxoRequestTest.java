package org.hyperledger.indy.sdk.payment;

import org.hyperledger.indy.sdk.payments.IncompatiblePaymentException;
import org.hyperledger.indy.sdk.payments.Payments;
import org.hyperledger.indy.sdk.payments.UnknownPaymentMethodException;
import org.junit.Test;

import java.util.concurrent.ExecutionException;

import static org.hamcrest.CoreMatchers.isA;

public class BuildGetUtxoRequestTest extends PaymentIntegrationTest {

	@Test
	public void testBuildGetUtxoRequestWorksForUnknownPaymentMethod() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(UnknownPaymentMethodException.class));

		Payments.buildGetUtxoRequest(wallet, DID_TRUSTEE, paymentAddress).get();
	}

	@Test
	public void testBuildGetUtxoRequestWorksForInvalidPaymentAddress() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(IncompatiblePaymentException.class));

		Payments.buildGetUtxoRequest(wallet, DID_TRUSTEE, "pay:null1").get();
	}
}
