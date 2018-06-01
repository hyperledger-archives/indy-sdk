package org.hyperledger.indy.sdk.payment;

import org.hyperledger.indy.sdk.InvalidStructureException;
import org.hyperledger.indy.sdk.payments.IncompatiblePaymentException;
import org.hyperledger.indy.sdk.payments.Payments;
import org.hyperledger.indy.sdk.payments.UnknownPaymentMethodException;
import org.junit.Test;

import java.util.concurrent.ExecutionException;

import static org.hamcrest.CoreMatchers.isA;

public class BuildPaymentRequestTest extends PaymentIntegrationTest {

	@Test
	public void testBuildPaymentRequestWorksForUnknownPaymentMethod() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(UnknownPaymentMethodException.class));

		Payments.buildPaymentRequest(wallet, DID_TRUSTEE, inputs, outputs).get();
	}

	@Test
	public void testBuildPaymentRequestWorksForEmptyInputs() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		Payments.buildPaymentRequest(wallet, DID_TRUSTEE, emptyArray, outputs).get();
	}

	@Test
	public void testBuildPaymentRequestWorksForEmptyOutputs() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		Payments.buildPaymentRequest(wallet, DID_TRUSTEE, inputs, emptyObject).get();
	}

	@Test
	public void testBuildPaymentRequestWorksForIncompatiblePaymentMethods() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(IncompatiblePaymentException.class));

		Payments.buildPaymentRequest(wallet, DID_TRUSTEE, incompatibleInputs, outputs).get();
	}

	@Test
	public void testBuildPaymentRequestWorksForInvalidInputs() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		Payments.buildPaymentRequest(wallet, DID_TRUSTEE, invalidInputs, outputs).get();
	}
}
