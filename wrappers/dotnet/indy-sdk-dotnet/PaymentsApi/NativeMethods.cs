using System;
using System.Runtime.InteropServices;
using static Hyperledger.Indy.Utils.CallbackHelper;

namespace Hyperledger.Indy.PaymentsApi
{
    /// <summary>
    /// Native methods.
    /// </summary>
    public class NativeMethods
    {
        internal delegate int PaymentMethodResultDelegate(int command_handle, int err, string arg);

        internal delegate ErrorCode CreatePaymentAddressCallbackDelegate(int command_handle, int wallet_handle, string config, PaymentMethodResultDelegate cb);

        internal delegate ErrorCode AddRequestFeesCallbackDelegate(int command_handle, int wallet_handle, string submitter_did, string req_json, string inputs_json, string outputs_json, string extra, PaymentMethodResultDelegate cb);

        internal delegate ErrorCode ParseResponseWithFeesCallbackDelegate(int command_handle, string resp_json, PaymentMethodResultDelegate cb);

        internal delegate ErrorCode BuildGetPaymentSourcesRequestCallbackDelegate(int command_handle, int wallet_handle, string submitter_did, string payment_address, PaymentMethodResultDelegate cb);

        internal delegate ErrorCode ParseGetPaymentSourcesResponseCallbackDelegate(int command_handle, string resp_json, PaymentMethodResultDelegate cb);

        internal delegate ErrorCode BuildPaymentRequestCallbackDelegate(int command_handle, int wallet_handle, string submitter_did, string inputs_json, string outputs_json, string extra, PaymentMethodResultDelegate cb);

        internal delegate ErrorCode ParsePaymentResponseCallbackDelegate(int command_handle, string resp_json, PaymentMethodResultDelegate cb);

        internal delegate ErrorCode BuildMintReqCallbackDelegate(int command_handle, int wallet_handle, string submitter_did, string outputs_json, string extra, PaymentMethodResultDelegate cb);

        internal delegate ErrorCode BuildSetTxnFeesReqCallbackDelegate(int command_handle, int wallet_handle, string submitter_did, string fees_json, PaymentMethodResultDelegate cb);

        internal delegate ErrorCode BuildGetTxnFeesReqCallbackDelegate(int command_handle, int wallet_handle, string submitter_did, PaymentMethodResultDelegate cb);

        internal delegate ErrorCode ParseGetTxnFeesResponseCallbackDelegate(int command_handle, string resp_json, PaymentMethodResultDelegate cb);

        internal delegate ErrorCode BuildVerifyPaymentRequestCallbackDelegate(int command_handle, int wallet_handle, string submitter_did, string receipt, PaymentMethodResultDelegate cb);

        internal delegate ErrorCode ParseVerifyPaymentResponseCallbackDelegate(int command_handle, string resp_json, PaymentMethodResultDelegate cb);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false)]
        internal static extern int indy_register_payment_method(int command_handle, string payment_method,
                                                                CreatePaymentAddressCallbackDelegate create_payment_address,
                                                                AddRequestFeesCallbackDelegate add_request_fees,
                                                                ParseResponseWithFeesCallbackDelegate parse_response_with_fees,
                                                                BuildGetPaymentSourcesRequestCallbackDelegate build_get_payment_sources_request,
                                                                ParseGetPaymentSourcesResponseCallbackDelegate parse_get_payment_sources_response,
                                                                BuildPaymentRequestCallbackDelegate build_payment_req,
                                                                ParsePaymentResponseCallbackDelegate parse_payment_response,
                                                                BuildMintReqCallbackDelegate build_mint_req,
                                                                BuildSetTxnFeesReqCallbackDelegate build_set_txn_fees_req,
                                                                BuildGetTxnFeesReqCallbackDelegate build_get_txn_fees_req,
                                                                ParseGetTxnFeesResponseCallbackDelegate parse_get_txn_fees_response,
                                                                BuildVerifyPaymentRequestCallbackDelegate build_verify_payment_req,
                                                                ParseVerifyPaymentResponseCallbackDelegate parse_verify_payment_response,
                                                                IndyMethodCompletedDelegate cb);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false)]
        internal static extern int indy_create_payment_address(int command_handle, int wallet_handle, string payment_method, string config, CreatePaymentAddressDelegate cb);

        /// <summary>
        /// Create payment address delegate.
        /// </summary>
        public delegate void CreatePaymentAddressDelegate(int command_handle, int err, string payment_address);
        /// <summary>
        /// Create payment address callback delegate.
        /// </summary>

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false)]
        internal static extern int indy_list_payment_addresses(int command_handle, int wallet_handle, ListPaymentAddressesDelegate cb);

        internal delegate void ListPaymentAddressesDelegate(int command_handle, int err, string payment_addresses_json);


        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false)]
        internal static extern int indy_add_request_fees(int command_handle, int wallet_handle, string submitter_did, string req_json, string inputs_json, string outputs_json, string extra, AddRequestFeesDelegate cb);

        /// <summary>
        /// Add request fees delegate.
        /// </summary>
        public delegate void AddRequestFeesDelegate(int command_handle, int err, string req_with_fees_json, string payment_method);


        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false)]
        internal static extern int indy_parse_response_with_fees(int command_handle, string payment_method, string resp_json, ParseResponseWithFeesDelegate cb);

        /// <summary>
        /// Parse response with fees delegate.
        /// </summary>
        public delegate void ParseResponseWithFeesDelegate(int command_handle, int err, string receipts_json);


        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false)]
        internal static extern int indy_build_get_payment_sources_request(int command_handle, int wallet_handle, string submitter_did, string payment_address, BuildGetUtxoRequstDelegate cb);

        /// <summary>
        /// Build get utxo requst delegate.
        /// </summary>
        public delegate void BuildGetUtxoRequstDelegate(int command_handle, int err, string get_sources_txn_json, string payment_method);


        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false)]
        internal static extern int indy_parse_get_payment_sources_response(int command_handle, string payment_method, string resp_json, ParseGetUtxoResponseDelegate cb);

        /// <summary>
        /// Parse get utxo response delegate.
        /// </summary>
        public delegate void ParseGetUtxoResponseDelegate(int command_handle, int err, string sources_json);


        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false)]
        internal static extern int indy_build_payment_req(int command_handle, int wallet_handle, string submitter_did, string inputs_json, string outputs_json, string extra, BuildPaymentRequestDelegate cb);

        /// <summary>
        /// Build payment request delegate.
        /// </summary>
        public delegate void BuildPaymentRequestDelegate(int command_handle, int err, string payment_req_json, string payment_method);


        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false)]
        internal static extern int indy_parse_payment_response(int command_handle, string payment_method, string resp_json, ParsePaymentResponseDelegate cb);

        internal delegate void ParsePaymentResponseDelegate(int command_handle, int err, string receipts_json);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false)]
        internal static extern int indy_prepare_payment_extra_with_acceptance_data(int command_handle, string extra_json, string text, string version, string taa_digest, string mechanism, ulong time, PreparePaymentExtraWithAcceptanceDataDelegate cb);

        internal delegate void PreparePaymentExtraWithAcceptanceDataDelegate(int command_handle, int err, string extra_with_acceptance);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false)]
        internal static extern int indy_build_mint_req(int command_handle, int wallet_handle, string submitter_did, string outputs_json, string extra, BuildMintReqDelegate cb);

        internal delegate void BuildMintReqDelegate(int command_handle, int err, string mint_req_json, string payment_method);


        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false)]
        internal static extern int indy_build_set_txn_fees_req(int command_handle, int wallet_handle, string submitter_did, string payment_method, string fees_json, BuildSetTxnFeesReqDelegate cb);

        internal delegate void BuildSetTxnFeesReqDelegate(int command_handle, int err, string set_txn_fees_json);


        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false)]
        internal static extern int indy_build_get_txn_fees_req(int command_handle, int wallet_handle, string submitter_did, string payment_method, BuildGetTxnFeesReqDelegate cb);

        public delegate void BuildGetTxnFeesReqDelegate(int command_handle, int err, string get_txn_fees_json);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false)]
        internal static extern int indy_parse_get_txn_fees_response(int command_handle, string payment_method, string resp_json, ParseGetTxnFeesResponseDelegate cb);
        public delegate void ParseGetTxnFeesResponseDelegate(int command_handle, int err, string fees_json);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false)]
        internal static extern int indy_build_verify_payment_req(int command_handle, int wallet_handle, string submitter_did, string receipt, BuildVerifyPaymentRequestDelegate cb);
        public delegate void BuildVerifyPaymentRequestDelegate(int command_handle, int err, string verify_txn_json, string payment_method);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false)]
        internal static extern int indy_parse_verify_payment_response(int command_handle, string payment_method, string resp_json, ParseVerifyPaymentResponseDelegate cb);
        public delegate void ParseVerifyPaymentResponseDelegate(int command_handle, int err, string txn_json);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false)]
        internal static extern int indy_get_request_info(int command_handle, string get_auth_rule_response_json, string requester_info_json, string fees_json, GetRequestInfoDelegate cb);
        internal delegate void GetRequestInfoDelegate(int command_handle, int err, string request_info_json);
                                                
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false)]
        internal static extern int indy_sign_with_address(int command_handle, int wallet_handle, string address, byte[] message_raw, uint message_len, SignWithAddressDelegate cb);
        internal delegate void SignWithAddressDelegate(int command_handle, int err, IntPtr signature_raw, uint signature_len);
    
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false)]
        internal static extern int indy_verify_with_address(int command_handle, string address, byte[] message_raw, uint message_len, byte[] signature_raw, uint signature_len, VerifyWithAddressDelegate cb);
        internal delegate void VerifyWithAddressDelegate(int command_handle, int err, bool result);
    }
}
