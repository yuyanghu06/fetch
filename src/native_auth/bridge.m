// Fetch-owned bridge. No password is returned to Rust, a browser or a child process.
#import <Foundation/Foundation.h>
#import <AppKit/AppKit.h>
#import <LocalAuthentication/LocalAuthentication.h>
#import <Security/Security.h>
#include <stdint.h>
#include <string.h>

// AppKit owns the panel. Rust owns the callback context until this one completion.
typedef void (*FetchConfigFileCallback)(void *context, const char *path, int32_t status);
static NSOpenPanel *fetch_config_file_panel = nil;
void fetch_choose_config_file(FetchConfigFileCallback callback, void *context) {
    dispatch_async(dispatch_get_main_queue(), ^{
        @autoreleasepool {
            NSWindow *parent = NSApp.keyWindow ?: NSApp.mainWindow;
            if (fetch_config_file_panel || !parent) { callback(context, NULL, 2); return; }
            NSOpenPanel *panel = [NSOpenPanel openPanel];
            fetch_config_file_panel = panel;
            panel.title = @"Choose an MCP configuration file";
            panel.prompt = @"Choose";
            panel.canChooseFiles = YES;
            panel.canChooseDirectories = NO;
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
