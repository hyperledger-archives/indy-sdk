package org.hyperledger.indy.sdk;

import org.hyperledger.indy.sdk.utils.InitHelper;
import org.hyperledger.indy.sdk.utils.StorageUtils;
import org.junit.After;
import org.junit.Before;
import org.junit.Rule;
import org.junit.rules.ExpectedException;
import org.junit.rules.Timeout;

import java.util.concurrent.TimeUnit;

public class IndyIntegrationTest {
	@Rule
	public ExpectedException thrown = ExpectedException.none();

	@Rule
	public Timeout globalTimeout = new Timeout(1, TimeUnit.SECONDS);

	@Before
	public void setUp() throws Exception {
		InitHelper.init();
		StorageUtils.cleanupStorage();
	}

	@After
	public void tearDown() throws Exception {
		StorageUtils.cleanupStorage();
	}
}
