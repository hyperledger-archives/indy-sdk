/**
 * @file test_gcm_encrypt.c
 * @author Kealan McCusker
 * @brief Test function for Galois Counter Mode encryption,
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

  gcc -std=c99 -g ./test_gcm_encrypt.c -I/opt/amcl/include -L/opt/amcl/lib -lamcl -o test_gcm_encrypt

*/

#include "arch.h"
#include "amcl.h"
#include "utils.h"
#include <stdio.h>
#include <string.h>
#include <stdlib.h>

typedef enum { false, true } bool;

#define LINE_LEN 300
//#define DEBUG

int test_gcm_encrypt(int argc, char** argv)
{
    printf("test_gcm_encrypt() started\n");
    if (argc != 2)
    {
        printf("usage: ./test_gcm_encrypt [path to test vector file]\n");
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
    int PTLen = 0;
    const char* PTStr = "PT = ";

    char * AAD = NULL;
    int AADLen = 0;
    const char* AADStr = "AAD = ";

    char * CT = NULL;
    int CTLen = 0;
    const char* CTStr = "CT = ";

    char * CT1 = NULL;
    octet CT1Oct;

    char Tag[16];
    const char* TagStr = "Tag = ";

    char * Tag1 = NULL;
    octet Tag1Oct;
    int TagLen;

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
            //printf("line %d %s\n", i,line);
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
            //printf("line %d %s\n", i,line);
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

        if (!strncmp(line, PTStr, strlen(PTStr)))
        {
#ifdef DEBUG
            //printf("line %d %s\n", i,line);
#endif
            // Find hex value in string
            linePtr = line + strlen(PTStr);

            // Allocate memory
            l1 = strlen(linePtr)-1;
            PTLen = l1/2;
            PT = (char*) malloc (PTLen);
            if (PT==NULL)
                exit(EXIT_FAILURE);

            // PT binary value
            amcl_hex2bin(linePtr, PT, l1);
        }

        if (!strncmp(line, AADStr, strlen(AADStr)))
        {
#ifdef DEBUG
            //printf("line %d %s\n", i,line);
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

        if (!strncmp(line, CTStr, strlen(CTStr)))
        {
#ifdef DEBUG
            //printf("line %d %s\n", i,line);
#endif
            // Find hex value in string
            linePtr = line + strlen(CTStr);

            // Allocate memory
            l1 = strlen(linePtr);
            CTLen = l1/2;
            CT1 = (char*) malloc (CTLen);
            if (CT1==NULL)
                exit(EXIT_FAILURE);
            CT = (char*) malloc (CTLen);
            if (CT==NULL)
                exit(EXIT_FAILURE);

            // Golden CT value
            amcl_hex2bin(linePtr, CT1, l1);

            CT1Oct.len=CTLen;
            CT1Oct.max=CTLen;
            CT1Oct.val=CT1;
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


            gcm g;

            GCM_init(&g,KeyLen,Key,IVLen,IV);
            GCM_add_header(&g,AAD,AADLen);
            GCM_add_plain(&g,CT,PT,PTLen);
            GCM_finish(&g,Tag);


            octet CTOct = {CTLen,CTLen, CT};
            int rc = OCT_comp(&CT1Oct,&CTOct);
            if (!rc)
            {
                printf("TEST GCM ENCRYPT FAILED COMPARE CT LINE %d\n",i);
                exit(EXIT_FAILURE);
            }

            octet TagOct = {TagLen,TagLen, Tag};
            rc = OCT_comp(&Tag1Oct,&TagOct);
            if (!rc)
            {
                printf("TEST GCM ENCRYPT FAILED COMPARE Tag LINE %d\n",i);
                exit(EXIT_FAILURE);
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
            free(CT1);
            CT1 = NULL;
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
    printf("test_gcm_encrypt() SUCCESS TEST AES-GCM ENCRYPT PASSED\n");
    return EXIT_SUCCESS;
}
