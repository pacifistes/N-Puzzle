/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.c                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bbrunell <bbrunell@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/02/12 17:24:21 by bbrunell          #+#    #+#             */
/*   Updated: 2019/02/18 19:30:14 by bbrunell         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "npuzzle.h"

void print_state(char *str, t_vector state)
{
	ft_printf("%s\n", str);
	ft_printf("size = %d\n", state.size);
	for (int i=0; i < state.size * state.size; i++) {
		ft_printf(" %d", ((int8_t *)state.values)[i]);
	}
	ft_printf("\n");
}

void do_all(char *filename)
{

	t_parser parser = parser_new(filename);
	if (parser.state == NULL)
		ft_printf("puzzle is null\n");
	else
	{
		ft_printf("puzzle is not null\n");
		ft_printf("size: %d\n", parser.state->size);
		print_state("puzzle of file", *parser.state);

		t_vector sorted_state = c_generate_sorted_puzzle(4);
		print_state("sorted puzzle", sorted_state);

		t_vector random_state = c_generate_random_puzzle();
		print_state("random puzzle", random_state);
	}
	ft_printf("error = %s\n", parser.error);
	parser_free(parser);
}

int main(int ac, char **av) {
	if (ac == 2) {
		do_all(av[1]);
		// while (1);
	}
	else
	{
		//Generate random Puzzle
	}
	return (0);
}
