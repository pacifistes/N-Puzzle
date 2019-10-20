/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.cpp                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bbrunell <bbrunell@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/02/12 17:24:21 by bbrunell          #+#    #+#             */
/*   Updated: 2019/10/20 20:17:51 by bbrunell         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "npuzzle.h"

void run(t_created_puzzle *created_puzzle, t_algo algo, t_heuristic *heuristic_list, int size) {
	puzzle_t *puzzle = puzzle_new(*created_puzzle->state);
	t_created_puzzle goal_state = c_generate_sorted_state((uint32_t) sqrt(created_puzzle->state->size));
	puzzle_t *goal = puzzle_new(*goal_state.state);

	print_state("puzzle", *created_puzzle->state);
	printf("size = %d\n", created_puzzle->state->size);
	if (c_is_solvable(puzzle, goal))
	{
		resolver_t *resolver = resolver_new(puzzle, goal);
		printf("the puzzle solvable\n");

		c_set_heuristics(resolver, heuristic_list, size);
		c_set_algo(resolver, algo);
		t_resolver_info *info = c_resolve(resolver);
		print_info(info);
		resolve_info_free(info);
		resolver_free(resolver);
	}
	else
	{
		printf("the puzzle is not solvable\n");
	}
	created_puzzle->state->values = NULL;
	goal_state.state->values = NULL;
	puzzle_free(puzzle);
	puzzle_free(goal);
	created_puzzle_free(goal_state);
}

void do_all(char *filename, int random, t_algo avalue, t_heuristic *hvalue, int size)
{
	t_created_puzzle created_puzzle;
	if (filename)
		created_puzzle = parser_new(filename);
	else
		created_puzzle = c_generate_random_state(random);
	if (created_puzzle.state == NULL)
		printf("Error : %s\n", created_puzzle.error);
	else
	{
		run(&created_puzzle, avalue, hvalue, size);
	}
	created_puzzle_free(created_puzzle);
}

int 	main(int argc, char **argv) {
	int			i;
	int			j;
	int			tab_size;
	int			randomValue;
	int			heuristic;
	bool		isWaiting;
	char		*filename;
	t_algo		algoValue;
	t_option	option;

	i = 1;
	j = 0;
	tab_size = 0;
	heuristic = 0;
	randomValue = 3;
	isWaiting = false;
	while (i < argc)
	{
		if (!isWaiting)
		{
			if(!strcmp(argv[i], "--help"))
				print_usage("");
			if(!strcmp(argv[i], "-a"))
				option = AOPTION;
			else if(!strcmp(argv[i], "-h"))
				option = HOPTION;
			else if(!strcmp(argv[i], "-r"))
				option = ROPTION;
			else
			{
				if (argc == 2 || i == argc - 1)
				{
					filename = argv[i];
					break;
				}
				else
					print_usage("Invalid filename");
			}
			isWaiting = true;
		}
		else
		{
			check_arguments(option, argv[i], &randomValue, &algoValue, &heuristic);
			isWaiting = false;
		}
		i++;
	}
	if (isWaiting == true)
		print_usage("missing argument");
	while (j < 6)
	{
		if ((heuristic >> j) & 1)
			tab_size++;
		j++;
	}
	t_heuristic* heuristic_list = new t_heuristic[tab_size];
    set_heuristic(heuristic, heuristic_list);

    do_all(filename, randomValue, algoValue, heuristic_list, tab_size);
	delete[] heuristic_list;
	return (0);
}