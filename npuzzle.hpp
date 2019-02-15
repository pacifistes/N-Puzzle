#ifndef NPUZZLE_HPP
# define NPUZZLE_HPP
# include <string>
# include <stdint.h>
// # include <vector>


/*
**	if parse(filename) == true : set the Puzzle and the error is set to null
*/
class Parser {
	public:
		Parser();
		int test;
		// Puzzle puzzle;
		std::string error;
		// virtual ~Parser();
		bool parse(std::string filename);


	// private:
		// Parser(Parser const &parser);
		// Parser& operator=(Parser const &parser);
};
// enum Heuristic {
// 	MANATHAN,
//     CHEBYSHEV,
//     EUCLIDIENNE,
//     OCTILE,
//     HAMMING,
//     LINEAR_CONFLICT};

// enum Algo {
//     UNIFORM_COST,
//     A_STAR,
//     GREEDY
// };

// class Puzzle {
// 	public:
// 		std::vector<uint8_t> state;

// 		Puzzle(std::vector<uint8_t>);
// 	private:
// 		Puzzle();
// 		Puzzle(Puzzle const &puzzle);
// 		Puzzle& operator=(Puzzle const &puzzle);
// };

// class Resolver {
// 	public:
// 		Resolver(Puzzle startPuzzle, Puzzle goalPuzzle);
// 		void setHeuristics(std::vector<Heuristic> heuristics);
// 		void setHeuristics(Algo algo);
// 	private:
// 		Resolver();
// 		Resolver(Resolver const &resolver);
// 		Resolver& operator=(Resolver const &resolver);
// };

// std::vector<uint8_t> generate_sorted_puzzle(uint8_t size);
// Puzzle generate_random_puzzle();

#endif