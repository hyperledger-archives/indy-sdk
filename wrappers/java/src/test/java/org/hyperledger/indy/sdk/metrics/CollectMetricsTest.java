package org.hyperledger.indy.sdk.metrics;

import org.hyperledger.indy.sdk.IndyIntegrationTest;
import org.json.JSONObject;
import org.junit.Test;

import java.math.BigInteger;
import java.util.Map;
import static org.junit.Assert.assertNotNull;


public class CollectMetricsTest extends IndyIntegrationTest {

	@Test
	public void testCollectMetricsMethod() throws Exception {
		String metricsResult = Metrics.collectMetrics().get();
		assertNotNull(metricsResult);
		Map<String, Object> metricMap = (new JSONObject(metricsResult)).toMap();
		assert(metricMap.containsKey("threadpool_threads_count"));
	}
}
