//
//  FinClipExt.m
//  clip
//
//  Created by c. liang on 26/4/2022.
//

#import <Foundation/Foundation.h>
#import "FinClipExt.h"

@implementation FinClipExt

static FinClipExt *_sharedMySingleton = nil;

+(FinClipExt*)singleton{
    @synchronized([FinClipExt class]) {
        if (!_sharedMySingleton){
            _sharedMySingleton = [[self alloc] init];
        }
        return _sharedMySingleton;
    }
    return nil;
}

+(id)alloc {
    @synchronized([FinClipExt class])
    {
        NSAssert(_sharedMySingleton == nil, @"Singleton already initialized.");
        _sharedMySingleton = [super alloc];
        return _sharedMySingleton;
    }
    return nil;
}

-(id)init {
    self = [super init];
    if (self != nil) {
        // initialize stuff here
    }   return self;
}

- (NSDictionary *)generate_wallet:(NSDictionary *)param
{
    NSLog(@"OC: generate_wallet ============>");
    NSLog(@"%p, param:%@", __func__, param);
    cwallet = generate_cwallet();
    char *addr = cwallet.public_addr;
    char *pub_key = cwallet.public_key;
    char *sec_key = cwallet.private_key;
    NSLog(@"OC: generate_wallet addr ====> %@", [NSString stringWithUTF8String:addr]);
    
    NSDictionary *resultDict = @{
        @"errMsg":@"generate_wallet:ok",  //in real case, check to return ok or fail
        @"public_address":[NSString stringWithUTF8String:addr],
        @"public_key":[NSString stringWithUTF8String:pub_key],
        @"private_key":[NSString stringWithUTF8String:sec_key]
    };
    return resultDict;
}

//remember to free the C memory allocation of the wallet structure
//free_cwallet(cw);
- (NSDictionary *)release_wallet:(NSDictionary *)param
{
    NSLog(@"OC: release_wallet ============>");
    free_cwallet(cwallet);
    
    NSDictionary *resultDict = @{
        @"errMsg":@"release_wallet:ok"
    };
    return resultDict;
}

- (NSDictionary *)save_wallet:(NSDictionary *)param
{
    NSLog(@"OC: save_wallet ============>");
    save_wallet(&cwallet);
    
    NSDictionary *resultDict = @{
        @"errMsg":@"release_wallet:ok"
    };
    return resultDict;
}

- (NSDictionary *)fetch_wallet:(NSDictionary *)param
{
    NSLog(@"OC: fetch_wallet ============>");
    cwallet = fetch_cwallet();
    char *addr = cwallet.public_addr;
    char *pub_key = cwallet.public_key;
    char *sec_key = cwallet.private_key;
    
    NSDictionary *resultDict = @{
        @"errMsg":@"release_wallet:ok",
        @"public_address":[NSString stringWithUTF8String:addr],
        @"public_key":[NSString stringWithUTF8String:pub_key],
        @"private_key":[NSString stringWithUTF8String:sec_key]
    };
    return resultDict;
}
@end
