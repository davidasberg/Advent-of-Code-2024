
package main

import "core:fmt"
import "core:os"
import "core:strconv"
import "core:strings"
import "core:unicode/utf8"

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

convert_string_to_grid :: proc(input: string) -> [][]rune {
	lines, err := strings.split(input, "\n")
	if err != nil {
		fmt.println("could not split string")
		return nil
	}
	grid := make([][]rune, len(lines))
	for line, i in lines {
		grid[i] = utf8.string_to_runes(line)
	}

	return grid
}

find_words :: proc(x: int, y: int, word: string, grid: [][]rune) -> int {
	assert(x < len(grid))
	assert(y < len(grid[x]))

	words_found := 0

	for i in -1 ..= 1 {
		for j in -1 ..= 1 {
			dir := [2]int{i, j}
			if check_word_in_direction(x, y, word, grid, dir) {
				words_found += 1
			}
		}
	}

	return words_found
}

check_word_in_direction :: proc(
	i: int,
	j: int,
	word: string,
	grid: [][]rune,
	direction: [2]int,
) -> bool {
	grid_width := len(grid)
	grid_height := len(grid[0])
	for k in 0 ..< len(word) {
		x := i + k * direction[0]
		y := j + k * direction[1]

		if x < 0 || x >= grid_width {
			return false
		}

		if y < 0 || y >= grid_height {
			return false
		}

		cell := grid[x][y]
		expected := rune(word[k])
		if cell != expected {
			return false
		}
	}

	return true
}

// checks if the word appears in an "x", the same word crossing a center point twice
// only works for words of odd length
check_for_x_word :: proc(x: int, y: int, word: string, grid: [][]rune) -> bool {
	assert(len(word) % 2 == 1)

	check_diagonal := proc(
		x: int,
		y: int,
		reverse_dir: bool,
		reverse_word: bool,
		word: string,
		grid: [][]rune,
	) -> bool {
		grid_width := len(grid)
		grid_height := len(grid[0])
		for i in 0 ..< len(word) {
			index := i
			if reverse_word {
				index = len(word) - 1 - i
			}

			row := y + i
			col := x + i * (reverse_dir ? -1 : 1)
			if row < 0 || row >= grid_width {
				return false
			}

			if col < 0 || col >= grid_height {
				return false
			}


			cell := grid[row][col]
			expected := rune(word[index])

			if cell != expected {
				return false
			}
		}
		return true
	}
	a := check_diagonal(x, y, false, false, word, grid)
	b := check_diagonal(x, y, false, true, word, grid)
	c := check_diagonal(x + len(word) - 1, y, true, false, word, grid)
	d := check_diagonal(x + len(word) - 1, y, true, true, word, grid)
	if (a || b) && (c || d) {
		fmt.println("x word found at x: ", x, " y: ", y)
		return true
	}


	return false
}

part01 :: proc() {
	input := get_input()
	grid := convert_string_to_grid(input)

	words_found := 0
	for y in 0 ..< len(grid) {
		for x in 0 ..< len(grid[y]) {
			words_found += find_words(x, y, "XMAS", grid)
		}
	}

	fmt.println(words_found)
}

part02 :: proc() {
	input := get_input()
	grid := convert_string_to_grid(input)

	words_found := 0
	for y in 0 ..< len(grid) {
		for x in 0 ..< len(grid[y]) {
			if check_for_x_word(x, y, "MAS", grid) {
				words_found += 1
			}
		}
	}

	fmt.println(words_found)

}
