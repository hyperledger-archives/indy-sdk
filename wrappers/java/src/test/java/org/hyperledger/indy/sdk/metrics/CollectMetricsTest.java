package org.hyperledger.indy.sdk.metrics;

import org.hyperledger.indy.sdk.IndyIntegrationTest;
import org.junit.Test;
import static org.junit.Assert.assertNotNull;


public class CollectMetricsTest extends IndyIntegrationTest {

	@Test
	public void testCollectMetricsMethod() throws Exception {
		String metrics_map = Metrics.collectMetrics().get();
		assertNotNull(metrics_map);

	}
}
