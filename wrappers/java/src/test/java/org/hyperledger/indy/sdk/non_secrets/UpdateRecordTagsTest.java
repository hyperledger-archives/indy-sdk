package org.hyperledger.indy.sdk.non_secrets;

import org.hyperledger.indy.sdk.wallet.WalletItemNotFoundException;
import org.junit.Test;

import java.util.concurrent.ExecutionException;

import static org.hamcrest.CoreMatchers.isA;


public class UpdateRecordTagsTest extends NonSecretsIntegrationTest {

	@Test
	public void testUpdateRecordTagsWorks() throws Exception {
		WalletRecord.add(wallet, type, id, value, tagsEmpty).get();

		checkRecordField(wallet, type, id, "tags", tagsEmpty);

		WalletRecord.updateTags(wallet, type, id, tags).get();

		checkRecordField(wallet, type, id, "tags", tags);
	}

	@Test
	public void testUpdateRecordTagsWorksForTwice() throws Exception {
		WalletRecord.add(wallet, type, id, value, tagsEmpty).get();

		checkRecordField(wallet, type, id, "tags", tagsEmpty);

		WalletRecord.updateTags(wallet, type, id, tags).get();

		checkRecordField(wallet, type, id, "tags", tags);

		WalletRecord.updateTags(wallet, type, id, tags2).get();

		checkRecordField(wallet, type, id, "tags", tags2);
	}

	@Test
	public void testUpdateRecordTagsWorksForNotFoundRecord() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(WalletItemNotFoundException.class));

		WalletRecord.updateTags(wallet, type, id, tags).get();
	}
}