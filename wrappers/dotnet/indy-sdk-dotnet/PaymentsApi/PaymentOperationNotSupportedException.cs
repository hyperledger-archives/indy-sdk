namespace Hyperledger.Indy.PaymentsApi
{
    /// <summary>
    /// Operation is not supported for payment method.
    /// </summary>
    public class PaymentOperationNotSupportedException : IndyException
    {
        const string message = "Operation is not supported for payment method.";

        internal PaymentOperationNotSupportedException() : base(message, (int)ErrorCode.PaymentOperationNotSupportedError)
        {
        }
    }
}
