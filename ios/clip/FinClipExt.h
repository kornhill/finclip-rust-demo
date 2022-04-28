//
//  FinClipExt.h
//  clip
//
//  Created by c. liang on 26/4/2022.
//

#ifndef FinClipExt_h
#define FinClipExt_h

#import <FinApplet/FinApplet.h>
#import "rustywallet.h"

@interface FinClipExt : NSObject {
    CWallet cwallet;
}

+(FinClipExt*)singleton;

@end
#endif /* FinClipExt_h */
