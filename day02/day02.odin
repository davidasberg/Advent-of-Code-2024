
package main

import "core:fmt"
import "core:os"
import "core:strconv"
import "core:strings"

Report :: struct {
	values: []int,
}

dampen_report_at_index :: proc(report: Report, i: int) -> Report {
	new_values: [dynamic]int
	for j := 0; j < len(report.values); j += 1 {
		if j != i {
			append(&new_values, report.values[j])
		}
	}
	return Report{new_values[:]}
}

report_is_safe :: proc(report: Report) -> bool {
	// if all values are either descending or ascending
	// and if any two adjacent values differ by at least one and at most 3
	ascending := false
	descending := false
	prev := report.values[0]
	for i := 1; i < len(report.values); i += 1 {
		current := report.values[i]
		if current == prev {
			return false
		}

		if current > prev && !descending {
			ascending = true
		} else if current < prev && !ascending {
			descending = true
		} else {
			return false
		}

		abs_diff := abs(current - prev)

		if abs_diff < 1 || abs_diff > 3 {
			return false
		}

		prev = current
	}
	return true
}

main :: proc() {
	part01()
	part02()
}

part01 :: proc() {
	reports := get_reports()
	defer delete(reports)

	safe_report_count := 0
	for report in reports {
		if report_is_safe(report) {
			safe_report_count += 1
		}
	}

	fmt.println("Part 1:", safe_report_count)
}

part02 :: proc() {
	reports := get_reports()
	defer delete(reports)

	safe_report_count := 0
	for report in reports {
		if report_is_safe(report) {
			safe_report_count += 1
		} else {
			for _, i in report.values {
				if report_is_safe(dampen_report_at_index(report, i)) {
					safe_report_count += 1
					break
				}
			}
		}
	}

	fmt.println("Part 2:", safe_report_count)
}

get_reports :: proc() -> []Report {
	data, ok := os.read_entire_file("input.txt", context.allocator)
	if !ok {
		fmt.println("could not read file")
		return nil
	}

	input := string(data)
	lines, _ := strings.split_lines(input)
	reports := make([]Report, len(lines))
	for line, row in lines {
		numbers, _ := strings.split(line, " ")
		report := Report{make([]int, len(numbers))}
		for &number, column in numbers {
			value, _ := strconv.parse_int(number)
			report.values[column] = value
		}
		reports[row] = report
	}

	return reports
}
