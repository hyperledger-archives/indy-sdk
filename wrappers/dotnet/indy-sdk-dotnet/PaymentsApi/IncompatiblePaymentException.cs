namespace Hyperledger.Indy.PaymentsApi
{
    /// <summary>
    /// Information passed to libindy is incompatible.
    /// </summary>
    public class IncompatiblePaymentException : IndyException
    {
        const string message = "Information passed to libindy is incompatible.";

        internal IncompatiblePaymentException() : base(message, (int)ErrorCode.IncompatiblePaymentError)
        {
        }
    }
}
