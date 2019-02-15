/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.c                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bbrunell <bbrunell@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/02/12 17:24:21 by bbrunell          #+#    #+#             */
/*   Updated: 2019/02/15 17:46:58 by bbrunell         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "npuzzle.h"

int main(int ac, char **av) {
	if (ac == 2)
	{
		t_parser parser = parser_new(av[1]);
		// (void)parser;
		if (parser.puzzle == NULL)
			ft_printf("puzzle is null\n");
		else
			ft_printf("puzzle is not null\n");
		ft_printf("error = %s\n", parser.error);
	}
	else
	{
		//Generate random Puzzle
	}
	return (0);
}
