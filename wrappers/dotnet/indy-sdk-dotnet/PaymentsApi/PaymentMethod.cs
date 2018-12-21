using Hyperledger.Indy.Utils;
using System;
using System.Threading.Tasks;
using static Hyperledger.Indy.PaymentsApi.NativeMethods;

namespace Hyperledger.Indy.PaymentsApi
{
    /// <summary>
    /// Payment method.
    /// </summary>
    public abstract class PaymentMethod
    {
        /// <summary>
        /// Initializes a new instance of the <see cref="T:Hyperledger.Indy.PaymentsApi.PaymentMethod"/> class.
        /// </summary>
        protected PaymentMethod()
        {
            CreatePaymentAddressCallback = CreatePaymentAddressHandler;
            AddRequestFeesCallback = AddRequestFeesHandler;
            ParseResponseWithFeesCallback = ParseResponseWithFeesHandler;
            BuildGetPaymentSourcesRequstCallback = BuildGetPaymentSourcesRequstHandler;
            ParseGetPaymentSourcesResponseCallback = ParseGetPaymentSourcesResponseHandler;
            BuildPaymentRequestCallback = BuildPaymentRequestHandler;
            ParsePaymentResponseCallback = ParsePaymentResponseHandler;
            BuildMintReqCallback = BuildMintReqHandler;
            BuildSetTxnFeesReqCallback = BuildSetTxnFeesReqHandler;
            BuildGetTxnFeesReqCallback = BuildGetTxnFeesReqHandler;
            ParseGetTxnFeesResponseCallback = ParseGetTxnFeesResponseHandler;
            BuildVerifyPaymentRequestCallback = BuildVerifyPaymentRequestHandler;
            ParseVerifyPaymentResponseCallback = ParseVerifyPaymentResponseHandler;
        }

        ErrorCode CreatePaymentAddressHandler(int command_handle, int wallet_handle, string config, PaymentMethodResultDelegate cb)
        {
            CreatePaymentAddressAsync(config)
                .ContinueWith(paymentAddress =>
                {
                    var result = cb(command_handle, 0, paymentAddress.Result);
                    CallbackHelper.CheckCallback(result);
                });

            return ErrorCode.Success;
        }

        ErrorCode AddRequestFeesHandler(int command_handle, int wallet_handle, string submitter_did, string req_json, string inputs_json, string outputs_json, string extra, PaymentMethodResultDelegate cb)
        {
            AddRequestFeesAsync(submitter_did, req_json, inputs_json, outputs_json, extra)
                .ContinueWith(reqWithFeesJson =>
                {
                    var result = cb(command_handle, 0, reqWithFeesJson.Result);
                    CallbackHelper.CheckCallback(result);
                });

            return ErrorCode.Success;
        }

        ErrorCode ParseResponseWithFeesHandler(int command_handle, string resp_json, PaymentMethodResultDelegate cb)
        {
            ParseResponseWithFeesAsync(resp_json)
                .ContinueWith(utxoJson =>
                {
                    var result = cb(command_handle, 0, utxoJson.Result);
                    CallbackHelper.CheckCallback(result);
                });

            return ErrorCode.Success;
        }

        ErrorCode BuildGetPaymentSourcesRequstHandler(int command_handle, int wallet_handle, string submitter_did, string payment_address, PaymentMethodResultDelegate cb)
        {
            BuildGetPaymentSourcesRequestAsync(submitter_did, payment_address)
                .ContinueWith(getUtxoTxnJson =>
                {
                    var result = cb(command_handle, 0, getUtxoTxnJson.Result);
                    CallbackHelper.CheckCallback(result);
                });

            return ErrorCode.Success;
        }

        ErrorCode ParseGetPaymentSourcesResponseHandler(int command_handle, string resp_json, PaymentMethodResultDelegate cb)
        {
            ParseGetPaymentSourcesResponseAsync(resp_json)
                .ContinueWith(utxoJson =>
                {
                    var result = cb(command_handle, 0, utxoJson.Result);
                    CallbackHelper.CheckCallback(result);
                });

            return ErrorCode.Success;
        }

        ErrorCode BuildPaymentRequestHandler(int command_handle, int wallet_handle, string submitter_did, string inputs_json, string outputs_json, string extra, PaymentMethodResultDelegate cb)
        {
            BuildPaymentRequestAsync(submitter_did, inputs_json, outputs_json, extra)
                .ContinueWith(paymentReqJson =>
                {
                    var result = cb(command_handle, 0, paymentReqJson.Result);
                    CallbackHelper.CheckCallback(result);
                });

            return ErrorCode.Success;
        }

        ErrorCode ParsePaymentResponseHandler(int command_handle, string resp_json, PaymentMethodResultDelegate cb)
        {
            ParsePaymentResponseAsync(resp_json)
                .ContinueWith(utxoJson =>
                {
                    var result = cb(command_handle, 0, utxoJson.Result);
                    CallbackHelper.CheckCallback(result);
                });

            return ErrorCode.Success;
        }

        ErrorCode BuildMintReqHandler(int command_handle, int wallet_handle, string submitter_did, string outputs_json, string extra, PaymentMethodResultDelegate cb)
        {
            BuildMintRequestAsync(submitter_did, outputs_json, extra)
                .ContinueWith(mintReqJson =>
                {
                    var result = cb(command_handle, 0, mintReqJson.Result);
                    CallbackHelper.CheckCallback(result);
                });

            return ErrorCode.Success;
        }

        ErrorCode BuildSetTxnFeesReqHandler(int command_handle, int wallet_handle, string submitter_did, string fees_json, PaymentMethodResultDelegate cb)
        {
            BuildSetTxnFeesAsync(submitter_did, fees_json)
                .ContinueWith(setTxnFeesJson =>
                {
                    var result = cb(command_handle, 0, setTxnFeesJson.Result);
                    CallbackHelper.CheckCallback(result);
                });

            return ErrorCode.Success;
        }

        ErrorCode BuildGetTxnFeesReqHandler(int command_handle, int wallet_handle, string submitter_did, PaymentMethodResultDelegate cb)
        {
            BuildGetTxnFeesAsync(submitter_did)
                .ContinueWith(getTxnFeesJson =>
                {
                    var result = cb(command_handle, 0, getTxnFeesJson.Result);
                    CallbackHelper.CheckCallback(result);
                });

            return ErrorCode.Success;
        }

        ErrorCode ParseGetTxnFeesResponseHandler(int command_handle, string resp_json, PaymentMethodResultDelegate cb)
        {
            ParseGetTxnFeesResponseAsync(resp_json)
                .ContinueWith(feesJson =>
                {
                    var result = cb(command_handle, 0, feesJson.Result);
                    CallbackHelper.CheckCallback(result);
                });

            return ErrorCode.Success;
        }

        ErrorCode BuildVerifyPaymentRequestHandler(int command_handle, int wallet_handle, string submitterDid, string receipt, PaymentMethodResultDelegate cb)
        {
            BuildVerifyPaymentRequestAsync(submitterDid, receipt)
                .ContinueWith(feesJson =>
                {
                    var result = cb(command_handle, 0, feesJson.Result);
                    CallbackHelper.CheckCallback(result);
                });

            return ErrorCode.Success;
        }

        ErrorCode ParseVerifyPaymentResponseHandler(int command_handle, string responseJson, PaymentMethodResultDelegate cb)
        {
            ParseVerifyPaymentResponseAsync(responseJson)
                .ContinueWith(feesJson =>
                {
                    var result = cb(command_handle, 0, feesJson.Result);
                    CallbackHelper.CheckCallback(result);
                });

            return ErrorCode.Success;
        }

        internal CreatePaymentAddressCallbackDelegate CreatePaymentAddressCallback { get; }

        internal AddRequestFeesCallbackDelegate AddRequestFeesCallback { get; }

        internal ParseResponseWithFeesCallbackDelegate ParseResponseWithFeesCallback { get; }

        internal BuildGetPaymentSourcesRequestCallbackDelegate BuildGetPaymentSourcesRequstCallback { get; }

        internal ParseGetPaymentSourcesResponseCallbackDelegate ParseGetPaymentSourcesResponseCallback { get; }

        internal BuildPaymentRequestCallbackDelegate BuildPaymentRequestCallback { get; }

        internal ParsePaymentResponseCallbackDelegate ParsePaymentResponseCallback { get; }

        internal BuildMintReqCallbackDelegate BuildMintReqCallback { get; }

        internal BuildSetTxnFeesReqCallbackDelegate BuildSetTxnFeesReqCallback { get; }

        internal BuildGetTxnFeesReqCallbackDelegate BuildGetTxnFeesReqCallback { get; }

        internal ParseGetTxnFeesResponseCallbackDelegate ParseGetTxnFeesResponseCallback { get; }

        internal BuildVerifyPaymentRequestCallbackDelegate BuildVerifyPaymentRequestCallback { get; }

        internal ParseVerifyPaymentResponseCallbackDelegate ParseVerifyPaymentResponseCallback { get; }

        /// <summary>
        /// Creates the payment address async.
        /// </summary>
        /// <returns>The payment address async.</returns>
        /// <param name="config">Config.</param>
        public abstract Task<string> CreatePaymentAddressAsync(string config);

        /// <summary>
        /// Adds the request fees async.
        /// </summary>
        /// <returns>The request fees async.</returns>
        /// <param name="submitterDid">Submitter did.</param>
        /// <param name="reqJson">Req json.</param>
        /// <param name="inputsJson">Inputs json.</param>
        /// <param name="outputsJson">Outputs json.</param>
        /// <param name="extra">Optional exrra information about payment.</param>
        public abstract Task<string> AddRequestFeesAsync(string submitterDid, string reqJson, string inputsJson, string outputsJson, string extra);

        /// <summary>
        /// Parses the response with fees async.
        /// </summary>
        /// <returns>The response with fees async.</returns>
        /// <param name="responseJson">Response json.</param>
        public abstract Task<string> ParseResponseWithFeesAsync(string responseJson);

        /// <summary>
        /// Builds the get utxo request async.
        /// </summary>
        /// <returns>The get utxo request async.</returns>
        /// <param name="submittedDid">Submitted did.</param>
        /// <param name="paymentAddress">Payment address.</param>
        public abstract Task<string> BuildGetPaymentSourcesRequestAsync(string submittedDid, string paymentAddress);

        /// <summary>
        /// Parses the get utxo response async.
        /// </summary>
        /// <returns>The get utxo response async.</returns>
        /// <param name="responseJson">Response json.</param>
        public abstract Task<string> ParseGetPaymentSourcesResponseAsync(string responseJson);

        /// <summary>
        /// Builds the payment request async.
        /// </summary>
        /// <returns>The payment request async.</returns>
        /// <param name="submitterDid">Submitter did.</param>
        /// <param name="inputsJson">Inputs json.</param>
        /// <param name="outputsJson">Outputs json.</param>
        /// <param name="extra">Optional exrra information about payment.</param>
        public abstract Task<string> BuildPaymentRequestAsync(string submitterDid, string inputsJson, string outputsJson, string extra);

        /// <summary>
        /// Parses the payment response async.
        /// </summary>
        /// <returns>The payment response async.</returns>
        /// <param name="responseJson">Response json.</param>
        public abstract Task<string> ParsePaymentResponseAsync(string responseJson);

        /// <summary>
        /// Builds the mint request async.
        /// </summary>
        /// <returns>The mint request async.</returns>
        /// <param name="submitterDid">Submitter did.</param>
        /// <param name="outputsJson">Outputs json.</param>
        /// <param name="extra">Optional exrra information about payment.</param>
        public abstract Task<string> BuildMintRequestAsync(string submitterDid, string outputsJson, string extra);

        /// <summary>
        /// Builds the set txn fees async.
        /// </summary>
        /// <returns>The set txn fees async.</returns>
        /// <param name="submitterDid">Submitter did.</param>
        /// <param name="feesJson">Fees json.</param>
        public abstract Task<string> BuildSetTxnFeesAsync(string submitterDid, string feesJson);

        /// <summary>
        /// Builds the get txn fees async.
        /// </summary>
        /// <returns>The get txn fees async.</returns>
        /// <param name="submitterDid">Submitter did.</param>
        public abstract Task<string> BuildGetTxnFeesAsync(string submitterDid);

        /// <summary>
        /// Parses the get txn fees response async.
        /// </summary>
        /// <returns>The get txn fees response async.</returns>
        /// <param name="responseJson">Response json.</param>
        public abstract Task<string> ParseGetTxnFeesResponseAsync(string responseJson);

        /// <summary>
        /// Builds the verify payment request async.
        /// </summary>
        /// <returns>The verify payment request async.</returns>
        /// <param name="submitterDid">Submitter did.</param>
        /// <param name="receipt">Receipt.</param>
        public abstract Task<string> BuildVerifyPaymentRequestAsync(string submitterDid, string receipt);

        /// <summary>
        /// Parses the verify payment response async.
        /// </summary>
        /// <returns>The verify payment response async.</returns>
        /// <param name="responseJson">Response json.</param>
        public abstract Task<string> ParseVerifyPaymentResponseAsync(string responseJson);
    }
}