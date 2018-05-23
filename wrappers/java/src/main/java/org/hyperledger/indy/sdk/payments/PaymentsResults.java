package org.hyperledger.indy.sdk.payments;

public class PaymentsResults {

    /**
     * Result from calling addRequestFees
     */
    public static class AddRequestFeesResult {
        private String reqWithFeesJson;
        private String paymentMethod;

        public AddRequestFeesResult(String reqWithFeesJson, String paymentMethod) {
            this.reqWithFeesJson = reqWithFeesJson;
            this.paymentMethod = paymentMethod;
        }

        public String getReqWithFeesJson() {
            return reqWithFeesJson;
        }

        public String getPaymentMethod() {
            return paymentMethod;
        }
    }

    /**
     * Result from calling buildGetUtxoRequest
     */
    public static class BuildGetUtxoRequestResult {
        private String getUtxoTxnJson;
        private String paymentMethod;

        public BuildGetUtxoRequestResult(String getUtxoTxnJson, String paymentMethod) {
            this.getUtxoTxnJson = getUtxoTxnJson;
            this.paymentMethod = paymentMethod;
        }

        public String getGetUtxoTxnJson() {
            return getUtxoTxnJson;
        }

        public String getPaymentMethod() {
            return paymentMethod;
        }
    }

    /**
     * Result from calling buildPaymentRequest
     */
    public static class BuildPaymentReqResult {
        private String paymentReqJson;
        private String paymentMethod;

        public BuildPaymentReqResult(String paymentReqJson, String paymentMethod) {
            this.paymentReqJson = paymentReqJson;
            this.paymentMethod = paymentMethod;
        }

        public String getPaymentReqJson() {
            return paymentReqJson;
        }

        public String getPaymentMethod() {
            return paymentMethod;
        }
    }

    /**
     * Result from calling buildMintRequest
     */
    public static class BuildMintReqResult {
        private String mintReqJson;
        private String paymentMethod;

        public BuildMintReqResult(String mintReqJson, String paymentMethod) {
            this.mintReqJson = mintReqJson;
            this.paymentMethod = paymentMethod;
        }

        public String getMintReqJson() {
            return mintReqJson;
        }

        public String getPaymentMethod() {
            return paymentMethod;
        }
    }
}
