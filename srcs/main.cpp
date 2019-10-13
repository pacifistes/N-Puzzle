/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.cpp                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bbrunell <bbrunell@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/02/12 17:24:21 by bbrunell          #+#    #+#             */
/*   Updated: 2019/10/13 18:12:36 by bbrunell         ###   ########.fr       */
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
		printf(" %d", ((int8_t *)state.values)[i]);
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

	t_parser parser = parser_new(filename);
	if (parser.state == NULL)
		printf("puzzle is null\n");
	else
	{
		printf("error: %s\n", parser.error);
		printf("puzzle is not null\n");
		printf("size: %d\n", parser.state->size);
		print_state("puzzle of file", *parser.state);

		t_vector random_state = c_generate_random_state(3);
		print_state("random puzzle", random_state);
		vector_free(random_state);

		t_vector sorted_state = c_generate_sorted_state(4);
		print_state("sorted puzzle", sorted_state);
		vector_free(sorted_state);
		run(*parser.state);
	}
	printf("error = %s\n", parser.error);
	parser_free(parser);
}

int main(int argc, char **argv) {

/////***** hmoussa *****/////
	char	*avalue = NULL;
	char	*hvalue[6];
	int		rvalue = 0;
	int		index;
	int		i;
	int		c;
	
	opterr = 0;
	i = 0;

  while ((c = getopt (argc, argv, "a:h:r:")) != -1)
  	switch (c)
      {
      case 'a':
        avalue = optarg;
        break;
      case 'h':
        hvalue[i] = optarg;
        for( ;optind < argc - 1 && *argv[optind] != '-' && i++ < 5; optind++)
              hvalue[i] = argv[optind]; 
        break;
      case 'r':
        rvalue = atoi(optarg);
        break;
      case '?':
      	if (optopt == 'a' || optopt == 'h' || optopt == 'r')
        	fprintf (stderr, "Option -%c requires an argument.\n", optopt);
        else if (isprint (optopt))
        	fprintf (stderr, "Unknown option `-%c'.\n", optopt);
        else
          fprintf (stderr,
                   "Unknown option character `\\x%x'.\n",
                   optopt);
        return 1;
      default:
        abort ();
      }
	printf ("aflag = %s, hflag = %s, rvalue = %d\n", avalue, hvalue[3], rvalue);

	index = optind;
	filename = argv[index];
	printf ("filename %s\n", argv[index]);
	do_all(filename);
  return (0);
/////***** hmoussa *****/////

	// if (argc == 2) {
	// 	do_all(argv[1]);
	// 	// while (1);
	// }
	// else
	// {
	// 	//Generate random Puzzle
	// }
	// return (0);
}

