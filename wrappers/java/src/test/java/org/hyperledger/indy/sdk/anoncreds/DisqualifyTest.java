package org.hyperledger.indy.sdk.anoncreds;

import org.junit.Test;

import static org.junit.Assert.assertEquals;


public class DisqualifyTest extends AnoncredsIntegrationTest {

	@Test
	public void testDisqualifyWorks() throws Exception {
		String qualified = "did:sov:NcYxiDXkpYi6ov5FcYDi1e";
		String unqualified = "NcYxiDXkpYi6ov5FcYDi1e";
		assertEquals(unqualified, Anoncreds.disqualify(qualified).get());
		assertEquals(unqualified, Anoncreds.disqualify(unqualified).get());

	}
}
