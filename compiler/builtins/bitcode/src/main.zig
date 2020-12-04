const builtin = @import("builtin");
const std = @import("std");
const testing = std.testing;

// Num Module
const num = @import("num.zig");
comptime {
    exportNumFn(num.atan, "atan");
    exportNumFn(num.isFinite, "is_finite");
    exportNumFn(num.powInt, "pow_int");
    exportNumFn(num.acos, "acos");
    exportNumFn(num.asin, "asin");
}

// Str Module
const str = @import("str.zig");
comptime {
    exportStrFn(str.strSplitInPlaceC, "str_split_in_place");
    exportStrFn(str.countSegments, "count_segments");
    exportStrFn(str.countGraphemeClusters, "count_grapheme_clusters");
    exportStrFn(str.startsWith, "starts_with");
    exportStrFn(str.endsWith, "ends_with");
    exportStrFn(str.strConcatC, "concat");
    exportStrFn(str.strNumberOfBytes, "number_of_bytes");
    exportStrFn(str.strFromIntC, "from_int");
}

// Export helpers - Must be run inside a comptime
fn exportBuiltinFn(comptime func: anytype, comptime funcName: []const u8) void {
    @export(func, .{ .name = "roc_builtins." ++ funcName, .linkage = .Strong });
}
fn exportNumFn(comptime func: anytype, comptime funcName: []const u8) void {
    exportBuiltinFn(func, "num." ++ funcName);
}
fn exportStrFn(comptime func: anytype, comptime funcName: []const u8) void {
    exportBuiltinFn(func, "str." ++ funcName);
}

// Run all tests in imported modules
// https://github.com/ziglang/zig/blob/master/lib/std/std.zig#L94
test "" {
    testing.refAllDecls(@This());
}
