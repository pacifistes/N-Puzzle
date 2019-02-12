NAME = ft_ssl

SRCSPATH = srcs/
INCLUDES = includes/
HEADERS = ft_ssl.h
LIBFT = libftprintf/
LIBFTINCL = libftprintf/printf/includes/
RUST_LIB = rust_lib2/target/debug/
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
	$(CC) -g -o $(NAME) $(OBJ) $(WFLAGS) -I $(LIBFTINCL) -I $(INCLUDES) -L $(LIBFT) -lftprintf


clean :
	rm -rf $(OBJ)
	make -C $(LIBFT) clean

fclean : clean
	rm -rf $(NAME)
	make -C $(LIBFT) fclean

re : fclean all

.PHONY: all clean fclean re
