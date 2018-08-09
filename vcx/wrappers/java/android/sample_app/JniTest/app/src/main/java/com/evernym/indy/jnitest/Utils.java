package com.evernym.indy.jnitest;

import java.util.HashMap;
import java.util.Map;
import java.util.concurrent.atomic.AtomicInteger;

import java9.util.concurrent.CompletableFuture;

/**
 * Created by abdussami on 24/04/18.
 */

public class Utils {
    private static AtomicInteger atomicInteger = new AtomicInteger();
    private static Map<Integer, CompletableFuture<?>> futures = new HashMap<Integer, CompletableFuture<?>>();

    protected static int newCommandHandle() {

        return Integer.valueOf(atomicInteger.incrementAndGet());
    }
    protected static boolean checkCallback(CompletableFuture<?> future, int err) {

        ErrorCode errorCode = ErrorCode.valueOf(err);
        if (! ErrorCode.Success.equals(errorCode)) { future.completeExceptionally(IndyException.fromSdkError(err)); return false; }

        return true;
    }
    protected static CompletableFuture<?> removeFuture(int xcommand_handle) {

        CompletableFuture<?> future = futures.remove(Integer.valueOf(xcommand_handle));
        assert(future != null);

        return future;
    }
    protected static int addFuture(CompletableFuture<?> future) {

        int commandHandle = newCommandHandle();
        assert(! futures.containsKey(Integer.valueOf(commandHandle)));
        futures.put(Integer.valueOf(commandHandle), future);

        return commandHandle;
    }

}
