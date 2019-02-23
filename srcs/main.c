/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.c                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bbrunell <bbrunell@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/02/12 17:24:21 by bbrunell          #+#    #+#             */
/*   Updated: 2019/02/23 17:49:04 by bbrunell         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "npuzzle.h"

void print_state(char *str, t_vector state)
{
	ft_printf("%s\n", str);
	ft_printf("size = %d\n", state.size);
	for (uint32_t i=0; i < state.size; i++) {
		ft_printf(" %d", ((int8_t *)state.values)[i]);
	}
	ft_printf("\n");
}

void print_move(t_move move) {
	switch (move) {
		case TOP:
			ft_printf("move: TOP\n");
			break;
		case LEFT:
			ft_printf("move: LEFT\n");
			break;
		case BOT:
			ft_printf("move: BOT\n");
			break;
		case RIGHT:
			ft_printf("move: RIGHT\n");
			break;
		default:
			ft_printf("move: None\n");
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
		ft_printf("the puzzle solvable\n");

		t_heuristic heuristics[] = {manathan, hamming};
		size_t size = sizeof heuristics / sizeof *heuristics;
		c_set_heuristics(resolver, heuristics, size);
		t_algo algo = Greedy;
		c_set_algo(resolver, algo);
		t_resolver_info *info = c_resolve(resolver);

		if (info == NULL) {
			ft_printf("timeout retry");
			return;
		}
		for (uint32_t i=0; i < info->size; i++) {
			print_move(info->all_state[i].move);
			print_state("", info->all_state[i].state);
		}
		ft_printf("Result:\n");
		ft_printf("number of move : %d\n", info->size);
		ft_printf("Time : %ss\n", info->time_use);
		ft_printf("Total state selected : %d\n", info->total_state_selected);
		ft_printf("Total state represented : %d\n", info->total_state_represented);
		resolve_info_free(info);
		resolver_free(resolver);
	}
	else
	{
		ft_printf("the puzzle is not solvable\n");
	}
	puzzle_free(puzzle);
	puzzle_free(goal);
}

void do_all(char *filename)
{

	t_parser parser = parser_new(filename);
	if (parser.state == NULL)
		ft_printf("puzzle is null\n");
	else
	{
		ft_printf("puzzle is not null\n");
		ft_printf("size: %d\n", parser.state->size);
		print_state("puzzle of file", *parser.state);
		// vector_free(*parser.state);

		t_vector random_state = c_generate_random_state();
		print_state("random puzzle", random_state);
		vector_free(random_state);

		t_vector sorted_state = c_generate_sorted_state(4);
		print_state("sorted puzzle", sorted_state);
		vector_free(sorted_state);
		run(*parser.state);

	}
	ft_printf("error = %s\n", parser.error);
	parser_free(parser);
}

int main(int ac, char **av) {
	if (ac == 2) {
		do_all(av[1]);
		// while (1);
	}
	else
	{
		//Generate random Puzzle
	}
	return (0);
}
