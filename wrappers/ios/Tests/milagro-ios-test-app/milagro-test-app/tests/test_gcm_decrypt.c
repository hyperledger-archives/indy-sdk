/**
 * @file test_gcm_decrypt.c
 * @author Kealan McCusker
 * @brief Test function for Galois Counter Mode decryption,
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

  gcc -std=c99 -g ./test_gcm_decrypt.c -I/opt/amcl/include -L/opt/amcl/lib -lamcl -o test_gcm_decrypt

*/

#include "arch.h"
#include "amcl.h"
#include "utils.h"
#include <stdio.h>
#include <string.h>
#include <stdlib.h>

typedef enum { false, true } bool;

#define LINE_LEN 300
// #define DEBUG

int test_gcm_decrypt(int argc, char** argv)
{
    printf("test_gcm_decrypt() started\n");
    if (argc != 2)
    {
        printf("usage: ./test_gcm_decrypt [path to test vector file]\n");
        exit(EXIT_FAILURE);
    }

    FILE * fp = NULL;
    char line[LINE_LEN];
    char * linePtr = NULL;
    int l1=0;

    char * Key = NULL;
    int KeyLen = 0;
    const char* KeyStr = "Key = ";

    char * IV = NULL;
    int IVLen = 0;
    const char* IVStr = "IV = ";

    char * PT = NULL;
    const char* PTStr = "PT = ";

    char * PT1 = NULL;
    octet PT1Oct;

    char * AAD = NULL;
    int AADLen = 0;
    const char* AADStr = "AAD = ";

    char * CT = NULL;
    int CTLen = 0;
    const char* CTStr = "CT = ";

    char Tag[16];
    const char* TagStr = "Tag = ";

    char * Tag1 = NULL;
    octet Tag1Oct;
    int TagLen=0;

    const char* FAILStr = "FAIL";

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
        if (!strncmp(line, KeyStr, strlen(KeyStr)))
        {
#ifdef DEBUG
         //   printf("line %d %s\n", i,line);
#endif
            // Find hex value in string
            linePtr = line + strlen(KeyStr);

            // Allocate memory
            l1 = strlen(linePtr)-1;
            KeyLen = l1/2;
            Key = (char*) malloc (KeyLen);
            if (Key==NULL)
                exit(EXIT_FAILURE);

            // Key binary value
            amcl_hex2bin(linePtr, Key, l1);
        }

        if (!strncmp(line, IVStr, strlen(IVStr)))
        {
#ifdef DEBUG
          //  printf("line %d %s\n", i,line);
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

        if (!strncmp(line, CTStr, strlen(CTStr)))
        {
#ifdef DEBUG
           // printf("line %d %s\n", i,line);
#endif
            // Find hex value in string
            linePtr = line + strlen(CTStr);

            // Allocate memory
            l1 = strlen(linePtr);
            CTLen = l1/2;
            CT = (char*) malloc (CTLen);
            if (CT==NULL)
                exit(EXIT_FAILURE);

            PT = (char*) malloc (CTLen);
            if (PT==NULL)
                exit(EXIT_FAILURE);

            // CT binary value
            amcl_hex2bin(linePtr, CT, l1);
        }

        if (!strncmp(line, AADStr, strlen(AADStr)))
        {
#ifdef DEBUG
           // printf("line %d %s\n", i,line);
#endif
            // Find hex value in string
            linePtr = line + strlen(AADStr);

            // Allocate memory
            l1 = strlen(linePtr)-1;
            AADLen = l1/2;
            AAD = (char*) malloc (AADLen);
            if (AAD==NULL)
                exit(EXIT_FAILURE);

            // AAD binary value
            amcl_hex2bin(linePtr, AAD, l1);
        }

        if (!strncmp(line, TagStr, strlen(TagStr)))
        {
#ifdef DEBUG
            //printf("line %d %s\n", i,line);
#endif
            // Find hex value in string
            linePtr = line + strlen(TagStr);

            // Allocate memory
            l1 = strlen(linePtr);
            TagLen = l1/2;
            Tag1 = (char*) malloc (TagLen);
            if (Tag1==NULL)
                exit(EXIT_FAILURE);

            // Golden Tag value
            amcl_hex2bin(linePtr, Tag1, l1);

            Tag1Oct.len=TagLen;
            Tag1Oct.max=TagLen;
            Tag1Oct.val=Tag1;
        }

        if ( !strncmp(line, PTStr, strlen(PTStr)) || !strncmp(line, FAILStr, strlen(FAILStr)) )
        {
#ifdef DEBUG
         //   printf("line %d %s\n", i,line);
#endif

            if (!strncmp(line, PTStr, strlen(PTStr)))
            {
                // Find hex value in string
                linePtr = line + strlen(PTStr);

                // Allocate memory
                l1 = strlen(linePtr)-1;
                PT1 = (char*) malloc (CTLen);
                if (PT1==NULL)
                    exit(EXIT_FAILURE);

                // Golden PT value
                amcl_hex2bin(linePtr, PT1, l1);

                PT1Oct.len=CTLen;
                PT1Oct.max=CTLen;
                PT1Oct.val=PT1;
            }

            gcm g;

            GCM_init(&g,KeyLen,Key,IVLen,IV);
            GCM_add_header(&g,AAD,AADLen);
            GCM_add_cipher(&g,PT,CT,CTLen);
            GCM_finish(&g,Tag);

            octet PTOct = {CTLen,CTLen, PT};
            octet TagOct = {TagLen,TagLen, Tag};

            int rc;
            if (!strncmp(line, PTStr, strlen(PTStr)))
            {
                rc =  OCT_comp(&PT1Oct,&PTOct);
                if (!rc)
                {
                    printf("TEST GCM DECRYPT FAILED COMPARE PT LINE %d\n",i);
                    exit(EXIT_FAILURE);
                }
            }

            if (!strncmp(line, PTStr, strlen(PTStr)))
            {
                rc = OCT_comp(&Tag1Oct,&TagOct);
                if (!rc)
                {
                    printf("TEST GCM DECRYPT FAILED COMPARE Tag LINE %d\n",i);
                    exit(EXIT_FAILURE);
                }
            }

            // Expect tag to be different from input tag
            if (!strncmp(line, FAILStr, strlen(FAILStr)))
            {
                rc = OCT_comp(&Tag1Oct,&TagOct);
                if (rc)
                {
                    printf("TEST GCM DECRYPT FAILED COMPARE Tag LINE %d\n",i);
                    exit(EXIT_FAILURE);
                }
            }

            free(Key);
            Key = NULL;
            free(IV);
            IV = NULL;
            free(PT);
            PT = NULL;
            free(AAD);
            AAD = NULL;
            free(CT);
            CT = NULL;
            free(PT1);
            PT1 = NULL;
            free(Tag1);
            Tag1 = NULL;
        }
    }
    fclose(fp);
    if (!readLine)
    {
        printf("ERROR Empty test vector file\n");
        exit(EXIT_FAILURE);
    }
    printf("test_gcm_decrypt() SUCCESS TEST AES-GCM DECRYPT PASSED\n");
    return EXIT_SUCCESS;
}
