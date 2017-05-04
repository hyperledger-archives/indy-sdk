/**
 * @file test_utils.c
 * @author Kealan McCusker
 * @brief Test driver and function exerciser for util Functions
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

/* test driver and function exerciser for util Functions */
/* Version 3.0 - supports Time Permits */

/* Build executible after installation:

  gcc -std=c99 -g ./test_utils.c -I/opt/amcl/include -L/opt/amcl/lib -lamcl -lmpin -o test_utils

*/

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include "mpin.h"
#include "utils.h"
#include "randapi.h"

const int nIter = 32;

const int V1[32] = {822451, 201197, 585864, 720907, 90685, 361405, 461117, 804736, 882396, 392044, 677418, 227782, 67488, 755001, 202246, 715416, 267781, 950041, 440029, 566511, 965344, 403316, 756645, 272421, 633043, 868514, 201805, 234612, 169985, 670987, 93770, 961852};

char* V2[32] = {"7ec2d70f9883b3ff1d7963d069ff7576f3e5d782511766854cd6d6eb1ad8863c", "02def0995797f3408573edd6e06895220ce17f2a2a85c01c993fcbd265a4d76d", "c5c7d86e71c5fea739a54beb952cbad1a16a18fac3f9bd4fe846728e80936f0e", "f4cdd3ac0611961b7ab8fda1c52ac76fd9ed61a17d75ba0d2ab2f76409a6a757", "6ed936c269224750a74685b3d9d0c3f2bdf8a67ec25f25befb2f91ba525364ec", "8cd0a96bbd5e14557a3c47a90d51d46029be3b1a4429a96e595c21ed7c505280", "e97b3ab62ea88aceae7de70067dd07c9d6d69c4b981bd8784e8fd826a2a2ec6f", "5c0166bbbf175f2ce55ea04cf1e0f8e2967092c23a03a7250090876612ff1c2c", "e8d3ae8fb6a42ec7e1bf43d1ad75ac63d54b3034dc16bb215f69a875e697870f", "e4599f0a66b60254beda71dab4cdac88af9fb405e4cde02a72144425296feda3", "09d6330a165363dceb87a21c4784af57060221fbb2c61add5d34b38d7e9089ac", "4aa4efa6e639999ad220743ae899749f26900ed753dc4b5993ba09b91f227830", "7ec39006697d6db8310f7f8f7bc1501a3251ba2aaf4d0418a55a5d598f67a5f4", "a6afb85fa403645cc0e45054386e98adb119491070371b4fac4e375fa8bf2fa6", "eac07fe6d84b4181bbaac33aea6494ea1557809b4f62bf8011a33359aec4feca", "5f96dfea25fb205ba21ae6c7b491c65c0b65f96d88320a39a0d0e58c42ad8a9c", "bf97a5180ac699ff2bd8046e77e32495a9774c557844363686a6203159eeb8c4", "921a5321fc5b16ab46755a3a8da130c727556e26279b3659d46945c85a92230d", "85f87cfd7b915d989c62d7abbf93c851d845f2b850e014caf01370d45e123af5", "45b1d889b8e36c4320476007bb93908c8c993d728cbfb66014335fa557a289ff", "ea6fcfad4286d7f15cef2011d4e79fb4e6bd5ea5680b0f0a5eace597270f2723", "0b075ac0885abd70bac20a436a272adea717af6dad7c94f16d1d20ec49d9d146", "4109be2fbd2d4c959f0e6bf2550c02860b5ca06a09913f004bb1f1b9fff91d47", "15afd5efad90f3768775c7097dd0feb95215e9a9013239d2d76ca8b95fc10915", "21f03a1f17e46a1d69c35c6985e275297db68ed4254166f4c8cd5595ebbf0fcb", "a0ee64a19552712708f2262017e7bb4edfa03873895f976c791a45ca733ef375", "5b06bce9f720d5588b6758241607040a12d2a1b7c4db82546fc894fd0c95d05d", "26654e23604fa6269e8944d8a9edda1073ecba5bde9173a277ca9bac3969bb77", "e570ede6303222e2fe055c4a33463de0406f441c5df32494b9432cc7f24dcf95", "14d82666b07485a88accf5533b06a4c637ad7c7316117e1963e0d369e2dbaeb2", "9ac996b9dc3d99e13db15569ca7adc927e0b4c9ad38da427ed6be7863d551d85", "571957d348629392f7f7f186d72f3f41ddb23813b116b3de776d2fe888adf82e"};

char temp[32+1];
char bintemp[32+1];

int test_utils()
{
    printf("test_utils() started\n");
    int i,otp;
    char x[32];
    octet X= {sizeof(x),sizeof(x),x};

    char vector[32];
    octet VECTOR= {sizeof(vector),sizeof(vector),vector};

    char seed[32] = {0};
    octet SEED = {0,sizeof(seed),seed};
    csprng RNG;

    /* unrandom seed value! */
    SEED.len=32;
    for (i=0; i<32; i++)
        SEED.val[i]=i+1;

    /* initialise random number generator */
    CREATE_CSPRNG(&RNG,&SEED);

    for (i=0; i<nIter; i++)
    {
        otp = generateOTP(&RNG);
        if (otp != V1[i])
        {
            printf("test_utils() FAILURE generateOTP failure\n");
            return 1;
        }
    }

    for (i=0; i<nIter; i++)
    {
        generateRandom(&RNG,&X);
        OCT_fromHex(&VECTOR,V2[i]);
        if (!OCT_comp(&X,&VECTOR))
        {
            printf("test_utils() FAILURE generateRandom failure\n");
            return 1;
        }
    }

    for (i=0; i<nIter; i++)
    {
        amcl_hex2bin(V2[i], bintemp, 64);
        amcl_bin2hex(bintemp, temp, 32);
        if (strncmp(V2[i], temp, 64))
        {
            printf("test_utils() FAILURE conversion hex/bin\n");
            return 1;
        }
    }

    printf("test_utils() SUCCESS\n");
    return 0;
}
