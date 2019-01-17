namespace Hyperledger.Indy.PaymentsApi
{
    /// <summary>
    /// Extra funds on inputs.
    /// </summary>
    public class ExtraFundsException : IndyException
    {
        const string message = "Extra funds on inputs.";

        internal ExtraFundsException() : base(message, (int)ErrorCode.PaymentExtraFundsError)
        {
        }
    }
}
