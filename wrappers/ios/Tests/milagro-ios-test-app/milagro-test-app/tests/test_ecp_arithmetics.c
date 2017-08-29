/**
 * @file test_ecp_consistency.c
 * @author Alessandro Budroni
 * @brief Test for aritmetics with ECP
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
#define MAX_STRING 400
#define PIN 1234

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

static int read_ECP(ECP *ecp, char* string)
{
    BIG x;
#if CURVETYPE!=MONTGOMERY
    BIG y;
#endif
    char *stringy = strchr(string,':');
    stringy[0] = '\0';
    read_BIG(x,string);
#if CURVETYPE==MONTGOMERY
    return ECP_set(ecp,x);
#else
    stringy++;
    read_BIG(y,stringy);
    return ECP_set(ecp,x,y);
#endif
}

int test_ecp_arithmetics(int argc, char** argv)
{
    printf("test_ecp_arithmetics() started\n");
    if (argc != 2)
    {
        printf("usage: ./test_ecp_arithmetics [path to test vector file]\n");
        exit(EXIT_FAILURE);
    }

    int i=0, len=0;

    char line[LINE_LEN];
    char * linePtr = NULL;

    ECP inf, ECPaux1;
    BIG BIGaux1, Mod;

    char oct[LINE_LEN];
    octet OCTaux = {0,sizeof(oct),oct};
#if CURVETYPE!=MONTGOMERY
    BIG BIGaux2;
    ECP ECPaux2;
#endif
    ECP ecp1;
    const char* ECP1line = "ECP1 = ";
#if CURVETYPE!=MONTGOMERY
    ECP ecp2;
    const char* ECP2line = "ECP2 = ";
    ECP ecpsum;
    const char* ECPsumline = "ECPsum = ";
    ECP ecpneg;
    const char* ECPnegline = "ECPneg = ";
    ECP ecpsub;
    const char* ECPsubline = "ECPsub = ";
#endif
    ECP ecpdbl;
    const char* ECPdblline = "ECPdbl = ";
    BIG BIGscalar1;
    const char* BIGscalar1line = "BIGscalar1 = ";
    ECP ecpmul;
    const char* ECPmulline = "ECPmul = ";
    ECP ecpwrong;
    const char* ECPwrongline = "ECPwrong = ";
    ECP ecpinf;
    const char* ECPinfline = "ECPinf = ";
#if CURVETYPE!=MONTGOMERY
    ECP ecppinmul;
    const char* ECPpinmulline = "ECPpinmul = ";
    BIG BIGscalar2;
    const char* BIGscalar2line = "BIGscalar2 = ";
    ECP ecpmul2;
    const char* ECPmul2line = "ECPmul2 = ";
    ECP ecpeven;
    const char* ECPevenline = "ECPeven = ";
    ECP ecpodd;
    const char* ECPoddline = "ECPodd = ";
#endif
#if CURVETYPE==MONTGOMERY
    ECP ecpmul3;
    const char* ECPmul3line = "ECPmul3 = ";
#endif

    ECP_inf(&inf);
    BIG_rcopy(Mod,Modulus);

    if(!ECP_isinf(&inf))
    {
        printf("ERROR setting ECP to infinity\n");
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
        if (!strncmp(line,  ECP1line, strlen(ECP1line))) // get first test vector
        {
            len = strlen(ECP1line);
            linePtr = line + len;
            if(!read_ECP(&ecp1,linePtr) || ECP_isinf(&ecp1))
            {
                printf("ERROR getting test vector input ECP, line %d\n",i);
                exit(EXIT_FAILURE);
            }
#if CURVETYPE!=MONTGOMERY
            ECP_get(BIGaux1,BIGaux2,&ecp1);
            FP_nres(BIGaux1);
            FP_nres(BIGaux2);
            FP_sqr(BIGaux2,BIGaux2);
            ECP_rhs(BIGaux1,BIGaux1);
            FP_reduce(BIGaux1); // in case of lazy reduction
            FP_reduce(BIGaux2); // in case of lazy reduction
            if ((BIG_comp(BIGaux1,BIGaux2)!=0)) // test if y^2=f(x)
            {
                printf("ERROR computing right hand side of equation ECP, line %d\n",i);
                exit(EXIT_FAILURE);
            }
#endif
            ECP_toOctet(&OCTaux,&ecp1);
            ECP_fromOctet(&ECPaux1,&OCTaux);
            if(!ECP_equals(&ECPaux1,&ecp1)) // test octet conversion
            {
                printf("ERROR converting ECP to/from OCTET, line %d\n",i);
                exit(EXIT_FAILURE);
            }
        }
#if CURVETYPE!=MONTGOMERY
        if (!strncmp(line,  ECP2line, strlen(ECP2line))) //get second test vector
        {
            len = strlen(ECP2line);
            linePtr = line + len;
            if(!read_ECP(&ecp2,linePtr) || ECP_isinf(&ecp2))
            {
                printf("ERROR getting test vector input ECP, line %d\n",i);
                exit(EXIT_FAILURE);
            }
        }
        if (!strncmp(line,  ECPsumline, strlen(ECPsumline)))
        {
            len = strlen(ECPsumline);
            linePtr = line + len;
            if(!read_ECP(&ecpsum,linePtr) || ECP_isinf(&ecpsum))
            {
                printf("ERROR getting test vector input ECP, line %d\n",i);
                exit(EXIT_FAILURE);
            }
            ECP_copy(&ECPaux1,&ecp1);
            ECP_add(&ECPaux1,&ecp2);
            ECP_affine(&ECPaux1);
            ECP_copy(&ECPaux2,&ecp2);
            ECP_add(&ECPaux2,&ecp1);
            ECP_affine(&ECPaux2);
            if(!ECP_equals(&ECPaux1,&ecpsum) || !ECP_equals(&ECPaux2,&ecpsum)) // test addition P+Q and Q+P (commutativity)
            {
                printf("ERROR adding two ECPs, line %d\n",i);
                exit(EXIT_FAILURE);
            }
            ECP_copy(&ECPaux1,&ecp1); // test associativity
            ECP_add(&ECPaux1,&ecp2);
            ECP_add(&ECPaux1,&ecpsum);
            ECP_copy(&ECPaux2,&ecpsum);
            ECP_add(&ECPaux2,&ecp2);
            ECP_add(&ECPaux2,&ecp1);
            if(!ECP_equals(&ECPaux1,&ECPaux2)) // test associativity (P+Q)+R = P+(Q+R)
            {
                printf("ERROR testing associativity between three ECPs, line %d\n",i);
                exit(EXIT_FAILURE);
            }
        }
        if (!strncmp(line,  ECPsubline, strlen(ECPsubline)))
        {
            len = strlen(ECPsubline);
            linePtr = line + len;
            if(!read_ECP(&ecpsub,linePtr) || ECP_isinf(&ecpsub))
            {
                printf("ERROR getting test vector input ECP, line %d\n",i);
                exit(EXIT_FAILURE);
            }
            ECP_copy(&ECPaux1,&ecp1);
            ECP_sub(&ECPaux1,&ecp2);
            ECP_affine(&ECPaux1);
            if(!ECP_equals(&ECPaux1,&ecpsub)) // test subtraction P-Q
            {
                printf("ERROR computing subtraction of two ECPs, line %d\n",i);
                exit(EXIT_FAILURE);
            }
        }
        if (!strncmp(line,  ECPnegline, strlen(ECPnegline)))
        {
            len = strlen(ECPnegline);
            linePtr = line + len;
            if(!read_ECP(&ecpneg,linePtr) || ECP_isinf(&ecpneg))
            {
                printf("ERROR getting test vector input ECP, line %d\n",i);
                exit(EXIT_FAILURE);
            }
            ECP_copy(&ECPaux1,&ecp1);
            ECP_neg(&ECPaux1);
            ECP_affine(&ECPaux1);
            if(!ECP_equals(&ECPaux1,&ecpneg))
            {
                printf("ERROR computing negative of ECP, line %d\n",i);
                exit(EXIT_FAILURE);
            }
        }
#endif
        if (!strncmp(line,  ECPdblline, strlen(ECPdblline)))
        {
            len = strlen(ECPdblline);
            linePtr = line + len;
            if(!read_ECP(&ecpdbl,linePtr) || ECP_isinf(&ecpdbl))
            {
                printf("ERROR getting test vector input ECP, line %d\n",i);
                exit(EXIT_FAILURE);
            }
            ECP_copy(&ECPaux1,&ecp1);
            ECP_dbl(&ECPaux1);
            ECP_affine(&ECPaux1);
            if(!ECP_equals(&ECPaux1,&ecpdbl))
            {
                ECP_outputxyz(&ECPaux1);
                ECP_outputxyz(&ecpdbl);
                printf("ERROR computing double of ECP, line %d\n",i);
                exit(EXIT_FAILURE);
            }
        }
#if CURVETYPE==MONTGOMERY
        if (!strncmp(line,  ECPmul3line, strlen(ECPmul3line)))
        {
            len = strlen(ECPmul3line);
            linePtr = line + len;
            if(!read_ECP(&ecpmul3,linePtr) || ECP_isinf(&ecpmul3))
            {
                printf("ERROR getting test vector input ECP, line %d\n",i);
                exit(EXIT_FAILURE);
            }
            BIG_one(BIGaux1);
            BIG_inc(BIGaux1,2);
            BIG_norm(BIGaux1);
            ECP_copy(&ECPaux1,&ecp1);
            ECP_mul(&ECPaux1,BIGaux1);
            ECP_affine(&ECPaux1);
            if(!ECP_equals(&ECPaux1,&ecpmul3))
            {
                printf("ERROR computing multiplication of ECP by 3, line %d\n",i);
                exit(EXIT_FAILURE);
            }
            ECP_copy(&ECPaux1,&ecpdbl);
            ECP_add(&ECPaux1,&ecp1,&ecp1);
            if(!ECP_equals(&ECPaux1,&ecpmul3))
            {
                printf("ERROR computing multiplication of ECP by 3, line %d\n",i);
                exit(EXIT_FAILURE);
            }
        }
#endif
        if (!strncmp(line,  BIGscalar1line, strlen(BIGscalar1line)))
        {
            len = strlen(BIGscalar1line);
            linePtr = line + len;
            read_BIG(BIGscalar1,linePtr);
        }
        if (!strncmp(line,  ECPmulline, strlen(ECPmulline)))
        {
            len = strlen(ECPmulline);
            linePtr = line + len;
            if(!read_ECP(&ecpmul,linePtr))
            {
                printf("ERROR getting test vector input ECP, line %d\n",i);
                exit(EXIT_FAILURE);
            }
            ECP_copy(&ECPaux1,&ecp1);
            ECP_mul(&ECPaux1,BIGscalar1);
            ECP_affine(&ECPaux1);
            if(!ECP_equals(&ECPaux1,&ecpmul))
            {
                ECP_outputxyz(&ECPaux1);
                ECP_outputxyz(&ecpmul);
                printf("ERROR computing multiplication of ECP by a scalar, line %d\n",i);
                exit(EXIT_FAILURE);
            }
        }
#if CURVETYPE!=MONTGOMERY
        if (!strncmp(line,  ECPpinmulline, strlen(ECPpinmulline)))
        {
            len = strlen(ECPpinmulline);
            linePtr = line + len;
            if(!read_ECP(&ecppinmul,linePtr))
            {
                printf("ERROR getting test vector input ECP, line %d\n",i);
                exit(EXIT_FAILURE);
            }
            ECP_copy(&ECPaux1,&ecp1);
            ECP_pinmul(&ECPaux1,PIN,14);
            ECP_affine(&ECPaux1);
            if(!ECP_equals(&ECPaux1,&ecppinmul))
            {
                printf("ERROR computing multiplication of ECP by small integer, line %d\n",i);
                exit(EXIT_FAILURE);
            }
        }
        if (!strncmp(line,  BIGscalar2line, strlen(BIGscalar2line)))
        {
            len = strlen(BIGscalar2line);
            linePtr = line + len;
            read_BIG(BIGscalar2,linePtr);
        }
        if (!strncmp(line,  ECPmul2line, strlen(ECPmul2line)))
        {
            len = strlen(ECPmul2line);
            linePtr = line + len;
            if(!read_ECP(&ecpmul2,linePtr))
            {
                printf("ERROR getting test vector input ECP, line %d\n",i);
                exit(EXIT_FAILURE);
            }
            ECP_copy(&ECPaux1,&ecp1);
            ECP_copy(&ECPaux2,&ecp2);
            ECP_mul2(&ECPaux1,&ECPaux2,BIGscalar1,BIGscalar2);
            ECP_affine(&ECPaux1);
            if(!ECP_equals(&ECPaux1,&ecpmul2))
            {
                printf("ERROR computing linear combination of 2 ECPs, line %d\n",i);
                exit(EXIT_FAILURE);
            }
        }
#endif
        if (!strncmp(line,  ECPwrongline, strlen(ECPwrongline)))
        {
            len = strlen(ECPwrongline);
            linePtr = line + len;
            if(read_ECP(&ecpwrong,linePtr) || !ECP_isinf(&ecpwrong) || !ECP_equals(&ecpwrong,&inf))
            {
                printf("ERROR identifying wrong ECP, line %d\n",i);
                exit(EXIT_FAILURE);
            }
        }
        if (!strncmp(line,  ECPinfline, strlen(ECPinfline)))
        {
            len = strlen(ECPinfline);
            linePtr = line + len;
            if(read_ECP(&ecpinf,linePtr) || !ECP_isinf(&ecpinf) || !ECP_equals(&ecpinf,&inf))
            {
                printf("ERROR identifying infinite point ECP, line %d\n",i);
                exit(EXIT_FAILURE);
            }
        }
#if CURVETYPE!=MONTGOMERY
        if (!strncmp(line,  ECPevenline, strlen(ECPevenline)))
        {
            len = strlen(ECPevenline);
            linePtr = line + len;
            if(!read_ECP(&ecpeven,linePtr))
            {
                printf("ERROR getting test vector input ECP, line %d\n",i);
                exit(EXIT_FAILURE);
            }
            ECP_get(BIGaux1,BIGaux2,&ecp1);
            BIG_norm(BIGaux1);
            ECP_setx(&ECPaux1,BIGaux1,0);
            if(!ECP_equals(&ECPaux1,&ecpeven))
            {
                printf("ERROR computing ECP from coordinate x and with y even, line %d\n",i);
                exit(EXIT_FAILURE);
            }
        }
        if (!strncmp(line,  ECPoddline, strlen(ECPoddline)))
        {
            len = strlen(ECPoddline);
            linePtr = line + len;
            if(!read_ECP(&ecpodd,linePtr))
            {
                printf("ERROR getting test vector input ECP, line %d\n",i);
                exit(EXIT_FAILURE);
            }
            ECP_setx(&ECPaux1,BIGaux1,1);
            if(!ECP_equals(&ECPaux1,&ecpodd))
            {
                printf("ERROR computing ECP from coordinate x and with y odd, line %d\n",i);
                exit(EXIT_FAILURE);
            }
        }
#endif
    }
    fclose(fp);

    printf("test_ecp_arithmetics() SUCCESS TEST ARITMETIC OF ECP PASSED\n");
    return EXIT_SUCCESS;
}
