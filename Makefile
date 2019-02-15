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

OBJ = $(SRC:.c=.o)

all : $(NAME)


%.o: %.c $(HEADER)
	$(CC) -c $(WFLAGS) -I $(INCLUDES) $< -o $@

$(NAME) : $(OBJ)
	cargo build --release --manifest-path=$(addprefix $(RUST_LIB_NAME), /Cargo.toml)
	# $(CC) -o $(NAME) $(OBJ) $(WFLAGS) $(addprefix $(RUST_LIB_PATH),  $(addsuffix .a, $(addprefix lib, $(RUST_LIB_NAME)))) -I $(INCLUDES)
	# $(CC) -o $(NAME) $(OBJ) $(WFLAGS) -I $(INCLUDES) -L./rust_lib/target/release/ -lrust_lib
	$(CC) -o $(NAME) $(OBJ) $(WFLAGS) -L$(PWD)/rust_lib/target/release/ -lrust_lib -I $(INCLUDES)

clean :
	rm -rf $(OBJ)
	rm -rf $(addprefix $(RUST_LIB_NAME), /target)

fclean : clean
	rm -rf $(NAME)

re : fclean all

.PHONY: all clean fclean re
