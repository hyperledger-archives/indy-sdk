/**
 * @file test_rsa.c
 * @author Kealan McCusker
 * @brief Test RSA
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

/* test driver and function exerciser for RSA API Functions */


#include <stdio.h>
#include <time.h>
#include "rsa.h"
#include "randapi.h"

int test_rsa()
{
    int i;
    unsigned long ran;
    char m[RFS],ml[RFS],c[RFS],e[RFS],s[RFS],raw[100];
    rsa_public_key pub;
    rsa_private_key priv;
    csprng RNG;
    octet M= {0,sizeof(m),m};
    octet ML= {0,sizeof(ml),ml};
    octet C= {0,sizeof(c),c};
    octet E= {0,sizeof(e),e};
    octet S= {0,sizeof(s),s};
    octet RAW= {0,sizeof(raw),raw};

    time((time_t *)&ran);

    RAW.len=100;				/* fake random seed source */
    RAW.val[0]=ran;
    RAW.val[1]=ran>>8;
    RAW.val[2]=ran>>16;
    RAW.val[3]=ran>>24;
    for (i=0; i<100; i++) RAW.val[i]=i;

    CREATE_CSPRNG(&RNG,&RAW);   /* initialise strong RNG */

    printf("Generating public/private key pair\n");
    RSA_KEY_PAIR(&RNG,65537,&priv,&pub,NULL,NULL);

    printf("Encrypting test string\n");
    OCT_jstring(&M,(char *)"Hello World\n");

    OAEP_ENCODE(HASH_TYPE_RSA,&M,&RNG,NULL,&E); /* OAEP encode message m to e  */

    RSA_ENCRYPT(&pub,&E,&C);     /* encrypt encoded message */
    printf("Ciphertext= ");
    OCT_output(&C);

    printf("Decrypting test string\n");
    RSA_DECRYPT(&priv,&C,&ML);   /* ... and then decrypt it */

    OAEP_DECODE(HASH_TYPE_RSA,NULL,&ML);    /* decode it */
    OCT_output_string(&ML);


    if (!OCT_comp(&M,&ML))
    {
        printf("FAILURE RSA Encryption failed");
        return 1;
    }

    printf("Signing message\n");
    PKCS15(HASH_TYPE_RSA,&M,&C);

    RSA_DECRYPT(&priv,&C,&S); /* create signature in S */

    printf("Signature= ");
    OCT_output(&S);

    RSA_ENCRYPT(&pub,&S,&ML);

    if (OCT_comp(&C,&ML))
    {
        printf("Signature is valid\n");
    }
    else
    {
        printf("FAILURE RSA Signature Verification failed");
        return 1;
    }

    KILL_CSPRNG(&RNG);
    RSA_PRIVATE_KEY_KILL(&priv);

    OCT_clear(&M);
    OCT_clear(&ML);   /* clean up afterwards */
    OCT_clear(&C);
    OCT_clear(&RAW);
    OCT_clear(&E);

    printf("SUCCESS\n");
    return 0;
}
