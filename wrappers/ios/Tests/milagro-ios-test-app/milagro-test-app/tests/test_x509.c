/**
 * @file test_x509.c
 * @author Kealan McCusker
 * @brief Test x509 functions
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

#include "rsa.h"
#include "ecdh.h"
#include "x509.h"
#include "utils.h"
#include <stdio.h>
#include <string.h>
#include <stdlib.h>

typedef enum { false, true } bool;

//#define DEBUG

#define ECC 1
#define RSA 2
#define H160 1
#define H256 2
#define H384 3
#define H512 4

#define LINE_LEN 10000
#define MAX_STRING 300

#define MAXMODBYTES 72
#define MAXFFLEN 16

static char sig[MAXMODBYTES*MAXFFLEN];
static octet SIG= {0,sizeof(sig),sig};

static char r[MAXMODBYTES];
static octet R= {0,sizeof(r),r};

static char s[MAXMODBYTES];
static octet S= {0,sizeof(s),s};

static char cakey[MAXMODBYTES*MAXFFLEN];
static octet CAKEY= {0,sizeof(cakey),cakey};

static char certkey[MAXMODBYTES*MAXFFLEN];
static octet CERTKEY= {0,sizeof(certkey),certkey};

static char h[5000];
static octet H= {0,sizeof(h),h};

static char hh[5000];
static octet HH= {0,sizeof(hh),hh};

static char hp[RFS];
static octet HP= {0,sizeof(hp),hp};

// countryName
static char cn[3]= {0x55,0x04,0x06};
static octet CN= {3,sizeof(cn),cn};

// stateName
static char sn[3]= {0x55,0x04,0x08};
static octet SN= {3,sizeof(sn),sn};

// localName
static char ln[3]= {0x55,0x04,0x07};
static octet LN= {3,sizeof(ln),ln};

// orgName
static char on[3]= {0x55,0x04,0x0A};
static octet ON= {3,sizeof(on),on};

// unitName
static char un[3]= {0x55,0x04,0x0B};
static octet UN= {3,sizeof(un),un};

// myName
static char mn[3]= {0x55,0x04,0x03};
static octet MN= {3,sizeof(mn),mn};

// emailName
static char en[9]= {0x2a,0x86,0x48,0x86,0xf7,0x0d,0x01,0x09,0x01};
static octet EN= {9,sizeof(en),en};

extern void print_out(char *des,octet *c,int index,int len);
extern void print_date(char *des,octet *c,int index);

static int compare_data(octet *cert,octet *data,int index)
{
    int i;
    for (i=0; i<data->len; i++)
    {
        if (cert->val[index+i]!=data->val[i])
        {
            return 0;
        }
    }
    return 1;
}

int test_x509(int argc, char** argv)
{
    if (argc != 2)
    {
        printf("usage: ./test_x509 [path to test vector file]\n");
        exit(EXIT_FAILURE);
    }
    int sha;
    pktype st,pt;
    pktype ca = {0,0,0};
    FILE * fp = NULL;
    char line[LINE_LEN];
    char * linePtr = NULL;
    int l1=0;
    const char* CAStr = "CA = ";
    const char* CERTStr = "CERT = ";

    char issuerc[MAX_STRING];
    octet IssuerCOct= {0,MAX_STRING,issuerc};
    const char* IssuerCStr = "IssuerC = ";

    char issuerst[MAX_STRING];
    octet IssuerSTOct= {0,MAX_STRING,issuerst};
    const char* IssuerSTStr = "IssuerST = ";

    char issuerl[MAX_STRING];
    octet IssuerLOct= {0,MAX_STRING,issuerl};
    const char* IssuerLStr = "IssuerL = ";

    char issuero[MAX_STRING];
    octet IssuerOOct= {0,MAX_STRING,issuero};
    const char* IssuerOStr = "IssuerO = ";

    char issuerou[MAX_STRING];
    octet IssuerOUOct= {0,MAX_STRING,issuerou};
    const char* IssuerOUStr = "IssuerOU = ";

    char issuercn[MAX_STRING];
    octet IssuerCNOct= {0,MAX_STRING,issuercn};
    const char* IssuerCNStr = "IssuerCN = ";

    char issueremailaddress[MAX_STRING];
    octet IssuerEmailAddressOct= {0,MAX_STRING,issueremailaddress};
    const char* IssuerEmailAddressStr = "IssuerEmailAddress = ";


    char subjectc[MAX_STRING];
    octet SubjectCOct= {0,MAX_STRING,subjectc};
    const char* SubjectCStr = "SubjectC = ";

    char subjectst[MAX_STRING];
    octet SubjectSTOct= {0,MAX_STRING,subjectst};
    const char* SubjectSTStr = "SubjectST = ";

    char subjectl[MAX_STRING];
    octet SubjectLOct= {0,MAX_STRING,subjectl};
    const char* SubjectLStr = "SubjectL = ";

    char subjecto[MAX_STRING];
    octet SubjectOOct= {0,MAX_STRING,subjecto};
    const char* SubjectOStr = "SubjectO = ";

    char subjectou[MAX_STRING];
    octet SubjectOUOct= {0,MAX_STRING,subjectou};
    const char* SubjectOUStr = "SubjectOU = ";

    char subjectcn[MAX_STRING];
    octet SubjectCNOct= {0,MAX_STRING,subjectcn};
    const char* SubjectCNStr = "SubjectCN = ";

    char subjectemailaddress[MAX_STRING];
    octet SubjectEmailAddressOct= {0,MAX_STRING,subjectemailaddress};
    const char* SubjectEmailAddressStr = "SubjectEmailAddress = ";

    char vf[MAX_STRING];
    octet vfOct= {0,MAX_STRING,vf};
    const char* vfStr = "vf = ";

    char vt[MAX_STRING];
    octet vtOct= {0,MAX_STRING,vt};
    const char* vtStr = "vt = ";

    char cert_pk[512];
    octet CERT_PKOct= {0,sizeof(cert_pk),cert_pk};
    const char* CERT_PKStr = "CERT_PK = ";

    fp = fopen(argv[1], "r");
    if (fp == NULL)
    {
        printf("ERROR opening test vector file\n");
        exit(EXIT_FAILURE);
    }

    rsa_public_key PK;

    bool readLine = false;
    int i=0;
    while (fgets(line, LINE_LEN, fp) != NULL)
    {
        i++;
        readLine = true;

        if (!strncmp(line,  IssuerCStr, strlen(IssuerCStr)))
        {
#ifdef DEBUG
            printf("line %d %s\n", i,line);
#endif
            linePtr = line + strlen(IssuerCStr);
            OCT_clear(&IssuerCOct);
            OCT_jstring(&IssuerCOct,linePtr);
            IssuerCOct.len -= 1;

#ifdef DEBUG
            printf("IssuerCOct Hex: ");
            OCT_output(&IssuerCOct);
            printf("IssuerCOct ASCII: ");
            OCT_output_string(&IssuerCOct);
            printf("\n");
#endif
        }

        if (!strncmp(line,  IssuerSTStr, strlen(IssuerSTStr)))
        {
#ifdef DEBUG
            printf("line %d %s\n", i,line);
#endif
            linePtr = line + strlen(IssuerSTStr);
            OCT_clear(&IssuerSTOct);
            OCT_jstring(&IssuerSTOct,linePtr);
            IssuerSTOct.len -= 1;

#ifdef DEBUG
            printf("IssuerSTOct Hex: ");
            OCT_output(&IssuerSTOct);
            printf("IssuerSTOct ASCII: ");
            OCT_output_string(&IssuerSTOct);
            printf("\n");
#endif
        }

        if (!strncmp(line,  IssuerLStr, strlen(IssuerLStr)))
        {
#ifdef DEBUG
            printf("line %d %s\n", i,line);
#endif
            linePtr = line + strlen(IssuerLStr);
            OCT_clear(&IssuerLOct);
            OCT_jstring(&IssuerLOct,linePtr);
            IssuerLOct.len -= 1;

#ifdef DEBUG
            printf("IssuerLOct Hex: ");
            OCT_output(&IssuerLOct);
            printf("IssuerLOct ASCII: ");
            OCT_output_string(&IssuerLOct);
            printf("\n");
#endif
        }

        if (!strncmp(line,  IssuerOStr, strlen(IssuerOStr)))
        {
#ifdef DEBUG
            printf("line %d %s\n", i,line);
#endif
            linePtr = line + strlen(IssuerOStr);
            OCT_clear(&IssuerOOct);
            OCT_jstring(&IssuerOOct,linePtr);
            IssuerOOct.len -= 1;

#ifdef DEBUG
            printf("IssuerOOct Hex: ");
            OCT_output(&IssuerOOct);
            printf("IssuerOOct ASCII: ");
            OCT_output_string(&IssuerOOct);
            printf("\n");
#endif
        }

        if (!strncmp(line,  IssuerOUStr, strlen(IssuerOUStr)))
        {
#ifdef DEBUG
            printf("line %d %s\n", i,line);
#endif
            linePtr = line + strlen(IssuerOUStr);
            OCT_clear(&IssuerOUOct);
            OCT_jstring(&IssuerOUOct,linePtr);
            IssuerOUOct.len -= 1;

#ifdef DEBUG
            printf("IssuerOUOct Hex: ");
            OCT_output(&IssuerOUOct);
            printf("IssuerOUOct ASCII: ");
            OCT_output_string(&IssuerOUOct);
            printf("\n");
#endif
        }

        if (!strncmp(line,  IssuerCNStr, strlen(IssuerCNStr)))
        {
#ifdef DEBUG
            printf("line %d %s\n", i,line);
#endif
            linePtr = line + strlen(IssuerCNStr);
            OCT_clear(&IssuerCNOct);
            OCT_jstring(&IssuerCNOct,linePtr);
            IssuerCNOct.len -= 1;

#ifdef DEBUG
            printf("IssuerCNOct Hex: ");
            OCT_output(&IssuerCNOct);
            printf("IssuerCNOct ASCII: ");
            OCT_output_string(&IssuerCNOct);
            printf("\n");
#endif
        }

        if (!strncmp(line, IssuerEmailAddressStr, strlen(IssuerEmailAddressStr)))
        {
#ifdef DEBUG
            printf("line %d %s\n", i,line);
#endif
            linePtr = line + strlen(IssuerEmailAddressStr);
            OCT_clear(&IssuerEmailAddressOct);
            OCT_jstring(&IssuerEmailAddressOct,linePtr);
            IssuerEmailAddressOct.len -= 1;

#ifdef DEBUG
            printf("IssuerEmailAddressOct Hex: ");
            OCT_output(&IssuerEmailAddressOct);
            printf("IssuerEmailAddressOct ASCII: ");
            OCT_output_string(&IssuerEmailAddressOct);
            printf("\n");
#endif
        }

        if (!strncmp(line,  SubjectCStr, strlen(SubjectCStr)))
        {
#ifdef DEBUG
            printf("line %d %s\n", i,line);
#endif
            linePtr = line + strlen(SubjectCStr);
            OCT_clear(&SubjectCOct);
            OCT_jstring(&SubjectCOct,linePtr);
            SubjectCOct.len -= 1;

#ifdef DEBUG
            printf("SubjectCOct Hex: ");
            OCT_output(&SubjectCOct);
            printf("SubjectCOct ASCII: ");
            OCT_output_string(&SubjectCOct);
            printf("\n");
#endif
        }

        if (!strncmp(line,  SubjectSTStr, strlen(SubjectSTStr)))
        {
#ifdef DEBUG
            printf("line %d %s\n", i,line);
#endif
            linePtr = line + strlen(SubjectSTStr);
            OCT_clear(&SubjectSTOct);
            OCT_jstring(&SubjectSTOct,linePtr);
            SubjectSTOct.len -= 1;

#ifdef DEBUG
            printf("SubjectSTOct Hex: ");
            OCT_output(&SubjectSTOct);
            printf("SubjectSTOct ASCII: ");
            OCT_output_string(&SubjectSTOct);
            printf("\n");
#endif
        }

        if (!strncmp(line,  SubjectLStr, strlen(SubjectLStr)))
        {
#ifdef DEBUG
            printf("line %d %s\n", i,line);
#endif
            linePtr = line + strlen(SubjectLStr);
            OCT_clear(&SubjectLOct);
            OCT_jstring(&SubjectLOct,linePtr);
            SubjectLOct.len -= 1;

#ifdef DEBUG
            printf("SubjectLOct Hex: ");
            OCT_output(&SubjectLOct);
            printf("SubjectLOct ASCII: ");
            OCT_output_string(&SubjectLOct);
            printf("\n");
#endif
        }

        if (!strncmp(line,  SubjectOStr, strlen(SubjectOStr)))
        {
#ifdef DEBUG
            printf("line %d %s\n", i,line);
#endif
            linePtr = line + strlen(SubjectOStr);
            OCT_clear(&SubjectOOct);
            OCT_jstring(&SubjectOOct,linePtr);
            SubjectOOct.len -= 1;

#ifdef DEBUG
            printf("SubjectOOct Hex: ");
            OCT_output(&SubjectOOct);
            printf("SubjectOOct ASCII: ");
            OCT_output_string(&SubjectOOct);
            printf("\n");
#endif
        }

        if (!strncmp(line,  SubjectOUStr, strlen(SubjectOUStr)))
        {
#ifdef DEBUG
            printf("line %d %s\n", i,line);
#endif
            linePtr = line + strlen(SubjectOUStr);
            OCT_clear(&SubjectOUOct);
            OCT_jstring(&SubjectOUOct,linePtr);
            SubjectOUOct.len -= 1;

#ifdef DEBUG
            printf("SubjectOUOct Hex: ");
            OCT_output(&SubjectOUOct);
            printf("SubjectOUOct ASCII: ");
            OCT_output_string(&SubjectOUOct);
            printf("\n");
#endif
        }

        if (!strncmp(line,  SubjectCNStr, strlen(SubjectCNStr)))
        {
#ifdef DEBUG
            printf("line %d %s\n", i,line);
#endif
            linePtr = line + strlen(SubjectCNStr);
            OCT_clear(&SubjectCNOct);
            OCT_jstring(&SubjectCNOct,linePtr);
            SubjectCNOct.len -= 1;

#ifdef DEBUG
            printf("SubjectCNOct Hex: ");
            OCT_output(&SubjectCNOct);
            printf("SubjectCNOct ASCII: ");
            OCT_output_string(&SubjectCNOct);
            printf("\n");
#endif
        }

        if (!strncmp(line, SubjectEmailAddressStr, strlen(SubjectEmailAddressStr)))
        {
#ifdef DEBUG
            printf("line %d %s\n", i,line);
#endif
            linePtr = line + strlen(SubjectEmailAddressStr);
            OCT_clear(&SubjectEmailAddressOct);
            OCT_jstring(&SubjectEmailAddressOct,linePtr);
            SubjectEmailAddressOct.len -= 1;

#ifdef DEBUG
            printf("SubjectEmailAddressOct Hex: ");
            OCT_output(&SubjectEmailAddressOct);
            printf("SubjectEmailAddressOct ASCII: ");
            OCT_output_string(&SubjectEmailAddressOct);
            printf("\n");
#endif
        }

        if (!strncmp(line, vfStr, strlen(vfStr)))
        {
#ifdef DEBUG
            printf("line %d %s\n", i,line);
#endif
            linePtr = line + strlen(vfStr);
            OCT_clear(&vfOct);
            OCT_jstring(&vfOct,linePtr);
            vfOct.len -= 1;

#ifdef DEBUG
            printf("vfOct Hex: ");
            OCT_output(&vfOct);
            printf("vfOct ASCII: ");
            OCT_output_string(&vfOct);
            printf("\n");
#endif
        }

        if (!strncmp(line, vtStr, strlen(vtStr)))
        {
#ifdef DEBUG
            printf("line %d %s\n", i,line);
#endif
            linePtr = line + strlen(vtStr);
            OCT_clear(&vtOct);
            OCT_jstring(&vtOct,linePtr);
            vtOct.len -= 1;

#ifdef DEBUG
            printf("vtOct Hex: ");
            OCT_output(&vtOct);
            printf("vtOct ASCII: ");
            OCT_output_string(&vtOct);
            printf("\n");
#endif
        }

        if (!strncmp(line, CERT_PKStr, strlen(CERT_PKStr)))
        {
#ifdef DEBUG
            printf("line %d %s\n", i,line);
#endif
            // Find hex value in string
            linePtr = line + strlen(CERT_PKStr);

            // p binary value
            l1 = strlen(linePtr)-1;
            amcl_hex2bin(linePtr, CERT_PKOct.val, l1);
            CERT_PKOct.len = l1/2;

#ifdef DEBUG
            printf("CERT_PKOct Hex: ");
            OCT_output(&CERT_PKOct);
            printf("\n");
#endif
        }


        // Self-Signed CA cert
        if (!strncmp(line, CAStr, strlen(CAStr)))
        {
#ifdef DEBUG
            printf("line %d %s\n", i,line);
#endif
            // Find base64 value in string
            char io[5000];
            octet IO= {0,sizeof(io),io};
            linePtr = line + strlen(CAStr);
            l1 = strlen(linePtr);
            char* ca_b64 = (char*) calloc (l1,sizeof(char));
            strncpy(ca_b64,linePtr,l1-1);
            OCT_frombase64(&IO,ca_b64);

#ifdef DEBUG
            printf("CA Self-Signed Cert: \n");
            OCT_output(&IO);
            printf("\n");
#endif

            free(ca_b64);
            ca_b64 = NULL;

            // returns signature type
            st=X509_extract_cert_sig(&IO,&SIG);

            if (st.type==0)
            {
                printf("Unable to extract self-signed cert signature\r\n");
            }

            if (st.type==ECC)
            {
                OCT_chop(&SIG,&S,SIG.len/2);
                OCT_copy(&R,&SIG);
                // printf("SIG: ");
                // OCT_output(&R);
                // printf("\r\n");
                // OCT_output(&S);
                // printf("\r\n");
            }

            if (st.type==RSA)
            {
                //printf("SIG: ");
                //OCT_output(&SIG);
                //printf("\r\n");
            }

            // Extract Cert from signed Cert
            X509_extract_cert(&IO,&H);

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

            if (ca.type==ECC)
            {
                // printf("EXTRACTED ECC CA PUBLIC KEY: ");
                // OCT_output(&CAKEY);
                // printf("\n");
            }
            if (ca.type==RSA)
            {
                // printf("EXTRACTED RSA CA PUBLIC KEY:  ");
                //  OCT_output(&CAKEY);
                //  printf("\n");
            }

            // Cert is self-signed - so check signature
            // printf("Checking Self-Signed Signature\r\n");
            if (ca.type==ECC)
            {
                if (ca.curve!=CHOICE)
                {
                    printf("TEST X509 ERROR CURVE IS NOT SUPPORTED LINE %d\n",i);
                    exit(EXIT_FAILURE);
                }
                int res=ECP_PUBLIC_KEY_VALIDATE(1,&CAKEY);
                if (res!=0)
                {
                    printf("TEST X509 ERROR PUBLIC KEY IS INVALID LINE %d\n",i);
                    exit(EXIT_FAILURE);
                }

                sha=0;
                if (st.hash==H256) sha=SHA256;
                if (st.hash==H384) sha=SHA384;
                if (st.hash==H512) sha=SHA512;
                if (st.hash==0)
                {
                    printf("TEST X509 ERROR HASH FUNCTION NOT SUPPORTED LINE %d\n",i);
                    exit(EXIT_FAILURE);
                }

                if (ECPVP_DSA(sha,&CAKEY,&H,&R,&S)!=0)
                {
                    printf("X509 ERROR ECDSA VERIFICATION FAILED LINE %d\n",i);
                    exit(EXIT_FAILURE);
                }
            }

            if (ca.type==RSA)
            {
                PK.e=65537; // assuming this!
                FF_fromOctet(PK.n,&CAKEY,FFLEN);

                sha=0;
                if (st.hash==H256) sha=SHA256;
                if (st.hash==H384) sha=SHA384;
                if (st.hash==H512) sha=SHA512;
                if (st.hash==0)
                {
                    printf("TEST X509 ERROR Hash Function not supported LINE %d\n",i);
                    exit(EXIT_FAILURE);
                }
                PKCS15(sha,&H,&HP);

                RSA_ENCRYPT(&PK,&SIG,&HH);
                if (!OCT_comp(&HP,&HH))
                {
                    printf("TEST X509 ERROR RSA VERIFICATION FAILED LINE %d\n",i);
                    exit(EXIT_FAILURE);
                }
            }
        }

        /////////// CA Signed cert /////////////////
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
            char* cert_b64 = (char*) calloc (l1,sizeof(char));
            strncpy(cert_b64,linePtr,l1-1);
            OCT_frombase64(&IO,cert_b64);

#ifdef DEBUG
            printf("CA Signed Cert: \n");
            OCT_output(&IO);
            printf("\n");
#endif

            free(cert_b64);
            cert_b64 = NULL;

            // returns signature type
            st=X509_extract_cert_sig(&IO,&SIG);

            if (st.type==0)
            {
                printf("TEST X509 ERROR UNABLE TO CHECK CERT SIGNATURE LINE %d\n",i);
                exit(EXIT_FAILURE);
            }

            if (st.type==ECC)
            {
                OCT_chop(&SIG,&S,SIG.len/2);
                OCT_copy(&R,&SIG);
#ifdef DEBUG
                printf("ECC SIG: ");
                OCT_output(&R);
                printf("\r\n");
                OCT_output(&S);
                printf("\r\n");
#endif
            }

#ifdef DEBUG
            if (st.type==RSA)
            {
                printf("RSA SIG: ");
                OCT_output(&SIG);
                printf("\r\n");
            }
#endif

            // Extract Cert
            int c;
            c=X509_extract_cert(&IO,&H);

#ifdef DEBUG
            printf("Cert: ");
            OCT_output(&H);
            printf("\n");
#endif

            // Check Details
            int ic,len;
            // Issuer Details
            ic=X509_find_issuer(&H);

            c=X509_find_entity_property(&H,&CN,ic,&len);
#ifdef DEBUG
            print_out("countryName: ",&H,c,len);
            printf("\n");
#endif

            if (!compare_data(&H,&IssuerCOct,c))
            {
                printf("TEST X509 ERROR IssuerC LINE %d\n",i);
                exit(EXIT_FAILURE);
            }

            c=X509_find_entity_property(&H,&SN,ic,&len);
#ifdef DEBUG
            print_out("stateOrProvinceName: ",&H,c,len);
            printf("\n");
#endif

            if (!compare_data(&H,&IssuerSTOct,c))
            {
                printf("TEST X509 ERROR IssuerST LINE %d\n",i);
                exit(EXIT_FAILURE);
            }

            c=X509_find_entity_property(&H,&LN,ic,&len);
#ifdef DEBUG
            print_out("localityName: ",&H,c,len);
            printf("\n");
#endif

            if (!compare_data(&H,&IssuerLOct,c))
            {
                printf("TEST X509 ERROR IssuerL LINE %d\n",i);
                exit(EXIT_FAILURE);
            }

            c=X509_find_entity_property(&H,&ON,ic,&len);
#ifdef DEBUG
            print_out("organizationName: ",&H,c,len);
            printf("\n");
#endif

            if (!compare_data(&H,&IssuerOOct,c))
            {
                printf("TEST X509 ERROR IssuerO LINE %d\n",i);
                exit(EXIT_FAILURE);
            }

            c=X509_find_entity_property(&H,&UN,ic,&len);
#ifdef DEBUG
            print_out("organizationalUnitName: ",&H,c,len);
            printf("\n");
#endif

            if (!compare_data(&H,&IssuerOUOct,c))
            {
                printf("TEST X509 ERROR IssuerOU LINE %d\n",i);
                exit(EXIT_FAILURE);
            }

            c=X509_find_entity_property(&H,&MN,ic,&len);
#ifdef DEBUG
            print_out("commonName: ",&H,c,len);
            printf("\n");
#endif

            if (!compare_data(&H,&IssuerCNOct,c))
            {
                printf("TEST X509 ERROR IssuerCN LINE %d\n",i);
                exit(EXIT_FAILURE);
            }

            c=X509_find_entity_property(&H,&EN,ic,&len);
#ifdef DEBUG
            print_out("emailAddress: ",&H,c,len);
            printf("\n");
#endif

            if (!compare_data(&H,&IssuerEmailAddressOct,c))
            {
                printf("TEST X509 ERROR IssuerEmailAddress LINE %d\n",i);
                exit(EXIT_FAILURE);
            }

            // Subject details
#ifdef DEBUG
            printf("Subject Details\n");
#endif
            ic=X509_find_subject(&H);

            c=X509_find_entity_property(&H,&CN,ic,&len);
#ifdef DEBUG
            print_out("countryName: ",&H,c,len);
            printf("\n");
#endif

            if (!compare_data(&H,&SubjectCOct,c))
            {
                printf("TEST X509 ERROR SubjectC LINE %d\n",i);
                exit(EXIT_FAILURE);
            }

            c=X509_find_entity_property(&H,&SN,ic,&len);
#ifdef DEBUG
            print_out("stateOrProvinceName: ",&H,c,len);
            printf("\n");
#endif

            if (!compare_data(&H,&SubjectSTOct,c))
            {
                printf("TEST X509 ERROR SubjectST LINE %d\n",i);
                exit(EXIT_FAILURE);
            }

            c=X509_find_entity_property(&H,&LN,ic,&len);
#ifdef DEBUG
            print_out("localityName: ",&H,c,len);
            printf("\n");
#endif

            if (!compare_data(&H,&SubjectLOct,c))
            {
                printf("TEST X509 ERROR SubjectL LINE %d\n",i);
                exit(EXIT_FAILURE);
            }

            c=X509_find_entity_property(&H,&ON,ic,&len);
#ifdef DEBUG
            print_out("organizationName: ",&H,c,len);
            printf("\n");
#endif

            if (!compare_data(&H,&SubjectOOct,c))
            {
                printf("TEST X509 ERROR SubjectO LINE %d\n",i);
                exit(EXIT_FAILURE);
            }

            c=X509_find_entity_property(&H,&UN,ic,&len);
#ifdef DEBUG
            print_out("organizationalUnitName: ",&H,c,len);
            printf("\n");
#endif

            if (!compare_data(&H,&SubjectOUOct,c))
            {
                printf("TEST X509 ERROR SubjectOU LINE %d\n",i);
                exit(EXIT_FAILURE);
            }

            c=X509_find_entity_property(&H,&MN,ic,&len);
#ifdef DEBUG
            print_out("commonName: ",&H,c,len);
            printf("\n");
#endif

            if (!compare_data(&H,&SubjectCNOct,c))
            {
                printf("TEST X509 ERROR SubjectCN LINE %d\n",i);
                exit(EXIT_FAILURE);
            }

            c=X509_find_entity_property(&H,&EN,ic,&len);
#ifdef DEBUG
            print_out("emailAddress: ",&H,c,len);
            printf("\n");
#endif

            if (!compare_data(&H,&SubjectEmailAddressOct,c))
            {
                printf("TEST X509 ERROR SubjectEmailAddress LINE %d\n",i);
                exit(EXIT_FAILURE);
            }

            ic=X509_find_validity(&H);
            c=X509_find_start_date(&H,ic);
#ifdef DEBUG
            print_date("start date: ",&H,c);
            printf("\n");
#endif

            if (!compare_data(&H,&vfOct,c))
            {
                printf("TEST X509 ERROR VALID FROM LINE %d\n",i);
                exit(EXIT_FAILURE);
            }

            c=X509_find_expiry_date(&H,ic);
#ifdef DEBUG
            print_date("expiry date: ",&H,c);
            printf("\n");
#endif

            if (!compare_data(&H,&vtOct,c))
            {
                printf("TEST X509 ERROR VALID TO LINE %d\n",i);
                exit(EXIT_FAILURE);
            }

            pt=X509_extract_public_key(&H,&CERTKEY);

            if (pt.type==0)
            {
                printf("TEST X509 ERROR NOT SUPPORTED BY LIBRARY LINE %d\n",i);
                exit(EXIT_FAILURE);
            }

#ifdef DEBUG
            if (pt.type==ECC)
            {
                printf("EXTRACTED ECC PUBLIC KEY: ");
                OCT_output(&CERTKEY);
                printf("\n");
            }
            if (pt.type==RSA)
            {
                printf("EXTRACTED RSA PUBLIC KEY:  ");
                OCT_output(&CERTKEY);
                printf("\n");
            }
#endif
            if (!compare_data(&CERTKEY,&CERT_PKOct,0))
            {
                printf("TEST X509 ERROR CERT PUBLIC KEY  LINE %d\n",i);
                exit(EXIT_FAILURE);
            }

            // Check CA signature
            // printf("Checking CA Signed Signature\n");

#ifdef DEBUG
            printf("CA PUBLIC KEY:  ");
            OCT_output(&CAKEY);
            printf("\n");
#endif

            if (ca.type==ECC)
            {
                if (ca.curve!=CHOICE)
                {
                    printf("TEST X509 ERROR CURVE IS NOT SUPPORTED LINE %d\n",i);
                    exit(EXIT_FAILURE);
                }
                int res=ECP_PUBLIC_KEY_VALIDATE(1,&CAKEY);
                if (res!=0)
                {
                    printf("TEST X509 ERROR PUBLIC KEY IS INVALID LINE %d\n",i);
                    exit(EXIT_FAILURE);
                }

                sha=0;
                if (st.hash==H256) sha=SHA256;
                if (st.hash==H384) sha=SHA384;
                if (st.hash==H512) sha=SHA512;
                if (st.hash==0)
                {
                    printf("TEST X509 ERROR HASH FUNCTION NOT SUPPORTED LINE %d\n",i);
                    exit(EXIT_FAILURE);
                }

                if (ECPVP_DSA(sha,&CAKEY,&H,&R,&S)!=0)
                {
                    printf("X509 ERROR ECDSA VERIFICATION FAILED LINE %d\n",i);
                    exit(EXIT_FAILURE);
                }
            }

            if (ca.type==RSA)
            {
                PK.e=65537; // assuming this!
                FF_fromOctet(PK.n,&CAKEY,FFLEN);

                sha=0;
                if (st.hash==H256) sha=SHA256;
                if (st.hash==H384) sha=SHA384;
                if (st.hash==H512) sha=SHA512;
                if (st.hash==0)
                {
                    printf("TEST X509 ERROR Hash Function not supported LINE %d\n",i);
                    exit(EXIT_FAILURE);
                }
                PKCS15(sha,&H,&HP);

                RSA_ENCRYPT(&PK,&SIG,&HH);
                if (!OCT_comp(&HP,&HH))
                {
                    printf("TEST X509 ERROR RSA VERIFICATION FAILED LINE %d\n",i);
                    exit(EXIT_FAILURE);
                }
            }
        }

    }
    fclose(fp);
    if (!readLine)
    {
        printf("X509 ERROR Empty test vector file\n");
        exit(EXIT_FAILURE);
    }
    printf("SUCCESS TEST X509 PASSED\n");
    exit(EXIT_SUCCESS);
}
