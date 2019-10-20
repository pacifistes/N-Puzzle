#include "npuzzle.h"

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