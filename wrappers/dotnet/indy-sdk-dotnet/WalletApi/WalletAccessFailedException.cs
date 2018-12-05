namespace Hyperledger.Indy.WalletApi
{
    /// <summary>
    /// Exception thrown when attempting to open a wallet using invalid credentials.
    /// </summary>
    public class WalletAccessFailedException : IndyException
    {
        const string message = "The wallet could not be opened because invalid credentials were provided.";

        /// <summary>
        /// Initializes a new WalletAccessFailedException.
        /// </summary>
        internal WalletAccessFailedException() : base(message, (int)ErrorCode.WalletAccessFailed)
        {

        }
    }

}
