// Hidden, disposable AppKit window. No Fetch state, accounts or services.
#import "../src/native_auth/bridge.m"
#include <assert.h>

@interface FixtureWindow : NSWindow
@end
@implementation FixtureWindow
- (BOOL)isKeyWindow { return YES; }
- (BOOL)isMainWindow { return YES; }
@end
@interface CloseRecorder : NSObject <NSWindowDelegate>
@property(nonatomic) BOOL requested;
@end
@implementation CloseRecorder
- (BOOL)windowShouldClose:(NSWindow *)window { (void)window; self.requested = YES; return NO; }
@end

static NSData *capture(NSView *view, NSString *path) {
    NSBitmapImageRep *image = [view bitmapImageRepForCachingDisplayInRect:view.bounds];
    [view displayIfNeeded];
    [view cacheDisplayInRect:view.bounds toBitmapImageRep:image];
    NSData *png = [image representationUsingType:NSBitmapImageFileTypePNG properties:@{}];
    assert(png.length > 0);
    if (path) assert([png writeToFile:path atomically:YES]);
    return png;
}
int main(int argc, const char **argv) {
    @autoreleasepool {
        [NSApplication sharedApplication];
        FixtureWindow *window = [[FixtureWindow alloc] initWithContentRect:NSMakeRect(0,0,874,630)
            styleMask:NSWindowStyleMaskBorderless backing:NSBackingStoreBuffered defer:NO];
        window.releasedWhenClosed = NO;
        CloseRecorder *recorder = [CloseRecorder new]; window.delegate = recorder;
        NSView *view = window.contentView;
        NSWindow *untouched=[[FixtureWindow alloc]initWithContentRect:NSMakeRect(0,0,874,630) styleMask:NSWindowStyleMaskResizable backing:NSBackingStoreBuffered defer:NO];
        untouched.releasedWhenClosed=NO;
        NSInteger originalDirection=[untouched _resizeDirectionForMouseLocation:NSMakePoint(5,315) forTouch:NO];
        fetch_configure_window((__bridge void *)view);
        assert([untouched _resizeDirectionForMouseLocation:NSMakePoint(5,315) forTouch:NO]==originalDirection);
        [untouched close];

        [view layoutSubtreeIfNeeded];
        FetchWindowControls *controls = (FetchWindowControls *)view.subviews.lastObject;
        assert([controls isKindOfClass:FetchWindowControls.class]);
        assert(controls.subviews.count == 3);
        FetchResizeGrip *grip=nil;for(NSView *child in view.subviews)if([child isKindOfClass:FetchResizeGrip.class])grip=(FetchResizeGrip *)child;
        assert(grip);
        assert([grip hitTest:NSMakePoint(1,315)]==grip);
        assert([grip hitTest:NSMakePoint(5,315)]==grip);
        assert([grip hitTest:NSMakePoint(7,315)]==nil);
        assert([grip hitTest:NSMakePoint(-5,315)]==nil);
        assert([grip hitTest:NSMakePoint(10,10)]==grip);
        assert([grip hitTest:NSMakePoint(0,0)]==nil);
        assert([grip hitTest:NSMakePoint(437,315)]==nil);
        assert([window _resizeDirectionForMouseLocation:NSMakePoint(5,315) forTouch:NO]!=-1);
        assert([window _resizeDirectionForMouseLocation:NSMakePoint(-5,315) forTouch:NO]==-1);
        [grip resetCursorRects];
        assert(fabs(controls.frame.origin.x - 24) < 0.1);
        assert(fabs(NSMaxY(controls.frame) - (view.bounds.size.height - 18)) < 0.1);
        [controls updateTrackingAreas]; assert(controls.trackingAreas.count > 0);
        NSString *directory = argc > 1 ? @(argv[1]) : nil;
        NSMutableArray<NSData *> *idleButtons = [NSMutableArray array];
        for (NSView *button in controls.subviews) [idleButtons addObject:capture(button,nil)];
        NSData *idle = capture(controls, directory ? [directory stringByAppendingPathComponent:@"window-controls-idle.png"] : nil);
        NSEvent *entered = [NSEvent enterExitEventWithType:NSEventTypeMouseEntered location:[controls convertPoint:NSMakePoint(8,8) toView:nil] modifierFlags:0 timestamp:0 windowNumber:window.windowNumber context:nil eventNumber:1 trackingNumber:0 userData:NULL];
        [controls mouseEntered:entered];
        assert([controls _mouseInGroup:nil]);
        NSData *hover = capture(controls, directory ? [directory stringByAppendingPathComponent:@"window-controls-hover.png"] : nil);
        assert(![idle isEqualToData:hover]);
        for (NSUInteger index=0;index<3;index++) assert(![idleButtons[index] isEqualToData:capture(controls.subviews[index],nil)]);
        NSEvent *exited = [NSEvent enterExitEventWithType:NSEventTypeMouseExited location:NSMakePoint(400,400) modifierFlags:0 timestamp:1 windowNumber:window.windowNumber context:nil eventNumber:2 trackingNumber:0 userData:NULL];
        [controls mouseExited:exited];
        [[NSRunLoop currentRunLoop] runUntilDate:[NSDate dateWithTimeIntervalSinceNow:0.1]];
        assert(![controls _mouseInGroup:nil]);
        for (NSView *button in controls.subviews) [button display];
        NSData *left = capture(controls, directory ? [directory stringByAppendingPathComponent:@"window-controls-left.png"] : nil);
        assert([idle isEqualToData:left]);
        for (NSUInteger index=0;index<3;index++) assert([idleButtons[index] isEqualToData:capture(controls.subviews[index],nil)]);
        NSRect contentRect = [window contentRectForFrameRect:window.frame];
        assert(NSEqualSizes(contentRect.size,window.frame.size));
        assert(NSEqualSizes(view.bounds.size,window.frame.size));
        [window setContentSize:NSMakeSize(760,520)];[view layoutSubtreeIfNeeded];
        assert(fabs(controls.frame.origin.x - 24) < 0.1);
        assert(NSEqualSizes(grip.frame.size,view.bounds.size));
        assert([grip hitTest:NSMakePoint(759,260)]==grip);
        NSButton *close = (NSButton *)controls.subviews[0];
        assert(close.target == controls && close.action == @selector(closeWindow:));
        [close performClick:nil]; assert(recorder.requested);
        assert(!window.visible);
        window.delegate = nil; [window close];
        puts("PASS: top-left positioning, matching frame/content edges, six-point curved inside resize ring, outside exclusion, resize, each of three native hover glyphs, exact leave restoration and close delegation; hidden fixture only.");
    }
}
