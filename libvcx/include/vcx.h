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
typedef unsigned int vcx_bool_t;

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

const char *vcx_error_c_message(int);
const char *vcx_version();
vcx_error_t vcx_shutdown(vcx_bool_t delete_wallet);


/**
 * Schema object
 *
 * For creating, validating and committing a schema to the sovrin ledger.
 */

/** Creates a schema from a json string. Populates a handle to the new schema. */
vcx_error_t vcx_schema_create(vcx_command_handle_t command_handle, const char *source_id, const char *schema_name, const char *schema_data, void (*cb)(vcx_command_handle_t command_handle, vcx_error_t err, vcx_schema_handle_t schema_handle));

/** Populates status with the current state of this credential. */
vcx_error_t vcx_schema_serialize(vcx_command_handle_t command_handle, vcx_schema_handle_t schema_handle, void (*cb)(vcx_command_handle_t xcommand_handle, vcx_error_t err, const char *state));

/** Re-creates a credential object from the specified serialization. */
vcx_error_t vcx_schema_deserialize(vcx_command_handle_t command_handle, const char *serialized_schema, void (*cb)(vcx_command_handle_t xcommand_handle, vcx_error_t err, vcx_schema_handle_t schema_handle));

/** Populates data with the contents of the schema handle. */
vcx_error_t vcx_schema_get_attributes(vcx_command_handle_t command_handle, const char *source_id, vcx_schema_handle_t sequence_no,  void (*cb)(vcx_command_handle_t xcommand_handle, vcx_error_t err, const char *schema_attrs));

/** Populates sequence_no with the actual sequence number of the schema on the sovrin ledger. */
vcx_error_t vcx_schema_get_sequence_no(vcx_command_handle_t command_handle, vcx_schema_handle_t schema_handle, void (*cb)(vcx_command_handle_t xcommand_handle, vcx_error_t err, vcx_schema_handle_t sequence_no));

/** Release memory associated with schema object. */
vcx_error_t vcx_schema_release(vcx_schema_handle_t handle);


/**
 * credentialdef object
 *
 * For creating, validating and committing a credential definition to the sovrin ledger.
 */

/** Creates a credential definition from the given schema.  Populates a handle to the new credentialdef. */
vcx_error_t vcx_credentialdef_create(vcx_command_handle_t command_handle, const char *source_id, const char *credentialdef_name, vcx_schema_handle_t schema_seq_no, vcx_bool_t revocation, void (*cb)(vcx_command_handle_t command_handle, vcx_error_t err, vcx_credentialdef_handle_t credentialdef_handle));

/** Populates status with the current state of this credential. */
vcx_error_t vcx_credentialdef_serialize(vcx_command_handle_t command_handle, vcx_credentialdef_handle_t credentialdef_handle, void (*cb)(vcx_command_handle_t xcommand_handle, vcx_error_t err, const char *state));

/** Re-creates a credential object from the specified serialization. */
vcx_error_t vcx_credentialdef_deserialize(vcx_command_handle_t command_handle, const char *serialized_credentialdef, void (*cb)(vcx_command_handle_t xcommand_handle, vcx_error_t err, vcx_credentialdef_handle_t credentialdef_handle));

/** Asynchronously commits the credentialdef to the ledger.  */
vcx_error_t vcx_credentialdef_commit(vcx_credentialdef_handle_t credentialdef_handle);

/** Populates sequence_no with the actual sequence number of the credentialdef on the sovrin ledger. */
vcx_error_t vcx_credentialdef_get_sequence_no(vcx_credentialdef_handle_t credentialdef_handle, int *sequence_no);

/** Populates data with the contents of the credentialdef handle. */
vcx_error_t vcx_credentialdef_get(vcx_credentialdef_handle_t credentialdef_handle, char *data);


/**
 * connection object
 *
 * For creating a connection with an identity owner for interactions such as exchanging
 * credentials and proofs.
 */

/** Creates a connection object to a specific identity owner. Populates a handle to the new connection. */
vcx_error_t vcx_connection_create(vcx_command_handle_t command_handle, const char *source_id, void (*cb)(vcx_command_handle_t command_handle, vcx_error_t err, vcx_connection_handle_t connection_handle));

/** Asynchronously request a connection be made. */
vcx_error_t vcx_connection_connect(vcx_command_handle_t command_handle, vcx_connection_handle_t connection_handle, const char *connection_type, void (*cb)(vcx_command_handle_t, vcx_error_t err));

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
vcx_error_t vcx_issuer_create_credential(vcx_command_handle_t command_handle, const char *source_id, vcx_schema_handle_t schema_seq_no, const char *issuer_did, const char * credential_data, const char * credential_name, void (*cb)(vcx_command_handle_t command_handle, vcx_error_t err, vcx_credential_handle_t credential_handle));

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
vcx_error_t vcx_disclosed_proof_create_with_msgid(vcx_command_handle_t command_handle, const char *source_id, vcx_connection_handle_t connection, const char *msg_id, void (*cb)(vcx_command_handle_t command_handle, vcx_error_t err, vcx_proof_handle_t proof_handle));

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

/** Releases the disclosed_proof from memory. */
vcx_error_t vcx_disclosed_proof_release(vcx_proof_handle_t proof_handle);

/**
 * credential object
 *
 * Used for accepting and requesting a credential with an identity owner.
 */

/** Creates a credential object from the specified credentialdef handle. Populates a handle the new credential. */
vcx_error_t vcx_credential_create_with_offer(vcx_command_handle_t command_handle, const char *source_id, const char *credential_offer,void (*cb)(vcx_command_handle_t command_handle, vcx_error_t err, vcx_credential_handle_t credential_handle));

/** Creates a credential object from the connection and msg id. Populates a handle the new credential. */
vcx_error_t vcx_credential_create_with_msgid(vcx_command_handle_t command_handle, const char *source_id, vcx_connection_handle_t connection, const char *msg_id,void (*cb)(vcx_command_handle_t command_handle, vcx_error_t err, vcx_credential_handle_t credential_handle));

/** Asynchronously sends the credential request to the connection. */
vcx_error_t vcx_credential_send_request(vcx_command_handle_t command_handle, vcx_credential_handle_t credential_handle, vcx_connection_handle_t connection_handle, void (*cb)(vcx_command_handle_t xcommand_handle, vcx_error_t err));

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
