/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   npuzzle.h                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bbrunell <bbrunell@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2018/11/19 12:42:51 by bbrunell          #+#    #+#             */
/*   Updated: 2019/02/18 15:28:48 by bbrunell         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#ifndef NPUZZLE_H
# define NPUZZLE_H

# include "ft_printf.h"

// typedef struct puzzle t_puzzle;

typedef struct	s_vector {
	uint8_t			*values;
	uint8_t			size;
}				t_vector;

typedef struct	s_parser {
	t_vector	*state;
	char		*error;
}				t_parser;

extern t_parser parser_new(const char *filename);
extern void parser_free(t_parser parser);
extern t_vector c_generate_sorted_puzzle(uint8_t size);
extern t_vector c_generate_random_puzzle(void);


#endif
