namespace Hyperledger.Indy.BlobStorageApi
{
    /// <summary>
    /// BLOB storage reader.
    /// </summary>
    public class BlobStorageReader
    {
	    internal BlobStorageReader(int handle)
		{
			Handle = handle;
		}

        internal int Handle { get; }
    }
}
