// No event monitor/window is activated. Exercise the phase forwarding contract.
#import "../src/native_auth/bridge.m"
#include <assert.h>
int main(void){@autoreleasepool{
    FetchRecordScrollPhase(NSEventPhaseBegan,NSEventPhaseNone);
    FetchRecordScrollPhase(NSEventPhaseChanged,NSEventPhaseNone);
    FetchRecordScrollPhase(NSEventPhaseEnded,NSEventPhaseNone);
    FetchRecordScrollPhase(NSEventPhaseNone,NSEventPhaseBegan);
    FetchRecordScrollPhase(NSEventPhaseNone,NSEventPhaseChanged);
    FetchRecordScrollPhase(NSEventPhaseNone,NSEventPhaseEnded);
    FetchRecordScrollPhase(NSEventPhaseBegan,NSEventPhaseNone);
    uint32_t expected[]={7,3,3,3,3,3,7};
    for(int i=0;i<7;i++)assert(fetch_take_scroll_phase()==expected[i]);
    assert(fetch_take_scroll_phase()==0);
    puts("PASS: phase ordering, finger restart, and unchanged ownership through momentum");
}}
