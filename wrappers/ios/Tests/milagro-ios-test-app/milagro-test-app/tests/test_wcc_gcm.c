/**
 * @file test_wcc_gcm.c
 * @author Kealan McCusker
 * @brief Test WCC protocol with GCM
 *
 * LICENSE
 *
 * Licensed to the Apache Software Foundation (ASF) under one
 * or more contributor license agreements.  See the NOTICE file
 * distributed with this work for additional information
 * regarding copyright ownership.  The ASF licenses this file
 * to you under the Apache License, Version 2.0 (the
 * "License"); you may not use this file except in compliance
 * with the License.  You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing,
 * software distributed under the License is distributed on an
 * "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
 * KIND, either express or implied.  See the License for the
 * specific language governing permissions and limitations
 * under the License.
 */

#include <stdlib.h>
#include <string.h>
#include "amcl.h"
#include "wcc.h"
#include "utils.h"


int test_wcc_gcm()
{
    printf("test_wcc_gcm() started\n");
    char* KT="feffe9928665731c6d6a8f9467308308";
    char* MT="d9313225f88406e5a55909c5aff5269a86a7a9531534f7da2e4c303d8a318a721c3c0c95956809532fcf0e2449a6b525b16aedf5aa0de657ba637b39";
    char* HT="feedfacedeadbeeffeedfacedeadbeefabaddad2";
    char* NT="9313225df88406e555909c5aff5269aa6a7a9538534f7da1e4c303d2a318a728c3c0c95156809539fcf0e2429a6b525416aedbf5a0de6a57a637b39b";
    // Tag should be 619cc5aefffe0bfa462af43c1699d050

    int lenM=strlen(MT)/2;
    int lenH=strlen(HT)/2;
    int lenK=strlen(KT)/2;
    int lenIV=strlen(NT)/2;

    char t1[PTAG];  // Tag
    char t2[PTAG];  // Tag
    char k[PAS];   // AES Key
    char h[64];   // Header - to be included in Authentication, but not encrypted
    char iv[100]; // IV - Initialisation vector
    char m[100];  // Plaintext to be encrypted/authenticated
    char c[100];  // Ciphertext
    char p[100];  // Recovered Plaintext
    octet T1= {sizeof(t1),sizeof(t1),t1};
    octet T2= {sizeof(t2),sizeof(t2),t2};
    octet K= {0,sizeof(k),k};
    octet H= {0,sizeof(h),h};
    octet IV= {0,sizeof(iv),iv};
    octet M= {0,sizeof(m),m};
    octet C= {0,sizeof(c),c};
    octet P= {0,sizeof(p),p};
    M.len=lenM;
    K.len=lenK;
    H.len=lenH;
    IV.len=lenIV;

    OCT_fromHex(&M, MT);
    OCT_fromHex(&H, HT);
    OCT_fromHex(&IV, NT);
    OCT_fromHex(&K, KT);

//    printf("Plaintext: ");
//    OCT_output(&M);
//    printf("\n");

    WCC_AES_GCM_ENCRYPT(&K, &IV, &H, &M, &C, &T1);

//    printf("Ciphertext: ");
//    OCT_output(&C);
//    printf("\n");

//    printf("Encryption Tag: ");
//    OCT_output(&T1);
//    printf("\n");

    WCC_AES_GCM_DECRYPT(&K, &IV, &H, &C, &P, &T2);

//    printf("Plaintext: ");
//    OCT_output(&P);
//    printf("\n");

//    printf("Decryption Tag: ");
//    OCT_output(&T2);
//    printf("\n");

    if (!OCT_comp(&M,&P))
    {
        printf("test_wcc_gcm() FAILURE Decryption\n");
        return 1;
    }

    if (!OCT_comp(&T1,&T2))
    {
        printf("test_wcc_gcm() FAILURE TAG mismatch\n");
        return 1;
    }

    printf("test_wcc_gcm() SUCCESS\n");
    return 0;
}

