/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.cpp                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bbrunell <bbrunell@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/02/12 17:24:21 by bbrunell          #+#    #+#             */
/*   Updated: 2019/10/14 18:52:40 by bbrunell         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "npuzzle.h"
#include <stdio.h>
#include <ctype.h>
#include <stdlib.h>
#include <unistd.h>
#include <string>

void print_state(std::string str, t_vector state)
{
    printf("%s\n", str.c_str());
	printf("size = %d\n", state.size);
	for (uint32_t i=0; i < state.size; i++) {
		if (i != 0 && i % (int(sqrt(state.size))) == 0)
			printf("\n");
		printf("\t%d", ((int8_t *)state.values)[i]);
	}
	printf("\n");
}

void print_move(t_move move) {
	switch (move) {
		case TOP:
			printf("move: TOP\n");
			break;
		case LEFT:
			printf("move: LEFT\n");
			break;
		case BOT:
			printf("move: BOT\n");
			break;
		case RIGHT:
			printf("move: RIGHT\n");
			break;
		default:
			printf("move: None\n");
			break;
	}
}

void run(t_vector state) {
	puzzle_t *puzzle = puzzle_new(state);
	t_vector goal_state = c_generate_sorted_state((uint32_t) sqrt(state.size));
	puzzle_t *goal = puzzle_new(goal_state);


	if (c_is_solvable(puzzle, goal))
	{
		resolver_t *resolver = resolver_new(puzzle, goal);
		printf("the puzzle solvable\n");

		t_heuristic heuristics[] = {manathan, hamming};
		size_t size = sizeof heuristics / sizeof *heuristics;
		c_set_heuristics(resolver, heuristics, size);
		t_algo algo = Greedy;
		c_set_algo(resolver, algo);
		t_resolver_info *info = c_resolve(resolver);

		if (info == NULL) {
			printf("timeout retry");
			return;
		}
		for (uint32_t i=0; i < info->size; i++) {
			print_move(info->all_state[i].move);
			print_state("", info->all_state[i].state);
		}
		printf("Result:\n");
		printf("number of move : %d\n", info->size);
		printf("Time : %ss\n", info->time_use);
		printf("Total state selected : %d\n", info->total_state_selected);
		printf("Total state represented : %d\n", info->total_state_represented);
		resolve_info_free(info);
		resolver_free(resolver);
	}
	else
	{
		printf("the puzzle is not solvable\n");
	}
	puzzle_free(puzzle);
	puzzle_free(goal);
}

void do_all(char *filename)
{
	t_created_puzzle created_puzzle = parser_new(filename);
	if (created_puzzle.state == NULL)
		printf("puzzle is null\n");
	else
	{
		printf("error: %s\n", created_puzzle.error);
		printf("puzzle is not null\n");
		printf("size: %d\n", created_puzzle.state->size);
		print_state("puzzle of file", *created_puzzle.state);

		t_vector random_state = c_generate_random_state(3);
		print_state("random puzzle", random_state);
		vector_free(random_state);
		run(*created_puzzle.state);
	}
	printf("error = %s\n", created_puzzle.error);
	created_puzzle_free(created_puzzle);
}

int main(int argc, char **argv) {
	if (argc == 2) {
		do_all(argv[1]);
		while (1);
	}
	else
	{
		//Generate random Puzzle
	}
	return (0);
}

