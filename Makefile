NAME = npuzzle

SRCS_PATH = srcs/
HEADERS = npuzzle.h
INCLUDES = includes/

SRCS = main.cpp
RUST_RESOLVER_SRCS = generate.rs \
			heuristic.rs \
			parser.rs \
			puzzle.rs \
			resolver.rs

RUST_BINDING_SRCS = generate.rs \
			parser.rs \
			puzzle.rs \
			resolver.rs

RUST_LIB_NAME = rust_lib
RUST_LIB_PATH = $(addprefix $(RUST_LIB_NAME), /target/release/)
RUST_LIB = $(addprefix $(RUST_LIB_PATH),  $(addsuffix .a, $(addprefix lib, $(RUST_LIB_NAME))))
RUST_PATH_RESOLVER_SRCS = $(addprefix $(RUST_LIB_NAME), /src/resolver/)
RUST_PATH_BINDING_SRCS = $(addprefix $(RUST_LIB_NAME), /src/binding/)
RUST_RESOLVER_SRC = $(addprefix $(RUST_PATH_RESOLVER_SRCS), $(RUST_RESOLVER_SRCS))
RUST_BINDING_SRC = $(addprefix $(RUST_PATH_BINDING_SRCS), $(RUST_BINDING_SRCS))

SRC = $(addprefix $(SRCS_PATH), $(SRCS))
HEADER = $(addprefix $(INCLUDES), $(HEADERS))

WFLAGS = -g -Wall -Werror -Wextra #-fsanitize=address


ifeq ($(OS),Windows_NT)
	NATIVE_STATIC_LIBS = -lutil -lutil -ldl -lrt -lpthread -lgcc_s -lc -lm -lrt -lpthread -lutil -lutil
else
	NATIVE_STATIC_LIBS = -framework Security -lSystem -lresolv -lc -lm
endif


CC = clang++

OBJ = $(SRC:.cpp=.o)

all:$(NAME)

# $< ==  le nom de la dépendance (le .c)
# $@ == représente le nom de la règlE
%.o: %.cpp $(HEADER) Makefile
	$(CC) -c $(WFLAGS) -I $(INCLUDES) $< -o $@

# $^ ==représente tous ce qui est après le :

$(NAME) :  $(OBJ) $(RUST_RESOLVER_SRC) $(RUST_BINDING_SRC)
	RUSTFLAGS="--print=native-static-libs" cargo build --release --manifest-path=$(addprefix $(RUST_LIB_NAME), /Cargo.toml)
	$(CC) $(WFLAGS) -o $(NAME) $(OBJ) -v -I $(INCLUDES) $(addprefix $(RUST_LIB_PATH),  $(addsuffix .a, $(addprefix lib, $(RUST_LIB_NAME)))) $(NATIVE_STATIC_LIBS)



test:
	cargo test --release --manifest-path=$(addprefix $(RUST_LIB_NAME), /Cargo.toml) -- --nocapture

clean:
	rm -rf $(OBJ)
	cargo clean --manifest-path=$(addprefix $(RUST_LIB_NAME), /Cargo.toml)

fclean: clean
	rm -rf $(NAME)

re: fclean all

.PHONY: all clean fclean re
