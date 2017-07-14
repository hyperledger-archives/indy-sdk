package org.hyperledger.indy.sdk;

import java.util.HashMap;
import java.util.Iterator;
import java.util.Map;
import java.util.concurrent.CompletableFuture;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.atomic.AtomicInteger;

import org.apache.commons.lang3.builder.EqualsBuilder;
import org.apache.commons.lang3.builder.HashCodeBuilder;
import org.apache.commons.lang3.builder.ToStringBuilder;
import org.apache.commons.lang3.builder.ToStringStyle;

/**
 * Common functionality for the APIs, JSON parameters, and results used
 * by the Java wrapper of libindy.
 */
public class IndyJava {

	/*
	 * API
	 */

	public static class API {

		/*
		 * FUTURES
		 */

		private static AtomicInteger atomicInteger = new AtomicInteger();
		private static Map<Integer, CompletableFuture<?>> futures = new ConcurrentHashMap<Integer, CompletableFuture<?>> ();

		protected static int newCommandHandle() {

			return Integer.valueOf(atomicInteger.incrementAndGet());
		}

		protected static int addFuture(CompletableFuture<?> future) {

			int commandHandle = newCommandHandle();
			assert(! futures.containsKey(Integer.valueOf(commandHandle)));
			futures.put(Integer.valueOf(commandHandle), future);

			return commandHandle;
		}

		protected static CompletableFuture<?> removeFuture(int xcommand_handle) {

			CompletableFuture<?> future = futures.remove(Integer.valueOf(xcommand_handle));
			assert(future != null);

			return future;
		}

		/*
		 * ERROR CHECKING
		 */

		protected static boolean checkCallback(CompletableFuture<?> future, int err) {

			ErrorCode errorCode = ErrorCode.valueOf(err);
			if (! ErrorCode.Success.equals(errorCode)) { future.completeExceptionally(new IndyException(errorCode)); return false; }

			return true;
		}

		protected static void checkCallback(int err) throws IndyException {

			ErrorCode errorCode = ErrorCode.valueOf(err);
			if (! ErrorCode.Success.equals(errorCode)) throw new IndyException(errorCode);
		}

		protected static void checkResult(int err) throws IndyException {

			ErrorCode errorCode = ErrorCode.valueOf(err);
			if (! ErrorCode.Success.equals(errorCode)) throw new IndyException(errorCode);
		}

		/*
		 * OBJECT METHODS
		 */

		@Override
		public int hashCode() {

			return HashCodeBuilder.reflectionHashCode(this, false);
		}

		@Override
		public boolean equals(Object other) {

			return EqualsBuilder.reflectionEquals(this, other, false);
		}

		@Override
		public String toString() {

			return ToStringBuilder.reflectionToString(this, ToStringStyle.SHORT_PREFIX_STYLE);
		}
	}

	/*
	 * JSON PARAMETER
	 */

	public abstract static class JsonParameter {

		protected Map<String, Object> map = new HashMap<String, Object> ();

		/*
		 * JSON CREATION
		 */

		public final String toJson() {

			StringBuilder builder = new StringBuilder();
			builder.append("{");

			for (Iterator<Map.Entry<String, Object>> iterator = this.map.entrySet().iterator(); iterator.hasNext(); ) {

				Map.Entry<String, Object> entry = iterator.next();
				String key = entry.getKey();
				Object value = entry.getValue();
				builder.append("\"" + key + "\":");
				if (value instanceof String) builder.append("\"" + escapeJson(value.toString()) + "\"");
				else if (value instanceof Boolean) builder.append(value.toString());
				else if (value instanceof Number) builder.append(value.toString());
				else if (value == null) builder.append("null");
				else throw new IllegalArgumentException("Invalid value type: " + value + " (" + value.getClass() + ")");
				if (iterator.hasNext()) builder.append(",");
			}

			builder.append("}");

			return builder.toString();
		}

		private static String escapeJson(String string) {

			return string.replace("\\", "\\\\").replace("\"", "\\\"");
		}

		/*
		 * OBJECT METHODS
		 */

		@Override
		public int hashCode() {

			return this.map.hashCode();
		}

		@Override
		public boolean equals(Object other) {

			return this.map.equals(other);
		}

		@Override
		public String toString() {

			return this.toJson();
		}
	}

	/*
	 * Result
	 */

	public abstract static class Result {

		@Override
		public int hashCode() {

			return HashCodeBuilder.reflectionHashCode(this, false);
		}

		@Override
		public boolean equals(Object other) {

			return EqualsBuilder.reflectionEquals(this, other, false);
		}

		@Override
		public String toString() {

			return ToStringBuilder.reflectionToString(this, ToStringStyle.SHORT_PREFIX_STYLE);
		}
	}
}
