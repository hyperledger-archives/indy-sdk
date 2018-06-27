package org.hyperledger.indy.sdk.payments;

import com.sun.jna.Callback;
import org.hyperledger.indy.sdk.IndyException;
import org.hyperledger.indy.sdk.IndyJava;
import org.hyperledger.indy.sdk.LibIndy;
import org.hyperledger.indy.sdk.ParamGuard;
import org.hyperledger.indy.sdk.payments.PaymentsResults.*;
import org.hyperledger.indy.sdk.wallet.Wallet;

import java.util.concurrent.CompletableFuture;

public class Payments extends IndyJava.API {

    private Payments() {

    }

    /*
     * STATIC CALLBACKS
     */

    /**
     * Callback usedwhen method with string result completes
     */
    private static Callback stringCompleteCb = new Callback() {

        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int xcommandHandle, int err, String paymentAddress) {
            CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(xcommandHandle);
            if (!checkCallback(future, err)) return;

            future.complete(paymentAddress);
        }
    };

    /**
     * Callback used when addRequestFees completes.
     */
    private static Callback addRequestFeesCb = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int xcommandHandle, int err, String reqWithFeesJson, String paymentMethod) {
            CompletableFuture<AddRequestFeesResult> future = (CompletableFuture<AddRequestFeesResult>) removeFuture(xcommandHandle);
            if (!checkCallback(future, err)) return;

            AddRequestFeesResult addRequestFeesResult = new AddRequestFeesResult(reqWithFeesJson, paymentMethod);

            future.complete(addRequestFeesResult);
        }
    };

    /**
     * Callback used when buildGetUtxoRequest completes.
     */
    private static Callback buildGetUtxoRequestCb = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int xcommandHandle, int err, String utxoJson, String paymentMethod) {
            CompletableFuture<BuildGetUtxoRequestResult> future = (CompletableFuture<BuildGetUtxoRequestResult>) removeFuture(xcommandHandle);
            if (!checkCallback(future, err)) return;

            BuildGetUtxoRequestResult addRequestFeesResult = new BuildGetUtxoRequestResult(utxoJson, paymentMethod);

            future.complete(addRequestFeesResult);
        }
    };

    /**
     * Callback used when buildPaymentRequest completes.
     */
    private static Callback buildPaymentReqCb = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int xcommandHandle, int err, String paymentReqJson, String paymentMethod) {
            CompletableFuture<BuildPaymentReqResult> future = (CompletableFuture<BuildPaymentReqResult>) removeFuture(xcommandHandle);
            if (!checkCallback(future, err)) return;

            BuildPaymentReqResult addRequestFeesResult = new BuildPaymentReqResult(paymentReqJson, paymentMethod);

            future.complete(addRequestFeesResult);
        }
    };

    /**
     * Callback used when buildMintRequest completes.
     */
    private static Callback buildMintReqCb = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int xcommandHandle, int err, String mintReqJson, String paymentMethod) {
            CompletableFuture<BuildMintReqResult> future = (CompletableFuture<BuildMintReqResult>) removeFuture(xcommandHandle);
            if (!checkCallback(future, err)) return;

            BuildMintReqResult addRequestFeesResult = new BuildMintReqResult(mintReqJson, paymentMethod);

            future.complete(addRequestFeesResult);
        }
    };

    /*
     * STATIC METHODS
     */

    /**
     * Create the payment address for specified payment method
     *
     * This method generates private part of payment address
     * and stores it in a secure place. Ideally it should be
     * secret in libindy wallet (see crypto module).
     *
     * Note that payment method should be able to resolve this
     * secret by fully resolvable payment address format.
     *
     * Note this endpoint is EXPERIMENTAL. Function signature and behaviour may change
     * in the future releases.
     * @param wallet The wallet.
     * @param paymentMethod Payment method to use (for example, 'sov')
     * @param config payment address config as json:
     *               {
     *                  seed: <str>, // allows deterministic creation of payment address
     *               }
     * @return public identifier of payment address in fully resolvable payment address format
     * @throws IndyException
     */
    public static CompletableFuture<String> createPaymentAddress(
            Wallet wallet,
            String paymentMethod,
            String config
    ) throws IndyException {
        ParamGuard.notNullOrWhiteSpace(paymentMethod, "paymentMethod");
        ParamGuard.notNullOrWhiteSpace(config, "config");

        CompletableFuture<String> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);

        int walletHandle = wallet.getWalletHandle();

        int result = LibIndy.api.indy_create_payment_address(
                commandHandle,
                walletHandle,
                paymentMethod,
                config,
                stringCompleteCb
        );

        checkResult(result);

        return future;
    }

    /**
     * Lists all payment addresses that are stored in the wallet
     *
     * Note this endpoint is EXPERIMENTAL. Function signature and behaviour may change
     * in the future releases.
     * @param wallet The wallet.
     * @return json array of string with json addresses
     * @throws IndyException
     */
    public static CompletableFuture<String> listPaymentAddresses(
            Wallet wallet
    ) throws IndyException {
        CompletableFuture<String> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);

        int walletHandle = wallet.getWalletHandle();

        int result = LibIndy.api.indy_list_payment_addresses(
                commandHandle,
                walletHandle,
                stringCompleteCb
        );

        checkResult(result);

        return future;
    }

    /**
     * Modifies Indy request by adding information how to pay fees for this transaction
     * according to selected payment method.
     *
     * Payment selection is performed by looking to o
     *
     * This method consumes set of UTXO inputs and outputs. The difference between inputs balance
     * and outputs balance is the fee for this transaction.
     *
     * Not that this method also produces correct fee signatures.
     *
     * Format of inputs is specific for payment method. Usually it should reference payment transaction
     * with at least one output that corresponds to payment address that user owns.
     *
     * Note this endpoint is EXPERIMENTAL. Function signature and behaviour may change
     * in the future releases.
     * @param wallet The wallet.
     * @param submitterDid DID of request sender
     * @param reqJson initial transaction request as json
     * @param inputsJson The list of UTXO inputs as json array:
     *                   ["input1", ...]
     *                   Notes:
     *                      - each input should reference paymentAddress
     *                      - this param will be used to determine payment_method
     * @param outputsJson The list of UTXO outputs as json array:
     *                    [{
     *                      paymentAddress: <str>, // payment address used as output
     *                      amount: <int>, // amount of tokens to transfer to this payment address
     *                      extra: <str>, // optional data
     *                    }]
     * @return modified Indy request with added fees info
     * @throws IndyException
     */
    public static CompletableFuture<AddRequestFeesResult> addRequestFees(
            Wallet wallet,
            String submitterDid,
            String reqJson,
            String inputsJson,
            String outputsJson
    ) throws IndyException {
        ParamGuard.notNullOrWhiteSpace(submitterDid, "submitterDid");
        ParamGuard.notNullOrWhiteSpace(reqJson, "reqJson");
        ParamGuard.notNullOrWhiteSpace(inputsJson, "inputsJson");
        ParamGuard.notNullOrWhiteSpace(outputsJson, "outputsJson");

        CompletableFuture<AddRequestFeesResult> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);

        int walletHandle = wallet.getWalletHandle();

        int result = LibIndy.api.indy_add_request_fees(
                commandHandle,
                walletHandle,
                submitterDid,
                reqJson,
                inputsJson,
                outputsJson,
                addRequestFeesCb);

        checkResult(result);

        return future;
    }

    /**
     * Parses response for Indy request with fees.
     *
     * Note this endpoint is EXPERIMENTAL. Function signature and behaviour may change
     * in the future releases.
     * @param paymentMethod
     * @param respJson response for Indy request with fees
     * @return parsed (payment method and node version agnostic) utxo info as json:
     *          [{
     *              input: <str>, // UTXO input
     *              amount: <int>, // amount of tokens in this input
     *              extra: <str>, // optional data from payment transaction
     *          }]
     * @throws IndyException
     */
    public static CompletableFuture<String> parseResponseWithFees(
            String paymentMethod,
            String respJson
    ) throws IndyException {
        return parseResponse(paymentMethod, respJson, LibIndy.api::indy_parse_response_with_fees);
    }

    /**
     * Builds Indy request for getting UTXO list for payment address
     * according to this payment method.
     *
     * Note this endpoint is EXPERIMENTAL. Function signature and behaviour may change
     * in the future releases.
     * @param wallet The wallet.
     * @param submitterDid DID of request sender
     * @param paymentAddress target payment address
     * @return Indy request for getting UTXO list for payment address
     * @throws IndyException
     */
    public static CompletableFuture<BuildGetUtxoRequestResult> buildGetUtxoRequest(
            Wallet wallet,
            String submitterDid,
            String paymentAddress
    ) throws IndyException {
        ParamGuard.notNullOrWhiteSpace(submitterDid, "submitterDid");
        ParamGuard.notNullOrWhiteSpace(paymentAddress, "paymentAddress");

        CompletableFuture<BuildGetUtxoRequestResult> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);

        int walletHandle = wallet.getWalletHandle();

        int result = LibIndy.api.indy_build_get_utxo_request(
                commandHandle,
                walletHandle,
                submitterDid,
                paymentAddress,
                buildGetUtxoRequestCb);

        checkResult(result);

        return future;
    }

    /**
     * Parses response for Indy request for getting UTXO list.
     *
     * Note this endpoint is EXPERIMENTAL. Function signature and behaviour may change
     * in the future releases.
     * @param paymentMethod
     * @param respJson response for Indy request for getting UTXO list
     * @return parsed (payment method and node version agnostic) utxo info as json:
     * [{
     *    input: <str>, // UTXO input
     *    amount: <int>, // amount of tokens in this input
     *    extra: <str>, // optional data from payment transaction
     * }]
     * @throws IndyException
     */
    public static CompletableFuture<String> parseGetUtxoResponse(
            String paymentMethod,
            String respJson
    ) throws IndyException {
        return parseResponse(paymentMethod, respJson, LibIndy.api::indy_parse_get_utxo_response);
    }

    /**
     * Builds Indy request for doing tokens payment
     * according to this payment method.
     *
     * This method consumes set of UTXO inputs and outputs.
     *
     * Format of inputs is specific for payment method. Usually it should reference payment transaction
     * with at least one output that corresponds to payment address that user owns.
     *
     * Note this endpoint is EXPERIMENTAL. Function signature and behaviour may change
     * in the future releases.
     * @param wallet The wallet.
     * @param submitterDid DID of request sender
     * @param inputsJson The list of UTXO inputs as json array:
     *                      ["input1", ...]
     *                      Note that each input should reference paymentAddress
     * @param outputsJson The list of UTXO outputs as json array:
     *                      [{
     *                        paymentAddress: <str>, // payment address used as output
     *                        amount: <int>, // amount of tokens to transfer to this payment address
     *                        extra: <str>, // optional data
     *                      }]
     * @return
     * payment_req_json - Indy request for doing tokens payment
     * payment_method
     * @throws IndyException
     */
    public static CompletableFuture<BuildPaymentReqResult> buildPaymentRequest(
            Wallet wallet,
            String submitterDid,
            String inputsJson,
            String outputsJson
    ) throws IndyException {
        ParamGuard.notNullOrWhiteSpace(submitterDid, "submitterDid");
        ParamGuard.notNullOrWhiteSpace(inputsJson, "inputsJson");
        ParamGuard.notNullOrWhiteSpace(outputsJson, "outputsJson");

        CompletableFuture<BuildPaymentReqResult> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);

        int walletHandle = wallet.getWalletHandle();

        int result = LibIndy.api.indy_build_payment_req(
                commandHandle,
                walletHandle,
                submitterDid,
                inputsJson,
                outputsJson,
                buildPaymentReqCb);

        checkResult(result);

        return future;
    }

    /**
     * Parses response for Indy request for payment txn.
     *
     * Note this endpoint is EXPERIMENTAL. Function signature and behaviour may change
     * in the future releases.
     * @param paymentMethod
     * @param respJson response for Indy request for payment txn
     * @return parsed (payment method and node version agnostic) utxo info as json:
     * [{
     *    input: <str>, // UTXO input
     *    amount: <int>, // amount of tokens in this input
     *    extra: <str>, // optional data from payment transaction
     * }]
     * @throws IndyException
     */
    public static CompletableFuture<String> parsePaymentResponse (
            String paymentMethod,
            String respJson
    ) throws IndyException {
        return parseResponse(paymentMethod, respJson, LibIndy.api::indy_parse_payment_response);
    }

    /**
     * Builds Indy request for doing tokens minting
     * according to this payment method.
     *
     * Note this endpoint is EXPERIMENTAL. Function signature and behaviour may change
     * in the future releases.
     *
     * @param wallet The wallet.
     * @param submitterDid DID of request sender
     * @param outputsJson The list of UTXO outputs as json array:
     *                      [{
     *                        paymentAddress: <str>, // payment address used as output
     *                        amount: <int>, // amount of tokens to transfer to this payment address
     *                        extra: <str>, // optional data
     *                      }]
     * @return Indy request for doing tokens minting
     * @throws IndyException
     */
    public static CompletableFuture<BuildMintReqResult> buildMintRequest(
            Wallet wallet,
            String submitterDid,
            String outputsJson
    ) throws IndyException {
        ParamGuard.notNullOrWhiteSpace(submitterDid, "submitterDid");
        ParamGuard.notNullOrWhiteSpace(outputsJson, "outputsJson");

        CompletableFuture<BuildMintReqResult> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);

        int walletHandle = wallet.getWalletHandle();

        int result = LibIndy.api.indy_build_mint_req(
                commandHandle,
                walletHandle,
                submitterDid,
                outputsJson,
                buildMintReqCb);

        checkResult(result);

        return future;
    }

    /**
     * Builds Indy request for setting fees for transactions in the ledger
     *
     * Note this endpoint is EXPERIMENTAL. Function signature and behaviour may change
     * in the future releases.
     * @param wallet The wallet.
     * @param submitterDid DID of request sender
     * @param paymentMethod
     * @param feesJson {
     *   txnType1: amount1,
     *   txnType2: amount2,
     *   .................
     *   txnTypeN: amountN,
     * }
     * @return Indy request for setting fees for transactions in the ledger
     * @throws IndyException
     */
    public static CompletableFuture<String> buildSetTxnFeesRequest(
            Wallet wallet,
            String submitterDid,
            String paymentMethod,
            String feesJson
    ) throws IndyException {
        ParamGuard.notNullOrWhiteSpace(submitterDid, "submitterDid");
        ParamGuard.notNullOrWhiteSpace(paymentMethod, "paymentMethod");
        ParamGuard.notNullOrWhiteSpace(feesJson, "feesJson");

        CompletableFuture<String> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);

        int walletHandle = wallet.getWalletHandle();

        int result = LibIndy.api.indy_build_set_txn_fees_req(
                commandHandle,
                walletHandle,
                submitterDid,
                paymentMethod,
                feesJson,
                stringCompleteCb);

        checkResult(result);

        return future;
    }

    /**
     * Builds Indy get request for getting fees for transactions in the ledger
     *
     * Note this endpoint is EXPERIMENTAL. Function signature and behaviour may change
     * in the future releases.
     * @param wallet The wallet.
     * @param submitterDid DID of request sender
     * @param paymentMethod
     * @return Indy request for getting fees for transactions in the ledger
     * @throws IndyException
     */
    public static CompletableFuture<String> buildGetTxnFeesRequest(
            Wallet wallet,
            String submitterDid,
            String paymentMethod
    ) throws IndyException {
        ParamGuard.notNullOrWhiteSpace(submitterDid, "submitterDid");
        ParamGuard.notNullOrWhiteSpace(paymentMethod, "paymentMethod");

        CompletableFuture<String> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);

        int walletHandle = wallet.getWalletHandle();

        int result = LibIndy.api.indy_build_get_txn_fees_req(
                commandHandle,
                walletHandle,
                submitterDid,
                paymentMethod,
                stringCompleteCb);

        checkResult(result);

        return future;
    }

    /**
     * Parses response for Indy request for getting fees
     *
     * Note this endpoint is EXPERIMENTAL. Function signature and behaviour may change
     * in the future releases.
     * @param paymentMethod
     * @param respJson response for Indy request for getting fees
     * @return fees_json {
     *   txnType1: amount1,
     *   txnType2: amount2,
     *   .................
     *   txnTypeN: amountN,
     * }
     * @throws IndyException
     */
    public static CompletableFuture<String> parseGetTxnFeesResponse(
            String paymentMethod,
            String respJson
    ) throws IndyException {
        return parseResponse(paymentMethod, respJson, LibIndy.api::indy_parse_get_txn_fees_response);
    }

    private static CompletableFuture<String> parseResponse(
            String paymentMethod,
            String respJson,
            QuadFunction<Integer, String, String, Callback, Integer> method
    ) throws IndyException {
        ParamGuard.notNullOrWhiteSpace(paymentMethod, "paymentMethod");
        ParamGuard.notNullOrWhiteSpace(respJson, "respJson");

        CompletableFuture<String> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);

        int result = method.apply(commandHandle, paymentMethod, respJson, stringCompleteCb);

        checkResult(result);

        return future;
    }

    @FunctionalInterface
    interface QuadFunction<Arg1, Arg2, Arg3, Arg4, Res> {
        Res apply(Arg1 arg1, Arg2 arg2, Arg3 arg3, Arg4 arg4);
    }
}
