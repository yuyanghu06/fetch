// Construct and reuse a hidden panel. Never present a dialog or read a chosen file.
#import "../src/native_auth/bridge.m"
#include <assert.h>
int main(void) {
    @autoreleasepool {
        [NSApplication sharedApplication];
        [NSApp setActivationPolicy:NSApplicationActivationPolicyProhibited];
        CFAbsoluteTime start=CFAbsoluteTimeGetCurrent();
        NSOpenPanel *first=fetch_prepared_config_file_panel();
        double cold=(CFAbsoluteTimeGetCurrent()-start)*1000.0;
        start=CFAbsoluteTimeGetCurrent();
        NSOpenPanel *second=fetch_prepared_config_file_panel();
        double cached=(CFAbsoluteTimeGetCurrent()-start)*1000.0;
        assert(first && first==second);
        assert(!first.visible && fetch_config_file_panel==nil);
        printf("PASS: hidden panel reused; construction %.2f ms, cached access %.4f ms; no dialog presented\n",cold,cached);
    }
}
