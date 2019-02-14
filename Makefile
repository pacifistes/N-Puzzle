NAME = npuzzle

SRCSPATH = srcs/
INCLUDES = includes/
HEADERS = ft_ssl.h
LIBFT = libftprintf/
LIBFTINCL = libftprintf/printf/includes/
RUST_LIB_NAME = rust_lib2
RUST_LIB_PATH = $(addprefix $(RUST_LIB_NAME), /target/release/)
SRCS = main.c
SRC = $(addprefix $(SRCSPATH), $(SRCS))

HEADER = $(addprefix $(INCLUDES), $(HEADERS))
WFLAGS = -Wall -Werror -Wextra

CC = gcc -g

OBJ = $(SRC:.c=.o)

all : $(NAME)


libftprintf/libftprintf.a: libftprintf/libft/srcs/ libftprintf/libft/includes/ libftprintf/printf/ libftprintf/libft/includes/ libftprintf/Makefile
	make -C $(LIBFT) all

%.o: %.c $(HEADER)
	$(CC) -g -c $(WFLAGS) -I $(LIBFTINCL) -I $(INCLUDES) $< -o $@

$(NAME) : libftprintf/libftprintf.a $(OBJ)
	cargo build --release --manifest-path=$(addprefix $(RUST_LIB_NAME), /Cargo.toml)
	$(CC) -g -o $(NAME) $(OBJ) $(WFLAGS) $(addprefix $(RUST_LIB_PATH),  $(addsuffix .a, $(addprefix lib, $(RUST_LIB_NAME)))) -I $(LIBFTINCL) -I $(INCLUDES) -L $(LIBFT) -lftprintf


clean :
	rm -rf $(OBJ)
	rm -rf $(addprefix $(RUST_LIB_NAME), /target)
	make -C $(LIBFT) clean

fclean : clean
	rm -rf $(NAME)
	make -C $(LIBFT) fclean

re : fclean all

.PHONY: all clean fclean re
