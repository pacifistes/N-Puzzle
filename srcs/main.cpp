/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.cpp                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bbrunell <bbrunell@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/02/12 17:24:21 by bbrunell          #+#    #+#             */
/*   Updated: 2019/10/15 17:55:03 by hmoussa          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "npuzzle.h"

enum WaitingOption {
	heuristic,
	algo,
	randval
	};

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

void run(t_created_puzzle *created_puzzle, int random, char *avalue, char *hvalue) {
	printf("random : %d\n", random);
	printf("avalue : %s\n", avalue);
	printf("hvalue : %s\n", hvalue);
	puzzle_t *puzzle = puzzle_new(*created_puzzle->state);
	t_created_puzzle goal_state = c_generate_sorted_state((uint32_t) sqrt(created_puzzle->state->size));
	puzzle_t *goal = puzzle_new(*goal_state.state);

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

void do_all(char *filename, int random, char *avalue, char *hvalue)
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
		run(&created_puzzle, random, avalue, hvalue);
	}
	created_puzzle_free(created_puzzle);
}
int main(int argc, char **argv) {
	bool	isWaiting;
	char	*heuristic_list[6];
	int		i;
	int		randomValue;
	char	*algoValue;
	char	*filename;
	enum	WaitingOption option;

	isWaiting= false;
	i = 0;
	(void)heuristic_list;
	while (i < argc)
	{
		if (!isWaiting)
		{
			if(strcmp(argv[i], "-a") == 0 || strcmp(argv[i], "-h") == 0 || strcmp(argv[i], "-r") == 0)
			{
				if(strcmp(argv[i], "-a"))
					option = algo;
				if(strcmp(argv[i], "-h"))
					option = heuristic;
				if(strcmp(argv[i], "-r"))
					option = randval;
				isWaiting = true;
			}
			else
			{
				filename = argv[i];
				break;
			}
		}
		else
		{}
	}
	if (isWaiting)
	{
		switch (option)
		{
			case heuristic:
			case algo:
				if (strcmp("UniformCost", argv[i]) != 0 && strcmp("AStar", argv[i]) != 0 &&
					strcmp("Greedy", argv[i]) != 0)
	        	{
	        		printf("Algo value error.\n");
	        		exit(1);
	        	}
				else
	        		algoValue = argv[i];
			case randval:
				randomValue = atoi(argv[i]);
				if (randomValue < 2 || randomValue > 15)
				{
					printf("random value error.\n");
	        		exit(1);
				}
			i++;
		}
	}
	return (0);
}

// int main(int argc, char **argv) {
// 	char	*filename;
// 	char	*avalue;
// 	char	*hvalue = NULL;
// 	char	*heur[6];
// 	const char	*heuristic[6] = { "manathan", "chebyshev", "euclidienne",
//     						"octile", "hamming", "linear_conflict"};
// 	int		rvalue = 0;
// 	int		arglen;
// 	int		index;
// 	int		i;
// 	int		c;
	
// 	opterr = 0;
// 	i = 0;

// 	if (argc == 1)
// 	{
// 		printf("usage: ./npuzzle -a [ALGO] -h [HEURISTIC] -r [VALUE BETWEEN 2 AND 15] filename\n");
// 		exit(0);
// 	}
// 	while ((c = getopt (argc, argv, "a:h:r:")) != -1)
// 	  	switch (c)
// 	    {
// 		  case 'a':
// 	        if (strcmp("UniformCost", optarg) != 0 && strcmp("AStar", optarg) != 0 &&
// 				strcmp("Greedy", optarg) != 0)
// 	        {
// 	        	printf("Algo value error.\n");
// 	        	exit(1);
// 	        }
// 			else
// 	        	avalue = optarg;
// 	        break;
// 	      case 'h':
// 	        optind--;
// 			for( ;optind < argc && *argv[optind] != '-' && i++ < 5; optind++)
// 				if (strcmp("manathan", optarg) == 0 || strcmp("chebyshev", optarg) == 0 ||
// 				strcmp("euclidienne", optarg) == 0 || strcmp("octile", optarg) == 0 ||
// 				strcmp("hamming", optarg) == 0 || strcmp("linear_conflict", optarg) == 0)
// 				{
// 					arglen = strlen(optarg) + 1;               
// 			    	heur[i] = (char*)malloc(arglen * sizeof(char));
// 			    	strcpy(heur[i], optarg);
// 					printf("HEUR : %s\n", heur[i]);
// 					optarg++;
// 				}
// 				else
// 				{
// 					printf("ERROR HEURISTIC\n");
// 					exit(0);
// 				}
// 	        // 
// 	        // {
// 			// 	// if (argv[optind])
// 			// 	hvalue[i] = argv[optind]; 
// 			// }
// 	        break;
// 	      case 'r':
// 	        rvalue = atoi(optarg);
// 	        // if (rvalue < 2 || rvalue > 15)
// 	        // {
// 	        // 	printf("random value error.\n");
// 	        // 	exit(1);
// 	        // }
// 	        break;
// 	      case '?':
// 	      	if (optopt == 'a' || optopt == 'h' || optopt == 'r')
//           		fprintf (stderr, "Option -%c requires an argument.\n", optopt);
//         	else if (isprint (optopt))
//           		fprintf (stderr, "Unknown option `-%c'.\n", optopt);
//         	else
//           		fprintf (stderr, "Unknown option character `\\x%x'.\n", optopt);
// 	        return 1;
// 	      default:
// 	        abort ();
// 	      }
// 		(void)heuristic;
// 		printf ("FIRST TIME : aflag = %s, hflag = %s, rvalue = %d\n", avalue, hvalue, rvalue);		
// 		if (avalue == '\0')
// 			avalue = (char *)"AStar";
// 		if (hvalue == '\0')
// 			hvalue = (char*)"manathan";
// 		printf ("aflag = %s, hflag = %s, rvalue = %d\n", avalue, hvalue, rvalue);
// 		// for (index = optind; index < argc; index++)
//     	// 	printf ("Non-option argument %s\n", argv[index]);
// 		index = optind;
// 		if (index == argc - 1)
// 			printf("PAS DE NOM DE FICHIER\n");
// 		else index++;
// 		filename = argv[index];
// 		printf("%s\n", filename);
// 		if (filename)
// 			do_all(filename, 0, avalue, hvalue);
// 		else
		

// 	// if (argc == 2) {
// 	// 	do_all(argv[1]);
// 	// 	while (1);
// 	// }
// 	// else
// 	// {
// 	// 	//Generate random Puzzle
// 	// }
// 	return (0);
// }

