package main

import "core:fmt"
import "core:os"
import "core:sort"
import "core:strconv"
import "core:strings"

main :: proc() {
	part01()
	part02()
}

Safety_Page :: struct {
	page:           []int,
	ordering_rules: map[int][dynamic]int,
}

get_input :: proc() -> (rules: string, sections: string) {
	data, ok := os.read_entire_file("input.txt", context.allocator)

	if !ok {
		fmt.println("could not read file")
		return "", ""
	}

	input := string(data)

	parts := strings.split(input, "\n\n")

	rules = parts[0]
	sections = parts[1]
	return rules, sections
}

parse_rules :: proc(s: string) -> (ordering_rules: map[int][dynamic]int) {
	for line in strings.split(s, "\n") {
		numbers := strings.split(line, "|")

		key := strconv.atoi(numbers[0])
		values := ordering_rules[key]
		new_value := strconv.atoi(numbers[1])
		append(&values, new_value)
		ordering_rules[key] = values
	}
	return ordering_rules
}

parse_sections :: proc(s: string) -> (sections: [dynamic][dynamic]int) {
	for line, i in strings.split(s, "\n") {
		numbers := strings.split(line, ",")
		current_section: [dynamic]int
		for number in numbers {
			current_number := strconv.atoi(number)
			append(&current_section, current_number)
		}
		append(&sections, current_section)
	}
	return sections
}

check_page :: proc(
	page_section: [dynamic]int,
	ordering_rules: map[int][dynamic]int,
) -> (
	middle: int,
	ok: bool,
) {

	seen_pages := make(map[int]bool)
	for page in page_section {
		current_ordering_rules, ok := ordering_rules[page]
		if ok {
			for rule in current_ordering_rules {
				if seen_pages[rule] {
					return 0, false
				}
			}
		}
		seen_pages[page] = true
	}

	middle_index := len(page_section) / 2
	middle = page_section[middle_index]
	return middle, true
}

fix_page :: proc(page_section: [dynamic]int, ordering_rules: map[int][dynamic]int) {
    
	it := sort.Interface {
		len = proc(it: sort.Interface) -> int {
			page_section := cast(^[]int)it.collection
			return len(page_section)
		},
		less = proc(it: sort.Interface, i, j: int) -> bool {
			page_section := cast(^[]int)it.collection

		},
		swap = proc(it: sort.Interface, i, j: int) {
			page_section := cast(^[]int)it.collection
			page_section[i], page_section[j] = page_section[j], page_section[i]
		},
		collection = &page_section,
	}
	sort.sort(it)
}

part01 :: proc() {
	rules, sections := get_input()

	ordering_rules := parse_rules(rules)

	page_sections := parse_sections(sections)
	sum := 0
	for page_section in page_sections {
		middle, ok := check_page(page_section, ordering_rules)
		if ok {
			sum += middle
		}
	}

	fmt.println(sum)
}


part02 :: proc() {
	rules, sections := get_input()

	ordering_rules := parse_rules(rules)

	page_sections := parse_sections(sections)
	sum := 0
	for page_section in page_sections {
		middle, ok := check_page(page_section, ordering_rules)
		if ok {
			sum += middle
		} else {
			fix_page(page_section, ordering_rules)
			middle, ok := check_page(page_section, ordering_rules)
			if ok {
				sum += middle
			} else {
				fmt.println("could not fix page")
			}
		}
	}
}
