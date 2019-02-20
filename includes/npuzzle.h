/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   npuzzle.h                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bbrunell <bbrunell@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2018/11/19 12:42:51 by bbrunell          #+#    #+#             */
/*   Updated: 2019/02/20 17:25:16 by bbrunell         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#ifndef NPUZZLE_H
# define NPUZZLE_H

# include "ft_printf.h"
# include <math.h>

typedef struct	s_vector {
	void			*values;
	uint32_t		size;
}				t_vector;


typedef enum e_algo {
    UNIFORM_COST,
    A_STAR,
    GREEDY,
}			t_algo;

typedef enum e_move {
    TOP,
    LEFT,
    BOT,
	RIGHT,
	NONE,
}			t_move;

typedef enum e_heuristic {
	MANATHAN,
    CHEBYSHEV,
    EUCLIDIENNE,
    OCTILE,
    HAMMING,
    LINEAR_CONFLICT,
}			t_heuristic;

typedef struct	s_parser {
	t_vector	*state;
	char		*error;
}				t_parser;

typedef struct	s_puzzle {
	t_vector	state;
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

extern t_parser parser_new(const char *filename);
extern void parser_free(t_parser parser);
extern t_vector c_generate_sorted_state(uint32_t size);
extern t_vector c_generate_random_state(void);
extern void	vector_free(t_vector vector);
extern puzzle_t *puzzle_new(t_vector vector);

extern resolver_t *resolver_new(puzzle_t *puzzle, puzzle_t *goal);
extern void c_set_heuristics(resolver_t *resolver, t_heuristic *heuristics, size_t size);
extern void c_set_algo(resolver_t *resolver, t_algo algo);
extern t_resolver_info c_resolve(resolver_t *resolver);
extern int8_t c_is_solvable(puzzle_t *puzzle, puzzle_t *goal);
extern void resolver_free(resolver_t *resolver);
extern void puzzle_free(puzzle_t *puzzle);
extern void resolve_info_free(t_resolver_info info);

#endif
