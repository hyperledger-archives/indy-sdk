namespace Hyperledger.Indy.PaymentsApi
{
    /// <summary>
    /// No such source found.
    /// </summary>
    public class PaymentSourceDoesNotExistException : IndyException
    {
        const string message = "No such source found.";

        internal PaymentSourceDoesNotExistException() : base(message, (int)ErrorCode.PaymentSourceDoesNotExistError)
        {
        }
    }
}
