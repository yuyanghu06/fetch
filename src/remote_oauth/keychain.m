// Fetch-as-client remote MCP OAuth grants. No local provider credentials or
// dedicated file-encryption keys. Only called on Rust's blocking worker pool.
#import <Foundation/Foundation.h>
#import <Security/Security.h>
#include <stdint.h>
#include <string.h>

static NSDictionary *fetch_remote_query(const char *service, const char *account) {
    if (!service || !account) return nil;
    NSString *s = [NSString stringWithUTF8String:service];
    NSString *a = [NSString stringWithUTF8String:account];
    if (!s || !a) return nil;
    return @{(__bridge id)kSecClass:(__bridge id)kSecClassGenericPassword,
        (__bridge id)kSecAttrService:s, (__bridge id)kSecAttrAccount:a,
        (__bridge id)kSecUseAuthenticationUI:(__bridge id)kSecUseAuthenticationUIFail};
}
int32_t fetch_remote_grant_read(const char *service, const char *account, uint8_t *output, size_t capacity, size_t *length) {
    @autoreleasepool {
        NSDictionary *base = fetch_remote_query(service, account);
        if (!base || !output || !length || capacity > 1024 * 1024) return errSecParam;
        *length = 0;
        NSMutableDictionary *query = [base mutableCopy];
        query[(__bridge id)kSecMatchLimit] = (__bridge id)kSecMatchLimitOne;
        query[(__bridge id)kSecReturnData] = @YES;
        CFTypeRef value = NULL;
        OSStatus status = SecItemCopyMatching((__bridge CFDictionaryRef)query, &value);
        if (status != errSecSuccess) return status;
        NSData *data = CFBridgingRelease(value);
        if (![data isKindOfClass:[NSData class]] || data.length > capacity) return errSecDecode;
        memcpy(output, data.bytes, data.length);
        *length = data.length;
        return errSecSuccess;
    }
}
int32_t fetch_remote_grant_write(const char *service, const char *account, const uint8_t *input, size_t length) {
    @autoreleasepool {
        NSDictionary *query = fetch_remote_query(service, account);
        if (!query || !input || !length || length > 1024 * 1024) return errSecParam;
        NSData *data = [NSData dataWithBytes:input length:length];
        NSDictionary *change = @{(__bridge id)kSecValueData:data};
        OSStatus status = SecItemUpdate((__bridge CFDictionaryRef)query, (__bridge CFDictionaryRef)change);
        if (status != errSecItemNotFound) return status;
        SecTrustedApplicationRef app = NULL;
        status = SecTrustedApplicationCreateFromPath(NULL, &app);
        if (status != errSecSuccess) return status;
        SecAccessRef access = NULL;
        status = SecAccessCreate(CFSTR("Fetch remote MCP login"), (__bridge CFArrayRef)@[(__bridge id)app], &access);
        CFRelease(app);
        if (status != errSecSuccess) return status;
        NSMutableDictionary *attributes = [query mutableCopy];
        attributes[(__bridge id)kSecAttrLabel] = @"Fetch remote MCP login";
        attributes[(__bridge id)kSecAttrAccess] = (__bridge id)access;
        attributes[(__bridge id)kSecValueData] = data;
        status = SecItemAdd((__bridge CFDictionaryRef)attributes, NULL);
        CFRelease(access);
        if (status == errSecDuplicateItem) return SecItemUpdate((__bridge CFDictionaryRef)query, (__bridge CFDictionaryRef)change);
        return status;
    }
}
