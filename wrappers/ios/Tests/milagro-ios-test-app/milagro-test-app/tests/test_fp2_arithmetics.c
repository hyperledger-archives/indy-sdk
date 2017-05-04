/**
 * @file test_fp_arithmetics.c
 * @author Alessandro Budroni
 * @brief Test for aritmetics with FP
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


#include "arch.h"
#include "amcl.h"
#include "utils.h"
#include <stdio.h>
#include <string.h>
#include <stdlib.h>

#define LINE_LEN 10000
#define MAX_STRING 300

static void read_BIG(BIG A, char* string)
{
    int len;
    char support[LINE_LEN];
    BIG_zero(A);
    len = strlen(string)+1;
    amcl_hex2bin(string,support,len);
    len = (len-1)/2;;
    BIG_fromBytesLen(A,support,len);
    BIG_norm(A);
}

void read_FP2(FP2 *fp2, char* stringx)
{
    char *stringy;
    BIG x,y;

    stringy = strchr(stringx,',');
    stringy[0] = '\0';
    stringy++;

    read_BIG(x,stringx);
    read_BIG(y,stringy);

    FP2_from_BIGs(fp2,x,y);
}

int test_fp2_arithmetics(int argc, char** argv)
{
    printf("test_fp2_arithmetics() started\n");
    if (argc != 2)
    {
        printf("usage: ./test_fp2_arithmetics [path to test vector file]\n");
        exit(EXIT_FAILURE);
    }

    int i = 0, len = 0, j = 0;
    FILE *fp;

    char line[LINE_LEN];
    char * linePtr = NULL;

    BIG M;
    FP2 FP2aux1, FP2aux2, FP2aux3, FP2aux4;

    FP2 FP2_1;
    const char* FP2_1line = "FP2_1 = ";
    FP2 FP2_2;
    const char* FP2_2line = "FP2_2 = ";
    FP2 FP2add;
    const char* FP2addline = "FP2add = ";
    FP2 FP2neg;
    const char* FP2negline = "FP2neg = ";
    FP2 FP2sub;
    const char* FP2subline = "FP2sub = ";
    FP2 FP2conj;
    const char* FP2conjline = "FP2conj = ";
    BIG BIGsc;
    const char* BIGscline = "BIGsc = ";
    FP2 FP2pmul;
    const char* FP2pmulline = "FP2pmul = ";
    FP2 FP2imul;
    const char* FP2imulline = "FP2imul = ";
    FP2 FP2sqr;
    const char* FP2sqrline = "FP2sqr = ";
    FP2 FP2mul;
    const char* FP2mulline = "FP2mul = ";
    FP2 FP2inv;
    const char* FP2invline = "FP2inv = ";
    FP2 FP2div2;
    const char* FP2div2line = "FP2div2 = ";
    FP2 FP2_mulip;
    const char* FP2_mulipline = "FP2_mulip = ";
    FP2 FP2_divip;
    const char* FP2_divipline = "FP2_divip = ";
    FP2 FP2pow;
    const char* FP2powline = "FP2pow = ";

    BIG_rcopy(M,Modulus);

// Set to zero
    FP2_zero(&FP2aux1);
    FP2_zero(&FP2aux2);

// Testing equal function and set zero function
    if(!FP2_equals(&FP2aux1,&FP2aux2) || !FP2_iszilch(&FP2aux1) || !FP2_iszilch(&FP2aux2))
    {
        printf("ERROR comparing FP2s or setting FP2 to zero FP\n");
        exit(EXIT_FAILURE);
    }

// Set to one
    FP2_one(&FP2aux1);
    FP2_one(&FP2aux2);

// Testing equal function and set one function
    if(!FP2_equals(&FP2aux1,&FP2aux2) || !FP2_isunity(&FP2aux1) || !FP2_isunity(&FP2aux2))
    {
        printf("ERROR comparing FP2s or setting FP2 to unity FP\n");
        exit(EXIT_FAILURE);
    }


    fp = fopen(argv[1], "r");
    if (fp == NULL)
    {
        printf("ERROR opening test vector file\n");
        exit(EXIT_FAILURE);
    }

    while (fgets(line, LINE_LEN, fp) != NULL)
    {
        i++;
// Read first FP2 and perform some tests
        if (!strncmp(line,FP2_1line, strlen(FP2_1line)))
        {
            len = strlen(FP2_1line);
            linePtr = line + len;
            read_FP2(&FP2_1,linePtr);
            FP2_cmove(&FP2aux1,&FP2_1,0);
            if(FP2_equals(&FP2aux1,&FP2_1) != 0)
            {
                printf("ERROR in conditional copy of FP2, line %d\n",i);
                exit(EXIT_FAILURE);
            }
            FP2_cmove(&FP2aux1,&FP2_1,1);
            if(FP2_equals(&FP2aux1,&FP2_1) != 1)
            {
                printf("ERROR in conditional copy of FP2, line %d\n",i);
                exit(EXIT_FAILURE);
            }
            FP2_from_FPs(&FP2aux1,FP2_1.a,FP2_1.b);
            if(FP2_equals(&FP2aux1,&FP2_1) != 1)
            {
                printf("ERROR in generating FP2 from two FPs, line %d\n",i);
                exit(EXIT_FAILURE);
            }
            FP2_from_BIGs(&FP2aux1,FP2_1.a,FP2_1.b);
            FP_redc(FP2aux1.a);
            FP_redc(FP2aux1.b);
            if(FP2_equals(&FP2aux1,&FP2_1) != 1)
            {
                printf("ERROR in generating FP2 from two BIGs, line %d\n",i);
                exit(EXIT_FAILURE);
            }
            FP2_from_FP(&FP2aux1,FP2_1.a);
            FP2_copy(&FP2aux2,&FP2_1);
            BIG_zero(FP2aux2.b);
            if(FP2_equals(&FP2aux1,&FP2aux2) != 1)
            {
                printf("ERROR in generating FP2 from one FP, line %d\n",i);
                exit(EXIT_FAILURE);
            }
            FP2_from_BIG(&FP2aux1,FP2_1.a);
            FP_redc(FP2aux1.a);
            FP2_copy(&FP2aux2,&FP2_1);
            BIG_zero(FP2aux2.b);
            if(FP2_equals(&FP2aux1,&FP2aux2) != 1)
            {
                printf("ERROR in generating FP2 from one BIG, line %d\n",i);
                exit(EXIT_FAILURE);
            }
        }
// Read second FP2
        if (!strncmp(line,FP2_2line, strlen(FP2_2line)))
        {
            len = strlen(FP2_2line);
            linePtr = line + len;
            read_FP2(&FP2_2,linePtr);
        }
// Addition tests
        if (!strncmp(line,FP2addline, strlen(FP2addline)))
        {
            len = strlen(FP2addline);
            linePtr = line + len;
            read_FP2(&FP2add,linePtr);
            FP2_copy(&FP2aux1,&FP2_1);
            FP2_copy(&FP2aux2,&FP2_2);
            FP2_add(&FP2aux1,&FP2aux1,&FP2aux2);
// test commutativity P+Q = Q+P
            FP2_copy(&FP2aux3,&FP2_1);
            FP2_add(&FP2aux2,&FP2aux2,&FP2aux3);
            FP2_reduce(&FP2aux1);
            FP2_norm(&FP2aux1);
            FP2_reduce(&FP2aux2);
            FP2_norm(&FP2aux2);
            if(!FP2_equals(&FP2aux1,&FP2add) || !FP2_equals(&FP2aux2,&FP2add))
            {
                printf("ERROR adding two FP2, line %d\n",i);
                exit(EXIT_FAILURE);
            }
// test associativity (P+Q)+R = P+(Q+R)
            FP2_copy(&FP2aux1,&FP2_1);
            FP2_copy(&FP2aux3,&FP2_1);
            FP2_copy(&FP2aux2,&FP2_2);
            FP2_copy(&FP2aux4,&FP2add);
            FP2_add(&FP2aux1,&FP2aux1,&FP2aux2);
            FP2_add(&FP2aux1,&FP2aux1,&FP2aux4);
            FP2_add(&FP2aux2,&FP2aux2,&FP2aux4);
            FP2_add(&FP2aux2,&FP2aux2,&FP2aux3);
            FP2_reduce(&FP2aux1);
            FP2_reduce(&FP2aux2);
            FP2_norm(&FP2aux1);
            FP2_norm(&FP2aux2);
            if(!FP2_equals(&FP2aux1,&FP2aux2))
            {
                printf("ERROR testing associativity between three FP2s, line %d\n",i);
                exit(EXIT_FAILURE);
            }
        }
// Negative an FP2
        if (!strncmp(line,FP2negline, strlen(FP2negline)))
        {
            len = strlen(FP2negline);
            linePtr = line + len;
            read_FP2(&FP2neg,linePtr);
            FP2_copy(&FP2aux1,&FP2_1);
            FP2_neg(&FP2aux1,&FP2aux1);
            FP2_reduce(&FP2aux1);
            FP2_norm(&FP2aux1);
            if(!FP2_equals(&FP2aux1,&FP2neg))
            {
                printf("ERROR in computing negative of FP2, line %d\n",i);
                exit(EXIT_FAILURE);
            }
        }
// Subtraction test
        if (!strncmp(line,FP2subline, strlen(FP2subline)))
        {
            len = strlen(FP2subline);
            linePtr = line + len;
            read_FP2(&FP2sub,linePtr);
            FP2_copy(&FP2aux1,&FP2_1);
            FP2_copy(&FP2aux2,&FP2_2);
            FP2_sub(&FP2aux1,&FP2aux1,&FP2aux2);
            FP2_reduce(&FP2aux1);
            FP2_norm(&FP2aux1);
            if(FP2_equals(&FP2aux1,&FP2sub) == 0)
            {
                printf("ERROR subtraction between two FP2, line %d\n",i);
                exit(EXIT_FAILURE);
            }
        }
// Compute conjugate
        if (!strncmp(line,FP2conjline, strlen(FP2conjline)))
        {
            len = strlen(FP2conjline);
            linePtr = line + len;
            read_FP2(&FP2conj,linePtr);
            FP2_copy(&FP2aux1,&FP2_1);
            FP2_conj(&FP2aux1,&FP2aux1);
            FP2_reduce(&FP2aux1);
            FP2_norm(&FP2aux1);
            if(!FP2_equals(&FP2aux1,&FP2conj))
            {
                printf("ERROR computing conjugate of FP2, line %d\n",i);
                exit(EXIT_FAILURE);
            }
        }
// Read multiplicator
        if (!strncmp(line,BIGscline, strlen(BIGscline)))
        {
            len = strlen(BIGscline);
            linePtr = line + len;
            read_BIG(BIGsc,linePtr);
        }
// Multiplication by BIGsc
        if (!strncmp(line,FP2pmulline, strlen(FP2pmulline)))
        {
            len = strlen(FP2pmulline);
            linePtr = line + len;
            read_FP2(&FP2pmul,linePtr);
            FP2_pmul(&FP2aux1,&FP2_1,BIGsc);
            FP_nres(FP2aux1.a);
            FP_nres(FP2aux1.b);
            if(!FP2_equals(&FP2aux1,&FP2pmul))
            {
                printf("ERROR in multiplication by BIG, line %d\n",i);
                exit(EXIT_FAILURE);
            }
        }
// Raise FP2 by power BIGsc
        if (!strncmp(line,FP2powline, strlen(FP2powline)))
        {
            len = strlen(FP2powline);
            linePtr = line + len;
            read_FP2(&FP2pow,linePtr);
            FP2_pow(&FP2aux1,&FP2_1,BIGsc);
            FP2_reduce(&FP2aux1);
            if(!FP2_equals(&FP2aux1,&FP2pow))
            {
                printf("ERROR in raising FP2 by power BIG, line %d\n",i);
                exit(EXIT_FAILURE);
            }
        }
// Multiplication by j = 1..10
        if (!strncmp(line,FP2imulline, strlen(FP2imulline)))
        {
            len = strlen(FP2imulline);
            linePtr = line + len;
            read_FP2(&FP2imul,linePtr);
            FP2_imul(&FP2aux1,&FP2_1,j);
            j++;
            FP2_reduce(&FP2aux1);
            FP2_norm(&FP2aux1);
            if(!FP2_equals(&FP2aux1,&FP2imul))
            {
                printf("ERROR in multiplication by small integer, line %d\n",i);
                exit(EXIT_FAILURE);
            }
        }
// Square and square root
        if (!strncmp(line,FP2sqrline, strlen(FP2sqrline)))
        {
            len = strlen(FP2sqrline);
            linePtr = line + len;
            read_FP2(&FP2sqr,linePtr);
            FP2_copy(&FP2aux1,&FP2_1);
            FP2_sqr(&FP2aux1,&FP2aux1);
            FP2_reduce(&FP2aux1);
            FP2_norm(&FP2aux1);
            if(!FP2_equals(&FP2aux1,&FP2sqr))
            {
                printf("ERROR in squaring FP2, line %d\n",i);
                exit(EXIT_FAILURE);
            }
            FP2_sqrt(&FP2aux1,&FP2aux1);
            FP2_neg(&FP2aux2,&FP2aux1);
            if(!FP2_equals(&FP2aux1,&FP2_1) && !FP2_equals(&FP2aux2,&FP2_1))
            {
                printf("ERROR square/square root consistency FP2, line %d\n",i);
                exit(EXIT_FAILURE);
            }
        }
// Multiplication between two FP2s
        if (!strncmp(line,FP2mulline, strlen(FP2mulline)))
        {
            len = strlen(FP2mulline);
            linePtr = line + len;
            read_FP2(&FP2mul,linePtr);
            FP2_mul(&FP2aux1,&FP2_1,&FP2_2);
            FP2_reduce(&FP2aux1);
            FP2_norm(&FP2aux1);
            if(!FP2_equals(&FP2aux1,&FP2mul))
            {
                printf("ERROR in multiplication between two FP2s, line %d\n",i);
                exit(EXIT_FAILURE);
            }
        }
// Inverse
        if (!strncmp(line,FP2invline, strlen(FP2invline)))
        {
            len = strlen(FP2invline);
            linePtr = line + len;
            read_FP2(&FP2inv,linePtr);
            FP2_copy(&FP2aux1,&FP2_1);
            FP2_inv(&FP2aux1,&FP2aux1);
            if(!FP2_equals(&FP2aux1,&FP2inv))
            {
                printf("ERROR in computing inverse of FP2, line %d\n",i);
                exit(EXIT_FAILURE);
            }
        }
// Divide an FP2 by 2
        if (!strncmp(line,FP2div2line, strlen(FP2div2line)))
        {
            len = strlen(FP2div2line);
            linePtr = line + len;
            read_FP2(&FP2div2,linePtr);
            FP2_div2(&FP2aux1,&FP2_1);
            if(!FP2_equals(&FP2aux1,&FP2div2))
            {
                printf("ERROR in computing division FP2 by 2, line %d\n",i);
                exit(EXIT_FAILURE);
            }
        }
// Multiply an FP2 by (1+sqrt(-1))
        if (!strncmp(line,FP2_mulipline, strlen(FP2_mulipline)))
        {
            len = strlen(FP2_mulipline);
            linePtr = line + len;
            read_FP2(&FP2_mulip,linePtr);
            FP2_copy(&FP2aux1,&FP2_1);
            FP2_mul_ip(&FP2aux1);
            if(!FP2_equals(&FP2aux1,&FP2_mulip))
            {
                printf("ERROR in computing multiplication of FP2 by (1+sqrt(-1)), line %d\n",i);
                exit(EXIT_FAILURE);
            }
        }
// Divide an FP2 by (1+sqrt(-1))
        if (!strncmp(line,FP2_divipline, strlen(FP2_divipline)))
        {
            len = strlen(FP2_divipline);
            linePtr = line + len;
            read_FP2(&FP2_divip,linePtr);
            FP2_copy(&FP2aux1,&FP2_1);
            FP2_div_ip(&FP2aux1);
            if(!FP2_equals(&FP2aux1,&FP2_divip))
            {
                printf("ERROR in computing division of FP2 by (1+sqrt(-1)), line %d\n",i);
                exit(EXIT_FAILURE);
            }
        }
    }
    fclose(fp);

    printf("test_fp2_arithmetics() SUCCESS TEST ARITMETIC OF FP PASSED\n");
    return EXIT_SUCCESS;
}
