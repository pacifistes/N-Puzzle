NAME = npuzzle

SRCS_PATH = srcs/
INCLUDES = includes/
HEADERS = npuzzle.hpp

SRCS = main.cpp
RUST_SRCS = generate.rs \
			heuristic.rs \
			parser.rs \
			puzzle.rs \
			resolver.rs

RUST_LIB_NAME = rust_lib
RUST_LIB_PATH = $(addprefix $(RUST_LIB_NAME), /target/release/)
RUST_LIB = $(addprefix $(RUST_LIB_PATH),  $(addsuffix .a, $(addprefix lib, $(RUST_LIB_NAME))))
RUST_PATH_SRCS = $(addprefix $(RUST_LIB_NAME), /src/resolver/)
RUST_SRC = $(addprefix $(RUST_PATH_SRCS), $(RUST_SRCS))

SRC = $(addprefix $(SRCS_PATH), $(SRCS))
HEADER = $(addprefix $(INCLUDES), $(HEADERS))

WFLAGS = -g -std=c++11 -Wall -Werror -Wextra

CC = clang++

OBJ= $(SRC:.cpp=.o)

all : $(NAME)

# $< ==  le nom de la dépendance (le .c)
# $@ == représente le nom de la règlE
%.o: %.cpp
	$(CC) $(WFLAGS) -I $(INCLUDES) -c $< -o $@

# $^ ==représente tous ce qui est après le :

$(NAME) : $(RUST_SRC) $(OBJ)
	cargo build --release --manifest-path=$(addprefix $(RUST_LIB_NAME), /Cargo.toml)
	$(CC) $(WFLAGS) -I $(INCLUDES) $(addprefix $(RUST_LIB_PATH),  $(addsuffix .a, $(addprefix lib, $(RUST_LIB_NAME)))) $(SRC) -o $@

clean :
	rm -rf $(OBJ)
	rm -rf $(addprefix $(RUST_LIB_NAME), /target)

fclean : clean
	rm -rf $(NAME)

re : fclean all

.PHONY: all clean fclean re
