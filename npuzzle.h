#ifndef NPUZZLE_HPP
# define NPUZZLE_HPP
# include <stdint.h>

typedef enum	e_heuristic
{
	MANATHAN,
    CHEBYSHEV,
    EUCLIDIENNE,
    OCTILE,
    HAMMING,
    LINEAR_CONFLICT
}				t_heuristic;

typedef enum	e_algo
{
    UNIFORM_COST,
    A_STAR,
    GREEDY
}				t_algo;

typedef enum	e_move
{
    TOP,
	BOT,
	LEFT,
	RIGHT
}				t_move;

typedef struct	s_puzzle {
	int		*state;
	t_move	move;
}				t_puzzle;

typedef struct	s_parser {
	t_puzzle	puzzle;
	char		*error;
}				t_parser;

t_parser parse(char *filename);

int			*generate_sorted_puzzle(uint8_t size);
t_puzzle	generate_random_puzzle();

#endif
