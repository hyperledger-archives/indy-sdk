#ifndef __indy_agent__included__
#define __indy_agent__included__

#ifdef __cplusplus
extern "C" {
#endif

extern indy_error_t indy_prep_msg(indy_handle_t          command_handle,
                                  indy_handle_t          wallet_handle,
                                  const char *const      sender_vk,
                                  const char *const      recipient_vk,
                                  const indy_u8_t *const msg_data,
                                  indy_u32_t             msg_len,

                                  void                   (*cb)(indy_handle_t          command_handle,
                                                               indy_error_t           err,
                                                               const indy_u8_t *const encrypted_msg,
                                                               indy_u32_t             encrypted_len)
                                 );

extern indy_error_t indy_prep_anonymous_msg(indy_handle_t          command_handle,
                                            const char *const      recipient_vk,
                                            const indy_u8_t *const msg_data,
                                            indy_u32_t             msg_len,

                                            void                   (*cb)(indy_handle_t          command_handle,
                                                                         indy_error_t           err,
                                                                         const indy_u8_t *const encrypted_msg,
                                                                         indy_u32_t             encrypted_len)
                                 );

extern indy_error_t indy_parse_msg(indy_handle_t          command_handle,
                                   indy_handle_t          wallet_handle,
                                   const char *const      recipient_vk,
                                   const indy_u8_t *const encrypted_msg,
                                   indy_u32_t             encrypted_len,

                                   void                   (*cb)(indy_handle_t          command_handle,
                                                                indy_error_t           err,
                                                                const char *const      sender_vk,
                                                                const indy_u8_t *const msg_data,
                                                                indy_u32_t             msg_len)
                                  );

#ifdef __cplusplus
}
#endif

#endif
