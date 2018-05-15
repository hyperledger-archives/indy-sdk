package org.hyperledger.indy.sdk.non_secrets;

import org.hyperledger.indy.sdk.wallet.WalletItemNotFoundException;
import org.junit.Test;

import java.util.concurrent.ExecutionException;

import static org.hamcrest.CoreMatchers.isA;


public class AddRecordTagsTest extends NonSecretsIntegrationTest {

	@Test
	public void testAddRecordTagsWorks() throws Exception {
		WalletRecord.add(wallet, type, id, value, tagsEmpty).get();

		checkRecordField(wallet, type, id, "tags", tagsEmpty);

		WalletRecord.addTags(wallet, type, id, tags).get();

		checkRecordField(wallet, type, id, "tags", tags);
	}

	@Test
	public void testAddRecordTagsWorksForTwice() throws Exception {
		WalletRecord.add(wallet, type, id, value, tagsEmpty).get();

		checkRecordField(wallet, type, id, "tags", tagsEmpty);

		String tags1 = "{\"tagName1\": \"str1\"}";
		WalletRecord.addTags(wallet, type, id, tags1).get();

		checkRecordField(wallet, type, id, "tags", tags1);

		String tags2 = "{\"tagName2\": \"str2\"}";
		WalletRecord.addTags(wallet, type, id, tags2).get();

		String expectedTags = "{\"tagName1\": \"str1\", \"tagName2\": \"str2\"}";
		checkRecordField(wallet, type, id, "tags", expectedTags);
	}

	@Test
	public void testAddRecordTagsWorksForNotFoundRecord() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(WalletItemNotFoundException.class));

		WalletRecord.addTags(wallet, type, id, tags).get();
	}
}