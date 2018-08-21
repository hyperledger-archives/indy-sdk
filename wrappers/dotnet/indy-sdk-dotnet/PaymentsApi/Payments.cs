using System;
using System.Threading.Tasks;
using Hyperledger.Indy.Utils;
using Hyperledger.Indy.WalletApi;
using static Hyperledger.Indy.PaymentsApi.NativeMethods;
using static Hyperledger.Indy.Utils.CallbackHelper;

namespace Hyperledger.Indy.PaymentsApi
{
    /// <summary>
    /// Payments API
    /// </summary>
    public class Payments
    {
        static CreatePaymentAddressDelegate _createPaymentAddressCallback = (xcommand_handle, err, payment_address) =>
        {
            var taskCompletionSource = PendingCommands.Remove<string>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(payment_address);
        };

        static ListPaymentAddressesDelegate _listPaymentAddressesCallback = (xcommand_handle, err, payment_addressed_json) =>
        {
            var taskCompletionSource = PendingCommands.Remove<string>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(payment_addressed_json);
        };

        static IndyMethodCompletedDelegate _registerPaymentMethodCallback = (xcommand_handle, err) =>
        {
            var taskCompletionSource = PendingCommands.Remove<bool>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(true);
        };

        static AddRequestFeesDelegate _addRequestFeesCallback = (xcommand_handle, err, req_with_fees_json, payment_method) =>
        {
            var taskCompletionSource = PendingCommands.Remove<PaymentResult>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(new PaymentResult(req_with_fees_json, payment_method));
        };

        static ParseResponseWithFeesDelegate _parseResponseWithFeesCallback = (xcommand_handle, err, utxo_json) =>
        {
            var taskCompletionSource = PendingCommands.Remove<string>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(utxo_json);
        };

        static BuildGetUtxoRequstDelegate _buildGetUtxoRequestCallback = (xcommand_handle, err, get_utxo_txn_json, payment_method) =>
        {
            var taskCompletionSource = PendingCommands.Remove<PaymentResult>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(new PaymentResult(get_utxo_txn_json, payment_method));
        };

        static ParseGetUtxoResponseDelegate _parseGetUtxoResponseCallback = (xcommand_handle, err, utxo_json) =>
        {
            var taskCompletionSource = PendingCommands.Remove<string>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(utxo_json);
        };

        static BuildPaymentRequestDelegate _buildPaymentRequestCallback = (xcommand_handle, err, payment_req_json, payment_method) =>
        {
            var taskCompletionSource = PendingCommands.Remove<PaymentResult>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(new PaymentResult(payment_req_json, payment_method));
        };

        static ParsePaymentResponseDelegate _parsePaymentResponseCallback = (xcommand_handle, err, utxo_json) =>
        {
            var taskCompletionSource = PendingCommands.Remove<string>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(utxo_json);
        };

        static BuildMintReqDelegate _buildMintRequestCallback = (xcommand_handle, err, mint_req_json, payment_method) =>
        {
            var taskCompletionSource = PendingCommands.Remove<PaymentResult>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(new PaymentResult(mint_req_json, payment_method));
        };

        static BuildSetTxnFeesReqDelegate _buildSetTxnFeesReqCallback = (xcommand_handle, err, set_txn_fees_json) =>
        {
            var taskCompletionSource = PendingCommands.Remove<string>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(set_txn_fees_json);
        };

        static BuildGetTxnFeesReqDelegate _buildGetTxnFeesReqCallback = (xcommand_handle, err, get_txn_fees_json) =>
        {
            var taskCompletionSource = PendingCommands.Remove<string>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(get_txn_fees_json);
        };

        static ParseGetTxnFeesResponseDelegate _parseGetTxnFeesResponseCallback = (xcommand_handle, err, get_txn_fees_json) =>
        {
            var taskCompletionSource = PendingCommands.Remove<string>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(get_txn_fees_json);
        };

        static BuildVerifyPaymentRequestDelegate _buildVerifyPaymentRequestCallback = (xcommand_handle, err, verify_txn_json, payment_method) =>
        {
            var taskCompletionSource = PendingCommands.Remove<PaymentResult>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(new PaymentResult(verify_txn_json, payment_method));
        };

        static ParseVerifyPaymentResponseDelegate _parseVerifyPaymentResponseDelegate = (xcommand_handle, err, txn_json) =>
        {
            var taskCompletionSource = PendingCommands.Remove<string>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(txn_json);
        };


        /// <summary>
        /// Create the payment address for this payment method.
        ///
        /// This method generates private part of payment address
        /// and stores it in a secure place. Ideally it should be
        /// secret in libindy wallet (see crypto module).
        ///
        /// Note that payment method should be able to resolve this
        /// secret by fully resolvable payment address format.
        /// </summary>
        /// <returns>Public identifier of payment address in fully resolvable payment address format</returns>
        /// <param name="wallet">Wallet.</param>
        /// <param name="paymentMethod">Payment method to use (for example, 'sov')</param>
        /// <param name="config">
        /// <code>payment address config as json:
        ///   {
        ///     seed: &lt;str&gt;, // allows deterministic creation of payment address
        ///   }
        /// </code>
        /// </param>
        public static Task<string> CreatePaymentAddressAsync(Wallet wallet, string paymentMethod, string config)
        {
            ParamGuard.NotNull(wallet, "wallet");
            ParamGuard.NotNullOrWhiteSpace(paymentMethod, "paymentMethod");

            var taskCompletionSource = new TaskCompletionSource<string>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var result = NativeMethods.indy_create_payment_address(
                commandHandle,
                wallet.Handle,
                paymentMethod,
                config,
                _createPaymentAddressCallback);

            CallbackHelper.CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Lists all payment addresses that are stored in the wallet
        ///
        /// Note this endpoint is EXPERIMENTAL. Function signature and behaviour may change
        /// in the future releases.
        /// </summary>
        /// <returns>Json array of string with json addresses</returns>
        /// <param name="wallet">Wallet.</param>
        public static Task<string> ListPaymentAddressesAsync(Wallet wallet)
        {
            ParamGuard.NotNull(wallet, "wallet");

            var taskCompletionSource = new TaskCompletionSource<string>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var result = NativeMethods.indy_list_payment_addresses(
                commandHandle,
                wallet.Handle,
                _listPaymentAddressesCallback);

            CallbackHelper.CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Register custom payment implementation.
        ///
        /// It allows library user to provide custom payment method implementation as set of handlers.
        ///
        /// Note this endpoint is EXPERIMENTAL. Function signature and behaviour may change
        /// in the future releases.
        /// </summary>
        /// <returns>The payment method async.</returns>
        /// <param name="paymentMethod">The type of payment method also used as sub-prefix for fully resolvable payment address format ("sov" - for example)</param>
        /// <param name="implementation">Payment method.</param>
        public static Task RegisterPaymentMethodAsync(string paymentMethod, PaymentMethod implementation)
        {
            ParamGuard.NotNull(implementation, "implementation");
            ParamGuard.NotNullOrWhiteSpace(paymentMethod, "paymentMethod");

            var taskCompletionSource = new TaskCompletionSource<bool>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var result = NativeMethods.indy_register_payment_method(
                commandHandle,
                paymentMethod,
                implementation.CreatePaymentAddressCallback,
                implementation.AddRequestFeesCallback,
                implementation.ParseResponseWithFeesCallback,
                implementation.BuildGetPaymentSourcesRequstCallback,
                implementation.ParseGetPaymentSourcesResponseCallback,
                implementation.BuildPaymentRequestCallback,
                implementation.ParsePaymentResponseCallback,
                implementation.BuildMintReqCallback,
                implementation.BuildSetTxnFeesReqCallback,
                implementation.BuildGetTxnFeesReqCallback,
                implementation.ParseGetTxnFeesResponseCallback,
                implementation.BuildVerifyPaymentRequestCallback,
                implementation.ParseVerifyPaymentResponseCallback,
                _registerPaymentMethodCallback);

            CallbackHelper.CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Modifies Indy request by adding information how to pay fees for this transaction
        /// according to selected payment method.
        ///
        /// Payment selection is performed by looking to o
        ///
        /// This method consumes set of UTXO inputs and outputs. The difference between inputs balance
        /// and outputs balance is the fee for this transaction.
        ///
        /// Not that this method also produces correct fee signatures.
        ///
        /// Format of inputs is specific for payment method. Usually it should reference payment transaction
        /// with at least one output that corresponds to payment address that user owns.
        ///
        /// Note this endpoint is EXPERIMENTAL. Function signature and behaviour may change
        /// in the future releases.
        /// </summary>
        /// <returns>The request fees async.</returns>
        /// <param name="wallet">Wallet.</param>
        /// <param name="submitterDid">DID of request sender</param>
        /// <param name="reqJson">Initial transaction request as json</param>
        /// <param name="inputsJson">The list of UTXO inputs as json array:
        ///   ["input1", ...]
        ///   Notes:
        ///     - each input should reference paymentAddress
        ///     - this param will be used to determine payment_method
        /// </param>
        /// <param name="outputsJson">outputs_json: The list of UTXO outputs as json array:
        ///   [{
        ///     paymentAddress: &lt;str>, // payment address used as output
        ///     amount: &lt;int>, // amount of tokens to transfer to this payment address
        ///     extra: &lt;str>, // optional data
        ///   }]</param>
        public static Task<PaymentResult> AddRequestFeesAsync(Wallet wallet, string submitterDid, string reqJson, string inputsJson, string outputsJson)
        {
            ParamGuard.NotNull(wallet, "wallet");
            ParamGuard.NotNullOrWhiteSpace(submitterDid, "submitterDid");

            var taskCompletionSource = new TaskCompletionSource<PaymentResult>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var result = NativeMethods.indy_add_request_fees(
                commandHandle,
                wallet.Handle,
                submitterDid,
                reqJson,
                inputsJson,
                outputsJson,
                _addRequestFeesCallback);

            CallbackHelper.CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Parses response for Indy request with fees.
        ///
        /// Note this endpoint is EXPERIMENTAL. Function signature and behaviour may change
        /// in the future releases.
        /// </summary>
        /// <returns>utxo_json - parsed (payment method and node version agnostic) utxo info as json:
        ///   [{
        ///      txo: &lt;str&gt;, // UTXO input
        ///      paymentAddress: &lt;str&gt;, //payment address for this UTXO
        ///      amount: &lt;int&gt;, // amount of tokens in this input
        ///      extra: &lt;str&gt;, // optional data from payment transaction
        ///   }]</returns>
        /// <param name="paymentMethod">Payment method.</param>
        /// <param name="responseJson">response for Indy request with fees
        ///   Note: this param will be used to determine payment_method</param>
        public static Task<string> ParseResponseWithFeesAsync(string paymentMethod, string responseJson)
        {
            ParamGuard.NotNullOrWhiteSpace(paymentMethod, "paymentMethod");
            ParamGuard.NotNullOrWhiteSpace(responseJson, "responseJson");

            var taskCompletionSource = new TaskCompletionSource<string>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var result = NativeMethods.indy_parse_response_with_fees(
                commandHandle,
                paymentMethod,
                responseJson,
                _parseResponseWithFeesCallback);

            CallbackHelper.CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Builds Indy request for getting UTXO list for payment address
        /// according to this payment method.
        ///
        /// Note this endpoint is EXPERIMENTAL. Function signature and behaviour may change
        /// in the future releases.
        /// </summary>
        /// <returns>get_utxo_txn_json - Indy request for getting UTXO list for payment address
        /// payment_method</returns>
        /// <param name="wallet">Wallet.</param>
        /// <param name="submittedDid">DID of request sender</param>
        /// <param name="paymentAddress">target payment address</param>
        public static Task<PaymentResult> BuildGetUtxoRequestAsync(Wallet wallet, string submittedDid, string paymentAddress)
        {
            ParamGuard.NotNullOrWhiteSpace(submittedDid, "submittedDid");
            ParamGuard.NotNullOrWhiteSpace(paymentAddress, "paymentAddress");

            var taskCompletionSource = new TaskCompletionSource<PaymentResult>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var result = NativeMethods.indy_build_get_payment_sources_request(
                commandHandle,
                wallet.Handle,
                submittedDid,
                paymentAddress,
                _buildGetUtxoRequestCallback);

            CallbackHelper.CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Parses response for Indy request for getting UTXO list.
        ///
        /// Note this endpoint is EXPERIMENTAL. Function signature and behaviour may change
        /// in the future releases.
        /// </summary>
        /// <returns>utxo_json - parsed (payment method and node version agnostic) utxo info as json:
        ///   [{
        ///      txo: &lt;str>, // UTXO input
        ///      paymentAddress: &lt;str>, //payment address for this UTXO
        ///      amount: &lt;int>, // amount of tokens in this input
        ///      extra: &lt;str>, // optional data from payment transaction
        ///   }]</returns>
        /// <param name="paymentMethod">Payment method.</param>
        /// <param name="responseJson">response for Indy request for getting UTXO list
        ///   Note: this param will be used to determine payment_method</param>
        public static Task<string> ParseGetUtxoResponseAsync(string paymentMethod, string responseJson)
        {
            ParamGuard.NotNullOrWhiteSpace(responseJson, "responseJson");

            var taskCompletionSource = new TaskCompletionSource<string>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var result = NativeMethods.indy_parse_get_payment_sources_response(
                commandHandle,
                paymentMethod,
                responseJson,
                _parseGetUtxoResponseCallback);

            CallbackHelper.CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Builds Indy request for doing tokens payment
        /// according to this payment method.
        ///
        /// This method consumes set of UTXO inputs and outputs.
        ///
        /// Format of inputs is specific for payment method. Usually it should reference payment transaction
        /// with at least one output that corresponds to payment address that user owns.
        ///
        /// Note this endpoint is EXPERIMENTAL. Function signature and behaviour may change
        /// in the future releases.
        /// </summary>
        /// <returns>Indy request for doing tokens payment</returns>
        /// <param name="wallet">Wallet.</param>
        /// <param name="submitterDid">Submitter did.</param>
        /// <param name="inputsJson">The list of UTXO inputs as json array:
        ///   ["input1", ...]
        ///   Note that each input should reference paymentAddress</param>
        /// <param name="outputsJson">The list of UTXO outputs as json array:
        ///   [{
        ///     paymentAddress: &lt;str>, // payment address used as output
        ///     amount: &lt;int>, // amount of tokens to transfer to this payment address
        ///     extra: &lt;str>, // optional data
        ///   }]</param>
        /// <param name="extra">Optional information for payment operation</param>
        public static Task<PaymentResult> BuildPaymentRequestAsync(Wallet wallet, string submitterDid, string inputsJson, string outputsJson, string extra)
        {
            ParamGuard.NotNull(wallet, "wallet");
            ParamGuard.NotNullOrWhiteSpace(submitterDid, "submitterDid");
            ParamGuard.NotNullOrWhiteSpace(inputsJson, "inputsJson");
            ParamGuard.NotNullOrWhiteSpace(outputsJson, "outputsJson");

            var taskCompletionSource = new TaskCompletionSource<PaymentResult>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var result = NativeMethods.indy_build_payment_req(
                commandHandle,
                wallet.Handle,
                submitterDid,
                inputsJson,
                outputsJson,
                extra,
                _buildPaymentRequestCallback);

            CallbackHelper.CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Parses response for Indy request for payment txn.
        ///
        /// Note this endpoint is EXPERIMENTAL. Function signature and behaviour may change
        /// in the future releases.
        /// </summary>
        /// <returns>utxo_json - parsed (payment method and node version agnostic) utxo info as json:
        ///   [{
        ///      txo: &lt;str>, // UTXO input
        ///      paymentAddress: &lt;str>, //payment address for this UTXO
        ///      amount: &lt;int>, // amount of tokens in this input
        ///      extra: &lt;str>, // optional data from payment transaction
        ///   }]</returns>
        /// <param name="paymentMethod">Payment method.</param>
        /// <param name="responseJson">response for Indy request for payment txn
        ///   Note: this param will be used to determine payment_method</param>
        public static Task<string> ParsePaymentResponseAsync(string paymentMethod, string responseJson)
        {
            ParamGuard.NotNullOrWhiteSpace(paymentMethod, "paymentMethod");
            ParamGuard.NotNullOrWhiteSpace(responseJson, "responseJson");

            var taskCompletionSource = new TaskCompletionSource<string>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var result = NativeMethods.indy_parse_payment_response(
                commandHandle,
                paymentMethod,
                responseJson,
                _parsePaymentResponseCallback);

            CallbackHelper.CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Builds Indy request for doing tokens minting
        /// according to this payment method.
        ///
        /// Note this endpoint is EXPERIMENTAL. Function signature and behaviour may change
        /// in the future releases.
        /// </summary>
        /// <returns>Indy request for doing tokens minting.</returns>
        /// <param name="wallet">Wallet.</param>
        /// <param name="submitterDid">Submitter did.</param>
        /// <param name="outputsJson">The list of UTXO outputs as json array:
        ///   [{
        ///     paymentAddress: &lt;str>, // payment address used as output
        ///     amount: &lt;int>, // amount of tokens to transfer to this payment address
        ///     extra: &lt;str>, // optional data
        ///   }]</param>
        /// <param name="extra">Optional information for payment operation</param>
        public static Task<PaymentResult> BuildMintRequestAsync(Wallet wallet, string submitterDid, string outputsJson, string extra)
        {
            ParamGuard.NotNull(wallet, "wallet");
            ParamGuard.NotNullOrWhiteSpace(submitterDid, "submitterDid");
            ParamGuard.NotNullOrWhiteSpace(outputsJson, "outputsJson");

            var taskCompletionSource = new TaskCompletionSource<PaymentResult>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var result = NativeMethods.indy_build_mint_req(
                commandHandle,
                wallet.Handle,
                submitterDid,
                outputsJson,
                extra,
                _buildMintRequestCallback);

            CallbackHelper.CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Builds Indy request for setting fees for transactions in the ledger
        ///
        /// Note this endpoint is EXPERIMENTAL. Function signature and behaviour may change
        /// in the future releases.
        /// </summary>
        /// <returns>Indy request for setting fees for transactions in the ledger</returns>
        /// <param name="wallet">Wallet.</param>
        /// <param name="submitterDid">Submitter did.</param>
        /// <param name="paymentMethod">Payment method.</param>
        /// <param name="feesJson">Fees json.
        /// {
        ///   txnType1: amount1,
        ///   txnType2: amount2,
        ///   .................
        ///   txnTypeN: amountN,
        /// }</param>
        public static Task<string> BuildSetTxnFeesRequestAsync(Wallet wallet, string submitterDid, string paymentMethod, string feesJson)
        {
            ParamGuard.NotNull(wallet, "wallet");
            ParamGuard.NotNullOrWhiteSpace(submitterDid, "submitterDid");
            ParamGuard.NotNullOrWhiteSpace(paymentMethod, "paymentMethod");
            ParamGuard.NotNullOrWhiteSpace(feesJson, "feesJson");

            var taskCompletionSource = new TaskCompletionSource<string>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var result = NativeMethods.indy_build_set_txn_fees_req(
                commandHandle,
                wallet.Handle,
                submitterDid,
                paymentMethod,
                feesJson,
                _buildSetTxnFeesReqCallback);

            CallbackHelper.CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Builds Indy get request for getting fees for transactions in the ledger
        ///
        /// Note this endpoint is EXPERIMENTAL. Function signature and behaviour may change
        /// in the future releases.
        /// </summary>
        /// <returns>Indy request for getting fees for transactions in the ledger.</returns>
        /// <param name="wallet">Wallet.</param>
        /// <param name="submitterDid">DID of request sender</param>
        /// <param name="paymentMethod">Payment method.</param>
        public static Task<string> BuildGetTxnFeesRequestAsync(Wallet wallet, string submitterDid, string paymentMethod)
        {
            ParamGuard.NotNull(wallet, "wallet");
            ParamGuard.NotNullOrWhiteSpace(submitterDid, "submitterDid");
            ParamGuard.NotNullOrWhiteSpace(paymentMethod, "paymentMethod");

            var taskCompletionSource = new TaskCompletionSource<string>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var result = NativeMethods.indy_build_get_txn_fees_req(
                commandHandle,
                wallet.Handle,
                submitterDid,
                paymentMethod,
                _buildGetTxnFeesReqCallback);

            CallbackHelper.CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Parses response for Indy request for getting fees
        ///
        /// Note this endpoint is EXPERIMENTAL. Function signature and behaviour may change
        /// in the future releases.
        /// </summary>
        /// <returns>{
        ///   txnType1: amount1,
        ///   txnType2: amount2,
        ///   .................
        ///   txnTypeN: amountN,
        /// }</returns>
        /// <param name="paymentMethod">Payment method.</param>
        /// <param name="responseJson">Response for Indy request for getting fees</param>
        public static Task<string> ParseGetTxnFeesResponseAsync(string paymentMethod, string responseJson)
        {
            ParamGuard.NotNullOrWhiteSpace(paymentMethod, "paymentMethod");
            ParamGuard.NotNullOrWhiteSpace(responseJson, "responseJson");

            var taskCompletionSource = new TaskCompletionSource<string>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var result = NativeMethods.indy_parse_get_txn_fees_response(
                commandHandle,
                paymentMethod,
                responseJson,
                _parseGetTxnFeesResponseCallback);

            CallbackHelper.CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Builds Indy request for information to verify the payment receipt
        /// </summary>
        /// <returns>Indy request for verification receipt.</returns>
        /// <param name="wallet">Wallet.</param>
        /// <param name="submitterDid">DID of request sender</param>
        /// <param name="receipt">Payment receipt to verify.</param>
        public static Task<PaymentResult> BuildVerifyPaymentRequestAsync(Wallet wallet, string submitterDid, string receipt)
        {
            ParamGuard.NotNull(wallet, "wallet");
            ParamGuard.NotNullOrWhiteSpace(submitterDid, "submitterDid");
            ParamGuard.NotNullOrWhiteSpace(receipt, "receipt");

            var taskCompletionSource = new TaskCompletionSource<PaymentResult>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var result = NativeMethods.indy_build_verify_payment_req(
                commandHandle,
                wallet.Handle,
                submitterDid,
                receipt,
                _buildVerifyPaymentRequestCallback);

            CallbackHelper.CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Parses Indy response with information to verify receipt
        /// </summary>
        /// <returns>txn_json: {
        ///     sources: [&lt;str>, ]
        ///     receipts: [ {
        ///         recipient: &lt;str>, // payment address of recipient
        ///         receipt: &lt;str>, // receipt that can be used for payment referencing and verification
        ///         amount: &lt;int>, // amount
        ///     } ],
        ///     extra: &lt;str>, //optional data
        /// }</returns>
        /// <param name="paymentMethod">Payment method to use.</param>
        /// <param name="responseJson">Response of the ledger for verify txn.</param>
        public static Task<string> ParseVerifyPaymentResponseAsync(string paymentMethod, string responseJson)
        {
            ParamGuard.NotNullOrWhiteSpace(paymentMethod, "paymentMethod");
            ParamGuard.NotNullOrWhiteSpace(responseJson, "responseJson");

            var taskCompletionSource = new TaskCompletionSource<string>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var result = NativeMethods.indy_parse_verify_payment_response(
                commandHandle,
                paymentMethod,
                responseJson,
                _parseVerifyPaymentResponseDelegate);

            CallbackHelper.CheckResult(result);

            return taskCompletionSource.Task;
        }
    }
}
