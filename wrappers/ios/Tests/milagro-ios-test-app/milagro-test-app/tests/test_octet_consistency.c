/**
 * @file test_octet_consistency.c
 * @author Alessandro Budroni
 * @brief Test function for octect consistency
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

/* test driver and function exerciser for ECDH/ECIES/ECDSA API Functions */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
#include <time.h>
#include "amcl.h"

int test_octet_consistency()
{
    int i,j,len=100;
    int len64 = ((len/3) + 2)*4+1, lenHex = 28*len;
    char raw[256], bytes[len+1], bytes64[len64+1], bytesHex[lenHex+1], v[len], w[len];
    octet V= {0,sizeof(v),v}, W= {0,sizeof(w),w};
    csprng rng;

    /* Fake random source */
    RAND_clean(&rng);
    for (i=0; i<256; i++) raw[i]=(char)i;
    RAND_seed(&rng,256,raw);

    /* test comparison */
    for (len = 1; len <= 101; len=len+10)
    {
        OCT_rand(&W,&rng,len);
        OCT_copy(&V,&W);
        if(!OCT_comp(&V,&W))
        {
            printf("ERROR comparing two equal octet, OCTET\n");
            exit(EXIT_FAILURE);
        }
        for (i = 0; i < len; ++i)
        {
            if(!OCT_ncomp(&V,&W,i))
            {
                printf("ERROR comparing two equal octet, OCTET\n");
                exit(EXIT_FAILURE);
            }
        }
        OCT_rand(&V,&rng,len);
        if(OCT_comp(&V,&W))
        {
            printf("ERROR comparing two different octet, OCTET\n");
            exit(EXIT_FAILURE);
        }
    }
    OCT_rand(&W,&rng,0);
    OCT_copy(&V,&W);
    if(!OCT_comp(&V,&W))
    {
        printf("ERROR comparing two equal octet, OCTET\n");
        exit(EXIT_FAILURE);
    }

    for (len = 100; len > 0; len=len-10)
    {

        W.max = len;
        V.max = len;
        /* test conversion to and from base64 */
        for (j = 0; j < 10; ++j)
        {
            OCT_rand(&W,&rng,len);
            OCT_copy(&V,&W);
            OCT_tobase64(bytes64,&W);
            OCT_frombase64(&W,bytes64);
            if(!OCT_comp(&V,&W))
            {
                printf("ERROR converting to and from base64 OCTET\n");
                exit(EXIT_FAILURE);
            }
        }

        /* test conversion to and from hex */
        for (i = 0; i < 10; ++i)
        {
            OCT_rand(&W,&rng,len);
            OCT_copy(&V,&W);
            OCT_toHex(&W,bytesHex);
            OCT_fromHex(&W,bytesHex);
            if(!OCT_comp(&V,&W))
            {
                printf("ERROR converting to and from Hex OCTET\n");
                exit(EXIT_FAILURE);
            }
        }

        /* test conversion to and from string */
        for (i = 0; i < 10; ++i)
        {
            OCT_rand(&W,&rng,len);
            OCT_copy(&V,&W);
            OCT_toStr(&W,bytes);
            OCT_jstring(&W,bytes);
            if(!OCT_comp(&V,&W))
            {
                printf("ERROR converting to and from string, OCTET\n");
                exit(EXIT_FAILURE);
            }
        }
    }



    printf("SUCCESS\n");
    return 0;
}

