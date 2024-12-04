package main

import "core:fmt"
import "core:os"
import "core:strconv"
import "core:strings"
import "core:text/match"
import "core:text/regex"

main :: proc() {
	part01()
	part02()
}

get_input :: proc() -> string {
	data, ok := os.read_entire_file("input.txt", context.allocator)
	if !ok {
		fmt.println("could not read file")
		return ""
	}

	input := string(data)
	return input
}

MUL_PATTERN :: "/(mul\\(\\d+,\\d+\\))/"
DO_PATTERN :: "/do\\(\\)/"
DONT_PATTERN :: "/don't\\(\\)/"

part01 :: proc() {
	program := get_input()
	mul_pattern, err := regex.create_by_user(MUL_PATTERN)
	if err != nil {
		fmt.println(err)
		return
	}

	sum := 0
	for i in 0 ..< len(program) {
		capture, success := regex.match(mul_pattern, program[i:])
		if success {
			fmt.println(capture.groups[0])
			split, err := strings.split_after(capture.groups[0], "mul(")
			numbers_with_comma := strings.split(split[1], ")")
			fmt.println(numbers_with_comma)
			numbers := strings.split(numbers_with_comma[0], ",")

			sum += strconv.atoi(numbers[0]) * strconv.atoi(numbers[1])
		}
	}

	fmt.println(sum)
}


part02 :: proc() {
	program := get_input()


	mul_pattern, mul_err := regex.create_by_user(MUL_PATTERN)
	do_pattern, do_err := regex.create_by_user(DO_PATTERN)
	dont_pattern, dont_err := regex.create_by_user(DONT_PATTERN)
	if mul_err != nil || do_err != nil || dont_err != nil {
		fmt.println(mul_err)
		fmt.println(do_err)
		fmt.println(dont_err)
		return
	}

	sum := 0
	should_do := true
	for i in 0 ..< len(program) {
		mul_capture, mul_success := regex.match(mul_pattern, program[i:])
		do_capture, do_success := regex.match(do_pattern, program[i:])
		dont_capture, dont_success := regex.match(dont_pattern, program[i:])

		if mul_success && should_do {
			split, err := strings.split_after(mul_capture.groups[0], "mul(")
			numbers_with_comma := strings.split(split[1], ")")
			numbers := strings.split(numbers_with_comma[0], ",")

			sum += strconv.atoi(numbers[0]) * strconv.atoi(numbers[1])
		}

		if do_success {
			should_do = true
		}

		if dont_success {
			should_do = false
		}

	}

	fmt.println(sum)
}
