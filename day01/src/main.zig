const std = @import("std");

pub fn main() !void {
    var file = try std.fs.cwd().openFile("input.txt", .{ .mode = .read_only });

    defer file.close();

    const contents = try file.readToEndAlloc(std.heap.page_allocator, std.math.maxInt(usize));
    defer std.heap.page_allocator.free(contents);

    try part_01(contents);
    try part_02(contents);
}

fn part_01(contents: []const u8) !void {
    var stdout = std.io.getStdOut().writer();
    var lines = std.mem.tokenizeSequence(u8, contents, "\n");

    var left = std.ArrayList(u32).init(std.heap.page_allocator);
    defer left.deinit();

    var right = std.ArrayList(u32).init(std.heap.page_allocator);
    defer right.deinit();

    while (lines.next()) |line| {
        var split = std.mem.tokenizeScalar(u8, line, ' ');
        const left_val = try std.fmt.parseInt(u32, split.next().?, 10);
        const right_val = try std.fmt.parseInt(u32, split.next().?, 10);

        try left.append(left_val);
        try right.append(right_val);
    }

    std.mem.sort(u32, left.items, {}, std.sort.asc(u32));
    std.mem.sort(u32, right.items, {}, std.sort.asc(u32));

    var sum: u32 = 0;
    for (0.., left.items, right.items) |i, left_val, right_val| {
        stdout.print("{i}", .{i});
        const l: i32 = @intCast(left_val);
        const r: i32 = @intCast(right_val);

        const diff = @abs(l - r);
        sum += diff;
    }

    try stdout.print("{d}\n", .{sum});
}

fn part_02(contents: []const u8) !void {
    var lines = std.mem.tokenizeSequence(u8, contents, "\n");

    var left = std.ArrayList(u32).init(std.heap.page_allocator);
    defer left.deinit();

    var right = std.ArrayList(u32).init(std.heap.page_allocator);
    defer right.deinit();

    while (lines.next()) |line| {
        var split = std.mem.tokenizeScalar(u8, line, ' ');
        const left_val = try std.fmt.parseInt(u32, split.next().?, 10);
        const right_val = try std.fmt.parseInt(u32, split.next().?, 10);

        try left.append(left_val);
        try right.append(right_val);
    }

    var map = std.AutoHashMap(u32, u32).init(std.heap.page_allocator);
    defer map.deinit();

    for (right.items) |right_val| {
        // get value from map, or 0
        var val = if (map.get(right_val)) |v| v else 0;
        val += 1;
        try map.put(right_val, val);
    }

    var sum: u32 = 0;
    for (left.items) |left_val| {
        const val = map.get(left_val).?;
        sum += val * left_val;
    }

    var stdout = std.io.getStdOut().writer();
    try stdout.print("{d}\n", .{sum});
}
