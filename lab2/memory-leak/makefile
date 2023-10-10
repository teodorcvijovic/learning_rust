C_BINARIES = linkedlist bracket-parser
CXX_BINARIES = fibonacci
BINARIES = $(C_BINARIES) $(CXX_BINARIES)

CC = clang
CXX = clang++

CALLFLAGS = -g -O0 -Wall -Wextra
CFLAGS = $(CALLFLAGS) -std=gnu99
CXXFLAGS = $(CALLFLAGS) -std=c++17
LDFLAGS =

all: $(BINARIES)

%.o: %.c
	$(CC) $(CFLAGS) -c -o $@ $<

$(C_BINARIES): %:%.o
	$(CC) $^ $(LDFLAGS) -o $@
$(CXX_BINARIES): %:%.o
	$(CXX) $^ $(LDFLAGS) -o $@

distclean: clean
clean:
	rm -f *.o $(BINARIES)