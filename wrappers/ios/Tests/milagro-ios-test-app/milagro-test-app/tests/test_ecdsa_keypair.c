/**
 * @file test_ecdsa_keypair.c
 * @author Kealan McCusker
 * @brief Test function for ECDSA keypair,
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

  gcc -std=c99 -g ./test_ecdsa_keypair.c -I/opt/amcl/include -L/opt/amcl/lib -lamcl -lecdh -o test_ecdsa_keypair

*/

#include "ecdh.h"
#include "utils.h"
#include <stdio.h>
#include <string.h>
#include <stdlib.h>

typedef enum { false, true } bool;

#define LINE_LEN 300
// #define DEBUG

int test_ecdsa_keypair(int argc, char** argv)
{
    if (argc != 2)
    {
        printf("usage: ./test_ecdsa_sign [path to test vector file]\n");
        exit(EXIT_FAILURE);
    }
    int rc;
    FILE * fp = NULL;
    char line[LINE_LEN];
    char * linePtr = NULL;
    int l1=0;
    int l2=0;
    char * d = NULL;
    const char* dStr = "d = ";
    octet dOct;
    char Qx[EGS];
    const char* QxStr = "Qx = ";
    octet QxOct = {EGS,EGS,Qx};
    char Qy[EGS];
    const char* QyStr = "Qy = ";
    octet QyOct = {EGS,EGS,Qy};

    char q2[2*EFS+1];
    octet Q2Oct= {0,sizeof(q2),q2};

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

            // Assign Public Key
            BIG qx, qy;
            char q[2*EFS+1];
            BIG_fromBytes(qx,QxOct.val);
            BIG_fromBytes(qy,QyOct.val);
            octet QOct= {sizeof(q),sizeof(q),q};
            QOct.val[0]=4;
            BIG_toBytes(&(QOct.val[1]),qx);
            BIG_toBytes(&(QOct.val[EFS+1]),qy);

            // Generate Key pair
            ECP_KEY_PAIR_GENERATE(NULL,&dOct,&Q2Oct);

#ifdef DEBUG
            printf("QOct: ");
            OCT_output(&QOct);
            printf("\r\n");
            printf("Q2Oct: ");
            OCT_output(&Q2Oct);
            printf("\r\n");
#endif
            rc = OCT_comp(&QOct,&Q2Oct);
            if (!rc)
            {
                printf("TEST ECDSA KEYPAIR FAILED LINE %d\n",i);
                exit(EXIT_FAILURE);
            }
            free(d);
            d = NULL;
        }
    }
    fclose(fp);
    if (!readLine)
    {
        printf("ERROR Empty test vector file\n");
        exit(EXIT_FAILURE);
    }
    printf("SUCCESS TEST ECDSA KEYPAIR PASSED\n");
    exit(EXIT_SUCCESS);
}
