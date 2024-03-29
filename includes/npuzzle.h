/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   npuzzle.h                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bbrunell <bbrunell@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2018/11/19 12:42:51 by bbrunell          #+#    #+#             */
/*   Updated: 2019/10/15 17:55:17 by bbrunell         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#ifndef NPUZZLE_H
# define NPUZZLE_H

# include <math.h>
# include <cstdint>
# include <stdio.h>
# include <ctype.h>
# include <stdlib.h>
# include <unistd.h>
# include <string>

# define C 0
# define E 1
# define H 2
# define L 3
# define M 4
# define O 5

typedef struct	s_vector {
	void			*values;
	uint32_t		size;
}				t_vector;

typedef enum e_option {
	HOPTION,
	AOPTION,
	ROPTION
	} t_option;

typedef enum e_algo {
    UniformCost,
    AStar,
    Greedy,
}			t_algo;

typedef enum e_move {
    TOP,
    LEFT,
    BOT,
	RIGHT,
	NONE,
}			t_move;

typedef enum e_heuristic {
    chebyshev,
    euclidienne,
    hamming,
    linear_conflict,
	manathan,
    octile,
}			t_heuristic;

typedef struct	s_created_puzzle {
	t_vector	*state;
	char		*error;
}				t_created_puzzle;

typedef struct	s_puzzle {
	t_vector	state;
	uint8_t		index_case_move;
	t_move		move;
}				t_puzzle;

typedef struct	s_resolver_info {
	t_puzzle	*all_state;
	uint32_t	size;
	char		*time_use;
	uint32_t	total_state_selected;
	uint32_t	total_state_represented;
}				t_resolver_info;

typedef struct Resolver resolver_t;
typedef struct Puzzle puzzle_t;

extern "C" t_created_puzzle parser_new(const char *filename);
extern "C" void created_puzzle_free(t_created_puzzle created_puzzle);
extern "C" t_created_puzzle c_generate_sorted_state(uint32_t size);
extern "C" t_created_puzzle c_generate_random_state(uint32_t size);
extern "C" void	vector_free(t_vector vector);
extern "C" puzzle_t *puzzle_new(t_vector vector);

extern "C" resolver_t *resolver_new(puzzle_t *puzzle, puzzle_t *goal);
extern "C" void c_set_heuristics(resolver_t *resolver, t_heuristic *heuristics, size_t size);
extern "C" void c_set_algo(resolver_t *resolver, t_algo algo);
extern "C" t_resolver_info *c_resolve(resolver_t *resolver);
extern "C" int8_t c_is_solvable(puzzle_t *puzzle, puzzle_t *goal);
extern "C" void resolver_free(resolver_t *resolver);
extern "C" void puzzle_free(puzzle_t *puzzle);
extern "C" void resolve_info_free(t_resolver_info *info);
void	print_state(std::string str, t_vector state);
void	print_move(t_move move);
void	print_info(t_resolver_info *info);
void	print_usage(const char *error);
int		check_heuristic(int heuristic, char *arg);
t_algo	check_algo(char *arg);
int		check_random_value(char* arg);
void	set_heuristic(int heuristic, t_heuristic *heuristic_list);
void	check_arguments(t_option option, char *arg, int *randomValue, t_algo *algoValue, int *heuristic);

#endif
