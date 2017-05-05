/**
 * @file test_mpinfull.c
 * @author Kealan McCusker
 * @brief Test M-pin Full
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

/* Test M-Pin Full */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include "mpin.h"
#include "randapi.h"

int test_mpinfull(int argc, char** argv)
{
    printf("test_mpinfull() started\n");
    if (argc != 2)
    {
        printf("usage: ./test_mpinfull [hash:sha256||sha384||sha512]\n");
        exit(EXIT_FAILURE);
    }

    int i,PIN1,PIN2,rtn;

    char id[256];
    octet ID = {0,sizeof(id),id};

    char x[PGS],y[PGS];
    octet X= {0,sizeof(x),x};
    octet Y= {0,sizeof(y),y};

    /* Master secret shares */
    char ms1[PGS], ms2[PGS];
    octet MS1= {0,sizeof(ms1),ms1};
    octet MS2= {0,sizeof(ms2),ms2};

    /* Hash values of client ID */
    char hcid[PFS];
    octet HCID= {0,sizeof(hcid), hcid};

    /* Hash values of messages */
    char hm[PFS];
    octet HM= {0,sizeof(hm), hm};

    /* Client secret and shares */
    char cs1[2*PFS+1], cs2[2*PFS+1], sec[2*PFS+1];
    octet SEC= {0,sizeof(sec),sec};
    octet CS1= {0,sizeof(cs1), cs1};
    octet CS2= {0,sizeof(cs2), cs2};

    /* Server secret and shares */
    char ss1[4*PFS], ss2[4*PFS], serverSecret[4*PFS];
    octet ServerSecret= {0,sizeof(serverSecret),serverSecret};
    octet SS1= {0,sizeof(ss1),ss1};
    octet SS2= {0,sizeof(ss2),ss2};

    /* Time Permit and shares */
    char tp1[2*PFS+1], tp2[2*PFS+1], tp[2*PFS+1];
    octet TP= {0,sizeof(tp),tp};
    octet TP1= {0,sizeof(tp1),tp1};
    octet TP2= {0,sizeof(tp2),tp2};

    /* Token stored on device */
    char token[2*PFS+1];
    octet TOKEN= {0,sizeof(token),token};

    /* Precomputed values stored on device */
    char g1[12*PFS],g2[12*PFS];
    octet G1= {0,sizeof(g1),g1};
    octet G2= {0,sizeof(g2),g2};

    char ut[2*PFS+1],u[2*PFS+1];
    octet UT= {0,sizeof(ut),ut};
    octet U= {0,sizeof(u),u};

    char hid[2*PFS+1],htid[2*PFS+1];
    octet HID= {0,sizeof(hid),hid};
    octet HTID= {0,sizeof(htid),htid};

    char e[12*PFS], f[12*PFS];
    octet E= {0,sizeof(e),e};
    octet F= {0,sizeof(f),f};

    char r[PGS],z[2*PFS+1],w[PGS],t[2*PFS+1];

    char ck[PAS],sk[PAS];
    octet R= {0,sizeof(r),r};
    octet Z= {0,sizeof(z),z};
    octet W= {0,sizeof(w),w};
    octet T= {0,sizeof(t),t};
    octet SK= {0,sizeof(sk),sk};
    octet CK= {0,sizeof(ck),ck};

    /* AES-GCM */
    char raw[256], header[16], ciphertext[32], res[32], plaintext[32], tag[16], iv[16];
    octet HEADER= {0,0,header}, Ciphertext= {0,sizeof(ciphertext),ciphertext};
    octet Plaintext= {0,sizeof(plaintext),plaintext}, Res= {0,sizeof(res),res}, Tag= {0,sizeof(tag),tag}, IV= {0,sizeof(iv),iv};
    csprng rng;

    int hash;
    if (!strcmp(argv[1], "sha256"))
    {
        hash = SHA256;
    }
    else if (!strcmp(argv[1], "sha384"))
    {
        hash = SHA384;
    }
    else
    {
        hash = SHA512;
    }

    /* Fake random source */
    RAND_clean(&rng);
    for (i=0; i<256; i++) raw[i]=(char)i;
    RAND_seed(&rng,256,raw);

    PIN1 = 1234;
    PIN2 = 1234;

    /* Assign the End-User an ID */
    char* user = "testuser@miracl.com";
    OCT_jstring(&ID,user);
    //printf("CLIENT: ID %s\n", user);

    int date = 0;
    char seed[100] = {0};
    octet SEED = {0,sizeof(seed),seed};
    csprng RNG;

    /* unrandom seed value! */
    SEED.len=100;
    for (i=0; i<100; i++) SEED.val[i]=i+1;

    /* initialise random number generator */
    CREATE_CSPRNG(&RNG,&SEED);

    /* Hash ID */
    MPIN_HASH_ID(hash,&ID,&HCID);
    //OCT_output(&HCID);

    /* When set only send hashed IDs to server */
    octet *pID;
#ifdef USE_ANONYMOUS
    pID = &HCID;
#else
    pID = &ID;
#endif

    /* Generate Client master secret for MIRACL and Customer */
    rtn = MPIN_RANDOM_GENERATE(&RNG,&MS1);
    if (rtn != 0)
    {
        printf("MPIN_RANDOM_GENERATE(&RNG,&MS1) Error %d\n", rtn);
        return 1;
    }
    rtn = MPIN_RANDOM_GENERATE(&RNG,&MS2);
    if (rtn != 0)
    {
        printf("MPIN_RANDOM_GENERATE(&RNG,&MS2) Error %d\n", rtn);
        return 1;
    }
    //printf("MASTER SECRET MIRACL:= 0x");
    //OCT_output(&MS1);
    //printf("MASTER SECRET CUSTOMER:= 0x");
    //OCT_output(&MS2);

    /* Generate server secret shares */
    rtn = MPIN_GET_SERVER_SECRET(&MS1,&SS1);
    if (rtn != 0)
    {
        printf("MPIN_GET_SERVER_SECRET(&MS1,&SS1) Error %d\n", rtn);
        return 1;
    }
    rtn = MPIN_GET_SERVER_SECRET(&MS2,&SS2);
    if (rtn != 0)
    {
        printf("MPIN_GET_SERVER_SECRET(&MS2,&SS2) Error %d\n", rtn);
        return 1;
    }
    //printf("SS1 = 0x");
    //OCT_output(&SS1);
    //printf("SS2 = 0x");
    //OCT_output(&SS2);

    /* Combine server secret share */
    rtn = MPIN_RECOMBINE_G2(&SS1, &SS2, &ServerSecret);
    if (rtn != 0)
    {
        printf("MPIN_RECOMBINE_G2(&SS1, &SS2, &ServerSecret) Error %d\n", rtn);
        return 1;
    }
    //printf("ServerSecret = 0x");
    //OCT_output(&ServerSecret);

    /* Generate client secret shares */
    rtn = MPIN_GET_CLIENT_SECRET(&MS1,&HCID,&CS1);
    if (rtn != 0)
    {
        printf("MPIN_GET_CLIENT_SECRET(&MS1,&HCID,&CS1) Error %d\n", rtn);
        return 1;
    }
    rtn = MPIN_GET_CLIENT_SECRET(&MS2,&HCID,&CS2);
    if (rtn != 0)
    {
        printf("MPIN_GET_CLIENT_SECRET(&MS2,&HCID,&CS2) Error %d\n", rtn);
        return 1;
    }
    //printf("CS1 = 0x");
    //OCT_output(&CS1);
    //printf("CS2 = 0x");
    //OCT_output(&CS2);

    /* Combine client secret shares : TOKEN is the full client secret */
    rtn = MPIN_RECOMBINE_G1(&CS1, &CS2, &TOKEN);
    if (rtn != 0)
    {
        printf("MPIN_RECOMBINE_G1(&CS1, &CS2, &TOKEN) Error %d\n", rtn);
        return 1;
    }
    //printf("Client Secret = 0x");
    //OCT_output(&TOKEN);

    /* Generate Time Permit shares */
    date = MPIN_today();
    //printf("Date %d \n", date);
    rtn = MPIN_GET_CLIENT_PERMIT(hash,date,&MS1,&HCID,&TP1);
    if (rtn != 0)
    {
        printf("MPIN_GET_CLIENT_PERMIT(hash,date,&MS1,&HCID,&TP1) Error %d\n", rtn);
        return 1;
    }
    rtn = MPIN_GET_CLIENT_PERMIT(hash,date,&MS2,&HCID,&TP2);
    if (rtn != 0)
    {
        printf("MPIN_GET_CLIENT_PERMIT(hash,date,&MS2,&HCID,&TP2) Error %d\n", rtn);
        return 1;
    }
    //printf("TP1 = 0x");
    //OCT_output(&TP1);
    //printf("TP2 = 0x");
    //OCT_output(&TP2);

    /* Combine Time Permit shares */
    rtn = MPIN_RECOMBINE_G1(&TP1, &TP2, &TP);
    if (rtn != 0)
    {
        printf("MPIN_RECOMBINE_G1(&TP1, &TP2, &TP) Error %d\n", rtn);
        return 1;
    }
    //printf("Time Permit = 0x");
    //OCT_output(&TP);

    /* This encoding makes Time permit look random */
    if (MPIN_ENCODING(&RNG,&TP)!=0) printf("Encoding error\n");
    //printf("Encoded Time Permit= ");
    //OCT_output(&TP);
    if (MPIN_DECODING(&TP)!=0) printf("Decoding error\n");
    //printf("Decoded Time Permit= ");
    //OCT_output(&TP);

    /* Client extracts PIN1 from secret to create Token */
    rtn = MPIN_EXTRACT_PIN(hash,&ID, PIN1, &TOKEN);
    if (rtn != 0)
    {
        printf("MPIN_EXTRACT_PIN( &ID, PIN, &TOKEN) Error %d\n", rtn);
        return 1;
    }
    //printf("Token = 0x");
    //OCT_output(&TOKEN);

    /* Client precomputation */
    MPIN_PRECOMPUTE(&TOKEN,&HCID,NULL,&G1,&G2);

    /* Client first pass */
    rtn = MPIN_CLIENT_1(hash,date,&ID,&RNG,&X,PIN2,&TOKEN,&SEC,&U,&UT,&TP);
    if (rtn != 0)
    {
        printf("MPIN_CLIENT_1 ERROR %d\n", rtn);
        return 1;
    }

    /* Client sends Z=r.ID to Server */
    MPIN_GET_G1_MULTIPLE(&RNG,1,&R,&HCID,&Z);

    /* Server calculates H(ID) and H(T|H(ID)) (if time permits enabled), and maps them to points on the curve HID and HTID resp. */
    MPIN_SERVER_1(hash,date,pID,&HID,&HTID);

    /* Server generates Random number Y and sends it to Client */
    rtn = MPIN_RANDOM_GENERATE(&RNG,&Y);
    if (rtn != 0)
    {
        printf("MPIN_RANDOM_GENERATE(&RNG,&Y) Error %d\n", rtn);
        return 1;
    }
    //printf("Y = 0x");
    //OCT_output(&Y);

    /* Server sends T=w.ID to client */
    MPIN_GET_G1_MULTIPLE(&RNG,0,&W,&HTID,&T);
    //printf("T = 0x");
    //OCT_output(&T);

    /* Client second pass */
    rtn = MPIN_CLIENT_2(&X,&Y,&SEC);
    if (rtn != 0)
    {
        printf("MPIN_CLIENT_2(&X,&Y,&SEC) Error %d\n", rtn);
    }
    //printf("V = 0x");
    //OCT_output(&SEC);

    /* Server second pass */
    rtn = MPIN_SERVER_2(date,NULL,&HTID,&Y,&ServerSecret,NULL,&UT,&SEC,&E,&F);
    if (rtn != 0)
    {
        printf("FAILURE Invalid Token Error Code %d\n", rtn);
    }

    MPIN_HASH_ALL(hash,&HCID,NULL,&UT,&SEC,&Y,&Z,&T,&HM);
    MPIN_CLIENT_KEY(hash,&G1,&G2,PIN2,&R,&X,&HM,&T,&CK);
    //printf("Client Key = ");
    //OCT_output(&CK);

    /* Server will use the hashed ID if anonymous connection required.
    MPIN_HASH_ID(hash,&ID,&HSID);
    MPIN_HASH_ALL(&HSID,NULL,&UT,&SEC,&Y,&Z,&T,&HM);
    */
    MPIN_SERVER_KEY(hash,&Z,&ServerSecret,&W,&HM,&HID,NULL,&UT,&SK);
    //printf("Server Key = ");
    //OCT_output(&SK);

    if (!OCT_comp(&CK,&SK))
    {
        printf("FAILURE Keys are different\n");
        return 1;
    }

    for (i=0; i<10; i++)
    {
        /* Self test AES-GCM encyption/decryption */
        OCT_rand(&IV,&rng,16);
        OCT_rand(&Plaintext,&rng,32);
        OCT_copy(&Res,&Plaintext);
#ifdef DEBUG
        //printf("Plaintext = ");
        //OCT_output(&Plaintext);
        //printf("IV = ");
        //OCT_output(&IV);
#endif
        MPIN_AES_GCM_ENCRYPT(&CK,&IV,&HEADER,&Plaintext,&Ciphertext,&Tag);
        MPIN_AES_GCM_DECRYPT(&CK,&IV,&HEADER,&Ciphertext,&Plaintext,&Tag);
#ifdef DEBUG
        //printf("Ciphertext = ");
        //OCT_output(&Ciphertext);
#endif

        if (!OCT_comp(&Res,&Plaintext))
        {
            printf("test_mpinfull() FAILURE Encryption/Decryption with AES-GCM\n");
            return 1;
        }
    }
    printf("test_mpinfull() SUCCESS\n");
    return 0;
}
