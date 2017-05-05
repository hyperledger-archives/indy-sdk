/**
 * @file test_ecdh.c
 * @author Kealan McCusker
 * @brief Test function for ECDH
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

/* Build executible after installation:

  gcc -std=c99 -g ./test_ecdh.c -I/opt/amcl/include -L/opt/amcl/lib -lamcl -lecdh -o test_ecdh

*/

#include "ecdh.h"
#include "utils.h"
#include <stdio.h>
#include <string.h>
#include <stdlib.h>

typedef enum { false, true } bool;

#define LINE_LEN 500
//#define DEBUG

int test_ecdh(int argc, char** argv)
{
    if (argc != 2)
    {
        printf("usage: ./test_ecdh [path to test vector file]\n");
        exit(EXIT_FAILURE);
    }
    int rc;
    FILE * fp = NULL;
    char line[LINE_LEN];
    char * linePtr = NULL;
    int l1=0, l2=0, i=0;

    char raw[256], key[EAS], ciphertext[EAS*2], res[EAS*2], plaintext[EAS*2];
    octet Key= {0,sizeof(key),key}, Ciphertext= {0,sizeof(ciphertext),ciphertext}, Plaintext= {0,sizeof(plaintext),plaintext}, Res= {0,sizeof(res),res};
    csprng rng;

    /* Fake random source */
    RAND_clean(&rng);
    for (i=0; i<256; i++) raw[i]=(char)i;
    RAND_seed(&rng,256,raw);


    char QCAVSx[EGS];
    const char* QCAVSxStr = "QCAVSx = ";
    octet QCAVSxOct = {EGS,EGS,QCAVSx};

#if CURVETYPE!=MONTGOMERY
    char QCAVSy[EGS];
    const char* QCAVSyStr = "QCAVSy = ";
    octet QCAVSyOct = {EGS,EGS,QCAVSy};
#endif

    char * dIUT = NULL;
    const char* dIUTStr = "dIUT = ";
    octet dIUTOct;

    char QIUTx[EGS];
    const char* QIUTxStr = "QIUTx = ";
    octet QIUTxOct = {EGS,EGS,QIUTx};

#if CURVETYPE!=MONTGOMERY
    char QIUTy[EGS];
    const char* QIUTyStr = "QIUTy = ";
    octet QIUTyOct = {EGS,EGS,QIUTy};
#endif

    char * ZIUT = NULL;
    const char* ZIUTStr = "ZIUT = ";
    octet ZIUTOct;

    char q[2*EFS+1];
    octet QOct= {0,sizeof(q),q};
    char z[EFS];
    octet ZOct= {0,sizeof(z),z};

    fp = fopen(argv[1], "r");
    if (fp == NULL)
    {
        printf("ERROR opening test vector file\n");
        exit(EXIT_FAILURE);
    }

    bool readLine = false;
    while (fgets(line, LINE_LEN, fp) != NULL)
    {
        i++;
        readLine = true;
        if (!strncmp(line, QCAVSxStr, strlen(QCAVSxStr)))
        {
#ifdef DEBUG
            printf("line %d %s\n", i,line);
#endif
            // Find hex value in string
            linePtr = line + strlen(QCAVSxStr);

            // Allocate data
            l1 = strlen(linePtr)-1;

            // QCAVSx binary value
            amcl_hex2bin(linePtr, QCAVSx, l1);
        }

#if CURVETYPE!=MONTGOMERY
        if (!strncmp(line, QCAVSyStr, strlen(QCAVSyStr)))
        {
#ifdef DEBUG
            printf("line %d %s\n", i,line);
#endif
            // Find hex value in string
            linePtr = line + strlen(QCAVSyStr);

            // Allocate data
            l1 = strlen(linePtr)-1;

            // QCAVSy binary value
            amcl_hex2bin(linePtr, QCAVSy, l1);
        }
#endif

        if (!strncmp(line, dIUTStr, strlen(dIUTStr)))
        {
#ifdef DEBUG
            printf("line %d %s\n", i,line);
#endif
            // Find hex value in string
            linePtr = line + strlen(dIUTStr);

            // Allocate memory
            l1 = strlen(linePtr)-1;
            l2 = l1/2;
            dIUT = (char*) malloc (l2);
            if (dIUT==NULL)
                exit(EXIT_FAILURE);

            // dIUT binary value
            amcl_hex2bin(linePtr, dIUT, l1);

            dIUTOct.len=l2;
            dIUTOct.max=l2;
            dIUTOct.val=dIUT;
        }

        if (!strncmp(line, QIUTxStr, strlen(QIUTxStr)))
        {
#ifdef DEBUG
            printf("line %d %s\n", i,line);
#endif
            // Find hex value in string
            linePtr = line + strlen(QIUTxStr);

            // Allocate data
            l1 = strlen(linePtr)-1;

            // QIUTx binary value
            amcl_hex2bin(linePtr, QIUTx, l1);
        }

#if CURVETYPE!=MONTGOMERY
        if (!strncmp(line, QIUTyStr, strlen(QIUTyStr)))
        {
#ifdef DEBUG
            printf("line %d %s\n", i,line);
#endif
            // Find hex value in string
            linePtr = line + strlen(QIUTyStr);

            // Allocate data
            l1 = strlen(linePtr)-1;

            // QIUTy binary value
            amcl_hex2bin(linePtr, QIUTy, l1);
        }
#endif

        if (!strncmp(line, ZIUTStr, strlen(ZIUTStr)))
        {
#ifdef DEBUG
            printf("line %d %s\n", i,line);
#endif
            // Find hex value in string
            linePtr = line + strlen(ZIUTStr);

            // Allocate memory
            l1 = strlen(linePtr)-1;
            l2 = l1/2;
            ZIUT = (char*) malloc (l2);
            if (ZIUT==NULL)
                exit(EXIT_FAILURE);

            // ZIUT binary value
            amcl_hex2bin(linePtr, ZIUT, l1);

            ZIUTOct.len=l2;
            ZIUTOct.max=l2;
            ZIUTOct.val=ZIUT;

            // Assign QIUT
            char q1[2*EFS+1];
            octet QIUTOct= {0,sizeof(q1),q1};
#if CURVETYPE!=MONTGOMERY
            QIUTOct.val[0]=4;
            QIUTOct.len=1;
            OCT_joctet(&QIUTOct,&QIUTxOct);
            OCT_joctet(&QIUTOct,&QIUTyOct);
#else
            QIUTOct.val[0]=2;
            QIUTOct.len=1;
            OCT_joctet(&QIUTOct,&QIUTxOct);
#endif

            // Assign QCAVS
            char q2[2*EFS+1];
            octet QCAVSOct= {0,sizeof(q2),q2};
#if CURVETYPE!=MONTGOMERY
            QCAVSOct.val[0]=4;
            QCAVSOct.len=1;
            OCT_joctet(&QCAVSOct,&QCAVSxOct);
            OCT_joctet(&QCAVSOct,&QCAVSyOct);
#else
            QCAVSOct.val[0]=2;
            QCAVSOct.len=1;
            OCT_joctet(&QCAVSOct,&QCAVSxOct);
#endif
            // Check correct public key generated
            ECP_KEY_PAIR_GENERATE(NULL,&dIUTOct,&QOct);
            rc = OCT_comp(&QOct,&QIUTOct);
            if (!rc)
            {
                printf("ERROR: TEST ECDH KEYPAIR FAILED LINE %d\n",i);
#ifdef DEBUG
                printf("\nline %d QOct:    ",i);
                OCT_output(&QOct);
                printf("\nline %d QIUTOct: ",i);
                OCT_output(&QIUTOct);
                printf("\n");
#endif
                exit(EXIT_FAILURE);
            }

            // Check correct shared value generated
            ECPSVDP_DH(&dIUTOct,&QCAVSOct,&ZOct);
            rc = OCT_comp(&ZOct,&ZIUTOct);
            if (!rc)
            {

                printf("TEST ECDH Z FAILED LINE %d\n",i);
#ifdef DEBUG
                printf("\nline %d ZOct: ",i);
                OCT_output(&ZOct);
                printf("\nline %dZIUTOct: ",i);
                OCT_output(&ZIUTOct);
                printf("\n");
#endif
                exit(EXIT_FAILURE);
            }
            free(dIUT);
            dIUT = NULL;
            free(ZIUT);
            ZIUT = NULL;
        }
    }
    fclose(fp);
    if (!readLine)
    {
        printf("ERROR Empty test vector file\n");
        exit(EXIT_FAILURE);
    }

// Self test AES-CBC
    for(i=0; i<20; i++)
    {
        OCT_rand(&Key,&rng,EAS*2);
        OCT_rand(&Plaintext,&rng,EAS);
        OCT_copy(&Res,&Plaintext);

        AES_CBC_IV0_ENCRYPT(&Key,&Plaintext,&Ciphertext);
        rc = AES_CBC_IV0_DECRYPT(&Key,&Ciphertext,&Plaintext);
        if (!rc || !OCT_comp(&Plaintext,&Res))
        {
            printf("ERROR AES_CBC decryption failed\n");
            exit(EXIT_FAILURE);
        }
    }

    printf("SUCCESS TEST ECDH KEYPAIR PASSED\n");
    exit(EXIT_SUCCESS);
}
