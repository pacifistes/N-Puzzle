/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.c                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bbrunell <bbrunell@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/02/12 17:24:21 by bbrunell          #+#    #+#             */
/*   Updated: 2019/02/15 16:25:07 by bbrunell         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "npuzzle.h"

int main(int ac, char **av) {
	if (ac == 2)
	{
		t_parser parser = parser_new(av[1]);
		// (void)parser;
		ft_printf("error = %s", parser.error);
	}
	else
	{
		//Generate random Puzzle
	}
	return (0);
}
