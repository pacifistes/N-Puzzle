/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   npuzzle.hpp                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bbrunell <bbrunell@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2018/11/19 12:42:51 by bbrunell          #+#    #+#             */
/*   Updated: 2019/02/16 17:36:10 by bbrunell         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#ifndef NPUZZLE_H
# define NPUZZLE_H

# include <vector>
# include <iostream>
# include <string>
# include <cinttypes>

// typedef struct puzzle t_puzzle;

typedef struct	s_puzzle {
	uint8_t			*state;
	uint8_t			size;
}				t_puzzle;

typedef struct	s_parser {
	t_puzzle	*puzzle;
	char		*error;
}				t_parser;

extern "C" t_parser parser_new(const char *filename);
extern "C" void parser_free(t_parser parser);


#endif
