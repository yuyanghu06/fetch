// Fetch-owned bridge. No password is returned to Rust, a browser or a child process.
#import <Foundation/Foundation.h>
#import <AppKit/AppKit.h>
#import <LocalAuthentication/LocalAuthentication.h>
#import <Security/Security.h>
#include <stdint.h>
#include <string.h>

// App-local wheel phases are forwarded in event order because Iced's wheel
// event drops AppKit's finger/momentum phases. The NSEvent is never changed.
static NSMutableArray<NSNumber *> *fetch_scroll_samples;
static NSHashTable<NSWindow *> *fetch_scroll_windows;
static id fetch_scroll_monitor;
static NSObject *FetchScrollLock(void) {
    static NSObject *lock;static dispatch_once_t once;
    dispatch_once(&once, ^{lock=[NSObject new];fetch_scroll_samples=[NSMutableArray new];});return lock;
}
static void FetchRecordScrollPhase(NSEventPhase phase,NSEventPhase momentum) {
    uint32_t sample=1;
    if(phase!=NSEventPhaseNone||momentum!=NSEventPhaseNone)sample|=2;
    // Inertia stays with the previous owner. Only a fresh finger gesture relatches.
    if((phase&NSEventPhaseBegan)!=0&&momentum==NSEventPhaseNone)sample|=4;
    @synchronized(FetchScrollLock()) {[fetch_scroll_samples addObject:@(sample)];}
}
uint32_t fetch_take_scroll_phase(void) {
    @synchronized(FetchScrollLock()) {
        if(!fetch_scroll_samples.count)return 0;
        uint32_t sample=fetch_scroll_samples.firstObject.unsignedIntValue;
        [fetch_scroll_samples removeObjectAtIndex:0];return sample;
    }
}
static void FetchObserveScroll(NSWindow *window) {
    if(!fetch_scroll_windows)fetch_scroll_windows=[NSHashTable weakObjectsHashTable];
    [fetch_scroll_windows addObject:window];
    if(!fetch_scroll_monitor)fetch_scroll_monitor=[NSEvent addLocalMonitorForEventsMatchingMask:NSEventMaskScrollWheel handler:^NSEvent *(NSEvent *event){
        if([fetch_scroll_windows containsObject:event.window])FetchRecordScrollPhase(event.phase,event.momentumPhase);
        return event;
    }];
}

// AppKit owns the panel. Rust owns the callback context until this one completion.
typedef void (*FetchConfigFileCallback)(void *context, const char *path, int32_t status);
static NSOpenPanel *fetch_config_file_panel = nil;
static NSOpenPanel *fetch_cached_config_file_panel = nil;
// Main-thread only. Preparing the panel does not present it or select/read a file.
static NSOpenPanel *fetch_prepared_config_file_panel(void) {
    if (!fetch_cached_config_file_panel) fetch_cached_config_file_panel = [NSOpenPanel openPanel];
    return fetch_cached_config_file_panel;
}
void fetch_prepare_config_file_picker(void) {
    // Let the window paint before doing AppKit's expensive first allocation.
    dispatch_after(dispatch_time(DISPATCH_TIME_NOW, 500 * NSEC_PER_MSEC), dispatch_get_main_queue(), ^{
        @autoreleasepool { (void)fetch_prepared_config_file_panel(); }
    });
}
void fetch_choose_config_file(FetchConfigFileCallback callback, void *context) {
    dispatch_async(dispatch_get_main_queue(), ^{
        @autoreleasepool {
            NSWindow *parent = NSApp.keyWindow ?: NSApp.mainWindow;
            if (fetch_config_file_panel || !parent) { callback(context, NULL, 2); return; }
            NSOpenPanel *panel = fetch_prepared_config_file_panel();
            fetch_config_file_panel = panel;
            panel.title = @"Choose an MCP configuration file or base folder";
            panel.prompt = @"Choose";
            panel.canChooseFiles = YES;
            panel.canChooseDirectories = YES;
            panel.allowsMultipleSelection = NO;
            panel.showsHiddenFiles = YES;
            panel.directoryURL = [NSFileManager defaultManager].homeDirectoryForCurrentUser;
            [panel beginSheetModalForWindow:parent completionHandler:^(NSModalResponse response) {
                if (fetch_config_file_panel == panel) fetch_config_file_panel = nil;
                if (response != NSModalResponseOK) { callback(context, NULL, 1); return; }
                NSURL *url = panel.URL;
                const char *path = url.isFileURL ? url.path.UTF8String : NULL;
                callback(context, path, path ? 0 : 2);
            }];
        }
    });
}
void fetch_cancel_config_file_picker(void) {
    // Preserve ordering with queued presentation, including a close immediately after Choose.
    dispatch_async(dispatch_get_main_queue(), ^{ [fetch_config_file_panel cancel:nil]; });
}

int32_t fetch_confirm_owner(const char *reason, uint32_t timeout_seconds) {
    @autoreleasepool {
        if (!reason || [NSThread isMainThread]) return 3;
        NSString *text = [NSString stringWithUTF8String:reason];
        if (!text) return 3;
        LAContext *context = [[LAContext alloc] init];
        context.touchIDAuthenticationAllowableReuseDuration = 0;
        context.localizedCancelTitle = @"Cancel";
        NSError *error = nil;
        if (![context canEvaluatePolicy:LAPolicyDeviceOwnerAuthentication error:&error]) return 3;
        dispatch_semaphore_t done = dispatch_semaphore_create(0);
        __block int32_t outcome = 3;
        dispatch_async(dispatch_get_main_queue(), ^{ [NSApp activateIgnoringOtherApps:YES]; });
        [context evaluatePolicy:LAPolicyDeviceOwnerAuthentication localizedReason:text reply:^(BOOL success, NSError *failure) {
            @synchronized (context) {
                if (outcome != 2) {
                    if (success) outcome = 0;
                    else if (failure.code == LAErrorUserCancel || failure.code == LAErrorAuthenticationFailed || failure.code == LAErrorUserFallback) outcome = 1;
                    else outcome = 3;
                }
            }
            dispatch_semaphore_signal(done);
        }];
        uint32_t bounded = timeout_seconds > 120 ? 120 : timeout_seconds;
        if (dispatch_semaphore_wait(done, dispatch_time(DISPATCH_TIME_NOW, (int64_t)bounded * NSEC_PER_SEC)) != 0) { @synchronized (context) { outcome = 2; } }
        [context invalidate];
        @synchronized (context) { return outcome; }
    }
}

// Traditional login Keychain, not the separately provisioned data-protection store.
// All background reads fail rather than displaying an unlock/access prompt.
int32_t fetch_keychain_read(const char *service, const char *account, uint8_t *output, size_t expected) {
    @autoreleasepool {
        if (!service || !account || !output || expected != 32) return errSecParam;
        NSDictionary *query = @{ (__bridge id)kSecClass:(__bridge id)kSecClassGenericPassword,
            (__bridge id)kSecAttrService:[NSString stringWithUTF8String:service],
            (__bridge id)kSecAttrAccount:[NSString stringWithUTF8String:account],
            (__bridge id)kSecMatchLimit:(__bridge id)kSecMatchLimitOne,
            (__bridge id)kSecReturnData:@YES,
            (__bridge id)kSecUseAuthenticationUI:(__bridge id)kSecUseAuthenticationUIFail };
        CFTypeRef value = NULL;
        OSStatus status = SecItemCopyMatching((__bridge CFDictionaryRef)query, &value);
        if (status != errSecSuccess) return status;
        NSData *data = CFBridgingRelease(value);
        if (![data isKindOfClass:[NSData class]] || data.length != expected) return errSecDecode;
        memcpy(output, data.bytes, expected);
        return errSecSuccess;
    }
}
int32_t fetch_keychain_create(const char *service, const char *account, const uint8_t *input, size_t length) {
    @autoreleasepool {
        if (!service || !account || !input || length != 32) return errSecParam;
        SecTrustedApplicationRef application = NULL;
        OSStatus status = SecTrustedApplicationCreateFromPath(NULL, &application);
        if (status != errSecSuccess) return status;
        NSArray *trusted = @[(__bridge id)application];
        SecAccessRef access = NULL;
        status = SecAccessCreate(CFSTR("Fetch encrypted local state"), (__bridge CFArrayRef)trusted, &access);
        CFRelease(application);
        if (status != errSecSuccess) return status;
        NSDictionary *attributes = @{ (__bridge id)kSecClass:(__bridge id)kSecClassGenericPassword,
            (__bridge id)kSecAttrService:[NSString stringWithUTF8String:service],
            (__bridge id)kSecAttrAccount:[NSString stringWithUTF8String:account],
            (__bridge id)kSecAttrLabel:@"Fetch encrypted local state",
            (__bridge id)kSecAttrAccess:(__bridge id)access,
            (__bridge id)kSecValueData:[NSData dataWithBytes:input length:length],
            (__bridge id)kSecUseAuthenticationUI:(__bridge id)kSecUseAuthenticationUIFail };
        status = SecItemAdd((__bridge CFDictionaryRef)attributes, NULL);
        CFRelease(access);
        return status;
    }
}

// Preserve Iced's application delegate and event loop. Actions are consumed by its UI subscription.
#include <stdatomic.h>
static _Atomic int fetch_app_action = 0;
@interface FetchMenuActions : NSObject
- (void)showFetch:(id)sender;
- (void)quitFetch:(id)sender;
- (void)didBecomeActive:(NSNotification *)notification;
@end
@implementation FetchMenuActions
- (void)showFetch:(id)sender { (void)sender; atomic_store(&fetch_app_action, 1); }
- (void)quitFetch:(id)sender { (void)sender; atomic_store(&fetch_app_action, 2); }
- (void)didBecomeActive:(NSNotification *)notification {
    (void)notification;
    BOOL visible = NO;
    for (NSWindow *window in NSApp.windows) { if (window.visible) { visible = YES; break; } }
    if (!visible) atomic_store(&fetch_app_action, 1);
}
@end
static FetchMenuActions *fetch_menu_actions = nil;
// The same embedded artwork is used by the unbundled development executable.
void fetch_set_app_icon(const uint8_t *bytes, size_t length) {
    @autoreleasepool {
        NSData *data = [NSData dataWithBytes:bytes length:length];
        void (^install)(void) = ^{
            @autoreleasepool {
                NSImage *icon = [[NSImage alloc] initWithData:data];
                if (!icon) return;
                NSApplication *application = [NSApplication sharedApplication];
                application.applicationIconImage = icon;
                [application.dockTile display];
            }
        };
        if ([NSThread isMainThread]) install(); else dispatch_async(dispatch_get_main_queue(), install);
    }
}
void fetch_install_app_menu(void) {
    void (^install)(void) = ^{
        if (fetch_menu_actions) return;
        fetch_menu_actions = [[FetchMenuActions alloc] init];
        [[NSNotificationCenter defaultCenter] addObserver:fetch_menu_actions selector:@selector(didBecomeActive:) name:NSApplicationDidBecomeActiveNotification object:NSApp];
        NSMenu *main = NSApp.mainMenu;
        if (!main) { main = [[NSMenu alloc] initWithTitle:@""]; NSApp.mainMenu = main; }
        NSMenuItem *root = main.numberOfItems > 0 ? [main itemAtIndex:0] : nil;
        if (!root) { root = [[NSMenuItem alloc] initWithTitle:@"Fetch" action:nil keyEquivalent:@""]; [main insertItem:root atIndex:0]; }
        NSMenu *application = [[NSMenu alloc] initWithTitle:@"Fetch"];
        NSMenuItem *show = [[NSMenuItem alloc] initWithTitle:@"Show Fetch" action:@selector(showFetch:) keyEquivalent:@"0"];
        show.target = fetch_menu_actions; [application addItem:show];
        [application addItem:[NSMenuItem separatorItem]];
        NSMenuItem *quit = [[NSMenuItem alloc] initWithTitle:@"Quit Fetch" action:@selector(quitFetch:) keyEquivalent:@"q"];
        quit.target = fetch_menu_actions; [application addItem:quit];
        root.submenu = application;
    };
    if ([NSThread isMainThread]) install(); else dispatch_async(dispatch_get_main_queue(), install);
}
int fetch_take_app_action(void) { return atomic_exchange(&fetch_app_action, 0); }

// Real AppKit traffic lights on the exact content view supplied by Iced.
// Keeping the frame borderless avoids a second outline around Fetch's surface.
// The native theme widgets cache their group-hover drawing state.
@interface NSButton (FetchThemeHover)
- (void)mouseEnteredOrExited;
@end

@interface FetchWindowControls : NSView
@property(nonatomic, strong) NSTrackingArea *hoverArea;
@property(nonatomic) BOOL mouseInside;
- (void)closeWindow:(id)sender;
- (void)minimizeWindow:(id)sender;
- (void)zoomWindow:(id)sender;
@end
@implementation FetchWindowControls
- (void)updateTrackingAreas {
    [super updateTrackingAreas];
    if (self.hoverArea) [self removeTrackingArea:self.hoverArea];
    self.hoverArea = [[NSTrackingArea alloc] initWithRect:NSZeroRect
        options:NSTrackingMouseEnteredAndExited | NSTrackingActiveAlways | NSTrackingInVisibleRect
        owner:self userInfo:nil];
    [self addTrackingArea:self.hoverArea];
}
- (void)setGroupHovered:(BOOL)hovered {
    self.mouseInside = hovered;
    for (NSButton *button in self.subviews) {
        if ([button respondsToSelector:@selector(mouseEnteredOrExited)]) [button mouseEnteredOrExited];
        button.needsDisplay = YES;
    }
}
- (void)mouseEntered:(NSEvent *)event {
    [self setGroupHovered:YES];
    (void)event;
}
- (void)mouseExited:(NSEvent *)event {
    [self setGroupHovered:NO];
    (void)event;
}
// AppKit's standard traffic-light buttons query their containing view for
// group hover. This hook keeps AppKit drawing the real system glyphs.
- (BOOL)_mouseInGroup:(NSButton *)button { (void)button; return self.mouseInside; }
- (void)closeWindow:(id)sender {
    (void)sender;
    // performClose requires a built-in titlebar close widget. A borderless
    // window instead delegates the same close request to winit/Iced directly.
    NSWindow *window = self.window;
    id<NSWindowDelegate> delegate = window.delegate;
    if (![delegate respondsToSelector:@selector(windowShouldClose:)] || [delegate windowShouldClose:window]) [window close];
}
- (void)minimizeWindow:(id)sender { [self.window miniaturize:sender]; }
- (void)zoomWindow:(id)sender { [self.window zoom:sender]; }
- (void)mouseDown:(NSEvent *)event { [self.window performWindowDragWithEvent:event]; }
@end

// Keep native AppKit resizing, but align its target with Fetch's rounded face.
#import <objc/runtime.h>
@interface NSWindow (FetchResizeSPI)
- (NSInteger)_resizeDirectionForMouseLocation:(NSPoint)point forTouch:(BOOL)touch;
- (NSCursor *)_cursorForResizeDirection:(NSInteger)direction;
- (void)_resizeWithEvent:(NSEvent *)event;
- (void)_noteExteriorResizeMarginsMayHaveChanged;
- (void)_updateEdgeResizingTrackingAreas;
@end
static BOOL FetchInsideRoundedRect(NSRect bounds,CGFloat radius,NSPoint point) {
    if(!NSPointInRect(point,bounds))return NO;
    CGFloat cx=MIN(MAX(point.x,NSMinX(bounds)+radius),NSMaxX(bounds)-radius);
    CGFloat cy=MIN(MAX(point.y,NSMinY(bounds)+radius),NSMaxY(bounds)-radius);
    CGFloat dx=point.x-cx,dy=point.y-cy;
    return dx*dx+dy*dy<=radius*radius;
}
static BOOL FetchOnResizeEdge(NSRect bounds,NSPoint point) {
    CGFloat radius=MIN(32,MIN(bounds.size.width,bounds.size.height)/2);
    return FetchInsideRoundedRect(bounds,radius,point)&&!FetchInsideRoundedRect(NSInsetRect(bounds,6,6),MAX(0,radius-6),point);
}
static char FetchResizeDirectionsKey;
static NSInteger FetchNativeResizeDirection(NSWindow *window,NSPoint point) {
    NSRect bounds=NSMakeRect(0,0,window.frame.size.width,window.frame.size.height);
    if (!FetchOnResizeEdge(bounds,point)) return -1;
    NSArray<NSNumber *> *directions=objc_getAssociatedObject(window,&FetchResizeDirectionsKey);
    BOOL left=point.x<32,right=point.x>bounds.size.width-32;
    BOOL bottom=point.y<32,top=point.y>bounds.size.height-32;
    NSUInteger edge=right?(top?1:(bottom?7:0)):(left?(top?3:(bottom?5:4)):(top?2:6));
    return directions.count==8 ? directions[edge].integerValue : -1;
}
typedef struct { IMP single; IMP touch; IMP exterior; } FetchResizeHooks;
static char FetchResizeHooksKey;
static FetchResizeHooks *FetchHooksForWindow(NSWindow *window) {
    for(Class type=object_getClass(window);type;type=class_getSuperclass(type)){
        NSValue *value=objc_getAssociatedObject(type,&FetchResizeHooksKey);
        if(value)return value.pointerValue;
    }
    return NULL;
}
static NSInteger FetchScopedResizeDirection(id window,SEL selector,NSPoint point) {
    if(objc_getAssociatedObject(window,&FetchResizeDirectionsKey))return FetchNativeResizeDirection(window,point);
    FetchResizeHooks *hooks=FetchHooksForWindow(window);
    NSInteger (*native)(id,SEL,NSPoint)=(void *)hooks->single;return native(window,selector,point);
}
static NSInteger FetchScopedResizeDirectionForTouch(id window,SEL selector,NSPoint point,BOOL touch) {
    if(objc_getAssociatedObject(window,&FetchResizeDirectionsKey))return FetchNativeResizeDirection(window,point);
    FetchResizeHooks *hooks=FetchHooksForWindow(window);
    NSInteger (*native)(id,SEL,NSPoint,BOOL)=(void *)hooks->touch;return native(window,selector,point,touch);
}
static BOOL FetchScopedExteriorResize(id window,SEL selector) {
    if(objc_getAssociatedObject(window,&FetchResizeDirectionsKey))return NO;
    FetchResizeHooks *hooks=FetchHooksForWindow(window);
    BOOL (*native)(id,SEL)=(void *)hooks->exterior;return native(window,selector);
}
@interface FetchResizeGrip : NSView
@end
@implementation FetchResizeGrip
- (BOOL)isOpaque { return NO; }
- (NSView *)hitTest:(NSPoint)point {
    NSPoint local=[self convertPoint:point fromView:self.superview];
    return FetchOnResizeEdge(self.bounds,local) ? self : nil;
}
- (void)resetCursorRects {
    [super resetCursorRects];
    // Register only pixels in the curved six-point ring, not transparent
    // corner squares or a larger rectangle outside the visible window.
    NSRect b=self.bounds;CGFloat w=b.size.width,h=b.size.height;
    NSRect straight[]={NSMakeRect(0,32,6,MAX(0,h-64)),NSMakeRect(w-6,32,6,MAX(0,h-64)),NSMakeRect(32,0,MAX(0,w-64),6),NSMakeRect(32,h-6,MAX(0,w-64),6)};
    for(int index=0;index<4;index++){
        NSPoint center=NSMakePoint(NSMidX(straight[index]),NSMidY(straight[index]));
        NSInteger direction=FetchNativeResizeDirection(self.window,center);
        if(direction!=-1)[self addCursorRect:straight[index] cursor:[self.window _cursorForResizeDirection:direction]];
    }
    for(int corner=0;corner<4;corner++){
        CGFloat ox=(corner&1)?w-32:0,oy=(corner&2)?h-32:0;
        NSPoint probe=NSMakePoint((corner&1)?w-10:10,(corner&2)?h-10:10);
        NSInteger direction=FetchNativeResizeDirection(self.window,probe);
        if(direction==-1)continue;
        NSCursor *cursor=[self.window _cursorForResizeDirection:direction];
        for(int y=0;y<32;y++){
            int first=-1,last=-1;
            for(int x=0;x<32;x++)if(FetchOnResizeEdge(b,NSMakePoint(ox+x+0.5,oy+y+0.5))){if(first<0)first=x;last=x;}
            if(first>=0)[self addCursorRect:NSMakeRect(ox+first,oy+y,last-first+1,1) cursor:cursor];
        }
    }
}
- (void)setFrameSize:(NSSize)size { [super setFrameSize:size];[self.window invalidateCursorRectsForView:self]; }
- (void)mouseDown:(NSEvent *)event { if(FetchNativeResizeDirection(self.window,event.locationInWindow)!=-1)[self.window _resizeWithEvent:event]; }
@end
static void FetchInstallResizeEdges(NSWindow *window,NSView *view) {
    for(NSView *child in view.subviews)if([child isKindOfClass:FetchResizeGrip.class])return;
    SEL direction=@selector(_resizeDirectionForMouseLocation:forTouch:);
    SEL single=NSSelectorFromString(@"_resizeDirectionForMouseLocation:");
    SEL exterior=NSSelectorFromString(@"_allowsExteriorResizing");
    if(![window respondsToSelector:exterior]||![window respondsToSelector:direction]||![window respondsToSelector:single]||![window respondsToSelector:@selector(_resizeWithEvent:)]||![window respondsToSelector:@selector(_cursorForResizeDirection:)])return;
    // Learn AppKit's direction values before installing the instance hooks.
    // Its implementation can call the other direction selector, so calling
    // an inherited implementation from a hook would recurse.
    CGFloat w=window.frame.size.width,h=window.frame.size.height;
    NSPoint probes[]={NSMakePoint(w-1,h/2),NSMakePoint(w-1,h-1),NSMakePoint(w/2,h-1),NSMakePoint(1,h-1),NSMakePoint(1,h/2),NSMakePoint(1,1),NSMakePoint(w/2,1),NSMakePoint(w-1,1)};
    NSMutableArray<NSNumber *> *directions=[NSMutableArray array];
    for(int index=0;index<8;index++)[directions addObject:@([window _resizeDirectionForMouseLocation:probes[index] forTouch:NO])];
    objc_setAssociatedObject(window,&FetchResizeDirectionsKey,directions,OBJC_ASSOCIATION_RETAIN_NONATOMIC);
    // Register on Fetch's own concrete window class, with per-instance guards.
    // Do not replace the instance class: AppKit's KVO observers own that class
    // relationship, and changing it breaks window teardown.
    Class base=window.class;
    if(base==NSWindow.class)return;
    if(!objc_getAssociatedObject(base,&FetchResizeHooksKey)){
        FetchResizeHooks *hooks=calloc(1,sizeof(*hooks));
        hooks->single=class_getMethodImplementation(base,single);
        hooks->touch=class_getMethodImplementation(base,direction);
        hooks->exterior=class_getMethodImplementation(base,exterior);
        objc_setAssociatedObject(base,&FetchResizeHooksKey,[NSValue valueWithPointer:hooks],OBJC_ASSOCIATION_RETAIN_NONATOMIC);
        class_replaceMethod(base,single,(IMP)FetchScopedResizeDirection,method_getTypeEncoding(class_getInstanceMethod(base,single)));
        class_replaceMethod(base,direction,(IMP)FetchScopedResizeDirectionForTouch,method_getTypeEncoding(class_getInstanceMethod(base,direction)));
        class_replaceMethod(base,exterior,(IMP)FetchScopedExteriorResize,method_getTypeEncoding(class_getInstanceMethod(base,exterior)));
    }
    FetchResizeGrip *grip=[[FetchResizeGrip alloc]initWithFrame:view.bounds];
    grip.autoresizingMask=NSViewWidthSizable|NSViewHeightSizable;
    [view addSubview:grip];
    if([window respondsToSelector:@selector(_noteExteriorResizeMarginsMayHaveChanged)])[window _noteExteriorResizeMarginsMayHaveChanged];
    if([window respondsToSelector:@selector(_updateEdgeResizingTrackingAreas)])[window _updateEdgeResizingTrackingAreas];
    [window invalidateCursorRectsForView:grip];
}

void fetch_configure_window(void *content_view) {
    if (!content_view || ![NSThread isMainThread]) return;
    NSView *view = (__bridge NSView *)content_view;
    NSWindow *window = view.window;
    if (!window) return;
    FetchObserveScroll(window);
    window.styleMask |= NSWindowStyleMaskClosable | NSWindowStyleMaskMiniaturizable | NSWindowStyleMaskResizable;
    window.opaque = NO;
    window.backgroundColor = NSColor.clearColor;
    window.hasShadow = NO;
    [window invalidateShadow];
    FetchInstallResizeEdges(window,view);
    for (NSView *child in view.subviews) { if ([child isKindOfClass:FetchWindowControls.class]) return; }
    FetchWindowControls *controls = [[FetchWindowControls alloc] initWithFrame:NSZeroRect];
    controls.translatesAutoresizingMaskIntoConstraints = NO;
    [view addSubview:controls];
    [NSLayoutConstraint activateConstraints:@[
        [controls.leadingAnchor constraintEqualToAnchor:view.leadingAnchor constant:24],
        [controls.topAnchor constraintEqualToAnchor:view.topAnchor constant:18],
        [controls.widthAnchor constraintEqualToConstant:54],
        [controls.heightAnchor constraintEqualToConstant:16]
    ]];
    NSWindowButton types[] = {NSWindowCloseButton, NSWindowMiniaturizeButton, NSWindowZoomButton};
    SEL actions[] = {@selector(closeWindow:), @selector(minimizeWindow:), @selector(zoomWindow:)};
    NSWindowStyleMask style = NSWindowStyleMaskTitled | NSWindowStyleMaskClosable | NSWindowStyleMaskMiniaturizable | NSWindowStyleMaskResizable;
    for (NSUInteger index = 0; index < 3; index++) {
        NSButton *button = [NSWindow standardWindowButton:types[index] forStyleMask:style];
        button.frame = NSMakeRect(index * 20, 1, 14, 14);
        button.target = controls;
        button.action = actions[index];
        [controls addSubview:button];
    }
    [controls setGroupHovered:NO];
}
