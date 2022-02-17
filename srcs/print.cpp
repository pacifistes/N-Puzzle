#include "npuzzle.h"

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

void	print_usage(const char *error)
{
	printf("%s\n\n", error);
	printf("usage: ./npuzzle -a [ALGO] -h [HEURISTIC] -r [VALUE BETWEEN 2 AND 15] filename\n");
	printf("ALGO : UniformCost || AStar || Greedy\n");
	printf("HEURISTIC : manathan,chebyshev,euclidienne,octile,hamming,linear_conflict\n");
	exit(1);
}