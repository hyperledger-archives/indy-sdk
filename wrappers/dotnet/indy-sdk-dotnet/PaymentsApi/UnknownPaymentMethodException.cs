namespace Hyperledger.Indy.PaymentsApi
{
    /// <summary>
    /// An unknown payment method was called.
    /// </summary>
    public class UnknownPaymentMethodException : IndyException
    {
        const string message = "An unknown payment method was called.";

        internal UnknownPaymentMethodException() : base(message, (int)ErrorCode.PaymentUnknownMethodError)
        {
        }
    }
}
