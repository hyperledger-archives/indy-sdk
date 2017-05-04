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


int test_fp_arithmetics(int argc, char** argv)
{
    printf("test_fp_arithmetics() started\n");
    if (argc != 2)
    {
        printf("usage: ./test_fp_arithmetics [path to test vector file]\n");
        exit(EXIT_FAILURE);
    }

    int i = 0, len = 0, j = 0, k = 0;
    FILE *fp;

    char line[LINE_LEN];
    char * linePtr = NULL;

    BIG M,supp, supp1, supp2, supp3;

    BIG FP_1;
    const char* FP_1line = "FP_1 = ";
    BIG FP_2;
    const char* FP_2line = "FP_2 = ";
    BIG FPadd;
    const char* FPaddline = "FPadd = ";
    BIG FPsub;
    const char* FPsubline = "FPsub = ";
    BIG FP_1nres;
    const char* FP_1nresline = "FP_1nres = ";
    BIG FP_2nres;
    const char* FP_2nresline = "FP_2nres = ";
    BIG FPmulmod;
    const char* FPmulmodline = "FPmulmod = ";
    BIG FPsmallmul;
    const char* FPsmallmulline = "FPsmallmul = ";
    BIG FPsqr;
    const char* FPsqrline = "FPsqr = ";
    BIG FPreduce;
    const char* FPreduceline = "FPreduce = ";
    BIG FPneg;
    const char* FPnegline = "FPneg = ";
    BIG FPdiv2;
    const char* FPdiv2line = "FPdiv2 = ";
    BIG FPinv;
    const char* FPinvline = "FPinv = ";
    BIG FPexp;
    const char* FPexpline = "FPexp = ";

// Set to zero
    BIG_zero(FP_1);
    BIG_zero(FP_2);
    BIG_rcopy(M,Modulus);

// Testing equal function and set zero function
    if(BIG_comp(FP_1,FP_2) || !FP_iszilch(FP_1) || !FP_iszilch(FP_2))
    {
        printf("ERROR comparing FPs or setting FP to zero FP\n");
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
// Read first FP
        if (!strncmp(line,FP_1line, strlen(FP_1line)))
        {
            len = strlen(FP_1line);
            linePtr = line + len;
            read_BIG(FP_1,linePtr);
        }
// Read second FP
        if (!strncmp(line,FP_2line, strlen(FP_2line)))
        {
            len = strlen(FP_2line);
            linePtr = line + len;
            read_BIG(FP_2,linePtr);
        }
// Addition test
        if (!strncmp(line,FPaddline, strlen(FPaddline)))
        {
            len = strlen(FPaddline);
            linePtr = line + len;
            read_BIG(FPadd,linePtr);
            BIG_copy(supp1,FP_2);
            BIG_copy(supp,FP_1);
            BIG_copy(supp2,FP_1);
            FP_add(supp,supp,supp1);
            FP_add(supp2,supp2,supp1); // test commutativity P+Q = Q+P
            BIG_norm(supp);
            FP_reduce(supp);
            BIG_norm(supp2);
            FP_reduce(supp2);
            if(BIG_comp(supp,FPadd) || BIG_comp(supp2,FPadd))
            {
                printf("ERROR adding two FPs, line %d\n",i);
                exit(EXIT_FAILURE);
            }
            BIG_copy(supp,FP_1); // test associativity (P+Q)+R = P+(Q+R)
            BIG_copy(supp2,FP_1);
            BIG_copy(supp1,FP_2);
            BIG_copy(supp3,FPadd);
            FP_add(supp,supp,supp1);
            FP_add(supp,supp,supp3);
            FP_add(supp1,supp1,supp3);
            FP_add(supp1,supp1,supp2);
            FP_reduce(supp);
            FP_reduce(supp1);
            BIG_norm(supp);
            BIG_norm(supp1);
            if(BIG_comp(supp,supp1))
            {
                printf("ERROR testing associativity between three FPs, line %d\n",i);
                exit(EXIT_FAILURE);
            }
        }
// Subtraction test
        if (!strncmp(line,FPsubline, strlen(FPsubline)))
        {
            len = strlen(FPsubline);
            linePtr = line + len;
            read_BIG(FPsub,linePtr);
            BIG_copy(supp,FP_1);
            BIG_copy(supp1,FP_2);
            FP_sub(supp,supp,supp1);
            FP_redc(supp);
            FP_nres(supp);
            BIG_sub(supp1,supp,M); // in case of lazy reduction
            BIG_norm(supp1);
            if((BIG_comp(supp,FPsub) != 0) && (BIG_comp(supp1,FPsub) != 0))
            {
                printf("ERROR subtraction between two FPs, line %d\n",i);
                exit(EXIT_FAILURE);
            }
        }
// Reduce first FP
        if (!strncmp(line,FP_1nresline, strlen(FP_1nresline)))
        {
            len = strlen(FP_1nresline);
            linePtr = line + len;
            read_BIG(FP_1nres,linePtr);
            BIG_copy(supp,FP_1);
            FP_nres(supp);
            FP_redc(supp);
            if(BIG_comp(supp,FP_1nres))
            {
                printf("comp ");
                BIG_output(supp);
                printf("\n\n");
                printf("read ");
                BIG_output(FP_1nres);
                printf("\n\n");
                printf("ERROR Converts from BIG integer to n-residue form, line %d\n",i);
                exit(EXIT_FAILURE);
            }
        }
// Reduce second FP
        if (!strncmp(line,FP_2nresline, strlen(FP_2nresline)))
        {
            len = strlen(FP_2nresline);
            linePtr = line + len;
            read_BIG(FP_2nres,linePtr);
            BIG_copy(supp,FP_2);
            FP_nres(supp);
            FP_redc(supp);
            if(BIG_comp(supp,FP_2nres))
            {
                printf("comp ");
                BIG_output(supp);
                printf("\n\n");
                printf("read ");
                BIG_output(FP_2nres);
                printf("\n\n");
                printf("ERROR Converts from BIG integer to n-residue form, line %d\n",i);
                exit(EXIT_FAILURE);
            }
        }
// Multiplication modulo
        if (!strncmp(line,FPmulmodline, strlen(FPmulmodline)))
        {
            len = strlen(FPmulmodline);
            linePtr = line + len;
            read_BIG(FPmulmod,linePtr);
            BIG_copy(supp,FP_1);
            BIG_copy(supp1,FP_2);
            FP_nres(supp);
            FP_nres(supp1);
            FP_mul(supp,supp,supp1);
            FP_redc(supp);
            if(BIG_comp(supp,FPmulmod))
            {
                printf("comp ");
                BIG_output(supp);
                printf("\n\n");
                printf("read ");
                BIG_output(FPmulmod);
                printf("\n\n");
                printf("ERROR in multiplication and reduction by Modulo, line %d\n",i);
                exit(EXIT_FAILURE);
            }
        }
// Small multiplication
        if (!strncmp(line,FPsmallmulline, strlen(FPsmallmulline)))
        {
            len = strlen(FPsmallmulline);
            linePtr = line + len;
            read_BIG(FPsmallmul,linePtr);
            FP_imul(supp,FP_1,0);
            BIG_norm(supp);
            if (!FP_iszilch(supp))
            {
                printf("ERROR in  multiplication by 0, line %d\n",i);
            }
            for (j = 1; j <= 10; ++j)
            {
                FP_imul(supp,FP_1,j);
                BIG_copy(supp1,FP_1);
                for (k = 1; k < j; ++k)
                {
                    BIG_norm(supp1);
                    FP_add(supp1,supp1,FP_1);
                }
                BIG_norm(supp1);
                if(BIG_comp(supp,supp1) != 0)
                {
                    printf("comp1 ");
                    BIG_output(supp);
                    printf("\n\n");
                    printf("comp2 ");
                    BIG_output(supp1);
                    printf("\n\n");
                    printf("ERROR in small multiplication or addition, line %d, multiplier %d\n",i,j);
                    exit(EXIT_FAILURE);
                }
            }
            FP_reduce(supp);
            FP_reduce(supp1);
            if(BIG_comp(supp,FPsmallmul) | BIG_comp(supp1,supp))
            {
                printf("comp1 ");
                BIG_output(supp);
                printf("\n\n");
                printf("comp2 ");
                BIG_output(supp1);
                printf("\n\n");
                printf("read  ");
                BIG_output(FPsmallmul);
                printf("\n\n");
                printf("ERROR in small multiplication, line %d\n",i);
                exit(EXIT_FAILURE);
            }
        }
// Square and square root
        if (!strncmp(line,FPsqrline, strlen(FPsqrline)))
        {
            len = strlen(FPsqrline);
            linePtr = line + len;
            read_BIG(FPsqr,linePtr);
            BIG_copy(supp,FP_1);
            FP_nres(supp);
            FP_sqr(supp,supp);
            FP_redc(supp);
            if(BIG_comp(supp,FPsqr))
            {
                printf("supp ");
                BIG_output(supp);
                printf("\n\n");
                printf("read ");
                BIG_output(FPsqr);
                printf("\n\n");
                printf("ERROR in squaring FP, line %d\n",i);
                exit(EXIT_FAILURE);
            }
            /*FP_nres(supp);
            FP_sqrt(supp,supp);
            FP_redc(supp);
            if(BIG_comp(supp,FP_1))
            {
                printf("supp ");BIG_output(supp);printf("\n\n");
                printf("read ");BIG_output(FP_1);printf("\n\n");
                printf("ERROR square/square root consistency FP, line %d\n",i);
                exit(EXIT_FAILURE);
            }*/
        }
// Reducing Modulo
        if (!strncmp(line,FPreduceline, strlen(FPreduceline)))
        {
            len = strlen(FPreduceline);
            linePtr = line + len;
            read_BIG(FPreduce,linePtr);
            BIG_copy(supp,FP_1);
            FP_reduce(supp);
            if(BIG_comp(supp,FPreduce))
            {
                printf("comp ");
                BIG_output(supp);
                printf("\n\n");
                printf("read ");
                BIG_output(FPreduce);
                printf("\n\n");
                printf("ERROR in reducing FP, line %d\n",i);
                exit(EXIT_FAILURE);
            }
        }
// Negative an FP
        if (!strncmp(line,FPnegline, strlen(FPnegline)))
        {
            len = strlen(FPnegline);
            linePtr = line + len;
            read_BIG(FPneg,linePtr);
            BIG_copy(supp,FP_1);
            FP_nres(supp);
            FP_neg(supp,supp);
            FP_redc(supp);
            BIG_sub(supp1,supp,M); // in case of lazy reduction
            BIG_norm(supp1);
            if((BIG_comp(supp,FPneg) != 0) && (BIG_comp(supp1,FPneg) != 0))
            {
                printf("comp ");
                BIG_output(supp);
                printf("\n\n");
                printf("read ");
                BIG_output(FPneg);
                printf("\n\n");
                printf("ERROR in computing FP_neg, line %d\n",i);
                exit(EXIT_FAILURE);
            }
        }
// Division by 2
        if (!strncmp(line,FPdiv2line, strlen(FPdiv2line)))
        {
            len = strlen(FPdiv2line);
            linePtr = line + len;
            read_BIG(FPdiv2,linePtr);
            BIG_copy(supp,FP_1);
            FP_redc(supp);
            FP_nres(supp);
            FP_div2(supp,supp);
            if(BIG_comp(supp,FPdiv2))
            {
                printf("comp ");
                BIG_output(supp);
                printf("\n\n");
                printf("read ");
                BIG_output(FPdiv2);
                printf("\n\n");
                printf("ERROR in division by 2, line %d\n",i);
                exit(EXIT_FAILURE);
            }
        }
// Inverse Modulo and FP_one
        if (!strncmp(line,FPinvline, strlen(FPinvline)))
        {
            len = strlen(FPinvline);
            linePtr = line + len;
            read_BIG(FPinv,linePtr);
            BIG_copy(supp,FP_1);
            BIG_copy(supp1,FP_1);
            FP_nres(supp);
            FP_inv(supp,supp);
            FP_redc(supp);
            if(BIG_comp(supp,FPinv))
            {
                printf("comp ");
                BIG_output(supp);
                printf("\n\n");
                printf("read ");
                BIG_output(FPinv);
                printf("\n\n");
                printf("ERROR computing inverse modulo, line %d\n",i);
                exit(EXIT_FAILURE);
            }
            FP_mul(supp,supp,supp1);
            FP_nres(supp);
            FP_reduce(supp);
            FP_one(supp1);
            FP_redc(supp1);
            if(BIG_comp(supp,supp1))
            {
                printf("comp1 ");
                BIG_output(supp);
                printf("\n\n");
                printf("comp2 ");
                BIG_output(supp1);
                printf("\n\n");
                printf("ERROR multipling FP and its inverse, line %d\n",i);
                exit(EXIT_FAILURE);
            }
        }
// modular exponentiation
        if (!strncmp(line,FPexpline, strlen(FPexpline)))
        {
            len = strlen(FPexpline);
            linePtr = line + len;
            read_BIG(FPexp,linePtr);
            BIG_copy(supp,FP_1);
            BIG_copy(supp1,FP_2);
            FP_nres(supp);
            FP_pow(supp,supp,supp1);
            FP_redc(supp);
            if(BIG_comp(supp,FPexp))
            {
                printf("supp ");
                BIG_output(supp);
                printf("\n\n");
                printf("read ");
                BIG_output(FPexp);
                printf("\n\n");
                printf("ERROR in modular exponentiation, line %d\n",i);
                exit(EXIT_FAILURE);
            }
        }
    }
    fclose(fp);

    printf("test_fp_arithmetics() SUCCESS TEST ARITMETIC OF FP PASSED\n");
    return EXIT_SUCCESS;
}
