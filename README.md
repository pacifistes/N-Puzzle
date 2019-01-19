# N-Puzzle

Realiser un solver de N-Puzzle (Taquin)


Algorithmes : A*
snail Taquin
Taquin from 1 to N


Heuristique : must be minimum 3 heuristique
Manhattan-distance is mandatory


/**********************************\


You have to manage both randomly determined states (of your own generation of
course), or input files that specify a starting board, the format of which is described
in the appendix.

• The cost associated with each transition is always 1.

• The user must be able to choose between at LEAST 3 (relevant) heuristic functions.
The Manhattan-distance heuristic is mandatory, the other two are up to you. By
"relevant" we mean they must be admissible (Read up on what this means) and
they must be something other than "just return a random value because #YOLO".



At the end of the search, the program has to provide the following values:
	Total number of states ever selected in the "opened" set (complexity in time)
	Maximum number of states ever represented in memory at the same time
		during the search (complexity in size)
	Number of moves required to transition from the initial state to the final state,
		according to the search
	The ordered sequence of states that make up the solution, according to the
		search
	The puzzle may be unsolvable, in which case you have to inform the user and
		exit

• Configure the appropriate g(x) and h(x) functions to run both the uniform-cost
and greedy searches. Execute with the same output (Of course, the solution may
be different. Read up on why, that’s the point.)



Pseudo-Implementation of A*

set opened { initial }


TODO
	Parser
		- ignorer les lignes commentaires
		- premiere ligne le nombre de case de cote du puzzle
		- les autres ligne peuvent avoir des commentaire

	A*


	Implementation des heuristique


	Preparation des Information
		Nombre d'etat total selectionne in the Opened (complexity in time)
		Nombre maximum d'etat representer dans la memoire au meme moment (Complexity in size)
		Nombre de move
		The ordered sequence of states that make up the solution, according to the search
		The puzzle may be unsolvable, in which case you have to inform the user and
		exit