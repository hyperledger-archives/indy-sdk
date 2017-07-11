package org.hyperledger.indy.sdk;

import java.util.HashMap;
import java.util.Iterator;
import java.util.Map;
import java.util.concurrent.CompletableFuture;

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

		protected static final int FIXED_COMMAND_HANDLE = 0;

		protected static boolean checkCallback(CompletableFuture<? extends IndyJava.Result> future, int xcommand_handle, int err) {

			assert(xcommand_handle == FIXED_COMMAND_HANDLE);

			ErrorCode errorCode = ErrorCode.valueOf(err);
			if (! ErrorCode.Success.equals(errorCode)) { future.completeExceptionally(IndyException.fromErrorCode(errorCode, err)); return false; }

			return true;
		}

		protected static boolean checkCallback(CompletableFuture<? extends IndyJava.Result> future, int err) {

			ErrorCode errorCode = ErrorCode.valueOf(err);
			if (! ErrorCode.Success.equals(errorCode)) { future.completeExceptionally(IndyException.fromErrorCode(errorCode, err)); return false; }

			return true;
		}

		protected static void checkResult(int result) throws IndyException {

			ErrorCode errorCode = ErrorCode.valueOf(result);
			if (! ErrorCode.Success.equals(errorCode)) throw IndyException.fromErrorCode(errorCode, result);
		}

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
	 * JSON parameter
	 */

	public abstract static class JsonParameter {

		protected Map<String, Object> map = new HashMap<String, Object> ();

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
