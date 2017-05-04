//
//  MilagroTest.m
//  milagro-test-app
//
//  Created by Kirill Neznamov on 03/05/2017.
//  Copyright © 2017 Kirill Neznamov. All rights reserved.
//

#import "MilagroTest.h"

extern int test_big_arithmetics(int argc, char** argv);
extern int test_fp_arithmetics(int argc, char** argv);
extern int test_fp2_arithmetics(int argc, char** argv);
extern int test_fp4_arithmetics(int argc, char** argv);
extern int test_ecp_arithmetics(int argc, char** argv);
extern int test_ecp2_arithmetics(int argc, char** argv);
extern int test_hash(int argc, char** argv);
extern int test_gcm_decrypt(int argc, char** argv);
extern int test_gcm_encrypt(int argc, char** argv);
extern int test_aes_encrypt(int argc, char** argv);
extern int test_aes_decrypt(int argc, char** argv);
extern int test_mpin();
extern int test_mpin_sign();
extern int test_mpin_good();
extern int test_mpin_bad_pin();
extern int test_mpin_bad_token();
extern int test_mpin_expired_tp();
extern int test_mpin_tp();
extern int test_mpin_random();
extern int test_mpinfull(int argc, char** argv);
extern int test_mpinfull_onepass();
extern int test_mpinfull_random();
extern int test_mpin_vectors(int argc, char** argv);
extern int test_utils();
extern int test_wcc(int argc, char** argv);
extern int test_wcc_gcm();
extern int test_wcc_random();

typedef int(*TestFunc)(int argc, char** argv);

@implementation MilagroTest

+(NSString*) pathForVector:(NSString*) vectorFileName
             withExtension:(NSString*) extension
               inFolder:(NSString*) folder
{
    NSString *filePath = [[NSBundle bundleForClass:[self class]] pathForResource:vectorFileName ofType:extension inDirectory:folder];
    
    return filePath;
}

+(void) run2:(TestFunc) func withVector:(NSString*) vectorName andExtension:(NSString*) extension inFolder:(NSString*) folder
{
    NSString *vector = nil;
    
    vector = [ MilagroTest pathForVector:vectorName
                           withExtension:extension
                                inFolder:folder];
    
    char *argv[] = { (char*)[vector UTF8String],
                     (char*)[vector UTF8String]  };

    func( 2, argv);
}

+(void) run2:(TestFunc) func withOption:(NSString*) option
{
    char *argv[] = { (char*)[option UTF8String],
                     (char*)[option UTF8String]  };
    
    func( 2, argv);
}


+(void) run3:(TestFunc) func withVector:(NSString*) vectorName andExtension:(NSString*) extension inFolder:(NSString*) folder withOption:(NSString*) option
{
    NSString *vector = nil;
    
    vector = [ MilagroTest pathForVector:vectorName
                           withExtension:extension
                                inFolder:folder];
    
    char *argv[] = { (char*)[vector UTF8String],
                     (char*)[vector UTF8String],
                     (char*)[option UTF8String]  };
    
    func( 3, argv);
}

+(void) testAll
{
    dispatch_async(dispatch_get_global_queue(0, 0), ^{

        [ MilagroTest run2: test_big_arithmetics withVector:@"amcl_CTRMCL128" andExtension:@".rsp" inFolder:@"testVectors/aes" ];
        
        [ MilagroTest run2: test_fp_arithmetics withVector:@"test_vector_BN254_CX" andExtension:@".txt" inFolder:@"testVectors/fp" ];
        [ MilagroTest run2: test_fp2_arithmetics withVector:@"test_vector_BN254_CX" andExtension:@".txt" inFolder:@"testVectors/fp" ];
        [ MilagroTest run2: test_fp4_arithmetics withVector:@"test_vector_BN254_CX" andExtension:@".txt" inFolder:@"testVectors/fp" ];
        
        [ MilagroTest run2: test_ecp_arithmetics withVector:@"test_vector_BN254_CX_WEIERSTRASS" andExtension:@".txt" inFolder:@"testVectors/ecp" ];
        [ MilagroTest run2: test_ecp2_arithmetics withVector:@"test_vector_BN254_CX_WEIERSTRASS" andExtension:@".txt" inFolder:@"testVectors/ecp" ];

        [ MilagroTest run3: test_hash withVector:@"SHA256ShortMsg" andExtension:@".rsp" inFolder:@"testVectors/sha/256" withOption:@"sha256" ];
        [ MilagroTest run3: test_hash withVector:@"SHA384ShortMsg" andExtension:@".rsp" inFolder:@"testVectors/sha/384" withOption:@"sha384" ];
        [ MilagroTest run3: test_hash withVector:@"SHA512ShortMsg" andExtension:@".rsp" inFolder:@"testVectors/sha/512" withOption:@"sha512" ];

        [ MilagroTest run2: test_gcm_encrypt withVector:@"gcmEncryptExtIV128" andExtension:@".rsp" inFolder:@"testVectors/gcm" ];
        [ MilagroTest run2: test_gcm_encrypt withVector:@"gcmEncryptExtIV256" andExtension:@".rsp" inFolder:@"testVectors/gcm" ];

        [ MilagroTest run2: test_gcm_decrypt withVector:@"gcmDecrypt128" andExtension:@".rsp" inFolder:@"testVectors/gcm" ];
        [ MilagroTest run2: test_gcm_decrypt withVector:@"gcmDecrypt256" andExtension:@".rsp" inFolder:@"testVectors/gcm" ];

        [ MilagroTest run3: test_aes_encrypt withVector:@"ECBMMT128" andExtension:@".rsp" inFolder:@"testVectors/aes" withOption:@"ECB" ];
        [ MilagroTest run3: test_aes_encrypt withVector:@"ECBMMT256" andExtension:@".rsp" inFolder:@"testVectors/aes" withOption:@"ECB" ];
        [ MilagroTest run3: test_aes_encrypt withVector:@"CBCMMT128" andExtension:@".rsp" inFolder:@"testVectors/aes" withOption:@"CBC" ];
        [ MilagroTest run3: test_aes_encrypt withVector:@"CFB8MMT128" andExtension:@".rsp" inFolder:@"testVectors/aes" withOption:@"CFB1" ];
        [ MilagroTest run3: test_aes_encrypt withVector:@"CBCMMT256" andExtension:@".rsp" inFolder:@"testVectors/aes" withOption:@"CBC" ];
        [ MilagroTest run3: test_aes_encrypt withVector:@"amcl_CTRMCL128" andExtension:@".rsp" inFolder:@"testVectors/aes" withOption:@"CTR" ];
        [ MilagroTest run3: test_aes_encrypt withVector:@"amcl_CTRMCL256" andExtension:@".rsp" inFolder:@"testVectors/aes" withOption:@"CTR" ];
        [ MilagroTest run3: test_aes_encrypt withVector:@"CFB8MMT256" andExtension:@".rsp" inFolder:@"testVectors/aes" withOption:@"CFB1" ];
        
        [ MilagroTest run3: test_aes_decrypt withVector:@"ECBMMT128" andExtension:@".rsp" inFolder:@"testVectors/aes" withOption:@"ECB" ];
        [ MilagroTest run3: test_aes_decrypt withVector:@"ECBMMT256" andExtension:@".rsp" inFolder:@"testVectors/aes" withOption:@"ECB" ];
        [ MilagroTest run3: test_aes_decrypt withVector:@"CBCMMT128" andExtension:@".rsp" inFolder:@"testVectors/aes" withOption:@"CBC" ];
        [ MilagroTest run3: test_aes_decrypt withVector:@"CFB8MMT128" andExtension:@".rsp" inFolder:@"testVectors/aes" withOption:@"CFB1" ];
        [ MilagroTest run3: test_aes_decrypt withVector:@"CBCMMT256" andExtension:@".rsp" inFolder:@"testVectors/aes" withOption:@"CBC" ];
        [ MilagroTest run3: test_aes_decrypt withVector:@"amcl_CTRMCL128" andExtension:@".rsp" inFolder:@"testVectors/aes" withOption:@"CTR" ];
        [ MilagroTest run3: test_aes_decrypt withVector:@"amcl_CTRMCL256" andExtension:@".rsp" inFolder:@"testVectors/aes" withOption:@"CTR" ];
        [ MilagroTest run3: test_aes_decrypt withVector:@"CFB8MMT256" andExtension:@".rsp" inFolder:@"testVectors/aes" withOption:@"CFB1" ];
        
        test_mpin();
        test_mpin_sign();
        test_mpin_good();
        test_mpin_bad_pin();
        test_mpin_bad_token();
        test_mpin_expired_tp();
        test_mpin_tp();
        test_mpin_random();
        
        [ MilagroTest run2: test_mpinfull withOption:@"sha256" ];
        [ MilagroTest run2: test_mpinfull withOption:@"sha384" ];
        [ MilagroTest run2: test_mpinfull withOption:@"sha512" ];
        
        test_mpinfull_onepass();
        test_mpinfull_random();

        [ MilagroTest run2: test_mpin_vectors withVector:@"BN254_CX" andExtension:@".txt" inFolder:@"testVectors/mpin" ];
        test_utils();
         
        [ MilagroTest run2: test_wcc withOption:@"sha256" ];
        [ MilagroTest run2: test_wcc withOption:@"sha384" ];
        [ MilagroTest run2: test_wcc withOption:@"sha512" ];
         test_wcc_gcm();
         test_wcc_random();
        
    });
}

@end
