/**
 * @file test_mpinfull_onepass.c
 * @author Kealan McCusker
 * @brief Test M-Pin Full Single pass
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

/* Test M-Pin Full Single pass */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include "mpin.h"
#include "randapi.h"

int test_mpinfull_onepass()
{
    printf("test_mpinfull_onepass() started\n");
    int i,PIN1,PIN2,rtn;

    char id[256];
    octet ID = {0,sizeof(id),id};

    char x[PGS],y1[PGS],y2[PGS];
    octet X= {0, sizeof(x),x};
    octet Y1= {0,sizeof(y1),y1};
    octet Y2= {0,sizeof(y2),y2};

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

    char ut[2*PFS+1];
    octet UT= {0,sizeof(ut),ut};

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

    int TimeValue = 0;

    PIN1 = 1234;
    PIN2 = 1234;

    /* Assign the End-User an ID */
    char* user = "testuser@miracl.com";
    OCT_jstring(&ID,user);
   // printf("CLIENT: ID %s\n", user);

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
    MPIN_HASH_ID(HASH_TYPE_MPIN,&ID,&HCID);
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
    rtn = MPIN_GET_CLIENT_PERMIT(HASH_TYPE_MPIN,date,&MS1,&HCID,&TP1);
    if (rtn != 0)
    {
        printf("MPIN_GET_CLIENT_PERMIT(HASH_TYPE_MPIN,date,&MS1,&HCID,&TP1) Error %d\n", rtn);
        return 1;
    }
    rtn = MPIN_GET_CLIENT_PERMIT(HASH_TYPE_MPIN,date,&MS2,&HCID,&TP2);
    if (rtn != 0)
    {
        printf("MPIN_GET_CLIENT_PERMIT(HASH_TYPE_MPIN,date,&MS2,&HCID,&TP2) Error %d\n", rtn);
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
    rtn = MPIN_EXTRACT_PIN(HASH_TYPE_MPIN,&ID, PIN1, &TOKEN);
    if (rtn != 0)
    {
        printf("MPIN_EXTRACT_PIN( &ID, PIN, &TOKEN) Error %d\n", rtn);
        return 1;
    }
    //printf("Token = 0x");
    //OCT_output(&TOKEN);

    /* Client precomputation */
    MPIN_PRECOMPUTE(&TOKEN,&HCID,NULL,&G1,&G2);

    /* Client  */
    TimeValue = MPIN_GET_TIME();
    //printf("TimeValue %d \n", TimeValue);
    rtn = MPIN_CLIENT(HASH_TYPE_MPIN,date,&ID,&RNG,&X,PIN2,&TOKEN,&SEC,NULL,&UT,&TP,NULL,TimeValue,&Y1);
    if (rtn != 0)
    {
        printf("MPIN_CLIENT ERROR %d\n", rtn);
        return 1;
    }
   // printf("Y1 = 0x");
    //OCT_output(&Y1);
    //printf("V = 0x");
    //OCT_output(&SEC);

    /* Client sends Z=r.ID to Server */
    MPIN_GET_G1_MULTIPLE(&RNG,1,&R,&HCID,&Z);

    /* Server  */
    rtn = MPIN_SERVER(HASH_TYPE_MPIN,date,&HID,&HTID,&Y2,&ServerSecret,NULL,&UT,&SEC,&E,&F,pID,NULL,TimeValue);

    //printf("Y2 = 0x");
    //OCT_output(&Y2);
    if (rtn != 0)
    {
        printf("FAILURE Invalid Token Error Code %d\n", rtn);
        return 1;
    }
    else
    {
        printf("Authenticated Error Code %d\n", rtn);
    }

    /* Server sends T=w.ID to client */
    MPIN_GET_G1_MULTIPLE(&RNG,0,&W,&HTID,&T);
    //printf("T = 0x");
    //OCT_output(&T);

    MPIN_HASH_ALL(HASH_TYPE_MPIN,&HCID,NULL,&UT,&SEC,&Y1,&Z,&T,&HM);
    MPIN_CLIENT_KEY(HASH_TYPE_MPIN,&G1,&G2,PIN2,&R,&X,&HM,&T,&CK);
    //printf("Client Key = ");
    //OCT_output(&CK);

    MPIN_HASH_ALL(HASH_TYPE_MPIN,&HCID,NULL,&UT,&SEC,&Y2,&Z,&T,&HM);
    MPIN_SERVER_KEY(HASH_TYPE_MPIN,&Z,&ServerSecret,&W,&HM,&HID,NULL,&UT,&SK);
    //printf("Server Key = ");
    //OCT_output(&SK);

    if (!OCT_comp(&CK,&SK))
    {
        printf("test_mpinfull_onepass() FAILURE Keys are different\n");
        return 1;
    }
    printf("test_mpinfull_onepass() SUCCESS\n");
    return 0;

}
