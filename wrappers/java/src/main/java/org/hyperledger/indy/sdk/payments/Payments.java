package org.hyperledger.indy.sdk.payments;

import com.sun.jna.Callback;
import org.hyperledger.indy.sdk.IndyException;
import org.hyperledger.indy.sdk.IndyJava;
import org.hyperledger.indy.sdk.LibIndy;
import org.hyperledger.indy.sdk.ParamGuard;
import org.hyperledger.indy.sdk.payments.PaymentsResults.*;

import java.util.concurrent.CompletableFuture;

public class Payments extends IndyJava.API {

    private Payments() {

    }

    /*
     * STATIC CALLBACKS
     */

    /**
     * Callback used when createPaymentAddress completes.
     */
    private static Callback createPaymentAddressCb = new Callback() {

        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int xcommandHandle, int err, String paymentAddress) {
            CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(xcommandHandle);
            if (!checkCallback(future, err)) return;

            future.complete(paymentAddress);
        }
    };

    /**
     * Callback used when listAddresses completes.
     */
    private static Callback listAddressesCb = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int xcommandHandle, int err, String paymentAddressesJson) {
            CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(xcommandHandle);
            if (!checkCallback(future, err)) return;

            future.complete(paymentAddressesJson);
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
     * Callback used when parseResponseWithFees completes.
     */
    private static Callback parseResponseWithFeesCb = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int xcommandHandle, int err, String utxoJson) {
            CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(xcommandHandle);
            if (!checkCallback(future, err)) return;

            future.complete(utxoJson);
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
     * Callback used when parseGetUtxoResponse completes.
     */
    private static Callback parseGetUtxoResponseCb = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int xcommandHandle, int err, String utxoJson) {
            CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(xcommandHandle);
            if (!checkCallback(future, err)) return;

            future.complete(utxoJson);
        }
    };

    /**
     * Callback used when buildPaymentReq completes.
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
     * Callback used when parsePaymentResponse completes.
     */
    private static Callback parsePaymentResponseCb = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int xcommandHandle, int err, String utxoJson) {
            CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(xcommandHandle);
            if (!checkCallback(future, err)) return;

            future.complete(utxoJson);
        }
    };

    /**
     * Callback used when buildMintReq completes.
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

    /**
     * Callback used when buildSetTxnFeesReq completes.
     */
    private static Callback buildSetTxnFeesReqCb = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int xcommandHandle, int err, String setTxnFeesJson) {
            CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(xcommandHandle);
            if (!checkCallback(future, err)) return;

            future.complete(setTxnFeesJson);
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
     * @param walletHandle wallet handle where to save new address
     * @param paymentMethod Payment method to use (for example, 'sov')
     * @param config payment address config as json:
     *               {
     *                  seed: <str>, // allows deterministic creation of payment address
     *               }
     * @return public identifier of payment address in fully resolvable payment address format
     * @throws IndyException
     */
    public static CompletableFuture<String> createPaymentAddress(
            int walletHandle,
            String paymentMethod,
            String config
    ) throws IndyException {
        ParamGuard.notNullOrWhiteSpace(paymentMethod, "paymentMethod");
        ParamGuard.notNullOrWhiteSpace(config, "config");

        CompletableFuture<String> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);

        int result = LibIndy.api.indy_create_payment_address(
                commandHandle,
                walletHandle,
                paymentMethod,
                config,
                createPaymentAddressCb
        );

        checkResult(result);

        return future;
    }

    /**
     * Lists all payment addresses that are stored in the wallet
     *
     * @param walletHandle wallet to search for payment_addresses in
     * @return json array of string with json addresses
     * @throws IndyException
     */
    public static CompletableFuture<String> listAddresses(
            int walletHandle
    ) throws IndyException {
        CompletableFuture<String> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);

        int result = LibIndy.api.indy_list_addresses(
                commandHandle,
                walletHandle,
                listAddressesCb
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
            String reqJson,
            String inputsJson,
            String outputsJson
    ) throws IndyException {
        ParamGuard.notNullOrWhiteSpace(reqJson, "reqJson");
        ParamGuard.notNullOrWhiteSpace(inputsJson, "inputsJson");
        ParamGuard.notNullOrWhiteSpace(outputsJson, "outputsJson");

        CompletableFuture<AddRequestFeesResult> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);

        int result = LibIndy.api.indy_add_request_fees(
                commandHandle,
                reqJson,
                inputsJson,
                outputsJson,
                addRequestFeesCb
        );

        checkResult(result);

        return future;
    }

    /**
     * Parses response for Indy request with fees.
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
        ParamGuard.notNullOrWhiteSpace(paymentMethod, "paymentMethod");
        ParamGuard.notNullOrWhiteSpace(respJson, "respJson");

        CompletableFuture<String> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);

        int result = LibIndy.api.indy_parse_response_with_fees(
                commandHandle,
                paymentMethod,
                respJson,
                parseResponseWithFeesCb
        );

        checkResult(result);

        return future;
    }

    /**
     * Builds Indy request for getting UTXO list for payment address
     * according to this payment method.
     * @param paymentAddress target payment address
     * @return Indy request for getting UTXO list for payment address
     * @throws IndyException
     */
    public static CompletableFuture<BuildGetUtxoRequestResult> buildGetUtxoRequest(
            String paymentAddress
    ) throws IndyException {
        ParamGuard.notNullOrWhiteSpace(paymentAddress, "paymentAddress");

        CompletableFuture<BuildGetUtxoRequestResult> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);

        int result = LibIndy.api.indy_build_get_utxo_request(
                commandHandle,
                paymentAddress,
                buildGetUtxoRequestCb
        );

        checkResult(result);

        return future;
    }

    /**
     * Parses response for Indy request for getting UTXO list.
     *
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
        ParamGuard.notNullOrWhiteSpace(paymentMethod, "paymentMethod");
        ParamGuard.notNullOrWhiteSpace(respJson, "respJson");

        CompletableFuture<String> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);

        int result = LibIndy.api.indy_parse_get_utxo_response(
                commandHandle,
                paymentMethod,
                respJson,
                parseGetUtxoResponseCb
        );

        checkResult(result);

        return future;
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
    public static CompletableFuture<BuildPaymentReqResult> buildPaymentReq(
            String inputsJson,
            String outputsJson
    ) throws IndyException {
        ParamGuard.notNullOrWhiteSpace(inputsJson, "inputsJson");
        ParamGuard.notNullOrWhiteSpace(outputsJson, "outputsJson");

        CompletableFuture<BuildPaymentReqResult> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);

        int result = LibIndy.api.indy_build_payment_req(
                commandHandle,
                inputsJson,
                outputsJson,
                buildPaymentReqCb
        );

        checkResult(result);

        return future;
    }

    /**
     * Parses response for Indy request for payment txn.
     *
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
        ParamGuard.notNullOrWhiteSpace(paymentMethod, "paymentMethod");
        ParamGuard.notNullOrWhiteSpace(respJson, "respJson");

        CompletableFuture<String> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);

        int result = LibIndy.api.indy_parse_payment_response(
                commandHandle,
                paymentMethod,
                respJson,
                parsePaymentResponseCb
        );

        checkResult(result);

        return future;
    }

    /**
     * Builds Indy request for doing tokens minting
     * according to this payment method.
     *
     * @param outputsJson The list of UTXO outputs as json array:
     *                      [{
     *                        paymentAddress: <str>, // payment address used as output
     *                        amount: <int>, // amount of tokens to transfer to this payment address
     *                        extra: <str>, // optional data
     *                      }]
     * @return Indy request for doing tokens minting
     * @throws IndyException
     */
    public static CompletableFuture<BuildMintReqResult> buildMintReq (
            String outputsJson
    ) throws IndyException {
        ParamGuard.notNullOrWhiteSpace(outputsJson, "outputsJson");

        CompletableFuture<BuildMintReqResult> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);

        int result = LibIndy.api.indy_build_mint_req(
                commandHandle,
                outputsJson,
                buildMintReqCb
        );

        checkResult(result);

        return future;
    }

    /**
     * Builds Indy request for setting fees for transactions in the ledger
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
    public static CompletableFuture<String> buildSetTxnFeesReq(
            String paymentMethod,
            String feesJson
    ) throws IndyException {
        ParamGuard.notNullOrWhiteSpace(paymentMethod, "paymentMethod");
        ParamGuard.notNullOrWhiteSpace(feesJson, "feesJson");

        CompletableFuture<String> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);

        int result = LibIndy.api.indy_build_set_txn_fees_req(
                commandHandle,
                paymentMethod,
                feesJson,
                buildSetTxnFeesReqCb
        );

        checkResult(result);

        return future;
    }


}
