NAME = npuzzle

SRCS_PATH = srcs/
HEADERS = npuzzle.h
INCLUDES = includes/
LIBFT = libftprintf/
LIBFTINCL = libftprintf/printf/includes/

SRCS = main.c
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

WFLAGS = -g -Wall -Werror -Wextra

CC = gcc

OBJ = $(SRC:.c=.o)

all:$(NAME)

libftprintf/libftprintf.a: libftprintf/libft/srcs/ libftprintf/libft/includes/ libftprintf/printf/ libftprintf/libft/includes/ libftprintf/Makefile
	make -C $(LIBFT) all

# $< ==  le nom de la dépendance (le .c)
# $@ == représente le nom de la règlE
%.o: %.c
	$(CC) -c $(WFLAGS) -I $(INCLUDES)  -I $(LIBFTINCL) $< -o $@

# $^ ==représente tous ce qui est après le :

$(NAME) : libftprintf/libftprintf.a $(OBJ) $(RUST_SRC)
	cargo build --release --manifest-path=$(addprefix $(RUST_LIB_NAME), /Cargo.toml)
	$(CC) $(WFLAGS) -I $(INCLUDES) -I $(LIBFTINCL) -L $(LIBFT) -lftprintf -framework Security $(addprefix $(RUST_LIB_PATH),  $(addsuffix .a, $(addprefix lib, $(RUST_LIB_NAME)))) -o $(NAME) $(OBJ)

clean:
	rm -rf $(OBJ)
	cargo clean --manifest-path=$(addprefix $(RUST_LIB_NAME), /Cargo.toml)
	make -C $(LIBFT) clean

fclean: clean
	rm -rf $(NAME)
	make -C $(LIBFT) fclean

re: fclean all

.PHONY: all clean fclean re
