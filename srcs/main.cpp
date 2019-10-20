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

typedef enum e_option {
	HOPTION,
	AOPTION,
	ROPTION
	} t_option;

void print_state(std::string str, t_vector state)
{
	printf("%s\n", str.c_str());
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

void print_info(t_resolver_info *info)
{
	if (info == NULL) {
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
}

void run(t_created_puzzle *created_puzzle, t_algo algo, t_heuristic *heuristic_list, int size) {
	// printf("rvalue : %d\n", rvalue);
	// printf("avalue : %s\n", avalue);
	// printf("hvalue : %s\n", hvalue);
	puzzle_t *puzzle = puzzle_new(*created_puzzle->state);
	t_created_puzzle goal_state = c_generate_sorted_state((uint32_t) sqrt(created_puzzle->state->size));
	puzzle_t *goal = puzzle_new(*goal_state.state);

	print_state("puzzle", *created_puzzle->state);
	printf("size = %d\n", created_puzzle->state->size);
	//	// for(int k = 0; k < tab_size; ++k)
 //    	printf("heuristic : %d\n", heuristic_list[k]);
	if (c_is_solvable(puzzle, goal))
	{
		resolver_t *resolver = resolver_new(puzzle, goal);
		printf("the puzzle solvable\n");

		// t_heuristic heuristics[] = {manathan, hamming};
		// size_t size = sizeof heuristics / sizeof *heuristics;
		c_set_heuristics(resolver, heuristic_list, size);
		// t_algo algo = Greedy;
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

void	print_usage(const char *error)
{
	printf("%s\n\n", error);
	printf("usage: ./npuzzle -a [ALGO] -h [HEURISTIC] -r [VALUE BETWEEN 2 AND 15] filename\n");
	printf("ALGO : UniformCost || AStar || Greedy\n");
	printf("HEURISTIC : manathan,chebyshev,euclidienne,octile,hamming,linear_conflict\n");
	exit(1);
}

// int		check_options(char *arg)
// {
// 	int	option;

// 	option = 0;
// 	if(!strcmp(arg, "-a"))
// 			option = AOPTION;
// 		else if(!strcmp(arg, "-h"))
// 			option = HOPTION;
// 		else if(!strcmp(arg, "-r"))
// 			option = ROPTION;
// 		else
// 			{
// 				if (argc == 2 || i == argc - 1)
// 				{
// 					filename = arg;
// 					break;
// 				}
// 				else
// 					print_usage("Invalid filename");
// 			}
// 	return (option);
// }

int		check_heuristic(int heuristic, char *arg)
{

	if (!strcmp(arg, "chebyshev"))
		return (heuristic |= 1 << C);
	else if (!strcmp(arg, "euclidienne"))
		return (heuristic |= 1 << E);
	else if (!strcmp(arg, "hamming"))
		return (heuristic |= 1 << H);
	else if (!strcmp(arg, "linear_conflict"))
		return (heuristic |= 1 << L);
	else if (!strcmp(arg, "manathan"))
		return (heuristic |= 1 << M);
	else if (!strcmp(arg, "octile"))
		return (heuristic |= 1 << O);
	else
		print_usage("Invalid Heuristic");

	return (0);
}

t_algo	check_algo(char *arg)
{
	printf("Algo : ");
	if (!strcmp("UniformCost", arg))
	{
		printf("UniformCost\n");
		return(UniformCost);
	}
	else if (!strcmp("Greedy", arg))
	{
		printf("Greedy\n");
		return(Greedy);
	}
	else if (strcmp("AStar", arg))
	{
		print_usage("Algo value error");
	}
	printf("AStar\n");
	return (AStar);
}

int		check_random_value(char* arg)
{
	int	randomValue;

	randomValue = atoi(arg);
	if (randomValue < 2 || randomValue > 15)
		print_usage("random value error");
	return (randomValue);
}

void	set_heuristic(int heuristic, t_heuristic *heuristic_list)
{
	int	j;

	j = 0;
	printf("Heuristiques: ");
	if ((heuristic >> C) & 1)
	{
		heuristic_list[j] = chebyshev;
		printf("chebyshev ");
		j++;
	}
	if ((heuristic >> E) & 1)
	{
		heuristic_list[j] = euclidienne;
		printf("euclidienne ");
		j++;
	}
	if ((heuristic >> H) & 1)
	{
		heuristic_list[j] = hamming;
		printf("hamming ");
		j++;
	}
	if ((heuristic >> L) & 1)
	{
		heuristic_list[j] = linear_conflict;
		printf("linear_conflict ");
		j++;
	}
	if ((heuristic >> M) & 1)
	{
		heuristic_list[j] = manathan;
		printf("manathan ");
		j++;
	}
	if ((heuristic >> O) & 1)
	{
		heuristic_list[j] = octile;
		printf("octile ");		
		j++;
	}
	printf("\n");
}

void	check_arguments(t_option option, char *arg, int *randomValue, t_algo *algoValue, int *heuristic)
{
	switch (option)
	{
		case HOPTION:
			*heuristic = check_heuristic(*heuristic, arg);
			break;
		case AOPTION:
			*algoValue = check_algo(arg);
			break;
		case ROPTION:
			*randomValue = check_random_value(arg);
			break;
		default:
			break;
	}
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
			// printf ("FIRST TIME : aflag = %d, hflag = %x, rvalue = %d\n", algoValue, heuristic, randomValue);
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