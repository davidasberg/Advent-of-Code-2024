package main

import "core:fmt"
import "core:os"
import "core:slice"
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

Page_Updates :: struct {
	pages:          [dynamic]int,
	ordering_rules: map[int][dynamic]int,
}

fix_page :: proc(page_section: [dynamic]int, ordering_rules: map[int][dynamic]int) {

	page_updates := Page_Updates {
		pages          = page_section,
		ordering_rules = ordering_rules,
	}

	it := sort.Interface {
		len = proc(it: sort.Interface) -> int {
			page_updates := cast(^Page_Updates)it.collection
			return len(page_updates.pages)
		},
		less = proc(it: sort.Interface, i, j: int) -> bool {
			page_section := cast(^Page_Updates)it.collection

			i_value := page_section.pages[i]
			j_value := page_section.pages[j]

			i_rules := page_section.ordering_rules[i_value]
			j_rules := page_section.ordering_rules[j_value]

			i_has_j := slice.contains(i_rules[:], j_value)
			j_has_i := slice.contains(j_rules[:], i_value)

			if i_has_j {
				return true
			}

			if j_has_i {
				return false
			}

			return page_section.pages[i] < page_section.pages[j]
		},
		swap = proc(it: sort.Interface, i, j: int) {
			page_section := cast(^Page_Updates)it.collection
			page_section.pages[i], page_section.pages[j] =
				page_section.pages[j], page_section.pages[i]
		},
		collection = &page_updates,
	}

	sort.sort(it)
}

part01 :: proc() {
	rules, sections := get_input()

	ordering_rules := parse_rules(rules)

	page_sections := parse_sections(sections)
	fmt.println(ordering_rules)
	fmt.println(page_sections)
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
	defer delete(page_sections)
	sum := 0
	for page_section, i in page_sections {
		middle, ok := check_page(page_section, ordering_rules)
		if !ok {
			fix_page(page_section, ordering_rules)
			middle, ok := check_page(page_section, ordering_rules)
			if ok {
				sum += middle
			} else {
				fmt.println("could not fix page")
			}
		}
	}

	fmt.println(sum)
}
