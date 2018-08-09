LOCAL_PATH := $(call my-dir)
LOCAL_ALLOW_UNDEFINED_SYMBOLS := true
include $(CLEAR_VARS)
PREBUILT_DEPS=$(LOCAL_PATH)/prebuilt/libs


include $(CLEAR_VARS)
LOCAL_MODULE := libzmq
LOCAL_SRC_FILES := $(PREBUILT_DEPS)/$(TARGET_ARCH_ABI)/libzmq_4.2.2/lib/libzmq.so
LOCAL_EXPORT_C_INCLUDES := $(PREBUILT_DEPS)/$(TARGET_ARCH_ABI)/libzmq_4.2.2/lib/include
include $(PREBUILT_SHARED_LIBRARY)

#include $(CLEAR_VARS)
#LOCAL_MODULE := libzmq_mike
#LOCAL_SRC_FILES := $(PREBUILT_DEPS)/$(TARGET_ARCH_ABI)/libzmq_4.2.5_arm/lib/libzmq.so
#LOCAL_EXPORT_C_INCLUDES := $(PREBUILT_DEPS)/$(TARGET_ARCH_ABI)/libzmq_4.2.5_arm/lib/include
#include $(PREBUILT_SHARED_LIBRARY)

include $(CLEAR_VARS)
LOCAL_MODULE := libvcx
LOCAL_SRC_FILES := $(PREBUILT_DEPS)/$(TARGET_ARCH_ABI)/libvcx/libvcx.so
#LOCAL_EXPORT_C_INCLUDES := $(PREBUILT_DEPS)/$(TARGET_ARCH_ABI)/libvcx/include/vcx.h
include $(PREBUILT_SHARED_LIBRARY)

include $(CLEAR_VARS)
LOCAL_MODULE := libssl
LOCAL_EXPORT_C_INCLUDES := $(PREBUILT_DEPS)/$(TARGET_ARCH_ABI)/openssl_1.1.0c/include/openssl
LOCAL_SRC_FILES := $(PREBUILT_DEPS)/$(TARGET_ARCH_ABI)/openssl_1.1.0c/lib/libssl.a
include $(PREBUILT_STATIC_LIBRARY)

include $(CLEAR_VARS)
LOCAL_MODULE := libcrypto
LOCAL_EXPORT_C_INCLUDES := $(PREBUILT_DEPS)/$(TARGET_ARCH_ABI)/openssl_1.1.0c/include/openssl
LOCAL_SRC_FILES := $(PREBUILT_DEPS)/$(TARGET_ARCH_ABI)/openssl_1.1.0c/lib/libcrypto.a
include $(PREBUILT_STATIC_LIBRARY)

include $(CLEAR_VARS)
LOCAL_MODULE := libsodium
LOCAL_SRC_FILES := $(PREBUILT_DEPS)/$(TARGET_ARCH_ABI)/libsodium_1.0.12/lib/libsodium.a
LOCAL_EXPORT_C_INCLUDES := $(PREBUILT_DEPS)/$(TARGET_ARCH_ABI)/libsodium_1.0.12/include
include $(PREBUILT_STATIC_LIBRARY)

include $(CLEAR_VARS)
LOCAL_MODULE := vcx_shim
LOCAL_WHOLE_STATIC_LIBRARIES := libcrypto libssl libsodium
LOCAL_SHARED_LIBRARIES :=  libzmq libvcx
LOCAL_LDLIBS += -lz -ldl -static-libgcc
include $(BUILD_SHARED_LIBRARY)

