
package main

import "core:fmt"
import "core:os"
import "core:strconv"
import "core:strings"

direction :: enum {
	North,
	South,
	East,
	West,
}

Cell :: enum {
	Obstruction,
	Empty,
}

Grid :: struct {
	cells:     [][]Cell,
	guard_pos: [2]int,
	guard_dir: direction,
}

get_input :: proc() -> string {
	data, ok := os.read_entire_file("example.txt", context.allocator)

	if !ok {
		fmt.println("could not read file")
		return ""
	}

	input := string(data)
	return input
}

parse_input :: proc(input: string) -> (grid: Grid) {
	lines := strings.split(input, "\n")
	grid.cells = make([][]Cell, len(lines))
	for line, y in lines {
		grid.cells[y] = make([]Cell, len(line))
		for r, x in line {
			if r == '^' {
				grid.guard_pos = [2]int{x, y}
				grid.guard_dir = direction.North
				grid.cells[y][x] = Cell.Empty
			}
			if r == '#' {
				grid.cells[y][x] = Cell.Obstruction
			}
			if r == '.' {
				grid.cells[y][x] = Cell.Empty
			}
		}
	}

	return grid
}

is_guard_inside_map :: proc(grid: Grid) -> bool {
	for i in 0 ..= 1 {
		if grid.guard_pos[i] < 0 || grid.guard_pos[i] >= len(grid.cells[0]) {
			return false
		}
	}
	return true
}

turn_right :: proc(dir: direction) -> direction {
	switch dir {
	case direction.North:
		return direction.East
	case direction.South:
		return direction.West
	case direction.East:
		return direction.South
	case direction.West:
		return direction.North
	}
	return direction.North
}

step_guard :: proc(grid: ^Grid) {


	next_pos: [2]int
	for {
		dir: [2]int
		switch grid.guard_dir {
		case direction.North:
			dir = [2]int{0, -1}
		case direction.South:
			dir = [2]int{0, 1}
		case direction.East:
			dir = [2]int{1, 0}
		case direction.West:
			dir = [2]int{-1, 0}
		}
		next_pos = [2]int{grid.guard_pos[0] + dir[0], grid.guard_pos[1] + dir[1]}
		if next_pos[0] < 0 ||
		   next_pos[0] >= len(grid.cells[0]) ||
		   next_pos[1] < 0 ||
		   next_pos[1] >= len(grid.cells) {
			break
		}

		x := next_pos[0]
		y := next_pos[1]
		if grid.cells[y][x] == Cell.Obstruction {
			grid.guard_dir = turn_right(grid.guard_dir)
		} else {
			break
		}
	}

	grid.guard_pos = next_pos

}

print_grid :: proc(grid: Grid) {
	for y in 0 ..< len(grid.cells) {
		for x in 0 ..< len(grid.cells[y]) {
			if grid.guard_pos[0] == x && grid.guard_pos[1] == y {
				switch grid.guard_dir {
				case direction.North:
					fmt.print("^")
				case direction.South:
					fmt.print("v")
				case direction.East:
					fmt.print(">")
				case direction.West:
					fmt.print("<")
				}
			}
			if grid.cells[y][x] == Cell.Obstruction {
				fmt.print("#")
			}
			if grid.cells[y][x] == Cell.Empty {
				fmt.print(".")
			}
		}
		fmt.println()
	}
	fmt.println()
}

add_obstruction :: proc(grid: ^Grid, x: int, y: int) {
	grid.cells[y][x] = Cell.Obstruction
}

part01 :: proc() {
	input := get_input()
	grid := parse_input(input)
	visited := make(map[[2]int]bool)
	for is_guard_inside_map(grid) {
		visited[grid.guard_pos] = true
		step_guard(&grid)
	}

	fmt.println(len(visited))
}

part02 :: proc() {
	input := get_input()
	grid := parse_input(input)
	possible_placements := 0
	for y in 0 ..< len(grid.cells) {
		for x in 0 ..< len(grid.cells[y]) {

			add_obstruction(&grid, x, y)
			visited := make(map[[2]int]bool)
			visited[grid.guard_pos] = true
			for is_guard_inside_map(grid) {
				step_guard(&grid)
				if visited[grid.guard_pos] {
					possible_placements += 1
					break
				}
				visited[grid.guard_pos] = true
			}
		}
	}

	fmt.println(possible_placements)

}

main :: proc() {

	part01()
	part02()
}
