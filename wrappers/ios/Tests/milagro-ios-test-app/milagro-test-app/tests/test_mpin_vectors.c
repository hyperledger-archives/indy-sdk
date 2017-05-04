/**
 * @file test_mpin_vectors.c
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

#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include "mpin.h"
#include "amcl.h"
#include "utils.h"

#define LINE_LEN 1000

void read_OCTET(octet* OCT, char* string)
{
    int len = strlen(string);
    char buff[len];
    strncpy(buff,string,len);
    char *end = strchr(buff,',');
    if (end == NULL)
    {
        printf("ERROR unexpected test vector %s\n",string);
        exit(EXIT_FAILURE);
    }
    end[0] = '\0';
    OCT_fromHex(OCT,buff);
}

int test_mpin_vectors(int argc, char** argv)
{
    printf("test_mpin_vectors() started\n");
    if (argc != 2)
    {
        printf("usage: ./test_mpin_vectors [path to test vector file]\n");
        exit(EXIT_FAILURE);
    }

    int i = 0, len = 0, rtn = 0;
    FILE *fp;

    char line[LINE_LEN];
    char * linePtr = NULL;

    char ms1[PFS];
    octet MS1= {0,sizeof(ms1),ms1};
    const char* MS1line = "MS1 = ";
    char ms2[PFS];
    octet MS2= {0,sizeof(ms2),ms2};
    const char* MS2line = "MS2 = ";
    char ss1[4*PFS], rtss1[4*PFS];
    octet SS1= {0,sizeof(ss1),ss1}, rtSS1= {0,sizeof(rtss1),rtss1};
    const char* SS1line = "SS1 = ";
    char ss2[4*PFS], rtss2[4*PFS];
    octet SS2= {0,sizeof(ss2),ss2}, rtSS2= {0,sizeof(rtss2),rtss2};
    const char* SS2line = "SS2 = ";
    int DATE = 0;
    const char* DATEline = "DATE = ";
    int PIN1 = 0;
    const char* PIN1line = "PIN1 = ";
    int PIN2 = 0;
    const char* PIN2line = "PIN2 = ";
    char server_secret[4*PFS], rt_server_secret[4*PFS];
    octet SERVER_SECRET= {0,sizeof(server_secret),server_secret}, rtSERVER_SECRET= {0,sizeof(rt_server_secret),rt_server_secret};
    const char* SERVER_SECRETline = "SERVER_SECRET = ";
    char tp1[2*PFS+1], rttp1[2*PFS+1];
    octet TP1= {0,sizeof(tp1),tp1}, rtTP1= {0,sizeof(rttp1),rttp1};
    const char* TP1line = "TP1 = ";
    char tp2[2*PFS+1], rttp2[2*PFS+1];
    octet TP2= {0,sizeof(tp2),tp2}, rtTP2= {0,sizeof(rttp2),rttp2};
    const char* TP2line = "TP2 = ";
    char cs1[2*PFS+1], rtcs1[2*PFS+1];
    octet CS1= {0,sizeof(cs1),cs1}, rtCS1= {0,sizeof(rtcs1),rtcs1};
    const char* CS1line = "CS1 = ";
    char cs2[2*PFS+1], rtcs2[2*PFS+1];
    octet CS2= {0,sizeof(cs2),cs2}, rtCS2= {0,sizeof(rtcs2),rtcs2};
    const char* CS2line = "CS2 = ";
    char client_secret[2*PFS+1], rtclient_secret[2*PFS+1];
    octet CLIENT_SECRET= {0,sizeof(client_secret),client_secret}, rtCLIENT_SECRET= {0,sizeof(rtclient_secret),rtclient_secret};
    const char* CLIENT_SECRETline = "CLIENT_SECRET = ";
    char hash_mpin_id_hex[PFS], rthash_mpin_id_hex[PFS];
    octet HASH_MPIN_ID_HEX= {0,sizeof(hash_mpin_id_hex),hash_mpin_id_hex}, rtHASH_MPIN_ID_HEX= {0,sizeof(rthash_mpin_id_hex),rthash_mpin_id_hex};
    const char* HASH_MPIN_ID_HEXline = "HASH_MPIN_ID_HEX = ";
    char time_permit[2*PFS+1], rttime_permit[2*PFS+1];
    octet TIME_PERMIT= {0,sizeof(time_permit),time_permit}, rtTIME_PERMIT= {0,sizeof(rttime_permit),rttime_permit};
    const char* TIME_PERMITline = "TIME_PERMIT = ";
    char mpin_id_hex[300+1];
    octet MPIN_ID_HEX= {0,sizeof(mpin_id_hex),mpin_id_hex};
    const char* MPIN_ID_HEXline = "MPIN_ID_HEX = ";
    char token[2*PFS+1];
    octet TOKEN= {0,sizeof(token),token};
    const char* TOKENline = "TOKEN = ";
    int SERVER_OUTPUT = 0;
    const char* SERVER_OUTPUTline = "SERVER_OUTPUT = ";
    char u[2*PFS+1], rtu[2*PFS+1];
    octet U= {0,sizeof(u),u}, rtU= {0,sizeof(rtu),rtu};
    const char* Uline = "U = ";
    char v[2*PFS+1];
    octet V= {0,sizeof(v),v};
    const char* Vline = "V = ";
    char y[PFS];
    octet Y= {0,sizeof(y),y};
    const char* Yline = "Y = ";
    char x[PFS];
    octet X= {0,sizeof(x),x};
    const char* Xline = "X = ";
    char ut[2*PFS+1], rtut[2*PFS+1];
    octet UT= {0,sizeof(ut),ut}, rtUT= {0,sizeof(rtut),rtut};
    const char* UTline = "UT = ";
    char sec[2*PFS+1], rtsec[2*PFS+1];
    octet SEC= {0,sizeof(sec),sec}, rtSEC= {0,sizeof(rtsec),rtsec};
    const char* SECline = "SEC = ";
    char hid[2*PFS+1], htid[2*PFS+1];
    octet HID= {0,sizeof(hid), hid}, HTID= {0,sizeof(htid),htid};
    char e[12*PFS];
    octet E= {0,sizeof(e),e};
    char f[12*PFS];
    octet F= {0,sizeof(f),f};

    octet *pID;

    fp = fopen(argv[1], "r");
    if (fp == NULL)
    {
        printf("ERROR opening test vector file\n");
        exit(EXIT_FAILURE);
    }

    while (fgets(line, LINE_LEN, fp) != NULL)
    {
        i++;
// Read MS1
        if (!strncmp(line,MS1line, strlen(MS1line)))
        {
            len = strlen(MS1line);
            linePtr = line + len;
            read_OCTET(&MS1,linePtr);
        }
// Read MS2
        if (!strncmp(line,MS2line, strlen(MS2line)))
        {
            len = strlen(MS2line);
            linePtr = line + len;
            read_OCTET(&MS2,linePtr);
        }
// Read SS1
        if (!strncmp(line,SS1line, strlen(SS1line)))
        {
            len = strlen(SS1line);
            linePtr = line + len;
            read_OCTET(&SS1,linePtr);
// Generate first server secret shares
            rtn = MPIN_GET_SERVER_SECRET(&MS1,&rtSS1);
            if (rtn != 0)
            {
                printf("ERROR MPIN_GET_SERVER_SECRET(&MS1,&SS1), %d, line %d\n", rtn,i);
                exit(EXIT_FAILURE);
            }
            else if (!OCT_comp(&rtSS1,&SS1))
            {
                printf("ERROR generating server secret, line %d\n",i);
                exit(EXIT_FAILURE);
            }
        }
// Read SS2
        if (!strncmp(line,SS2line, strlen(SS2line)))
        {
            len = strlen(SS2line);
            linePtr = line + len;
            read_OCTET(&SS2,linePtr);
// Generate second server secret shares
            rtn = MPIN_GET_SERVER_SECRET(&MS2,&rtSS2);
            if (rtn != 0)
            {
                printf("ERROR MPIN_GET_SERVER_SECRET(&MS1,&SS2), %d, line %d\n", rtn,i);
                exit(EXIT_FAILURE);
            }
            else if (!OCT_comp(&rtSS2,&SS2))
            {
                printf("ERROR generating server secret, line %d\n",i);
                exit(EXIT_FAILURE);
            }
        }
// Read SERVER_SECRET
        if (!strncmp(line,SERVER_SECRETline, strlen(SERVER_SECRETline)))
        {
            len = strlen(SERVER_SECRETline);
            linePtr = line + len;
            read_OCTET(&SERVER_SECRET,linePtr);
// Recombine server secret
            rtn = MPIN_RECOMBINE_G2(&SS1, &SS2, &rtSERVER_SECRET);
            if (rtn != 0)
            {
                printf("MPIN_RECOMBINE_G2(&SS1, &SS2, &SERVER_SECRET) Error %d, line %d\n", rtn,i);
                exit(EXIT_FAILURE);
            }
            else if (!OCT_comp(&SERVER_SECRET,&rtSERVER_SECRET))
            {
                printf("ERROR recombining server secret, line %d\n",i);
                exit(EXIT_FAILURE);
            }
        }
// Read MPIN_ID_HEX
        if (!strncmp(line,MPIN_ID_HEXline, strlen(MPIN_ID_HEXline)))
        {
            len = strlen(MPIN_ID_HEXline);
            linePtr = line + len;
            read_OCTET(&MPIN_ID_HEX,linePtr);
        }
// Read HASH_MPIN_ID_HEX
        if (!strncmp(line,HASH_MPIN_ID_HEXline, strlen(HASH_MPIN_ID_HEXline)))
        {
            len = strlen(HASH_MPIN_ID_HEXline);
            linePtr = line + len;
            read_OCTET(&HASH_MPIN_ID_HEX,linePtr);
// Hash MPIN_ID
            MPIN_HASH_ID(HASH_TYPE_MPIN,&MPIN_ID_HEX,&rtHASH_MPIN_ID_HEX);
            if (!OCT_comp(&HASH_MPIN_ID_HEX,&rtHASH_MPIN_ID_HEX))
            {
                printf("ERROR hashing identity, line %d\n",i);
                exit(EXIT_FAILURE);
            }
        }
// Read CS1
        if (!strncmp(line,CS1line, strlen(CS1line)))
        {
            len = strlen(CS1line);
            linePtr = line + len;
            read_OCTET(&CS1,linePtr);
// Generate first client secret shares
            rtn = MPIN_GET_CLIENT_SECRET(&MS1,&HASH_MPIN_ID_HEX,&rtCS1);
            if (rtn != 0)
            {
                printf("MPIN_GET_CLIENT_SECRET(&MS1,&HASH_MPIN_ID_HEX,&CS1) Error %d, line %d\n", rtn,i);
                exit(EXIT_FAILURE);
            }
            if (!OCT_comp(&CS1,&rtCS1))
            {
                printf("ERROR generating client share, line %d\n",i);
                exit(EXIT_FAILURE);
            }
        }
// Read CS2
        if (!strncmp(line,CS2line, strlen(CS2line)))
        {
            len = strlen(CS2line);
            linePtr = line + len;
            read_OCTET(&CS2,linePtr);
// Generate second client secret shares
            rtn = MPIN_GET_CLIENT_SECRET(&MS2,&HASH_MPIN_ID_HEX,&rtCS2);
            if (rtn != 0)
            {
                printf("MPIN_GET_CLIENT_SECRET(&MS2,&HASH_MPIN_ID_HEX,&CS1) Error %d, line %d\n", rtn,i);
                exit(EXIT_FAILURE);
            }
            if (!OCT_comp(&CS2,&rtCS2))
            {
                printf("ERROR generating client share, line %d\n",i);
                exit(EXIT_FAILURE);
            }
        }
// Read CLIENT_SECRET
        if (!strncmp(line,CLIENT_SECRETline, strlen(CLIENT_SECRETline)))
        {
            len = strlen(CLIENT_SECRETline);
            linePtr = line + len;
            read_OCTET(&CLIENT_SECRET,linePtr);
// Combine client secret shares : TOKEN is the full client secret
            rtn = MPIN_RECOMBINE_G1(&CS1, &CS2, &rtCLIENT_SECRET);
            if (rtn != 0)
            {
                printf("MPIN_RECOMBINE_G1(&CS1, &CS2, &rtCLIENT_SECRET) Error %d, line %d\n",rtn,i);
                exit(EXIT_FAILURE);
            }
            if (!OCT_comp(&CLIENT_SECRET,&rtCLIENT_SECRET))
            {
                printf("ERROR generating CLIENT_SECRET, line %d\n",i);
                exit(EXIT_FAILURE);
            }
        }
// Read DATE
        if (!strncmp(line,DATEline, strlen(DATEline)))
        {
            len = strlen(DATEline);
            linePtr = line + len;
            sscanf(linePtr,"%d\n",&DATE);
        }
// Read PIN1
        if (!strncmp(line,PIN1line, strlen(PIN1line)))
        {
            len = strlen(PIN1line);
            linePtr = line + len;
            sscanf(linePtr,"%d\n",&PIN1);
        }
// Read PIN2
        if (!strncmp(line,PIN2line, strlen(PIN2line)))
        {
            len = strlen(PIN2line);
            linePtr = line + len;
            sscanf(linePtr,"%d\n",&PIN2);
        }
// Read TP1
        if (!strncmp(line,TP1line, strlen(TP1line)))
        {
            len = strlen(TP1line);
            linePtr = line + len;
            read_OCTET(&TP1,linePtr);
// Generate first Time Permit
            rtn = MPIN_GET_CLIENT_PERMIT(HASH_TYPE_MPIN,DATE,&MS1,&HASH_MPIN_ID_HEX,&rtTP1);
            if (rtn != 0)
            {
                printf("MPIN_GET_CLIENT_PERMIT((HASH_TYPE_MPIN,DATE,&MS1,&HASH_MPIN_ID_HEX,&TP1) Error %d, line %d\n",rtn,i);
                exit(EXIT_FAILURE);
            }
            if (!OCT_comp(&TP1,&rtTP1))
            {
                printf("ERROR generating TIME PERMIT, line %d\n",i);
                exit(EXIT_FAILURE);
            }
        }
// Read TP2
        if (!strncmp(line,TP2line, strlen(TP2line)))
        {
            len = strlen(TP2line);
            linePtr = line + len;
            read_OCTET(&TP2,linePtr);
// Generate second Time Permit
            rtn = MPIN_GET_CLIENT_PERMIT(HASH_TYPE_MPIN,DATE,&MS2,&HASH_MPIN_ID_HEX,&rtTP2);
            if (rtn != 0)
            {
                printf("MPIN_GET_CLIENT_PERMIT((HASH_TYPE_MPIN,DATE,&MS1,&HASH_MPIN_ID_HEX,&TP2) Error %d, line %d\n",rtn,i);
                exit(EXIT_FAILURE);
            }
            if (!OCT_comp(&TP2,&rtTP2))
            {
                printf("ERROR generating TIME PERMIT, line %d\n",i);
                exit(EXIT_FAILURE);
            }
        }
// Read TIME_PERMIT
        if (!strncmp(line,TIME_PERMITline, strlen(TIME_PERMITline)))
        {
            len = strlen(TIME_PERMITline);
            linePtr = line + len;
            read_OCTET(&TIME_PERMIT,linePtr);
// Combine Time Permits
            rtn = MPIN_RECOMBINE_G1(&TP1, &TP2, &rtTIME_PERMIT);
            if (rtn != 0)
            {
                printf("MPIN_RECOMBINE_G1(&TP1, &TP2, &rtTIME_PERMIT) Error %d, line %d\n",rtn,i);
                exit(EXIT_FAILURE);
            }
            if (!OCT_comp(&TIME_PERMIT,&rtTIME_PERMIT))
            {
                printf("ERROR combining TIME PERMITs, line %d\n",i);
                exit(EXIT_FAILURE);
            }
        }
// Read TOKEN
        if (!strncmp(line,TOKENline, strlen(TOKENline)))
        {
            len = strlen(TOKENline);
            linePtr = line + len;
            read_OCTET(&TOKEN,linePtr);
// Client extracts PIN1 from secret to create Token
            rtn = MPIN_EXTRACT_PIN(HASH_TYPE_MPIN,&MPIN_ID_HEX, PIN1, &rtCLIENT_SECRET);
            if (rtn != 0)
            {
                printf("MPIN_EXTRACT_PIN( &ID, PIN, &TOKEN) Error %d, line %d\n",rtn,i);
                exit(EXIT_FAILURE);
            }
            if (!OCT_comp(&TOKEN,&rtCLIENT_SECRET))
            {
                printf("ERROR extracting PIN from CLIENT SECRET, line %d\n",i);
                exit(EXIT_FAILURE);
            }
        }
// Read X
        if (!strncmp(line,Xline, strlen(Xline)))
        {
            len = strlen(Xline);
            linePtr = line + len;
            read_OCTET(&X,linePtr);
        }
// Read U
        if (!strncmp(line,Uline, strlen(Uline)))
        {
            len = strlen(Uline);
            linePtr = line + len;
            read_OCTET(&U,linePtr);
        }
// Read UT
        if (!strncmp(line,UTline, strlen(UTline)))
        {
            len = strlen(UTline);
            linePtr = line + len;
            read_OCTET(&UT,linePtr);
        }
// Read SEC
        if (!strncmp(line,SECline, strlen(SECline)))
        {
            len = strlen(SECline);
            linePtr = line + len;
            read_OCTET(&SEC,linePtr);
// Client first pass
            rtn = MPIN_CLIENT_1(HASH_TYPE_MPIN,DATE,&MPIN_ID_HEX,NULL,&X,PIN2,&TOKEN,&rtSEC,&rtU,&rtUT,&TIME_PERMIT);
            if (rtn != 0)
            {
                printf("MPIN_CLIENT_1 ERROR %d, line %d\n",rtn,i);
                exit(EXIT_FAILURE);
            }
            else if (!OCT_comp(&U,&rtU) || !OCT_comp(&UT,&rtUT) || !OCT_comp(&SEC,&rtSEC))
            {
                printf("ERROR performing CLIENT FIRST PASS, line %d\n",i);
                exit(EXIT_FAILURE);
            }
        }
// Read V
        if (!strncmp(line,Vline, strlen(Vline)))
        {
            len = strlen(Vline);
            linePtr = line + len;
            read_OCTET(&V,linePtr);
        }
// Read Y
        if (!strncmp(line,Yline, strlen(Yline)))
        {
            len = strlen(Yline);
            linePtr = line + len;
            read_OCTET(&Y,linePtr);
        }
// Read SERVER_OUTPUT
        if (!strncmp(line,SERVER_OUTPUTline, strlen(SERVER_OUTPUTline)))
        {
            len = strlen(SERVER_OUTPUTline);
            linePtr = line + len;
            sscanf(linePtr,"%d\n",&SERVER_OUTPUT);
// Server calculates H(ID) and H(T|H(ID)) (if time permits enabled), and maps them to points on the curve HID and HTID resp.
// When set only send hashed IDs to server
#ifdef USE_ANONYMOUS
            pID = &HASH_MPIN_ID_HEX;
#else
            pID = &MPIN_ID_HEX;
#endif
            MPIN_SERVER_1(HASH_TYPE_MPIN,DATE,pID,&HID,&HTID);
// Client second pass
            rtn = MPIN_CLIENT_2(&X,&Y,&rtSEC);
            if (rtn != 0)
            {
                printf("MPIN_CLIENT_2(&X,&Y,&SEC) Error %d, line %d\n",rtn,i);
                exit(EXIT_FAILURE);
            }
            if (!OCT_comp(&V,&rtSEC))
            {
                printf("ERROR performing CLIENT SECOND PASS, line %d\n",i);
                exit(EXIT_FAILURE);
            }
// Server second pass
            rtn = MPIN_SERVER_2(DATE,&HID,&HTID,&Y,&SERVER_SECRET,&U,&UT,&V,&E,&F);
            if (rtn != SERVER_OUTPUT)
            {
                printf("ERROR performing SERVER SECOND PASS, line %d\n",i);
                exit(EXIT_FAILURE);
            }
        }
    }
    fclose(fp);
    printf("test_mpin_vectors() SUCCESS TEST MPIN PASSED\n");
    return EXIT_SUCCESS;
}

