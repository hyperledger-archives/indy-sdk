/**
 * @file test_ecp2_consistency.c
 * @author Alessandro Budroni
 * @brief Test for aritmetics with ECP2
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

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "arch.h"
#include "amcl.h"
#include "utils.h"

#define LINE_LEN 1000
#define MAX_STRING 1000

static void read_BIG(BIG A, char* string)
{
    int len;
    char bin[LINE_LEN];
    BIG_zero(A);
    len = strlen(string)+1;
    amcl_hex2bin(string,bin,len);
    len = (len-1)/2;;
    BIG_fromBytesLen(A,bin,len);
    BIG_norm(A);
}


static int read_ECP2(ECP2 *ecp2, char* stringx1)
{
    char *stringx2, *stringy1, *stringy2;
    BIG x1,x2,y1,y2;
    FP2 x,y;

    stringx2 = strchr(stringx1,':');
    stringx2[0] = '\0';
    stringx2++;
    stringy1 = strchr(stringx2,'&');
    stringy1[0] = '\0';
    stringy1++;
    stringy2 = strchr(stringy1,':');
    stringy2[0] = '\0';
    stringy2++;

    read_BIG(x1,stringx1);
    read_BIG(x2,stringx2);
    read_BIG(y1,stringy1);
    read_BIG(y2,stringy2);

    FP2_from_BIGs(&x,x1,x2);
    FP2_from_BIGs(&y,y1,y2);

    return ECP2_set(ecp2,&x,&y);
}

int test_ecp2_arithmetics(int argc, char** argv)
{
    printf("test_ecp2_arithmetics() started\n");

    if (argc != 2)
    {
        printf("usage: ./test_ecp2_arithmetics [path to test vector file]\n");
        exit(EXIT_FAILURE);
    }

    int i=0, len=0;

    char line[LINE_LEN];
    char * linePtr = NULL;

    ECP2 ECP2aux1, ECP2aux2, inf;
    FP2 FP2aux1,FP2aux2;

    char oct[LINE_LEN];
    octet OCTaux= {0,sizeof(oct),oct};

    ECP2 ecp2[4];
    const char* ECP21line = "ECP21 = ";
    const char* ECP22line = "ECP22 = ";
    const char* ECP23line = "ECP23 = ";
    const char* ECP24line = "ECP24 = ";
    ECP2 ecp2sum;
    const char* ECP2sumline = "ECP2sum = ";
    ECP2 ecp2neg;
    const char* ECP2negline = "ECP2neg = ";
    ECP2 ecp2sub;
    const char* ECP2subline = "ECP2sub = ";
    ECP2 ecp2dbl;
    const char* ECP2dblline = "ECP2dbl = ";
    BIG BIGscalar[4];
    const char* BIGscalar1line = "BIGscalar1 = ";
    const char* BIGscalar2line = "BIGscalar2 = ";
    const char* BIGscalar3line = "BIGscalar3 = ";
    const char* BIGscalar4line = "BIGscalar4 = ";
    ECP2 ecp2mul;
    const char* ECP2mulline = "ECP2mul = ";
    ECP2 ecp2mul4;
    const char* ECP2mul4line = "ECP2mul4 = ";
    ECP2 ecp2wrong;
    const char* ECP2wrongline = "ECP2wrong = ";
    ECP2 ecp2inf;
    const char* ECP2infline = "ECP2inf = ";
    ECP2 ecp2set1;
    const char* ECP2set1line = "ECP2set1 = ";
    ECP2 ecp2set2;
    const char* ECP2set2line = "ECP2set2 = ";

    ECP2_inf(&inf);

    if(!ECP2_isinf(&inf))
    {
        printf("ERROR setting ECP2 to infinity\n");
        exit(EXIT_FAILURE);
    }

    FILE *fp;
    fp = fopen(argv[1],"r");
    if (fp == NULL)
    {
        printf("ERROR opening test vector file\n");
        exit(EXIT_FAILURE);
    }

    while (fgets(line, LINE_LEN, fp) != NULL)
    {
        i++;
        if (!strncmp(line,  ECP21line, strlen(ECP21line)))
        {
            len = strlen(ECP21line);
            linePtr = line + len;
            if(!read_ECP2(&ecp2[0],linePtr) || ECP2_isinf(&ecp2[0]))
            {
                printf("ERROR getting test vector input ECP2, line %d\n",i);
                exit(EXIT_FAILURE);
            }
            ECP2_get(&FP2aux1,&FP2aux2,&ecp2[0]);
            FP2_sqr(&FP2aux2,&FP2aux2);
            ECP2_rhs(&FP2aux1,&FP2aux1);
            if (!FP2_equals(&FP2aux1,&FP2aux2))
            {
                printf("ERROR computing right hand side of equation ECP, line %d\n",i);
                exit(EXIT_FAILURE);
            }
            ECP2_toOctet(&OCTaux,&ecp2[0]);
            ECP2_fromOctet(&ECP2aux1,&OCTaux);
            if(!ECP2_equals(&ECP2aux1,&ecp2[0]))
            {
                printf("ERROR converting ECP2 to/from OCTET, line %d\n",i);
                exit(EXIT_FAILURE);
            }
        }
        if (!strncmp(line,  ECP22line, strlen(ECP22line)))
        {
            len = strlen(ECP22line);
            linePtr = line + len;
            if(!read_ECP2(&ecp2[1],linePtr) || ECP2_isinf(&ecp2[1]))
            {
                printf("ERROR getting test vector input ECP2, line %d\n",i);
                exit(EXIT_FAILURE);
            }
        }
        if (!strncmp(line,  ECP23line, strlen(ECP23line)))
        {
            len = strlen(ECP23line);
            linePtr = line + len;
            if(!read_ECP2(&ecp2[2],linePtr) || ECP2_isinf(&ecp2[2]))
            {
                printf("ERROR getting test vector input ECP2, line %d\n",i);
                exit(EXIT_FAILURE);
            }
        }
        if (!strncmp(line,  ECP24line, strlen(ECP24line)))
        {
            len = strlen(ECP24line);
            linePtr = line + len;
            if(!read_ECP2(&ecp2[3],linePtr) || ECP2_isinf(&ecp2[3]))
            {
                printf("ERROR getting test vector input ECP2, line %d\n",i);
                exit(EXIT_FAILURE);
            }
        }
        if (!strncmp(line,  ECP2sumline, strlen(ECP2sumline)))
        {
            len = strlen(ECP2sumline);
            linePtr = line + len;
            if(!read_ECP2(&ecp2sum,linePtr))
            {
                printf("ERROR reading test vector input ECP2s, line %d\n",i);
                exit(EXIT_FAILURE);
            }
            ECP2_copy(&ECP2aux1,&ecp2[0]);
            ECP2_add(&ECP2aux1,&ecp2[1]);
            ECP2_affine(&ECP2aux1);
            ECP2_copy(&ECP2aux2,&ecp2[1]); // testing commutativity P+Q = Q+P
            ECP2_add(&ECP2aux2,&ecp2[0]);
            ECP2_affine(&ECP2aux2);
            if(!ECP2_equals(&ECP2aux1,&ecp2sum) || !ECP2_equals(&ECP2aux2,&ecp2sum))
            {
                printf("ERROR adding two ECP2s, line %d\n",i);
                exit(EXIT_FAILURE);
            }
            ECP2_copy(&ECP2aux1,&ecp2[0]); // testing associativity (P+Q)+R = P+(Q+R)
            ECP2_add(&ECP2aux1,&ecp2[1]);
            ECP2_add(&ECP2aux1,&ecp2[2]);
            ECP2_affine(&ECP2aux1);
            ECP2_copy(&ECP2aux2,&ecp2[2]);
            ECP2_add(&ECP2aux2,&ecp2[1]);
            ECP2_add(&ECP2aux2,&ecp2[0]);
            ECP2_affine(&ECP2aux2);
            if(!ECP2_equals(&ECP2aux1,&ECP2aux2))
            {
                printf("ERROR testing associativity bewtween three ECP2s, line %d\n",i);
                exit(EXIT_FAILURE);
            }
        }
        if (!strncmp(line,  ECP2negline, strlen(ECP2negline)))
        {
            len = strlen(ECP2negline);
            linePtr = line + len;
            if(!read_ECP2(&ecp2neg,linePtr))
            {
                printf("ERROR getting test vector input ECP2, line %d\n",i);
                exit(EXIT_FAILURE);
            }
            ECP2_copy(&ECP2aux1,&ecp2[0]);
            ECP2_neg(&ECP2aux1);
            ECP2_affine(&ECP2aux1);
            if(!ECP2_equals(&ECP2aux1,&ecp2neg))
            {
                printf("ERROR computing negative of ECP2, line %d\n",i);
                exit(EXIT_FAILURE);
            }
        }
        if (!strncmp(line,  ECP2subline, strlen(ECP2subline)))
        {
            len = strlen(ECP2subline);
            linePtr = line + len;
            if(!read_ECP2(&ecp2sub,linePtr))
            {
                printf("ERROR getting test vector input ECP2, line %d\n",i);
                exit(EXIT_FAILURE);
            }
            ECP2_copy(&ECP2aux1,&ecp2[0]);
            ECP2_sub(&ECP2aux1,&ecp2[1]);
            ECP2_affine(&ECP2aux1);
            if(!ECP2_equals(&ECP2aux1,&ecp2sub))
            {
                printf("ERROR performing subtraction bewtween two ECP2s, line %d\n",i);
                exit(EXIT_FAILURE);
            }
        }
        if (!strncmp(line,  ECP2dblline, strlen(ECP2dblline)))
        {
            len = strlen(ECP2dblline);
            linePtr = line + len;
            if(!read_ECP2(&ecp2dbl,linePtr))
            {
                printf("ERROR getting test vector input ECP2, line %d\n",i);
                exit(EXIT_FAILURE);
            }
            ECP2_copy(&ECP2aux1,&ecp2[0]);
            ECP2_dbl(&ECP2aux1);
            ECP2_affine(&ECP2aux1);
            if(!ECP2_equals(&ECP2aux1,&ecp2dbl))
            {
                printf("ERROR computing double of ECP2, line %d\n",i);
                exit(EXIT_FAILURE);
            }
        }
        if (!strncmp(line,  BIGscalar1line, strlen(BIGscalar1line)))
        {
            len = strlen(BIGscalar1line);
            linePtr = line + len;
            read_BIG(BIGscalar[0],linePtr);
        }
        if (!strncmp(line,  BIGscalar2line, strlen(BIGscalar2line)))
        {
            len = strlen(BIGscalar2line);
            linePtr = line + len;
            read_BIG(BIGscalar[1],linePtr);
        }
        if (!strncmp(line,  BIGscalar3line, strlen(BIGscalar3line)))
        {
            len = strlen(BIGscalar3line);
            linePtr = line + len;
            read_BIG(BIGscalar[2],linePtr);
        }
        if (!strncmp(line,  BIGscalar4line, strlen(BIGscalar4line)))
        {
            len = strlen(BIGscalar4line);
            linePtr = line + len;
            read_BIG(BIGscalar[3],linePtr);
        }
        if (!strncmp(line,  ECP2mulline, strlen(ECP2mulline)))
        {
            len = strlen(ECP2mulline);
            linePtr = line + len;
            if(!read_ECP2(&ecp2mul,linePtr))
            {
                printf("ERROR getting test vector input ECP2, line %d\n",i);
                exit(EXIT_FAILURE);
            }
            ECP2_copy(&ECP2aux1,&ecp2[0]);
            ECP2_mul(&ECP2aux1,BIGscalar[0]);
            ECP2_affine(&ECP2aux1);
            if(!ECP2_equals(&ECP2aux1,&ecp2mul))
            {
                printf("ERROR computing multiplication of ECP2 by a scalar, line %d\n",i);
                exit(EXIT_FAILURE);
            }
        }
        if (!strncmp(line,  ECP2mul4line, strlen(ECP2mul4line)))
        {
            len = strlen(ECP2mul4line);
            linePtr = line + len;
            if(!read_ECP2(&ecp2mul4,linePtr))
            {
                printf("ERROR getting test vector input ECP2, line %d\n",i);
                exit(EXIT_FAILURE);
            }
            ECP2_mul4(&ECP2aux1,ecp2,BIGscalar);
            ECP2_affine(&ECP2aux1);
            if(!ECP2_equals(&ECP2aux1,&ecp2mul4))
            {
                printf("ERROR computing linear combination of 4 ECP2s, line %d\n",i);
                exit(EXIT_FAILURE);
            }
        }
        if (!strncmp(line,  ECP2wrongline, strlen(ECP2wrongline)))
        {
            len = strlen(ECP2wrongline);
            linePtr = line + len;
            if(read_ECP2(&ecp2wrong,linePtr) || !ECP2_isinf(&ecp2wrong) || !ECP2_equals(&ecp2wrong,&inf))
            {
                printf("ERROR identifying a wrong ECP2, line %d\n",i);
                exit(EXIT_FAILURE);
            }
        }
        if (!strncmp(line,  ECP2infline, strlen(ECP2infline)))
        {
            len = strlen(ECP2infline);
            linePtr = line + len;
            if(read_ECP2(&ecp2inf,linePtr) || !ECP2_isinf(&ecp2inf) || !ECP2_equals(&ecp2inf,&inf))
            {
                printf("ERROR identifying infinite point ECP2, line %d\n",i);
                exit(EXIT_FAILURE);
            }
        }
        if (!strncmp(line,  ECP2set1line, strlen(ECP2set1line)))
        {
            len = strlen(ECP2set1line);
            linePtr = line + len;
            if(!read_ECP2(&ecp2set1,linePtr))
            {
                printf("ERROR getting test vector input ECP2, line %d\n",i);
                exit(EXIT_FAILURE);
            }
            ECP2_get(&FP2aux1,&FP2aux2,&ecp2[0]);
            ECP2_setx(&ECP2aux1,&FP2aux1);
        }
        if (!strncmp(line,  ECP2set2line, strlen(ECP2set2line)))
        {
            len = strlen(ECP2set2line);
            linePtr = line + len;
            if(!read_ECP2(&ecp2set2,linePtr))
            {
                printf("ERROR getting test vector input ECP2, line %d\n",i);
                exit(EXIT_FAILURE);
            }
            if((!ECP2_equals(&ECP2aux1,&ecp2set2)) && (!ECP2_equals(&ECP2aux1,&ecp2set1)))
            {
                printf("ERROR computing ECP2 from coordinate x and with y set2, line %d\n",i);
                exit(EXIT_FAILURE);
            }
        }
    }
    fclose(fp);

    printf("test_ecp2_arithmetics() SUCCESS TEST ARITMETIC OF ECP2 PASSED\n");
    return EXIT_SUCCESS;
}
