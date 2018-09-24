namespace Hyperledger.Indy.PaymentsApi
{
    /// <summary>
    /// Payment result.
    /// </summary>
    public class PaymentResult
    {
        /// <summary>
        /// Initializes a new instance of the <see cref="T:Hyperledger.Indy.PaymentsApi.PaymentResult"/> class.
        /// </summary>
        /// <param name="result">Result.</param>
        /// <param name="paymentMethod">Payment method.</param>
        public PaymentResult(string result, string paymentMethod)
        {
            Result = result;
            PaymentMethod = paymentMethod;
        }
        /// <summary>
        /// Gets the payment method.
        /// </summary>
        /// <value>The payment method.</value>
        public string PaymentMethod { get; }

        /// <summary>
        /// Gets the result.
        /// </summary>
        /// <value>The result.</value>
        public string Result { get; }
    }
}
