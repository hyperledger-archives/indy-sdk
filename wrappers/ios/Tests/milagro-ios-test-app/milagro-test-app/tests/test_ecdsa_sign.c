/**
 * @file test_ecdsa_sign.c
 * @author Kealan McCusker
 * @brief Test function for ECDSA signature,
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

  gcc -std=c99 -g ./test_ecdsa_sign.c -I/opt/amcl/include -L/opt/amcl/lib -lamcl -lecdh -o test_ecdsa_sign

*/

#include "ecdh.h"
#include "utils.h"
#include <stdio.h>
#include <string.h>
#include <stdlib.h>

typedef enum { false, true } bool;

#define LINE_LEN 300
//#define DEBUG

int test_ecdsa_sign(int argc, char** argv)
{
    if (argc != 3)
    {
        printf("usage: ./test_ecdsa_sign [path to test vector file] [hash type-sha256||sha384||sha512] \n");
        exit(EXIT_FAILURE);
    }
    int rc;
    FILE * fp = NULL;
    char line[LINE_LEN];
    char * linePtr = NULL;
    int l1=0;
    int l2=0;
    char * Msg = NULL;
    const char* MsgStr = "Msg = ";
    octet MsgOct;
    char * d = NULL;
    const char* dStr = "d = ";
    octet dOct;
    char Qx[EGS];
    const char* QxStr = "Qx = ";
    octet QxOct = {EGS,EGS,Qx};
    char Qy[EGS];
    const char* QyStr = "Qy = ";
    octet QyOct = {EGS,EGS,Qy};
    char * k = NULL;
    const char* kStr = "k = ";
    octet kOct;
    char * R = NULL;
    const char* RStr = "R = ";
    octet ROct;
    char * S = NULL;
    const char* SStr = "S = ";
    octet SOct;

    char r2[EGS],s2[EGS];
    octet R2Oct= {0,sizeof(r2),r2};
    octet S2Oct= {0,sizeof(s2),s2};

    // Assign hash type
    int hash_type;
    if (!strcmp(argv[2], "sha256"))
    {
        hash_type = 32;
    }
    else if (!strcmp(argv[2], "sha384"))
    {
        hash_type = 48;
    }
    else if (!strcmp(argv[2], "sha512"))
    {
        hash_type = 64;
    }
    else
    {
        hash_type = 32;
    }

    fp = fopen(argv[1], "r");
    if (fp == NULL)
    {
        printf("ERROR opening test vector file\n");
        exit(EXIT_FAILURE);
    }

    bool readLine = false;
    int i=0;
    while (fgets(line, LINE_LEN, fp) != NULL)
    {
        i++;
        readLine = true;
        if (!strncmp(line, MsgStr, strlen(MsgStr)))
        {
#ifdef DEBUG
            printf("line %d %s\n", i,line);
#endif
            // Find hex value in string
            linePtr = line + strlen(MsgStr);

            // Allocate memory
            l1 = strlen(linePtr)-1;
            l2 = l1/2;
            Msg = (char*) malloc (l2);
            if (Msg==NULL)
                exit(EXIT_FAILURE);

            // Msg binary value
            amcl_hex2bin(linePtr, Msg, l1);

            MsgOct.len=l2;
            MsgOct.max=l2;
            MsgOct.val=Msg;
        }

        if (!strncmp(line, dStr, strlen(dStr)))
        {
#ifdef DEBUG
            printf("line %d %s\n", i,line);
#endif
            // Find hex value in string
            linePtr = line + strlen(dStr);

            // Allocate memory
            l1 = strlen(linePtr)-1;
            l2 = l1/2;
            d = (char*) malloc (l2);
            if (d==NULL)
                exit(EXIT_FAILURE);

            // d binary value
            amcl_hex2bin(linePtr, d, l1);

            dOct.len=l2;
            dOct.max=l2;
            dOct.val=d;
        }

        if (!strncmp(line, QxStr, strlen(QxStr)))
        {
#ifdef DEBUG
            printf("line %d %s\n", i,line);
#endif
            // Find hex value in string
            linePtr = line + strlen(QxStr);

            // Allocate data
            l1 = strlen(linePtr)-1;

            // Qx binary value
            amcl_hex2bin(linePtr, Qx, l1);
        }

        if (!strncmp(line, QyStr, strlen(QyStr)))
        {
#ifdef DEBUG
            printf("line %d %s\n", i,line);
#endif
            // Find hex value in string
            linePtr = line + strlen(QyStr);

            // Allocate data
            l1 = strlen(linePtr)-1;

            // Qy binary value
            amcl_hex2bin(linePtr, Qy, l1);
        }

        if (!strncmp(line, kStr, strlen(kStr)))
        {
#ifdef DEBUG
            printf("line %d %s\n", i,line);
#endif
            // Find hex value in string
            linePtr = line + strlen(kStr);

            // Allocate memory
            l1 = strlen(linePtr)-1;
            l2 = l1/2;
            k = (char*) malloc (l2);
            if (k==NULL)
                exit(EXIT_FAILURE);

            // k binary value
            amcl_hex2bin(linePtr, k, l1);

            kOct.len=l2;
            kOct.max=l2;
            kOct.val=k;
        }

        if (!strncmp(line, RStr, strlen(RStr)))
        {
#ifdef DEBUG
            printf("line %d %s\n", i,line);
#endif
            // Find hex value in string
            linePtr = line + strlen(RStr);

            // Allocate memory
            l1 = strlen(linePtr)-1;
            l2 = l1/2;
            R = (char*) malloc (l2);
            if (R==NULL)
                exit(EXIT_FAILURE);

            // R binary value
            amcl_hex2bin(linePtr, R, l1);

            ROct.len=l2;
            ROct.max=l2;
            ROct.val=R;
        }

        if (!strncmp(line, SStr, strlen(SStr)))
        {
#ifdef DEBUG
            printf("line %d %s\n", i,line);
#endif
            // Find hex value in string
            linePtr = line + strlen(SStr);

            // Allocate memory
            l1 = strlen(linePtr)-1;
            l2 = l1/2;
            S = (char*) malloc (l2);
            if (S==NULL)
                exit(EXIT_FAILURE);

            // S binary value
            amcl_hex2bin(linePtr, S, l1);

            SOct.len=l2;
            SOct.max=l2;
            SOct.val=S;

            // Assign Public Key
            BIG qx,qy;
            char q[2*EFS+1];
            BIG_fromBytes(qx,QxOct.val);
            BIG_fromBytes(qy,QyOct.val);
            octet QOct= {sizeof(q),sizeof(q),q};
            QOct.val[0]=4;
            BIG_toBytes(&(QOct.val[1]),qx);
            BIG_toBytes(&(QOct.val[EFS+1]),qy);

#ifdef DEBUG
            printf("hash_type %d\n",hash_type);
            printf("kOct: ");
            OCT_output(&kOct);
            printf("dOct: ");
            OCT_output(&dOct);
            printf("MsgOct: ");
            OCT_output(&MsgOct);
            printf("\n");
#endif

            ECPSP_DSA(hash_type,NULL,&kOct,&dOct,&MsgOct,&R2Oct,&S2Oct);

            rc = OCT_comp(&ROct,&R2Oct);
            if (!rc)
            {
                printf("TEST ECDSA SIGN FAILED COMPARE R LINE %d\n",i);
#ifdef DEBUG
                printf("ROct: ");
                OCT_output(&ROct);
                printf("\n");
                printf("R2Oct: ");
                OCT_output(&R2Oct);
                printf("\n");
#endif
                exit(EXIT_FAILURE);
            }

            rc = OCT_comp(&SOct,&S2Oct);
            if (!rc)
            {
                printf("TEST ECDSA SIGN FAILED COMPARE S LINE %d\n",i);
#ifdef DEBUG
                printf("SOct: ");
                OCT_output(&SOct);
                printf("\n");
                printf("S2Oct: ");
                OCT_output(&S2Oct);
                printf("\n");
#endif
                exit(EXIT_FAILURE);
            }

            free(Msg);
            Msg = NULL;
            free(d);
            d = NULL;
            free(k);
            k = NULL;
            free(R);
            R = NULL;
            free(S);
            S = NULL;
        }
    }
    fclose(fp);
    if (!readLine)
    {
        printf("ERROR Empty test vector file\n");
        exit(EXIT_FAILURE);
    }
    printf("SUCCESS TEST ECDSA %s SIGN PASSED\n", argv[2]);
    exit(EXIT_SUCCESS);
}
