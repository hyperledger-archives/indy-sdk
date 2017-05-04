/**
 * @file test_hash.c
 * @author Kealan McCusker
 * @brief Test for hash functions
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

  gcc -std=c99 -g ./test_hash.c -I/opt/amcl/include -L/opt/amcl/lib -lamcl -o test_hash

*/

#include "arch.h"
#include "amcl.h"
#include "utils.h"
#include <stdio.h>
#include <string.h>
#include <stdlib.h>

typedef enum { false, true } bool;

#define LINE_LEN 600
//#define DEBUG

int test_hash(int argc, char** argv)
{
    printf("test_hash() started\n");
    if (argc != 3)
    {
        printf("usage: ./test_hash [path to test vector file] [sha256||sha384||sha512]\n");
        exit(EXIT_FAILURE);
    }

    int i=0;
    FILE * fp = NULL;
    char line[LINE_LEN];
    char * linePtr = NULL;
    int l1=0;
    char * Msg = NULL;
    int MsgLen = 0;
    const char* MsgStr = "Msg = ";

    const char* MDStr = "MD = ";
    int MDLen = 0;
    char * MD = NULL;

    char * MD1 = NULL;

    hash256 sha256;
    hash384 sha384;
    hash512 sha512;

    // Hash initialization
    if (!strcmp(argv[2], "sha512"))
    {
        HASH512_init(&sha512);
    }
    else if (!strcmp(argv[2], "sha384"))
    {
        HASH384_init(&sha384);
    }
    else
    {
        HASH256_init(&sha256);
    }

    // Open file
    fp = fopen(argv[1], "r");
    if (fp == NULL)
    {
        printf("ERROR opening test vector file\n");
        exit(EXIT_FAILURE);
    }
    bool readLine = false;

    int lineNo=0;
    while ( (fgets(line, LINE_LEN, fp) != NULL))
    {
        readLine = true;
        if (!strncmp(line, MsgStr, strlen(MsgStr)))
        {
#ifdef DEBUG
          //  printf("line %d %s\n", lineNo,line);
#endif
            // Find hex value in string
            linePtr = line + strlen(MsgStr);

            // Allocate memory
            l1 = strlen(linePtr)-1;
            MsgLen = l1/2;
            Msg = (char*) malloc(MsgLen);
            if (Msg==NULL)
                exit(EXIT_FAILURE);

            // Msg binary value
            amcl_hex2bin(linePtr, Msg, l1);
        }

        if (!strncmp(line, MDStr, strlen(MDStr)))
        {
#ifdef DEBUG
            //printf("line %d %s\n", lineNo,line);
#endif
            // Find hex value in string
            linePtr = line + strlen(MDStr);

            // Allocate memory
            l1 = strlen(linePtr);

            // Allocate memory for digest
            MDLen = l1/2;
            MD = (char*) malloc(MDLen);
            if (MD==NULL)
                exit(EXIT_FAILURE);
            MD1 = (char*) malloc(MDLen);
            if (MD1==NULL)
                exit(EXIT_FAILURE);

            octet MD1Oct= {MDLen,MDLen,MD1};

            // Golden MD value
            amcl_hex2bin(linePtr, MD1, l1);

            if (!strcmp(argv[2], "sha512"))
            {
                for (i=0; i<MsgLen; i++)
                    HASH512_process(&sha512,Msg[i]);
                HASH512_hash(&sha512,MD);
            }
            else if (!strcmp(argv[2], "sha384"))
            {
                HASH384_init(&sha384);
                for (i=0; i<MsgLen; i++)
                    HASH384_process(&sha384,Msg[i]);
                HASH384_hash(&sha384,MD);
            }
            else
            {
                for (i=0; i<MsgLen; i++)
                    HASH256_process(&sha256,Msg[i]);
                HASH256_hash(&sha256,MD);
            }

            octet MDOct= {MDLen,MDLen,MD};
            int rc = OCT_comp(&MD1Oct,&MDOct);
            if (!rc)
            {
                printf("TEST HASH FAILED COMPARE MD LINE %d\n",lineNo);
                exit(EXIT_FAILURE);
            }
            free(Msg);
            Msg = NULL;
            free(MD1);
            MD1 = NULL;
            free(MD);
            MD = NULL;
        }
        lineNo++;
    }
    fclose(fp);
    if (!readLine)
    {
        printf("ERROR Empty test vector file\n");
        exit(EXIT_FAILURE);
    }
    printf("test_hash() TEST HASH %s PASSED\n", argv[2]);
    return EXIT_SUCCESS;
}
