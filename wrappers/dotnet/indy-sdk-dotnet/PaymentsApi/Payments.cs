using Hyperledger.Indy.Utils;
using Hyperledger.Indy.WalletApi;
using System.Threading.Tasks;
using static Hyperledger.Indy.PaymentsApi.NativeMethods;
#if __IOS__
using ObjCRuntime;
#endif

namespace Hyperledger.Indy.PaymentsApi
{
    /// <summary>
    /// Payments API
    /// </summary>
    public class Payments
    {
#if __IOS__
        [MonoPInvokeCallback(typeof(CreatePaymentAddressDelegate))]
#endif
        static void CreatePaymentAddressCallbackMethod(int xcommand_handle, int err, string payment_address)
        {
            var taskCompletionSource = PendingCommands.Remove<string>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(payment_address);
        }
        static CreatePaymentAddressDelegate CreatePaymentAddressCallback = CreatePaymentAddressCallbackMethod;

#if __IOS__
        [MonoPInvokeCallback(typeof(ListPaymentAddressesDelegate))]
#endif
        static void ListPaymentAddressesCallbackMethod(int xcommand_handle, int err, string payment_addressed_json)
        {
            var taskCompletionSource = PendingCommands.Remove<string>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(payment_addressed_json);
        }
        static ListPaymentAddressesDelegate ListPaymentAddressesCallback = ListPaymentAddressesCallbackMethod;

#if __IOS__
        [MonoPInvokeCallback(typeof(AddRequestFeesDelegate))]
#endif
        static void AddRequestFeesCallbackMethod(int xcommand_handle, int err, string req_with_fees_json, string payment_method)
        {
            var taskCompletionSource = PendingCommands.Remove<PaymentResult>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(new PaymentResult(req_with_fees_json, payment_method));
        }
        static AddRequestFeesDelegate AddRequestFeesCallback = AddRequestFeesCallbackMethod;

#if __IOS__
        [MonoPInvokeCallback(typeof(ParseResponseWithFeesDelegate))]
#endif
        static void ParseResponseWithFeesCallbackMethod(int xcommand_handle, int err, string utxo_json)
        {
            var taskCompletionSource = PendingCommands.Remove<string>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(utxo_json);
        }
        static ParseResponseWithFeesDelegate ParseResponseWithFeesCallback = ParseResponseWithFeesCallbackMethod;

#if __IOS__
        [MonoPInvokeCallback(typeof(BuildGetUtxoRequstDelegate))]
#endif
        static void BuildGetUtxoRequestCallbackMethod(int xcommand_handle, int err, string get_utxo_txn_json, string payment_method)
        {
            var taskCompletionSource = PendingCommands.Remove<PaymentResult>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(new PaymentResult(get_utxo_txn_json, payment_method));
        }
        static BuildGetUtxoRequstDelegate BuildGetUtxoRequestCallback = BuildGetUtxoRequestCallbackMethod;

#if __IOS__
        [MonoPInvokeCallback(typeof(ParseGetUtxoResponseDelegate))]
#endif
        static void ParseGetUtxoResponseCallbackMethod(int xcommand_handle, int err, string utxo_json)
        {
            var taskCompletionSource = PendingCommands.Remove<string>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(utxo_json);
        }
        static ParseGetUtxoResponseDelegate ParseGetUtxoResponseCallback = ParseGetUtxoResponseCallbackMethod;

#if __IOS__
        [MonoPInvokeCallback(typeof(BuildPaymentRequestDelegate))]
#endif
        static void BuildPaymentRequestCallbackMethod(int xcommand_handle, int err, string payment_req_json, string payment_method)
        {
            var taskCompletionSource = PendingCommands.Remove<PaymentResult>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(new PaymentResult(payment_req_json, payment_method));
        }
        static BuildPaymentRequestDelegate BuildPaymentRequestCallback = BuildPaymentRequestCallbackMethod;

#if __IOS__
        [MonoPInvokeCallback(typeof(ParsePaymentResponseDelegate))]
#endif
        static void ParsePaymentResponseCallbackMethod(int xcommand_handle, int err, string utxo_json)
        {
            var taskCompletionSource = PendingCommands.Remove<string>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(utxo_json);
        }
        static ParsePaymentResponseDelegate ParsePaymentResponseCallback = ParsePaymentResponseCallbackMethod;

#if __IOS__
        [MonoPInvokeCallback(typeof(PreparePaymentExtraWithAcceptanceDataDelegate))]
#endif
        static void PreparePaymentExtraWithAcceptanceDataCallbackMethod(int xcommand_handle, int err, string extra_with_acceptance)
        {
            var taskCompletionSource = PendingCommands.Remove<string>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(extra_with_acceptance);
        }
        static PreparePaymentExtraWithAcceptanceDataDelegate PreparePaymentExtraWithAcceptanceDataCallback = PreparePaymentExtraWithAcceptanceDataCallbackMethod;

#if __IOS__
        [MonoPInvokeCallback(typeof(BuildMintReqDelegate))]
#endif
        static void BuildMintRequestCallbackMethod(int xcommand_handle, int err, string mint_req_json, string payment_method)
        {
            var taskCompletionSource = PendingCommands.Remove<PaymentResult>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(new PaymentResult(mint_req_json, payment_method));
        }
        static BuildMintReqDelegate BuildMintRequestCallback = BuildMintRequestCallbackMethod;

#if __IOS__
        [MonoPInvokeCallback(typeof(BuildSetTxnFeesReqDelegate))]
#endif
        static void BuildSetTxnFeesReqCallbackMethod(int xcommand_handle, int err, string set_txn_fees_json)
        {
            var taskCompletionSource = PendingCommands.Remove<string>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(set_txn_fees_json);
        }
        static BuildSetTxnFeesReqDelegate BuildSetTxnFeesReqCallback = BuildSetTxnFeesReqCallbackMethod;

#if __IOS__
        [MonoPInvokeCallback(typeof(BuildGetTxnFeesReqDelegate))]
#endif
        static void BuildGetTxnFeesReqCallbackMethod(int xcommand_handle, int err, string get_txn_fees_json)
        {
            var taskCompletionSource = PendingCommands.Remove<string>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(get_txn_fees_json);
        }
        static BuildGetTxnFeesReqDelegate BuildGetTxnFeesReqCallback = BuildGetTxnFeesReqCallbackMethod;

#if __IOS__
        [MonoPInvokeCallback(typeof(ParseGetTxnFeesResponseDelegate))]
#endif
        static void ParseGetTxnFeesResponseCallbackMethod(int xcommand_handle, int err, string get_txn_fees_json)
        {
            var taskCompletionSource = PendingCommands.Remove<string>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(get_txn_fees_json);
        }
        static ParseGetTxnFeesResponseDelegate ParseGetTxnFeesResponseCallback = ParseGetTxnFeesResponseCallbackMethod;

#if __IOS__
        [MonoPInvokeCallback(typeof(BuildVerifyPaymentRequestDelegate))]
#endif
        static void BuildVerifyPaymentRequestCallbackMethod(int xcommand_handle, int err, string verify_txn_json, string payment_method)
        {
            var taskCompletionSource = PendingCommands.Remove<PaymentResult>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(new PaymentResult(verify_txn_json, payment_method));
        }
        static BuildVerifyPaymentRequestDelegate BuildVerifyPaymentRequestCallback = BuildVerifyPaymentRequestCallbackMethod;

#if __IOS__
        [MonoPInvokeCallback(typeof(ParseVerifyPaymentResponseDelegate))]
#endif
        static void ParseVerifyPaymentResponseDelegateMethod(int xcommand_handle, int err, string txn_json)
        {
            var taskCompletionSource = PendingCommands.Remove<string>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(txn_json);
        }
        static ParseVerifyPaymentResponseDelegate ParseVerifyPaymentResponseDelegate = ParseVerifyPaymentResponseDelegateMethod;

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
                CreatePaymentAddressCallback);

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
                ListPaymentAddressesCallback);

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
                CallbackHelper.TaskCompletingNoValueCallback);

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
        ///   <param name="extra">Optional information for payment operation.</param>
        public static Task<PaymentResult> AddRequestFeesAsync(Wallet wallet, string submitterDid, string reqJson, string inputsJson, string outputsJson, string extra)
        {
            ParamGuard.NotNull(wallet, "wallet");

            var taskCompletionSource = new TaskCompletionSource<PaymentResult>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var result = NativeMethods.indy_add_request_fees(
                commandHandle,
                wallet.Handle,
                submitterDid,
                reqJson,
                inputsJson,
                outputsJson,
                extra,
                AddRequestFeesCallback);

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
                ParseResponseWithFeesCallback);

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
        public static Task<PaymentResult> BuildGetPaymentSourcesAsync(Wallet wallet, string submittedDid, string paymentAddress)
        {
            ParamGuard.NotNullOrWhiteSpace(paymentAddress, "paymentAddress");

            var taskCompletionSource = new TaskCompletionSource<PaymentResult>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var result = NativeMethods.indy_build_get_payment_sources_request(
                commandHandle,
                wallet.Handle,
                submittedDid,
                paymentAddress,
                BuildGetUtxoRequestCallback);

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
        public static Task<string> ParseGetPaymentSourcesAsync(string paymentMethod, string responseJson)
        {
            ParamGuard.NotNullOrWhiteSpace(responseJson, "responseJson");

            var taskCompletionSource = new TaskCompletionSource<string>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var result = NativeMethods.indy_parse_get_payment_sources_response(
                commandHandle,
                paymentMethod,
                responseJson,
                ParseGetUtxoResponseCallback);

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
                BuildPaymentRequestCallback);

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
                ParsePaymentResponseCallback);

            CallbackHelper.CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Prepare payment extra JSON with TAA acceptance data
        ///
        /// EXPERIMENTAL
        ///
        /// This function may calculate digest by itself or consume it as a parameter.
        /// If all text, version and taa_digest parameters are specified, a check integrity of them will be done.
        /// </summary>
        /// <param name="extra_json">(optional) original extra json.</param>
        /// <param name="text">
        /// text and version - (optional) raw data about TAA from ledger.
        ///     These parameters should be passed together.
        ///     These parameters are required if taa_digest parameter is omitted.
        /// </param>
        /// <param name="version">
        /// text and version - (optional) raw data about TAA from ledger.
        ///     These parameters should be passed together.
        ///     These parameters are required if taa_digest parameter is omitted.
        /// </param>
        /// <param name="taa_digest">(optional) digest on text and version. This parameter is required if text and version parameters are omitted.</param>
        /// <param name="mechanism">mechanism how user has accepted the TAA</param>
        /// <param name="time">UTC timestamp when user has accepted the TAA</param>
        /// <returns>Updated request result as json.</returns>
        public static Task<string> PreparePaymentExtraWithAcceptanceDataAsync(string extra_json, string text, string version, string taa_digest, string mechanism, ulong time)
        {
            ParamGuard.NotNullOrWhiteSpace(mechanism, "mechanism");

            var taskCompletionSource = new TaskCompletionSource<string>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var result = NativeMethods.indy_prepare_payment_extra_with_acceptance_data(
                commandHandle,
                extra_json,
                text,
                version,
                taa_digest,
                mechanism,
                time,
                PreparePaymentExtraWithAcceptanceDataCallback);

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
            ParamGuard.NotNullOrWhiteSpace(outputsJson, "outputsJson");

            var taskCompletionSource = new TaskCompletionSource<PaymentResult>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var result = NativeMethods.indy_build_mint_req(
                commandHandle,
                wallet.Handle,
                submitterDid,
                outputsJson,
                extra,
                BuildMintRequestCallback);

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
                BuildSetTxnFeesReqCallback);

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
            ParamGuard.NotNullOrWhiteSpace(paymentMethod, "paymentMethod");

            var taskCompletionSource = new TaskCompletionSource<string>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var result = NativeMethods.indy_build_get_txn_fees_req(
                commandHandle,
                wallet.Handle,
                submitterDid,
                paymentMethod,
                BuildGetTxnFeesReqCallback);

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
                ParseGetTxnFeesResponseCallback);

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
            ParamGuard.NotNullOrWhiteSpace(receipt, "receipt");

            var taskCompletionSource = new TaskCompletionSource<PaymentResult>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var result = NativeMethods.indy_build_verify_payment_req(
                commandHandle,
                wallet.Handle,
                submitterDid,
                receipt,
                BuildVerifyPaymentRequestCallback);

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
                ParseVerifyPaymentResponseDelegate);

            CallbackHelper.CheckResult(result);

            return taskCompletionSource.Task;
        }
    }
}
