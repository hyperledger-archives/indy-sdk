namespace Hyperledger.Indy.PaymentsApi
{
    /// <summary>
    /// Information passed to libindy is incompatible.
    /// </summary>
    public class IncompatiblePaymentMethodsException : IndyException
    {
        const string message = "Information passed to libindy is incompatible.";

        internal IncompatiblePaymentMethodsException() : base(message, (int)ErrorCode.PaymentIncompatibleMethodsError)
        {
        }
    }
}
