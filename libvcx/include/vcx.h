#ifndef __VCX_H
#define __VCX_H

#ifdef __cplusplus
extern "C" {
#endif

typedef enum
{
  none = 0,
  initialized,
  offer_sent,
  request_received,
  accepted,
  unfulfilled,
  expired,
  revoked,
} vcx_state_t;

typedef enum
{
  undefined = 0,
  validated = 1,
  invalid = 2,
} vcx_proof_state_t;

typedef unsigned int vcx_error_t;
typedef unsigned int vcx_schema_handle_t;
typedef unsigned int vcx_credentialdef_handle_t;
typedef unsigned int vcx_connection_handle_t;
typedef unsigned int vcx_credential_handle_t;
typedef unsigned int vcx_proof_handle_t;
typedef unsigned int vcx_command_handle_t;
typedef unsigned int vcx_payment_handle_t;
typedef unsigned int vcx_wallet_search_handle_t;
typedef signed int indy_signed_t;
typedef unsigned int vcx_bool_t;
typedef unsigned int count_t;
typedef unsigned long vcx_price_t;

typedef struct {

  union {
    vcx_schema_handle_t schema_handle;
    vcx_credentialdef_handle_t credentialdef_handle;
    vcx_connection_handle_t connection_handle;
    vcx_credential_handle_t credential_handle;
    vcx_proof_handle_t proof_handle;
  } handle;

  vcx_error_t status;
  char *msg;

} vcx_status_t;


/**
 * Initialize the SDK
 */

vcx_error_t vcx_init(vcx_command_handle_t handle, const char *config_path,void (*cb)(vcx_command_handle_t command_handle, vcx_error_t err));
vcx_error_t vcx_create_agent(vcx_command_handle_t handle, const char *config, void (*cb)(vcx_command_handle_t xhandle, vcx_error_t err, const char *xconfig));
vcx_error_t vcx_update_agent_info(vcx_command_handle_t handle, const char *info, void (*cb)(vcx_command_handle_t xhandle, vcx_error_t err));

vcx_error_t vcx_ledger_get_fees(vcx_command_handle_t chandle, void (*cb)(vcx_command_handle_t xhandle, vcx_error_t err, const char *fees));

/**
 * Wallet
 */

/** Returns wallet token information including address, utxo, and token information */
vcx_error_t vcx_wallet_get_token_info(vcx_command_handle_t chandle, vcx_payment_handle_t phandle, void (*cb)(vcx_command_handle_t xhandle, vcx_error_t err, const char *info));

/** Creates a new payment address in the wallet */
vcx_error_t vcx_wallet_create_payment_address(vcx_command_handle_t chandle, void (*cb)(vcx_command_handle_t xhandle, vcx_error_t err, const char *address));
vcx_error_t vcx_wallet_send_tokens(vcx_command_handle_t chandle, vcx_payment_handle_t phandle, vcx_price_t tokens, const char *recipient, void (*cb)(vcx_command_handle_t xhandle, vcx_error_t err, const char *receipt));

/** Passthroughs to libindy wallet record API (see libindy documentation) */
vcx_error_t vcx_wallet_add_record(indy_signed_t chandle, const char * type_, const char *id, const char *value, const char *tags_json, void (*cb)(indy_signed_t xhandle, indy_signed_t err));
vcx_error_t vcx_wallet_update_record_value(indy_signed_t chandle, const char * type_, const char *id, const char *value, void (*cb)(indy_signed_t xhandle, indy_signed_t err));
vcx_error_t vcx_wallet_update_record_tags(vcx_command_handle_t chandle, const char * type_, const char *id, const char *tags_json, void (*cb)(vcx_command_handle_t xhandle, vcx_error_t err));
vcx_error_t vcx_wallet_add_record_tags(vcx_command_handle_t chandle, const char * type_, const char *id, const char *tags_json, void (*cb)(vcx_command_handle_t xhandle, vcx_error_t err));
vcx_error_t vcx_wallet_delete_record_tags(vcx_command_handle_t chandle, const char * type_, const char *id, const char *tags_json, void (*cb)(vcx_command_handle_t xhandle, vcx_error_t err));
vcx_error_t vcx_wallet_get_record(indy_signed_t chandle, const char * type_, const char *id, const char *options, void (*cb)(indy_signed_t xhandle, indy_signed_t err, const char *record_json));
vcx_error_t vcx_wallet_delete_record(indy_signed_t chandle, const char * type_, const char *id, void (*cb)(indy_signed_t xhandle, indy_signed_t err));
vcx_error_t vcx_wallet_open_search(vcx_command_handle_t chandle, const char * type_, const char *query_json, const char *options_json, void (*cb)(vcx_command_handle_t xhandle, vcx_error_t err, vcx_wallet_search_handle_t search_handle));
vcx_error_t vcx_wallet_close_search(vcx_command_handle_t chandle, vcx_command_handle_t shandle, void (*cb)(vcx_command_handle_t xhandle, vcx_error_t err));
vcx_error_t vcx_wallet_search_next_records(vcx_command_handle_t chandle, vcx_wallet_search_handle_t shandle, count_t count, void (*cb)(vcx_command_handle_t xhandle, vcx_error_t err, const char *results));
vcx_error_t vcx_wallet_import(vcx_command_handle_t chandle, const char * config, void (*cb)(vcx_command_handle_t xhandle, vcx_error_t err));
vcx_error_t vcx_wallet_export(vcx_command_handle_t chandle, const char * path, const char * backup_key, void (*cb)(vcx_command_handle_t xhandle, vcx_error_t err));
/** Functionality in Libindy for validating an address is NOT there yet **/
vcx_error_t vcx_wallet_validate_payment_address(vcx_command_handle_t chandle, const char * type_, void (*cb)(vcx_command_handle_t xhandle, vcx_error_t err));

/** Returns a human readable message for the associated error code */
const char *vcx_error_c_message(int);

/** Returns version information for libvcx */
const char *vcx_version();
/** Frees memory, resets configuration, closes wallet and pool, optionally deletes wallet */
vcx_error_t vcx_shutdown(vcx_bool_t delete_wallet);

/**
 * Schema object
 *
 * For creating, validating and committing a schema to the sovrin ledger.
 */

/** Creates a schema from a json string. Populates a handle to the new schema. */
vcx_error_t vcx_schema_create(vcx_command_handle_t command_handle, const char *source_id, const char *schema_name, const char *version, const char *schema_data, vcx_payment_handle_t payment_handle, void (*cb)(vcx_command_handle_t command_handle, vcx_error_t err, vcx_schema_handle_t schema_handle));

/** Populates status with the current state of this credential. */
vcx_error_t vcx_schema_serialize(vcx_command_handle_t command_handle, vcx_schema_handle_t schema_handle, void (*cb)(vcx_command_handle_t xcommand_handle, vcx_error_t err, const char *state));

/** Re-creates a credential object from the specified serialization. */
vcx_error_t vcx_schema_deserialize(vcx_command_handle_t command_handle, const char *serialized_schema, void (*cb)(vcx_command_handle_t xcommand_handle, vcx_error_t err, vcx_schema_handle_t schema_handle));

/** Populates data with the contents of the schema handle. */
vcx_error_t vcx_schema_get_attributes(vcx_command_handle_t command_handle, const char *source_id, vcx_schema_handle_t sequence_no,  void (*cb)(vcx_command_handle_t xcommand_handle, vcx_error_t err, const char *schema_attrs));

/** Retrieves schema_id from schema object. */
vcx_error_t vcx_schema_get_schema_id(vcx_command_handle_t command_handle, vcx_schema_handle_t schema_handle, void (*cb)(vcx_command_handle_t xcommand_handle, vcx_error_t err, const char *schema_id));

/** Release memory associated with schema object. */
vcx_error_t vcx_schema_release(vcx_schema_handle_t handle);


/**
 * credentialdef object
 *
 * For creating, validating and committing a credential definition to the sovrin ledger.
 */

/** Creates a credential definition from the given schema.  Populates a handle to the new credentialdef. */
vcx_error_t vcx_credentialdef_create(vcx_command_handle_t command_handle, const char *source_id, const char *credentialdef_name, const char *schema_id, const char *issuer_did, const char *tag,  const char *config, vcx_payment_handle_t payment_handle, void (*cb)(vcx_command_handle_t command_handle, vcx_error_t err, vcx_credentialdef_handle_t credentialdef_handle));


/** Populates status with the current state of this credential. */
vcx_error_t vcx_credentialdef_serialize(vcx_command_handle_t command_handle, vcx_credentialdef_handle_t credentialdef_handle, void (*cb)(vcx_command_handle_t xcommand_handle, vcx_error_t err, const char *state));

/** Re-creates a credential object from the specified serialization. */
vcx_error_t vcx_credentialdef_deserialize(vcx_command_handle_t command_handle, const char *serialized_credentialdef, void (*cb)(vcx_command_handle_t xcommand_handle, vcx_error_t err, vcx_credentialdef_handle_t credentialdef_handle));

/** Release memory associated with credentialdef object. */
vcx_error_t vcx_credentialdef_release(vcx_schema_handle_t handle);

/** Retrieves cred_def_id from credentialdef object. */
vcx_error_t vcx_credentialdef_get_cred_def_id(vcx_command_handle_t command_handle, vcx_credential_handle_t cred_def_handle, void (*cb)(vcx_command_handle_t xcommand_handle, vcx_error_t err, const char *cred_def_id));

/**
 * connection object
 *
 * For creating a connection with an identity owner for interactions such as exchanging
 * credentials and proofs.
 */

/** Deletes a connection object releases it from memory */
vcx_error_t vcx_connection_delete_connection(vcx_command_handle_t command_handle, vcx_connection_handle_t connection_handle, void (*cb)(vcx_command_handle_t command_handle, vcx_error_t err));

/** Creates a connection object to a specific identity owner. Populates a handle to the new connection. */
vcx_error_t vcx_connection_create(vcx_command_handle_t command_handle, const char *source_id, void (*cb)(vcx_command_handle_t command_handle, vcx_error_t err, vcx_connection_handle_t connection_handle));

/** Asynchronously request a connection be made. */
vcx_error_t vcx_connection_connect(vcx_command_handle_t command_handle, vcx_connection_handle_t connection_handle, const char *connection_type, void (*cb)(vcx_command_handle_t, vcx_error_t err, const char *details));

/** Returns the contents of the connection handle or null if the connection does not exist. */
vcx_error_t vcx_connection_serialize(vcx_command_handle_t command_handle, vcx_connection_handle_t connection_handle, void (*cb)(vcx_command_handle_t xcommand_handle, vcx_error_t err, const char *state));

/** Re-creates a connection object from the specified serialization. */
vcx_error_t vcx_connection_deserialize(vcx_command_handle_t command_handle, const char *serialized_credential, void (*cb)(vcx_command_handle_t xcommand_handle, vcx_error_t err, vcx_connection_handle_t connection_handle));

/** Request a state update from the agent for the given connection. */
vcx_error_t vcx_connection_update_state(vcx_command_handle_t command_handle, vcx_connection_handle_t connection_handle, void (*cb)(vcx_command_handle_t xcommand_handle, vcx_error_t err, vcx_state_t state));

/** Retrieves the state of the connection */
vcx_error_t vcx_connection_get_state(vcx_command_handle_t command_handle, vcx_connection_handle_t connection_handle, void (*cb)(vcx_command_handle_t xcommand_handle, vcx_error_t err, vcx_state_t state));

/** Releases the connection from memory. */
vcx_error_t vcx_connection_release(vcx_connection_handle_t connection_handle);

/** Get the invite details for the connection. */
vcx_error_t vcx_connection_invite_details(vcx_command_handle_t command_handle, vcx_connection_handle_t connection_handle, int abbreviated, void (*cb)(vcx_command_handle_t xcommand_handle, vcx_error_t err, const char *details));

/** Creates a connection from the invite details. */
vcx_error_t vcx_connection_create_with_invite(vcx_command_handle_t command_handle, const char *source_id, const char *invite_details, void (*cb)(vcx_command_handle_t xcommand_handle, vcx_error_t err, vcx_connection_handle_t connection_handle));


/**
 * credential issuer object
 *
 * Used for offering and managing a credential with an identity owner.
 */

/** Creates a credential object from the specified credentialdef handle. Populates a handle the new credential. */
vcx_error_t vcx_issuer_create_credential(vcx_command_handle_t command_handle, const char *source_id, const char *cred_def_id, const char *issuer_did, const char * credential_data, const char * credential_name, vcx_price_t price, void (*cb)(vcx_command_handle_t command_handle, vcx_error_t err, vcx_credential_handle_t credential_handle));

/** Asynchronously sends the credential offer to the connection. */
vcx_error_t vcx_issuer_send_credential_offer(vcx_command_handle_t command_handle, vcx_credential_handle_t credential_handle, vcx_connection_handle_t connection_handle, void (*cb)(vcx_command_handle_t xcommand_handle, vcx_error_t err));

/** Updates the state of the credential from the agency. */
vcx_error_t vcx_issuer_credential_update_state(vcx_command_handle_t command_handle, vcx_credential_handle_t credential_handle, void (*cb)(vcx_command_handle_t xcommand_handle, vcx_error_t err, vcx_state_t state));

/** Retrieves the state of the issuer_credential. */
vcx_error_t vcx_issuer_credential_get_state(vcx_command_handle_t command_handle, vcx_credential_handle_t credential_handle, void (*cb)(vcx_command_handle_t xcommand_handle, vcx_error_t err, vcx_state_t state));

/** Asynchronously send the credential to the connection. Populates a handle to the new transaction. */
vcx_error_t vcx_issuer_send_credential(vcx_command_handle_t command_handle, vcx_credential_handle_t credential_handle, vcx_connection_handle_t connection_handle, void (*cb)(vcx_command_handle_t command_handle, vcx_error_t err));

/** Populates status with the current state of this credential. */
vcx_error_t vcx_issuer_credential_serialize(vcx_command_handle_t command_handle, vcx_credential_handle_t credential_handle, void (*cb)(vcx_command_handle_t xcommand_handle, vcx_error_t err, const char *state));

/** Re-creates a credential object from the specified serialization. */
vcx_error_t vcx_issuer_credential_deserialize(vcx_command_handle_t, const char *serialized_credential, void (*cb)(vcx_command_handle_t xcommand_handle, vcx_error_t err, vcx_credential_handle_t credential_handle));

/** Terminates a credential for the specified reason. */
vcx_error_t vcx_issuer_terminate_credential(vcx_command_handle_t command_handle, vcx_credential_handle_t credential_handle, vcx_state_t state_type, const char *msg);

/** Releases the credential from memory. */
vcx_error_t vcx_issuer_credential_release(vcx_credential_handle_t credential_handle);

/** Populates credential_request with the latest credential request received. (not in MVP) */
vcx_error_t vcx_issuer_get_credential_request(vcx_credential_handle_t credential_handle, char *credential_request);

/** Sets the credential request in an accepted state. (not in MVP) */
vcx_error_t vcx_issuer_accept_credential(vcx_credential_handle_t credential_handle);

/**
 * proof object
 *
 * Used for requesting and managing a proof request with an identity owner.
 */

/** Creates a proof object.  Populates a handle to the new proof. */
vcx_error_t vcx_proof_create(vcx_command_handle_t command_handle, const char *source_id, const char *requested_attrs, const char *requested_predicates, const char *name, void (*cb)(vcx_command_handle_t command_handle, vcx_error_t err, vcx_proof_handle_t proof_handle));

/** Asynchronously send a proof request to the connection. */
vcx_error_t vcx_proof_send_request(vcx_command_handle_t command_handle, vcx_proof_handle_t proof_handle, vcx_connection_handle_t connection_handle, void (*cb)(vcx_command_handle_t xcommand_handle, vcx_error_t err));

/** Populate response_data with the latest proof offer received. */
vcx_error_t vcx_get_proof(vcx_command_handle_t command_handle, vcx_proof_handle_t proof_handle, vcx_connection_handle_t connection_handle, void (*cb)(vcx_command_handle_t xcommand_handle, vcx_error_t err, vcx_proof_state_t state, const char *proof_string));

/** Set proof offer as accepted. */
vcx_error_t vcx_proof_accepted(vcx_proof_handle_t proof_handle);

/** Populates status with the current state of this proof request. */
vcx_error_t vcx_proof_update_state(vcx_command_handle_t command_handle, vcx_proof_handle_t proof_handle, void (*cb)(vcx_command_handle_t xcommand_handle, vcx_error_t err, vcx_state_t state));

/** Retrieves the state of the proof. */
vcx_error_t vcx_proof_get_state(vcx_command_handle_t command_handle, vcx_proof_handle_t proof_handle, void (*cb)(vcx_command_handle_t xcommand_handle, vcx_error_t err, vcx_state_t state));

/** Populates status with the current state of this proof. */
vcx_error_t vcx_proof_serialize(vcx_command_handle_t command_handle, vcx_proof_handle_t proof_handle, void (*cb)(vcx_command_handle_t xcommand_handle, vcx_error_t err, const char *state));

/** Re-creates a proof object from the specified serialization. */
vcx_error_t vcx_proof_deserialize(vcx_command_handle_t command_handle, const char *serialized_proof, void (*cb)(vcx_command_handle_t xcommand_handle, vcx_error_t err, vcx_proof_handle_t proof_handle));

/** Releases the proof from memory. */
vcx_error_t vcx_proof_release(vcx_proof_handle_t proof_handle);

/**
 * disclosed_proof object
 *
 * Used for sending a disclosed_proof to an identity owner.
 */

/** Creates a disclosed_proof object from a proof request.  Populates a handle to the new disclosed_proof. */
vcx_error_t vcx_disclosed_proof_create_with_request(vcx_command_handle_t command_handle, const char *source_id, const char *proof_req, void (*cb)(vcx_command_handle_t command_handle, vcx_error_t err, vcx_proof_handle_t proof_handle));

/** Creates a disclosed_proof object from a msgid.  Populates a handle to the new disclosed_proof. */
vcx_error_t vcx_disclosed_proof_create_with_msgid(vcx_command_handle_t command_handle, const char *source_id, vcx_connection_handle_t connection, const char *msg_id, void (*cb)(vcx_command_handle_t command_handle, vcx_error_t err, vcx_proof_handle_t proof_handle, const char *proof_req));

/** Asynchronously send a proof to the connection. */
vcx_error_t vcx_disclosed_proof_send_proof(vcx_command_handle_t command_handle, vcx_proof_handle_t proof_handle, vcx_connection_handle_t connection_handle, void (*cb)(vcx_command_handle_t xcommand_handle, vcx_error_t err));

/** Populates status with the current state of this disclosed_proof request. */
vcx_error_t vcx_disclosed_proof_update_state(vcx_command_handle_t command_handle, vcx_proof_handle_t proof_handle, void (*cb)(vcx_command_handle_t xcommand_handle, vcx_error_t err, vcx_state_t state));

/** Check for any proof requests from the connection. */
vcx_error_t vcx_disclosed_proof_get_requests(vcx_command_handle_t command_handle, vcx_connection_handle_t connection_handle, void (*cb)(vcx_command_handle_t xcommand_handle, vcx_error_t err, const char *offers));

/** Retrieves the state of the disclosed_proof. */
vcx_error_t vcx_disclosed_proof_get_state(vcx_command_handle_t command_handle, vcx_proof_handle_t proof_handle, void (*cb)(vcx_command_handle_t xcommand_handle, vcx_error_t err, vcx_state_t state));

/** Populates status with the current state of this disclosed_proof. */
vcx_error_t vcx_disclosed_proof_serialize(vcx_command_handle_t command_handle, vcx_proof_handle_t proof_handle, void (*cb)(vcx_command_handle_t xcommand_handle, vcx_error_t err, const char *state));

/** Re-creates a disclosed_proof object from the specified serialization. */
vcx_error_t vcx_disclosed_proof_deserialize(vcx_command_handle_t command_handle, const char *serialized_proof, void (*cb)(vcx_command_handle_t xcommand_handle, vcx_error_t err, vcx_proof_handle_t proof_handle));

/** Generates a proof from proof request, selected credentials and self attested attributes */
vcx_error_t vcx_disclosed_proof_generate_proof(vcx_command_handle_t command_handle, vcx_proof_handle_t proof_handle, const char *selected_credentials, const char *self_attested_attrs, void (*cb)(vcx_command_handle_t xcommand_handle, vcx_error_t err));

/** Returns credentials matching provided proof request. */
vcx_error_t vcx_disclosed_proof_retrieve_credentials(vcx_command_handle_t command_handle, vcx_proof_handle_t proof_handle, void (*cb)(vcx_command_handle_t xcommand_handle, vcx_error_t err, const char *credentials));

/** Releases the disclosed_proof from memory. */
vcx_error_t vcx_disclosed_proof_release(vcx_proof_handle_t proof_handle);

/**
 * credential object
 *
 * Used for accepting and requesting a credential with an identity owner.
 */

/** Creates a credential object from the specified credentialdef handle. Populates a handle the new credential. */
vcx_error_t vcx_credential_create_with_offer(vcx_command_handle_t command_handle, const char *source_id, const char *credential_offer,void (*cb)(vcx_command_handle_t command_handle, vcx_error_t err, vcx_credential_handle_t credential_handle));

/** Retrieves payment information string from the specified credential handle */
vcx_error_t vcx_credential_get_payment_info(vcx_command_handle_t command_handle, vcx_credential_handle_t credential_handle, void (*cb)(vcx_command_handle_t command_handle, vcx_error_t err, const char *info));

/** Creates a credential object from the connection and msg id. Populates a handle the new credential. */
vcx_error_t vcx_credential_create_with_msgid(vcx_command_handle_t command_handle, const char *source_id, vcx_connection_handle_t connection, const char *msg_id,void (*cb)(vcx_command_handle_t command_handle, vcx_error_t err, vcx_credential_handle_t credential_handle));

/** Asynchronously sends the credential request to the connection. */
vcx_error_t vcx_credential_send_request(vcx_command_handle_t command_handle, vcx_credential_handle_t credential_handle, vcx_connection_handle_t connection_handle, vcx_payment_handle_t payment_handle, void (*cb)(vcx_command_handle_t xcommand_handle, vcx_error_t err));

/** Check for any credential offers from the connection. */
vcx_error_t vcx_credential_get_offers(vcx_command_handle_t command_handle, vcx_connection_handle_t connection_handle, void (*cb)(vcx_command_handle_t xcommand_handle, vcx_error_t err, const char *offers));

/** Updates the state of the credential from the agency. */
vcx_error_t vcx_credential_update_state(vcx_command_handle_t command_handle, vcx_credential_handle_t credential_handle, void (*cb)(vcx_command_handle_t xcommand_handle, vcx_error_t err, vcx_state_t state));

/** Retrieves the state of the credential - including storing the credential if it has been sent. */
vcx_error_t vcx_credential_get_state(vcx_command_handle_t command_handle, vcx_credential_handle_t credential_handle, void (*cb)(vcx_command_handle_t xcommand_handle, vcx_error_t err, vcx_state_t state));

/** Populates status with the current state of this credential. */
vcx_error_t vcx_credential_serialize(vcx_command_handle_t command_handle, vcx_credential_handle_t credential_handle, void (*cb)(vcx_command_handle_t xcommand_handle, vcx_error_t err, const char *state));

/** Re-creates a credential from the specified serialization. */
vcx_error_t vcx_credential_deserialize(vcx_command_handle_t, const char *serialized_credential, void (*cb)(vcx_command_handle_t xcommand_handle, vcx_error_t err, vcx_credential_handle_t credential_handle));

/** Releases the credential from memory. */
vcx_error_t vcx_credential_release(vcx_credential_handle_t credential_handle);

/** For testing purposes only */
void vcx_set_next_agency_response(int);
#ifdef __cplusplus
}
#endif

#endif
