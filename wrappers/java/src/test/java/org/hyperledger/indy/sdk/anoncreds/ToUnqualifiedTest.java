package org.hyperledger.indy.sdk.anoncreds;

import org.junit.Test;

import static org.junit.Assert.assertEquals;


public class ToUnqualifiedTest extends AnoncredsIntegrationTest {

	@Test
	public void testToUnqualifiedWorks() throws Exception {
		String qualified = "did:sov:NcYxiDXkpYi6ov5FcYDi1e";
		String unqualified = "NcYxiDXkpYi6ov5FcYDi1e";
		assertEquals(unqualified, Anoncreds.toUnqualified(qualified).get());
		assertEquals(unqualified, Anoncreds.toUnqualified(unqualified).get());

	}
}
