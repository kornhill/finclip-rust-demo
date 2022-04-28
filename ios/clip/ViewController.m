//
//  ViewController.m
//  clip
//
//  Created by c. liang on 12/4/2022.
//

#import "ViewController.h"
#import "rustywallet.h"
#import <FinApplet/FinApplet.h>

@interface ViewController ()

@end

@implementation ViewController

- (void)viewDidLoad {
    [super viewDidLoad];
    // Do any additional setup after loading the view.
}
-(IBAction)showLabel{
    
    FATAppletRequest *request = [[FATAppletRequest alloc] init];
    request.appletId = @"62621bba4fed110001781d9a";
    request.apiServer = @"http://127.0.0.1:8000";
    request.transitionStyle = FATTranstionStyleUp;
    //request.startParams = startParams;
        
    [[FATClient sharedClient] startAppletWithRequest:request InParentViewController:self completion:^(BOOL result, FATError *error) {
        NSLog(@"打开小程序:%@", error);
    } closeCompletion:^{
        NSLog(@"关闭小程序");
    }];
}

@end
