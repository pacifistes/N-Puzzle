/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.cpp                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bbrunell <bbrunell@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/02/12 17:24:21 by bbrunell          #+#    #+#             */
/*   Updated: 2019/02/15 21:39:36 by bbrunell         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "npuzzle.hpp"

int main(int ac, char **av) {
	if (ac == 2)
	{
		t_parser parser = parser_new(av[1]);
		if (parser.puzzle == NULL)
			std::cout << "puzzle is null" << std::endl;
		else
		{
			std::cout << "puzzle is not null" << std::endl;
			std::cout << "size: " << ((int)parser.puzzle->size) <<std::endl;
			std::cout << "puzzle:";
			for (int i=0; i < parser.puzzle->size * parser.puzzle->size; i++) {
				std::cout << " " << ((int)parser.puzzle->state[i]);
			}
			std::cout <<std::endl;
		}
		std::cout << "error = " << parser.error << std::endl;
	}
	else
	{
		//Generate random Puzzle
	}
	return (0);
}

// int x[3] = {1, 2, 3};
// std::vector<int> v(x, x + sizeof x / sizeof x[0]);