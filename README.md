7# N-Puzzle

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
***************************************************
set opened <- { initial }
			// States to be examined and candidates to expansion

set closed <- NULL
			// States already selected by the algorithm, compared to the solution and expanded
bool succes <- false
while (opened != NULLand SUCCES == FALSE)
	state e -< select_according_to_Astar_strategy_in (opened)
	If is_final (e) // Compares e to a solution state
		Then
			success <- true
		Else
			opened <- opened -e
			closed <- closed + e
			ForEach state s in expand(e) do
				If (s is not in opened and s is not in closed)
					opened <- opened + s
					predecessor(s) <- e
					g(s) <- g(e) + C(e-->s)
				Else // s is in opened or in closed
					If g(s) + h(s) > g(e) + C(e-->s) + h(s)
					// i.e f value >'potentially new' f value
						g(s) <- g(e) + C(e-->s)
						predecessor(s) <- e
						If s is in closed
							closed <- closed - s
							opened <- opened + s




***************************************************

*****************************************************
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



7	4	3
2	0	5
8	6	1

Step 1
2	2	0
2	2	1
1	2	4
total h = 16 g = 0


Object:

size 3
	coordonnee :

Puzzle (Position)

***********************

Step 2

7	0	3
2	4	5
8	6	1


total h = g = 1

------------------------------
7	4	3
2	5	0
8	6	1
total h = 17 g  = 1

-------------------------------

7	4	3
0	2	5
8	6	1
total h = 14 g  = 1

-----------------------------

7	4	3
2	6	5
8	0	1
total h = 18 g  = 1


****************************

Puzzle
	List<Taquin> taquins
	<!-- int		G; -->

	<!-- int		getH(); -->
	<!-- int		F() { G + this.getH()} -->

Taquin
	Coordonnees actualCoordonne;
	Coordonnees finalCoordonnee;

	int			getH();


Coordonnee
	int	x;
	int	y;;

Resolver:
	List<Puzzle>	opened // original State To put
	List<Puzzle>	closed

	bool		resolve()
	void		init() {
		Pour chaque Taquin initialisation de depart
	}

main ()
{
	Puzzle puzzle = parseFile();

	if (puzzle == Null)
		print('error in file')

	Resolver resolver = new Resolver(Puzzle);
	resolver.init()
	if (resolver.resolve())
		print('le probleme est resolu')
	else
		print(le puzzle est insolvable)

}