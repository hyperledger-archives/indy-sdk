package org.hyperledger.indy.sdk.wallet;

import org.hyperledger.indy.sdk.ErrorCode;

import com.sun.jna.Callback;
import com.sun.jna.Native;
import com.sun.jna.Pointer;
import com.sun.jna.ptr.PointerByReference;

/**
 * Base type for implementing custom wallet types.
 */
public abstract class WalletType {

	/**
	 * Callback called when a wallet is being created.
	 */
	private Callback createCb = new Callback() {

		@SuppressWarnings("unused")
		public int callback(String name, String config, String credentials) {

			try
			{
				return create(name, config, credentials).value();
			}
			catch(Exception e)
			{
				return ErrorCode.CommonInvalidState.value();
			}
		}
	};

	/**
	 * Callback called when a wallet is being opened.
	 */
	private Callback openCb = new Callback() {

		@SuppressWarnings("unused")
		public int callback(String name, String config, String runtime_config, String credentials, Pointer handle) {

			try
			{
				HandleByReference walletHandle = new HandleByReference(); 
				ErrorCode result = open(name, config, runtime_config, credentials, walletHandle);
				handle.setInt(0, walletHandle.getValue());
				
				return result.value();
			}
			catch(Exception e)
			{
				return ErrorCode.CommonInvalidState.value();
			}
		}
	};

	/**
	 * Callback called when a value is being set on a wallet.
	 */
	private Callback setCb = new Callback() {

		@SuppressWarnings("unused")
		public int callback(int handle, String key, String value) {
			
            try
			{
            	CustomWallet wallet = getWalletByHandle(handle);
                return wallet.set(key, value).value();  
			}
			catch(Exception e)
			{
				return ErrorCode.CommonInvalidState.value();
			}
		}
	};

	/**
	 * Callback called value is requested from a wallet.
	 */
	private Callback getCb = new Callback() {

		@SuppressWarnings("unused")
		public int callback(int handle, String key, PointerByReference value_ptr) {

			try
			{
            	CustomWallet wallet = getWalletByHandle(handle);
            	            	
            	StringByReference resultString = new StringByReference();
                ErrorCode result = wallet.get(key, resultString);
                
                if(result != ErrorCode.Success)
                	return result.value();
                
                Pointer marshalledValue = marshalToNative(resultString.getValue());
                value_ptr.setValue(marshalledValue);
                		
                return result.value();  
			}
			catch(Exception e)
			{
				return ErrorCode.CommonInvalidState.value();
			}
		}
	};

	/**
	 * Callback called when an unexpired value is being requested from a wallet.
	 */
	private Callback getNotExpiredCb = new Callback() {

		@SuppressWarnings("unused")
		public int callback(int handle, String key, PointerByReference value_ptr) {
			
			try
			{
            	CustomWallet wallet = getWalletByHandle(handle);
            	
            	StringByReference resultString = new StringByReference();
                ErrorCode result = wallet.getNotExpired(key, resultString);
                
                if(result != ErrorCode.Success)
                	return result.value();
                
                Pointer marshalledValue = marshalToNative(resultString.getValue());
                value_ptr.setValue(marshalledValue);
                		
                return result.value();  
			}
			catch(Exception e)
			{
				return ErrorCode.CommonInvalidState.value();
			}
		}
	};

	/**
	 * Callback called when a list of values is being requested from a wallet.
	 */
	private Callback listCb = new Callback() {

		@SuppressWarnings("unused")
		public int callback(int handle, String key_prefix, PointerByReference values_json_ptr) {

			try
			{
            	CustomWallet wallet = getWalletByHandle(handle);
            	
            	StringByReference resultString = new StringByReference();
                ErrorCode result = wallet.list(key_prefix, resultString);
                
                if(result != ErrorCode.Success)
                	return result.value();
                
                Pointer marshalledValue = marshalToNative(resultString.getValue());
                values_json_ptr.setValue(marshalledValue);
                		
                return result.value();  
			}
			catch(Exception e)
			{
				return ErrorCode.CommonInvalidState.value();
			}
		}
	};

	/**
	 * Callback called when a wallet is being closed.
	 */
	private Callback closeCb = new Callback() {

		@SuppressWarnings("unused")
		public int callback(int handle) {

			try
			{            	
            	return close(handle).value();  
			}
			catch(Exception e)
			{
				return ErrorCode.CommonInvalidState.value();
			}			
		}
	};

	/**
	 * Callback called when a wallet is being deleted.
	 */
	private Callback deleteCb = new Callback() {

		@SuppressWarnings("unused")
		public int callback(String name, String config, String credentials) {

			try
			{            	
            	return delete(name, config, credentials).value();  
			}
			catch(Exception e)
			{
				return ErrorCode.CommonInvalidState.value();
			}			
		}
	};

	/**
	 * Callback called when value requested from a wallet is no longer being used and should be freed.
	 */
	private Callback freeCb = new Callback() {

		@SuppressWarnings("unused")
		public int callback(int wallet_handle, Pointer value) {

			try
			{       			
				Native.free(Pointer.nativeValue(value));
            	return ErrorCode.Success.value();  
			}
			catch(Exception e)
			{
				return ErrorCode.CommonInvalidState.value();
			}			
		}
	};
	
	/**
	 * Gets the create callback.
	 * 
	 * @return The create callback.
	 */
	Callback getCreateCb() {
		return createCb;
	}

	/**
	 * Gets the open callback.
	 * 
	 * @return The open callback.
	 */
	Callback getOpenCb() {
		return openCb;
	}

	/**
	 * Gets the set callback.
	 * 
	 * @return The set callback.
	 */
	Callback getSetCb() {
		return setCb;
	}

	/**
	 * Gets the get callback.
	 * 
	 * @return The get callback.
	 */
	Callback getGetCb() {
		return getCb;
	}

	/**
	 * Gets the getNotExpired callback.
	 * 
	 * @return The getNotExpired callback.
	 */
	Callback getGetNotExpiredCb() {
		return getNotExpiredCb;
	}

	/**
	 * Gets the list callback.
	 * 
	 * @return The list callback.
	 */
	Callback getListCb() {
		return listCb;
	}

	/**
	 * Gets the close callback.
	 * 
	 * @return The close callback.
	 */
	Callback getCloseCb() {
		return closeCb;
	}

	/**
	 * Gets the delete callback.
	 * 
	 * @return The delete callback.
	 */
	Callback getDeleteCb() {
		return deleteCb;
	}

	/**
	 * Gets the free callback.
	 * 
	 * @return The free callback.
	 */
	Callback getFreeCb() {
		return freeCb;
	}
	
	/**
	 * Creates a new wallet.
	 * 
	 * @param name The name of the wallet to create.
	 * @param config The configuration of the wallet.
	 * @param credentials The credentials of the wallet.
	 * @return An Errorcode indicating the outcome.
	 */
	public abstract ErrorCode create(String name, String config, String credentials);
	
	/**
	 * Opens a wallet.
	 * 
	 * @param name The name of the wallet to open.
	 * @param config The configuration for the wallet specified on creation.
	 * @param runtimeConfig The runtime configuration for the wallet.
	 * @param credentials The credentials for the wallet.
	 * @param walletHandle A handle for the opened wallet instance to be set by the implementer.
	 * @return An Errorcode indicating the outcome.
	 */
	public abstract ErrorCode open(String name, String config, String runtimeConfig, String credentials, HandleByReference walletHandle);
		
	/**
	 * Closes a wallet.
	 * 
	 * @param handle The handle of the wallet to close.
	 * @return An ErrorCode indicating the outcome.
	 */
	public abstract ErrorCode close(int handle);
	
	/**
	 * Deletes a wallet.
	 * 
	 * @param name The name of the wallet.
	 * @param config The configuration of the wallet.
	 * @param credentials The credentials of the wallet.
	 * @return An ErrorCode indicating the outcome.
	 */
	public abstract ErrorCode delete(String name, String config, String credentials);
		
	/**
	 * Gets an open wallet by its handle.
	 * 
	 * @param handle The handle of the open wallet.
	 * @return The wallet instance associated with the handle.
	 */
	protected abstract CustomWallet getWalletByHandle(int handle);
	
	/**
	 * Marshals a string value to unmanaged memory and returns a pointer to the native memory.
	 * 
	 * @param value The value to marshal.
	 * @return A pointer to the native memory containing the marshalled value.
	 */
	private Pointer marshalToNative(String value) {
		byte[] bytes = Native.toByteArray(value, "UTF-8");
		Pointer pointer = new Pointer(Native.malloc(bytes.length));
		pointer.write(0, bytes, 0, bytes.length);
		return pointer;
	}
	
	/**
	 * A result returned from a wallet method that gets a value.
	 */
	public class StringByReference {
		
		private String value;
		
		/**
		 * Sets the value for the result.
		 * 
		 * @param value The value.
		 */
		public void setValue(String value) {
			this.value = value;
		}
		
		/**
		 * Gets the value from the result.
		 * 
		 * @return The value.
		 */
		public String getValue() {
			return this.value;
		}		
	}
	
	/**
	 * A handle returned from a wallet method.
	 */
	public class HandleByReference {
		private int handle;
		
		/**
		 * Sets the handle value.
		 * 
		 * @param value The handle.
		 */
		public void setValue(int value) {
			handle = value;
		}
		
		/**
		 * Gets the handle value.
		 * 
		 * @return The handle.
		 */
		public int getValue() {
			return handle;
		}
	}
	
}
