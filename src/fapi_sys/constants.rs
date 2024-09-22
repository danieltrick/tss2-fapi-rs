/* SPDX-License-Identifier: BSD-3-Clause */
/*******************************************************************************
 * Copyright 2024, Fraunhofer SIT sponsored by the ELISA research project
 * All rights reserved.
 ******************************************************************************/

#![allow(unused)]

use crate::fapi_sys::{TPM2_ALG_ID, TSS2_RC};

/* General error codes */
pub const TSS2_RC_SUCCESS: TSS2_RC = 0x0;

/* Layer codes */
pub const TSS2_TPM_RC_LAYER: TSS2_RC = 0x00000;
pub const TSS2_FEATURE_RC_LAYER: TSS2_RC = 0x60000;
pub const TSS2_ESAPI_RC_LAYER: TSS2_RC = 0x70000;
pub const TSS2_SYS_RC_LAYER: TSS2_RC = 0x80000;
pub const TSS2_MU_RC_LAYER: TSS2_RC = 0x90000;
pub const TSS2_TCTI_RC_LAYER: TSS2_RC = 0xA0000;
pub const TSS2_RESMGR_RC_LAYER: TSS2_RC = 0xB0000;
pub const TSS2_RESMGR_TPM_RC_LAYER: TSS2_RC = 0xC0000;

/* Error codes */
pub const TSS2_BASE_RC_GENERAL_FAILURE: TSS2_RC = 1;
pub const TSS2_BASE_RC_NOT_IMPLEMENTED: TSS2_RC = 2;
pub const TSS2_BASE_RC_BAD_CONTEXT: TSS2_RC = 3;
pub const TSS2_BASE_RC_ABI_MISMATCH: TSS2_RC = 4;
pub const TSS2_BASE_RC_BAD_REFERENCE: TSS2_RC = 5;
pub const TSS2_BASE_RC_INSUFFICIENT_BUFFER: TSS2_RC = 6;
pub const TSS2_BASE_RC_BAD_SEQUENCE: TSS2_RC = 7;
pub const TSS2_BASE_RC_NO_CONNECTION: TSS2_RC = 8;
pub const TSS2_BASE_RC_TRY_AGAIN: TSS2_RC = 9;
pub const TSS2_BASE_RC_IO_ERROR: TSS2_RC = 10;
pub const TSS2_BASE_RC_BAD_VALUE: TSS2_RC = 11;
pub const TSS2_BASE_RC_NOT_PERMITTED: TSS2_RC = 12;
pub const TSS2_BASE_RC_INVALID_SESSIONS: TSS2_RC = 13;
pub const TSS2_BASE_RC_NO_DECRYPT_PARAM: TSS2_RC = 14;
pub const TSS2_BASE_RC_NO_ENCRYPT_PARAM: TSS2_RC = 15;
pub const TSS2_BASE_RC_BAD_SIZE: TSS2_RC = 16;
pub const TSS2_BASE_RC_MALFORMED_RESPONSE: TSS2_RC = 17;
pub const TSS2_BASE_RC_INSUFFICIENT_CONTEXT: TSS2_RC = 18;
pub const TSS2_BASE_RC_INSUFFICIENT_RESPONSE: TSS2_RC = 19;
pub const TSS2_BASE_RC_INCOMPATIBLE_TCTI: TSS2_RC = 20;
pub const TSS2_BASE_RC_NOT_SUPPORTED: TSS2_RC = 21;
pub const TSS2_BASE_RC_BAD_TCTI_STRUCTURE: TSS2_RC = 22;
pub const TSS2_BASE_RC_MEMORY: TSS2_RC = 23;
pub const TSS2_BASE_RC_BAD_TR: TSS2_RC = 24;
pub const TSS2_BASE_RC_MULTIPLE_DECRYPT_SESSIONS: TSS2_RC = 25;
pub const TSS2_BASE_RC_MULTIPLE_ENCRYPT_SESSIONS: TSS2_RC = 26;
pub const TSS2_BASE_RC_RSP_AUTH_FAILED: TSS2_RC = 27;
pub const TSS2_BASE_RC_NO_CONFIG: TSS2_RC = 28;
pub const TSS2_BASE_RC_BAD_PATH: TSS2_RC = 29;
pub const TSS2_BASE_RC_NOT_DELETABLE: TSS2_RC = 30;
pub const TSS2_BASE_RC_PATH_ALREADY_EXISTS: TSS2_RC = 31;
pub const TSS2_BASE_RC_KEY_NOT_FOUND: TSS2_RC = 32;
pub const TSS2_BASE_RC_SIGNATURE_VERIFICATION_FAILED: TSS2_RC = 33;
pub const TSS2_BASE_RC_HASH_MISMATCH: TSS2_RC = 34;
pub const TSS2_BASE_RC_KEY_NOT_DUPLICABLE: TSS2_RC = 35;
pub const TSS2_BASE_RC_PATH_NOT_FOUND: TSS2_RC = 36;
pub const TSS2_BASE_RC_NO_CERT: TSS2_RC = 37;
pub const TSS2_BASE_RC_NO_PCR: TSS2_RC = 38;
pub const TSS2_BASE_RC_PCR_NOT_RESETTABLE: TSS2_RC = 39;
pub const TSS2_BASE_RC_BAD_TEMPLATE: TSS2_RC = 40;
pub const TSS2_BASE_RC_AUTHORIZATION_FAILED: TSS2_RC = 41;
pub const TSS2_BASE_RC_AUTHORIZATION_UNKNOWN: TSS2_RC = 42;
pub const TSS2_BASE_RC_NV_NOT_READABLE: TSS2_RC = 43;
pub const TSS2_BASE_RC_NV_TOO_SMALL: TSS2_RC = 44;
pub const TSS2_BASE_RC_NV_NOT_WRITEABLE: TSS2_RC = 45;
pub const TSS2_BASE_RC_POLICY_UNKNOWN: TSS2_RC = 46;
pub const TSS2_BASE_RC_NV_WRONG_TYPE: TSS2_RC = 47;
pub const TSS2_BASE_RC_NAME_ALREADY_EXISTS: TSS2_RC = 48;
pub const TSS2_BASE_RC_NO_TPM: TSS2_RC = 49;
pub const TSS2_BASE_RC_BAD_KEY: TSS2_RC = 50;
pub const TSS2_BASE_RC_NO_HANDLE: TSS2_RC = 51;
pub const TSS2_BASE_RC_NOT_PROVISIONED: TSS2_RC = 52;
pub const TSS2_BASE_RC_ALREADY_PROVISIONED: TSS2_RC = 53;
pub const TSS2_BASE_RC_CALLBACK_NULL: TSS2_RC = 54;

/* TPM 2.0 Format-Zero response codes */
pub const TPM_RC_INITIALIZE: TSS2_RC = 0x00;
pub const TPM_RC_FAILURE: TSS2_RC = 0x01;
pub const TPM_RC_SEQUENCE: TSS2_RC = 0x03;
pub const TPM_RC_PRIVATE: TSS2_RC = 0x0B;
pub const TPM_RC_HMAC: TSS2_RC = 0x19;
pub const TPM_RC_DISABLED: TSS2_RC = 0x20;
pub const TPM_RC_EXCLUSIVE: TSS2_RC = 0x21;
pub const TPM_RC_AUTH_TYPE: TSS2_RC = 0x24;
pub const TPM_RC_AUTH_MISSING: TSS2_RC = 0x25;
pub const TPM_RC_POLICY: TSS2_RC = 0x26;
pub const TPM_RC_PCR: TSS2_RC = 0x27;
pub const TPM_RC_PCR_CHANGED: TSS2_RC = 0x28;
pub const TPM_RC_UPGRADE: TSS2_RC = 0x2D;
pub const TPM_RC_TOO_MANY_CONTEXTS: TSS2_RC = 0x2E;
pub const TPM_RC_AUTH_UNAVAILABLE: TSS2_RC = 0x2F;
pub const TPM_RC_REBOOT: TSS2_RC = 0x30;
pub const TPM_RC_UNBALANCED: TSS2_RC = 0x31;
pub const TPM_RC_COMMAND_SIZE: TSS2_RC = 0x42;
pub const TPM_RC_COMMAND_CODE: TSS2_RC = 0x43;
pub const TPM_RC_AUTHSIZE: TSS2_RC = 0x44;
pub const TPM_RC_AUTH_CONTEXT: TSS2_RC = 0x45;
pub const TPM_RC_NV_RANGE: TSS2_RC = 0x46;
pub const TPM_RC_NV_SIZE: TSS2_RC = 0x47;
pub const TPM_RC_NV_LOCKED: TSS2_RC = 0x48;
pub const TPM_RC_NV_AUTHORIZATION: TSS2_RC = 0x49;
pub const TPM_RC_NV_UNINITIALIZED: TSS2_RC = 0x4A;
pub const TPM_RC_NV_SPACE: TSS2_RC = 0x4B;
pub const TPM_RC_NV_DEFINED: TSS2_RC = 0x4C;
pub const TPM_RC_BAD_CONTEXT: TSS2_RC = 0x50;
pub const TPM_RC_CPHASH: TSS2_RC = 0x51;
pub const TPM_RC_PARENT: TSS2_RC = 0x52;
pub const TPM_RC_NEEDS_TEST: TSS2_RC = 0x53;
pub const TPM_RC_NO_RESULT: TSS2_RC = 0x54;
pub const TPM_RC_SENSITIVE: TSS2_RC = 0x55;

/* TPM 2.0 Format-One response codes */
pub const TPM_RC_ASYMMETRIC: TSS2_RC = 0x01;
pub const TPM_RC_ATTRIBUTES: TSS2_RC = 0x02;
pub const TPM_RC_HASH: TSS2_RC = 0x03;
pub const TPM_RC_VALUE: TSS2_RC = 0x04;
pub const TPM_RC_HIERARCHY: TSS2_RC = 0x05;
pub const TPM_RC_KEY_SIZE: TSS2_RC = 0x07;
pub const TPM_RC_MGF: TSS2_RC = 0x08;
pub const TPM_RC_MODE: TSS2_RC = 0x09;
pub const TPM_RC_TYPE: TSS2_RC = 0x0A;
pub const TPM_RC_HANDLE: TSS2_RC = 0x0B;
pub const TPM_RC_KDF: TSS2_RC = 0x0C;
pub const TPM_RC_RANGE: TSS2_RC = 0x0D;
pub const TPM_RC_AUTH_FAIL: TSS2_RC = 0x0E;
pub const TPM_RC_NONCE: TSS2_RC = 0x0F;
pub const TPM_RC_PP: TSS2_RC = 0x10;
pub const TPM_RC_SCHEME: TSS2_RC = 0x12;
pub const TPM_RC_SIZE: TSS2_RC = 0x15;
pub const TPM_RC_SYMMETRIC: TSS2_RC = 0x16;
pub const TPM_RC_TAG: TSS2_RC = 0x17;
pub const TPM_RC_SELECTOR: TSS2_RC = 0x18;
pub const TPM_RC_INSUFFICIENT: TSS2_RC = 0x1A;
pub const TPM_RC_SIGNATURE: TSS2_RC = 0x1B;
pub const TPM_RC_KEY: TSS2_RC = 0x1C;
pub const TPM_RC_POLICY_FAIL: TSS2_RC = 0x1D;
pub const TPM_RC_INTEGRITY: TSS2_RC = 0x1F;
pub const TPM_RC_TICKET: TSS2_RC = 0x20;
pub const TPM_RC_RESERVED_BITS: TSS2_RC = 0x21;
pub const TPM_RC_BAD_AUTH: TSS2_RC = 0x22;
pub const TPM_RC_EXPIRED: TSS2_RC = 0x23;
pub const TPM_RC_POLICY_CC: TSS2_RC = 0x24;
pub const TPM_RC_BINDING: TSS2_RC = 0x25;
pub const TPM_RC_CURVE: TSS2_RC = 0x26;
pub const TPM_RC_ECC_POINT: TSS2_RC = 0x27;

/* TPM 2.0 Warning response codes */
pub const TPM_RC_CONTEXT_GAP: TSS2_RC = 0x01;
pub const TPM_RC_OBJECT_MEMORY: TSS2_RC = 0x02;
pub const TPM_RC_SESSION_MEMORY: TSS2_RC = 0x03;
pub const TPM_RC_MEMORY: TSS2_RC = 0x04;
pub const TPM_RC_SESSION_HANDLES: TSS2_RC = 0x05;
pub const TPM_RC_OBJECT_HANDLES: TSS2_RC = 0x06;
pub const TPM_RC_LOCALITY: TSS2_RC = 0x07;
pub const TPM_RC_YIELDED: TSS2_RC = 0x08;
pub const TPM_RC_CANCELED: TSS2_RC = 0x09;
pub const TPM_RC_TESTING: TSS2_RC = 0x0A;
pub const TPM_RC_REFERENCE_H0: TSS2_RC = 0x10;
pub const TPM_RC_REFERENCE_H1: TSS2_RC = 0x11;
pub const TPM_RC_REFERENCE_H2: TSS2_RC = 0x12;
pub const TPM_RC_REFERENCE_H3: TSS2_RC = 0x13;
pub const TPM_RC_REFERENCE_H4: TSS2_RC = 0x14;
pub const TPM_RC_REFERENCE_H5: TSS2_RC = 0x15;
pub const TPM_RC_REFERENCE_H6: TSS2_RC = 0x16;
pub const TPM_RC_REFERENCE_S0: TSS2_RC = 0x18;
pub const TPM_RC_REFERENCE_S1: TSS2_RC = 0x19;
pub const TPM_RC_REFERENCE_S2: TSS2_RC = 0x1A;
pub const TPM_RC_REFERENCE_S3: TSS2_RC = 0x1B;
pub const TPM_RC_REFERENCE_S4: TSS2_RC = 0x1C;
pub const TPM_RC_REFERENCE_S5: TSS2_RC = 0x1D;
pub const TPM_RC_REFERENCE_S6: TSS2_RC = 0x1E;
pub const TPM_RC_NV_RATE: TSS2_RC = 0x20;
pub const TPM_RC_LOCKOUT: TSS2_RC = 0x21;
pub const TPM_RC_RETRY: TSS2_RC = 0x22;
pub const TPM_RC_NV_UNAVAILABLE: TSS2_RC = 0x23;

/* TPM 2.0 algorithm identifiers */
pub const TPM2_ALG_ERROR: TPM2_ALG_ID = 0x00;
pub const TPM2_ALG_RSA: TPM2_ALG_ID = 0x01;
pub const TPM2_ALG_TDES: TPM2_ALG_ID = 0x03;
pub const TPM2_ALG_SHA: TPM2_ALG_ID = 0x04;
pub const TPM2_ALG_SHA1: TPM2_ALG_ID = 0x04;
pub const TPM2_ALG_HMAC: TPM2_ALG_ID = 0x05;
pub const TPM2_ALG_AES: TPM2_ALG_ID = 0x06;
pub const TPM2_ALG_MGF1: TPM2_ALG_ID = 0x07;
pub const TPM2_ALG_KEYEDHASH: TPM2_ALG_ID = 0x08;
pub const TPM2_ALG_XOR: TPM2_ALG_ID = 0x0A;
pub const TPM2_ALG_SHA256: TPM2_ALG_ID = 0x0B;
pub const TPM2_ALG_SHA384: TPM2_ALG_ID = 0x0C;
pub const TPM2_ALG_SHA512: TPM2_ALG_ID = 0x0D;
pub const TPM2_ALG_NULL: TPM2_ALG_ID = 0x10;
pub const TPM2_ALG_SM3_256: TPM2_ALG_ID = 0x12;
pub const TPM2_ALG_SM4: TPM2_ALG_ID = 0x13;
pub const TPM2_ALG_RSASSA: TPM2_ALG_ID = 0x14;
pub const TPM2_ALG_RSAES: TPM2_ALG_ID = 0x15;
pub const TPM2_ALG_RSAPSS: TPM2_ALG_ID = 0x16;
pub const TPM2_ALG_OAEP: TPM2_ALG_ID = 0x17;
pub const TPM2_ALG_ECDSA: TPM2_ALG_ID = 0x18;
pub const TPM2_ALG_ECDH: TPM2_ALG_ID = 0x19;
pub const TPM2_ALG_ECDAA: TPM2_ALG_ID = 0x1A;
pub const TPM2_ALG_SM2: TPM2_ALG_ID = 0x1B;
pub const TPM2_ALG_ECSCHNORR: TPM2_ALG_ID = 0x1C;
pub const TPM2_ALG_ECMQV: TPM2_ALG_ID = 0x1D;
pub const TPM2_ALG_KDF1_SP800_56A: TPM2_ALG_ID = 0x20;
pub const TPM2_ALG_KDF2: TPM2_ALG_ID = 0x21;
pub const TPM2_ALG_KDF1_SP800_108: TPM2_ALG_ID = 0x22;
pub const TPM2_ALG_ECC: TPM2_ALG_ID = 0x23;
pub const TPM2_ALG_SYMCIPHER: TPM2_ALG_ID = 0x25;
pub const TPM2_ALG_CAMELLIA: TPM2_ALG_ID = 0x26;
pub const TPM2_ALG_CMAC: TPM2_ALG_ID = 0x3F;
pub const TPM2_ALG_CTR: TPM2_ALG_ID = 0x40;
pub const TPM2_ALG_SHA3_256: TPM2_ALG_ID = 0x27;
pub const TPM2_ALG_SHA3_384: TPM2_ALG_ID = 0x28;
pub const TPM2_ALG_SHA3_512: TPM2_ALG_ID = 0x29;
pub const TPM2_ALG_OFB: TPM2_ALG_ID = 0x41;
pub const TPM2_ALG_CBC: TPM2_ALG_ID = 0x42;
pub const TPM2_ALG_CFB: TPM2_ALG_ID = 0x43;
pub const TPM2_ALG_ECB: TPM2_ALG_ID = 0x44;
