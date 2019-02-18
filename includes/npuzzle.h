/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   npuzzle.h                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bbrunell <bbrunell@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2018/11/19 12:42:51 by bbrunell          #+#    #+#             */
/*   Updated: 2019/02/18 17:36:58 by bbrunell         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#ifndef NPUZZLE_H
# define NPUZZLE_H

# include "ft_printf.h"

typedef struct	s_vector {
	void			*values;
	uint8_t			size;
}				t_vector;


typedef enum e_algo {
    UNIFORM_COST,
    A_STAR,
    GREEDY,
}			t_algo;

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
}				t_puzzle;

typedef struct	s_result {
	t_vector	all_state;
	char		*time_use;
	int			total_state_selected;
	int			total_state_represented;
}				t_result;

typedef struct Resolver resolver_t;

extern t_parser parser_new(const char *filename);
extern void parser_free(t_parser parser);
extern t_vector c_generate_sorted_puzzle(uint8_t size);
extern t_vector c_generate_random_puzzle(void);
extern resolver_t *resolver_new();
extern void c_set_heuristics();
extern void c_set_algo();
extern void c_resolve();

#endif
