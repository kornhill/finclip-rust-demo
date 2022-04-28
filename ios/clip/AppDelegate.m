//
//  AppDelegate.m
//  clip
//
//  Created by c. liang on 12/4/2022.
//

#import "AppDelegate.h"
#import <FinApplet/FinApplet.h>
#import "FinClipExt.h"
//#import "rustywallet.h"

@interface AppDelegate ()

@end

@implementation AppDelegate


- (BOOL)application:(UIApplication *)application didFinishLaunchingWithOptions:(NSDictionary *)launchOptions {
    
    NSString *appKey = @"22LyZEib0gLTQdU3MUauARgvo5OK1UkzIY2eR+LFy28NAKxKlxHnzqdyifD+rGyG";
    FATConfig *config = [FATConfig configWithAppSecret:@"8fe39ccd4c9862ae" appKey:appKey];
    config.apiServer = @"http://127.0.0.1:8000";
    [[FATClient sharedClient] initWithConfig:config error:nil];
    [[FATClient sharedClient] setEnableLog:NO];
    [[FATClient sharedClient] registerSyncExtensionApi:@"generate_wallet" target:[FinClipExt singleton]];
    [[FATClient sharedClient] registerSyncExtensionApi:@"release_wallet" target:[FinClipExt singleton]];
    [[FATClient sharedClient] registerSyncExtensionApi:@"save_wallet" target:[FinClipExt singleton]];
    [[FATClient sharedClient] registerSyncExtensionApi:@"fetch_wallet" target:[FinClipExt singleton]];
    //[self generate_wallet];
    
    return YES;
}


#pragma mark - UISceneSession lifecycle


- (UISceneConfiguration *)application:(UIApplication *)application configurationForConnectingSceneSession:(UISceneSession *)connectingSceneSession options:(UISceneConnectionOptions *)options {
    // Called when a new scene session is being created.
    // Use this method to select a configuration to create the new scene with.
    return [[UISceneConfiguration alloc] initWithName:@"Default Configuration" sessionRole:connectingSceneSession.role];
}


- (void)application:(UIApplication *)application didDiscardSceneSessions:(NSSet<UISceneSession *> *)sceneSessions {
    // Called when the user discards a scene session.
    // If any sessions were discarded while the application was not running, this will be called shortly after application:didFinishLaunchingWithOptions.
    // Use this method to release any resources that were specific to the discarded scenes, as they will not return.
}


@end
