namespace Hyperledger.Indy.PaymentsApi
{
    /// <summary>
    /// Insufficient funds on inputs.
    /// </summary>
    public class InsufficientFundsException : IndyException
    {
        const string message = "Insufficient funds on inputs.";

        internal InsufficientFundsException() : base(message, (int)ErrorCode.PaymentInsufficientFundsError)
        {
        }
    }
}
