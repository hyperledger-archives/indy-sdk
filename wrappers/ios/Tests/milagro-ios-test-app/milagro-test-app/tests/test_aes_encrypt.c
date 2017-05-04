/**
 * @file test_aes_encrypt.c
 * @author Kealan McCusker
 * @brief Test function for AES encryption,
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

  gcc -std=c99 -g ./test_aes_encrypt.c -I/opt/amcl/include -L/opt/amcl/lib -lamcl -o test_aes_encrypt

*/

#include "arch.h"
#include "amcl.h"
#include "utils.h"
#include <stdio.h>
#include <string.h>
#include <stdlib.h>

typedef enum { false, true } bool;

#define LINE_LEN 500
//#define DEBUG

int test_aes_encrypt(int argc, char** argv)
{
    printf("test_aes_encrypt() started\n");
    if (argc != 3)
    {
        printf("usage: ./test_aes_encrypt [path to test vector file] [mode:ECB||CBC||CTR||CFB1]\n");
        exit(EXIT_FAILURE);
    }

    bool readLine;
    FILE * fp = NULL;
    char line[LINE_LEN];
    char * linePtr = NULL;
    int l1=0;
    int blockSize;

    char * KEY = NULL;
    int KEYLen = 0;
    const char* KEYStr = "KEY = ";

    char * IV = NULL;
    int IVLen = 0;
    const char* IVStr = "IV = ";

    char * PLAINTEXT = NULL;
    int PLAINTEXTLen = 0;
    const char* PLAINTEXTStr = "PLAINTEXT = ";

    char * CIPHERTEXT1 = NULL;
    const char* CIPHERTEXTStr = "CIPHERTEXT = ";
    const char* DECRYPTStr = "[DECRYPT]";

    // Assign AES mode
    int mode;
    if (!strcmp(argv[2], "ECB"))
    {
        mode = ECB;
        blockSize=16;
    }
    else if (!strcmp(argv[2], "CBC"))
    {
        mode = CBC;
        blockSize=16;
    }
    else if (!strcmp(argv[2], "CTR"))
    {
        mode = CTR16;
        blockSize=16;
    }
    else if (!strcmp(argv[2], "CFB1"))
    {
        mode = CFB1;
        blockSize=1;
    }
    else
    {
        mode = CBC;
        blockSize=16;
    }

    // Open file
    fp = fopen(argv[1], "r");
    if (fp == NULL)
    {
        printf("ERROR opening test vector file\n");
        exit(EXIT_FAILURE);
    }

    int lineNo=0;
    readLine = true;
    while ( (fgets(line, LINE_LEN, fp) != NULL))
    {
        if (!strncmp(line, DECRYPTStr,strlen(DECRYPTStr)))
        {
#ifdef DEBUG
          //  printf("line %d %s\n", lineNo,line);
#endif
            readLine = false;
        }

        if(!readLine)
            continue;

        if (!strncmp(line, KEYStr, strlen(KEYStr)))
        {
#ifdef DEBUG
            //printf("line %d %s\n", lineNo,line);
#endif
            // Find hex value in string
            linePtr = line + strlen(KEYStr);

            // Allocate memory
            l1 = strlen(linePtr)-1;
            KEYLen = l1/2;
            KEY = (char*) malloc (KEYLen);
            if (KEY==NULL)
                exit(EXIT_FAILURE);

            // KEY binary value
            amcl_hex2bin(linePtr, KEY, l1);
        }

        if (!strncmp(line, IVStr, strlen(IVStr)))
        {
#ifdef DEBUG
            //printf("line %d %s\n", lineNo,line);
#endif
            // Find hex value in string
            linePtr = line + strlen(IVStr);

            // Allocate memory
            l1 = strlen(linePtr)-1;
            IVLen = l1/2;
            IV = (char*) malloc (IVLen);
            if (IV==NULL)
                exit(EXIT_FAILURE);

            // IV binary value
            amcl_hex2bin(linePtr, IV, l1);
        }

        if (!strncmp(line, PLAINTEXTStr, strlen(PLAINTEXTStr)))
        {
#ifdef DEBUG
            //printf("line %d %s\n", lineNo,line);
#endif
            // Find hex value in string
            linePtr = line + strlen(PLAINTEXTStr);

            // Allocate memory
            l1 = strlen(linePtr)-1;
            PLAINTEXTLen = l1/2;
            PLAINTEXT = (char*) malloc(PLAINTEXTLen);
            if (PLAINTEXT==NULL)
                exit(EXIT_FAILURE);

            // PLAINTEXT binary value
            amcl_hex2bin(linePtr, PLAINTEXT, l1);
        }

        if (!strncmp(line, CIPHERTEXTStr, strlen(CIPHERTEXTStr)))
        {
#ifdef DEBUG
            //printf("line %d %s\n", lineNo,line);
#endif
            // Find hex value in string
            linePtr = line + strlen(CIPHERTEXTStr);

            // Allocate memory
            l1 = strlen(linePtr);
            CIPHERTEXT1 = (char*) malloc(PLAINTEXTLen+1);
            if (CIPHERTEXT1==NULL)
                exit(EXIT_FAILURE);

            // Golden CIPHERTEXT value
            octet CIPHERTEXT1Oct= {PLAINTEXTLen,PLAINTEXTLen,CIPHERTEXT1};
            amcl_hex2bin(linePtr, CIPHERTEXT1, l1);

            amcl_aes a;

#ifdef DEBUG
            //printf("KEY = ");
           // amcl_print_hex(KEY, KEYLen);
            //printf("IV = ");
           // amcl_print_hex(IV, IVLen);
            //printf("PLAINTEXT = ");
            //amcl_print_hex(PLAINTEXT, PLAINTEXTLen);
#endif

            // Encrypt
            int i=0;
            AES_init(&a,mode,KEYLen,KEY,IV);
            for (i=0; i<(PLAINTEXTLen/blockSize); i++)
            {
                AES_encrypt(&a,&PLAINTEXT[i*blockSize]);
            }

            octet CIPHERTEXTOct= {PLAINTEXTLen,PLAINTEXTLen,PLAINTEXT};

#ifdef DEBUG
            //printf("CIPHERTEXT = ");
            //amcl_print_hex(PLAINTEXT, PLAINTEXTLen);
            //printf("\n\n");
#endif

            int rc = OCT_comp(&CIPHERTEXTOct,&CIPHERTEXT1Oct);
            if (!rc)
            {
                printf("TEST AES ENCRYPT FAILED COMPARE CIPHERTEXT LINE %d\n",lineNo);
                exit(EXIT_FAILURE);
            }

            free(KEY);
            KEY = NULL;
            free(IV);
            IV = NULL;
            free(PLAINTEXT);
            PLAINTEXT = NULL;
            free(CIPHERTEXT1);
            CIPHERTEXT1 = NULL;
        }
        lineNo++;
    }
    fclose(fp);
    if (readLine)
    {
        printf("ERROR No test vectors\n");
        exit(EXIT_FAILURE);
    }
    printf("test_aes_encrypt() SUCCESS TEST AES %s ENCRYPT PASSED\n", argv[2]);
    return EXIT_SUCCESS;
}
