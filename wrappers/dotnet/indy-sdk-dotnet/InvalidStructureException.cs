namespace Hyperledger.Indy
{
    /// <summary>
    /// Exception indicating that a value being processed was not considered a valid value.
    /// </summary>
    public class InvalidStructureException : IndyException
    {
        const string message = "A value being processed is not valid.";

        /// <summary>
        /// Initializes a new InvalidStructureException.
        /// </summary>
        internal InvalidStructureException() : base(message, (int)ErrorCode.CommonInvalidStructure)
        {

        }
    }

}
