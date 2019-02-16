NAME = npuzzle

SRCSPATH = srcs/
INCLUDES = includes/
HEADERS = npuzzle.hpp
RUST_LIB_NAME = rust_lib
RUST_LIB_PATH = $(addprefix $(RUST_LIB_NAME), /target/release/)
SRCS = main.cpp
SRC = $(addprefix $(SRCSPATH), $(SRCS))

HEADER = $(addprefix $(INCLUDES), $(HEADERS))
WFLAGS = -std=c++11 -Wall -Werror -Wextra

CC = clang++ -g

OBJ= $(SRC:.cpp=.o)

all : $(NAME)

# $< ==  le nom de la dépendance (le .c)
# $@ == représente le nom de la règlE
%.o: %.cpp
	$(CC) $(WFLAGS) -I $(INCLUDES) -c $< -o $@

# $^ ==représente tous ce qui est après le :
$(NAME) : $(OBJ)
	cargo build --release --manifest-path=$(addprefix $(RUST_LIB_NAME), /Cargo.toml)
	$(CC) $(WFLAGS) -L$(PWD)/rust_lib/target/release/ -lrust_lib -I $(INCLUDES) $^ -o $@

clean :
	rm -rf $(OBJ)
	rm -rf $(addprefix $(RUST_LIB_NAME), /target)

fclean : clean
	rm -rf $(NAME)

re : fclean all

.PHONY: all clean fclean re
