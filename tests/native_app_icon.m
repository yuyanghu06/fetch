// Disposable AppKit process only. No Fetch state, windows, accounts or services.
#import "../src/native_auth/bridge.m"
#include <assert.h>

static NSBitmapImageRep *raster(NSImage *image) {
    NSBitmapImageRep *bitmap = [[NSBitmapImageRep alloc]
        initWithBitmapDataPlanes:NULL pixelsWide:256 pixelsHigh:256
        bitsPerSample:8 samplesPerPixel:4 hasAlpha:YES isPlanar:NO
        colorSpaceName:NSDeviceRGBColorSpace bytesPerRow:1024 bitsPerPixel:32];
    memset(bitmap.bitmapData,0,bitmap.bytesPerRow * bitmap.pixelsHigh);
    NSGraphicsContext *context = [NSGraphicsContext graphicsContextWithBitmapImageRep:bitmap];
    [NSGraphicsContext saveGraphicsState];
    NSGraphicsContext.currentContext = context;
    [image drawInRect:NSMakeRect(0,0,256,256) fromRect:NSZeroRect
        operation:NSCompositingOperationCopy fraction:1];
    [context flushGraphics];
    [NSGraphicsContext restoreGraphicsState];
    return bitmap;
}

static void assertMatches(NSImage *expected) {
    assert(NSApp.applicationIconImage != nil);
    NSBitmapImageRep *reference = raster(expected);
    NSBitmapImageRep *actual = raster(NSApp.applicationIconImage);
    assert(reference.bytesPerRow == actual.bytesPerRow);
    size_t changed=0; unsigned int maximum=0; unsigned long long total=0;
    for (size_t i=0;i<actual.bytesPerRow*actual.pixelsHigh;i++) {
        unsigned int delta=abs((int)reference.bitmapData[i]-(int)actual.bitmapData[i]);
        if (delta) changed++;
        if (delta>maximum) maximum=delta;
        total+=delta;
    }
    size_t channelCount=actual.bytesPerRow*actual.pixelsHigh;
    // AppKit returns an NSCGImageSnapshotRep, resampled from the source PNG.
    // Compare the entire raster within one 8-bit channel level on average.
    // Exact byte equality would reject AppKit's own antialiasing at edges.
    double mean=(double)total/channelCount;
    printf("Native raster comparison: mean channel delta %.6f/255; max %u; changed %zu/%zu.\n",
        mean,maximum,changed,channelCount);
    assert(mean < 1.0);
}

int main(int argc, const char **argv) {
    @autoreleasepool {
        assert(argc == 3);
        [NSApplication sharedApplication];
        [NSApp setActivationPolicy:NSApplicationActivationPolicyProhibited];
        NSString *path = @(argv[1]);
        NSData *png = [NSData dataWithContentsOfFile:path];
        NSBitmapImageRep *source = [NSBitmapImageRep imageRepWithData:png];
        assert(source.pixelsWide == 1024 && source.pixelsHigh == 1024 && source.hasAlpha);
        NSImage *expected = [[NSImage alloc] initWithData:png];
        assert(expected != nil);
        assert(expected.size.width == 1024 && expected.size.height == 1024);

        fetch_set_app_icon(png.bytes, png.length);
        assertMatches(expected);
        assert(NSApp.applicationIconImage.size.width == 1024 && NSApp.applicationIconImage.size.height == 1024);
        puts("PASS: actual AppKit application icon matches the approved PNG within native rasterization tolerance, at 1024pt.");

        NSImage *sentinel = [[NSImage alloc] initWithSize:NSMakeSize(1024,1024)];
        [sentinel lockFocus];
        [NSColor.magentaColor setFill]; NSRectFill(NSMakeRect(0,0,1024,1024));
        [sentinel unlockFocus];
        NSApp.applicationIconImage = sentinel;
        __block BOOL applied = NO;
        dispatch_semaphore_t returned = dispatch_semaphore_create(0);
        dispatch_async(dispatch_get_global_queue(QOS_CLASS_DEFAULT,0), ^{
            @autoreleasepool {
                NSMutableData *temporary = [png mutableCopy];
                fetch_set_app_icon(temporary.bytes, temporary.length);
                [temporary resetBytesInRange:NSMakeRange(0,temporary.length)];
            }
            dispatch_async(dispatch_get_main_queue(), ^{ applied = YES; });
            dispatch_semaphore_signal(returned);
        });
        assert(dispatch_semaphore_wait(returned,
            dispatch_time(DISPATCH_TIME_NOW,5*NSEC_PER_SEC)) == 0);
        NSDate *deadline = [NSDate dateWithTimeIntervalSinceNow:5];
        while (!applied && deadline.timeIntervalSinceNow > 0) {
            [[NSRunLoop currentRunLoop] runUntilDate:[NSDate dateWithTimeIntervalSinceNow:0.01]];
        }
        assert(applied);
        assertMatches(expected);
        assert(NSApp.windows.count == 0);
        puts("PASS: background-thread assignment survives overwritten/released source bytes and creates no windows.");

        NSData *preview = [raster(NSApp.applicationIconImage)
            representationUsingType:NSBitmapImageFileTypePNG properties:@{}];
        assert([preview writeToFile:@(argv[2]) atomically:YES]);
    }
}
