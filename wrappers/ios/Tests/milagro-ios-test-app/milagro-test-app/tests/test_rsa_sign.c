/**
 * @file test_rsa_sign.c
 * @author Kealan McCusker
 * @brief Test RSA signature
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

  gcc -std=c99 -g ./test_rsa_sign.c -I/opt/amcl/include -L/opt/amcl/lib -lx509 -lrsa -lecdh -lamcl -o test_rsa_sign

*/

#include "rsa.h"
#include "ecdh.h"
#include "x509.h"
#include "utils.h"
#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include "randapi.h"

typedef enum { false, true } bool;

#define RSA 2
#define H160 1
#define H256 2
#define H384 3
#define H512 4

#define LINE_LEN 10000

#define MAXMODBYTES 66
#define MAXFFLEN 16

static char sig[MAXMODBYTES*MAXFFLEN];
static octet SIG= {0,sizeof(sig),sig};

static char sig2[MAXMODBYTES*MAXFFLEN];
static octet SIG2= {0,sizeof(sig2),sig2};

static char cakey[MAXMODBYTES*MAXFFLEN];
static octet CAKEY= {0,sizeof(cakey),cakey};

static char h[5000];
static octet H= {0,sizeof(h),h};

static char hh[5000];
static octet HH= {0,sizeof(hh),hh};


static void print_out(char *des,octet *c,int index,int len)
{
    int i;
    printf("%s [",des);
    for (i=0; i<len; i++)
        printf("%c",c->val[index+i]);
    printf("]\n");
}

int test_rsa_sign(int argc, char** argv)
{
    if (argc != 2)
    {
        printf("usage: ./test_rsa_sign [path to test vector file]\n");
        exit(EXIT_FAILURE);
    }
    int sha;
    pktype st,ca;
    int rc;
    FILE * fp = NULL;
    char line[LINE_LEN];
    char * linePtr = NULL;
    int l1=0;
    const char* CERTStr = "CERT = ";

    fp = fopen(argv[1], "r");
    if (fp == NULL)
    {
        printf("ERROR opening test vector file\n");
        exit(EXIT_FAILURE);
    }

    rsa_public_key pub;
    rsa_private_key priv;

    // Prime p:
    char p[RFS/2];
    octet P= {sizeof(p),sizeof(p),p};
    const char* PStr = "p = ";

    // Prime q:
    char q[RFS/2];
    octet Q= {sizeof(q),sizeof(q),q};
    const char* QStr = "q = ";

    bool readLine = false;
    int i=0;
    while (fgets(line, LINE_LEN, fp) != NULL)
    {
        i++;
        readLine = true;
        if (!strncmp(line, PStr, strlen(PStr)))
        {
#ifdef DEBUG
            printf("line %d %s\n", i,line);
#endif
            // Find hex value in string
            linePtr = line + strlen(PStr);

            // p binary value
            l1 = strlen(linePtr)-1;
            amcl_hex2bin(linePtr, P.val, l1);
        }

        if (!strncmp(line, QStr, strlen(QStr)))
        {
#ifdef DEBUG
            printf("line %d %s\n", i,line);
#endif
            // Find hex value in string
            linePtr = line + strlen(QStr);

            // q binary value
            l1 = strlen(linePtr)-1;
            amcl_hex2bin(linePtr, Q.val, l1);
        }

        // Self-Signed CA cert
        if (!strncmp(line, CERTStr, strlen(CERTStr)))
        {
#ifdef DEBUG
            printf("line %d %s\n", i,line);
#endif
            // Find base64 value in string
            char io[5000];
            octet IO= {0,sizeof(io),io};
            linePtr = line + strlen(CERTStr);
            l1 = strlen(linePtr);
            char* ca_b64 = (char*) calloc (l1,sizeof(char));
            strncpy(ca_b64,linePtr,l1-1);
            OCT_frombase64(&IO,ca_b64);

#ifdef DEBUG
            printf("CA Self-Signed Cert= \n");
            OCT_output(&IO);
            printf("\n");
#endif

            free(ca_b64);
            ca_b64 = NULL;

            // returns signature type
            st=X509_extract_cert_sig(&IO,&SIG);

#ifdef DEBUG
            printf("SIG= \n");
            OCT_output(&SIG);
            printf("\n");
#endif

            // Extract Cert from signed Cert
            X509_extract_cert(&IO,&H);

#ifdef DEBUG
            printf("Cert= \n");
            OCT_output(&H);
            printf("\n");

            int c;
            c=X509_extract_cert(&IO,&H);

            // Print email
            int ic,len;
            char en[9]= {0x2a,0x86,0x48,0x86,0xf7,0x0d,0x01,0x09,0x01};
            octet EN= {9,sizeof(en),en};
            printf("Issuer Details\n");
            ic=X509_find_issuer(&H);
            c=X509_find_entity_property(&H,&EN,ic,&len);
            print_out("email=",&H,c,len);
            printf("\n");
#endif

            ca=X509_extract_public_key(&H,&CAKEY);

            if (ca.type==0)
            {
                printf("Not supported by library\n");
                exit(EXIT_FAILURE);
            }
            if (ca.type!=st.type)
            {
                printf("Not self-signed\n");
                exit(EXIT_FAILURE);
            }

#ifdef DEBUG
            printf("EXTRACTED RSA PUBLIC KEY= \n");
            OCT_output(&CAKEY);
            printf("\n");
#endif

            // Assign public key
            pub.e=65537;
            FF_fromOctet(pub.n,&CAKEY,FFLEN);

#ifdef DEBUG
            printf("pub.n ");
            FF_output(pub.n,FFLEN);
            printf("\n");
#endif

            // Checking Self-Signed Signature
            sha=0;
            if (st.hash==H256) sha=SHA256;
            if (st.hash==H384) sha=SHA384;
            if (st.hash==H512) sha=SHA512;
            if (st.hash==0)
            {
                printf("Hash Function not supported\n");
                exit(EXIT_FAILURE);
            }
            char mp[RFS];
            octet MP= {0,sizeof(mp),mp};
            PKCS15(sha,&H,&MP);
            RSA_ENCRYPT(&pub,&SIG,&HH);

#ifdef DEBUG
            printf("MP ");
            OCT_output(&MP);
            printf("\n");
#endif

            rc = OCT_comp(&MP,&HH);
            if (!rc)
            {
                printf("TEST RSA VERIFICATION FAILED LINE %d\n",i);
                exit(EXIT_FAILURE);
            }

            // Generating public/private key pair from p amd q
            RSA_KEY_PAIR(NULL,65537,&priv,&pub,&P,&Q);

#ifdef DEBUG
            printf("priv.p ");
            FF_output(priv.p,FFLEN/2);
            printf("\n");
            printf("priv.q ");
            FF_output(priv.q,FFLEN/2);
            printf("\n");
            printf("priv.dp ");
            FF_output(priv.dp,FFLEN/2);
            printf("\n");
            printf("priv.dq ");
            FF_output(priv.dq,FFLEN/2);
            printf("\n");
            printf("priv.c ");
            FF_output(priv.c,FFLEN/2);
            printf("\n");
#endif

            char hp[RFS];
            octet HP= {0,sizeof(hp),hp};

            // Sign message
            PKCS15(sha,&H,&HP);
            RSA_DECRYPT(&priv,&HP,&SIG2);

#ifdef DEBUG
            printf("HP= ");
            OCT_output(&HP);
            printf("\r\n");

            printf("SIG2= ");
            OCT_output(&SIG2);
            printf("\r\n");
#endif
            rc = OCT_comp(&SIG,&SIG2);
            if (!rc)
            {
                printf("TEST RSA SIGNING FAILED LINE %d\n",i);
                exit(EXIT_FAILURE);
            }
        }
    }
    fclose(fp);
    if (!readLine)
    {
        printf("ERROR Empty test vector file\n");
        exit(EXIT_FAILURE);
    }
    printf("SUCCESS TEST RSA SIGNATURE PASSED\n");
    exit(EXIT_SUCCESS);
}
