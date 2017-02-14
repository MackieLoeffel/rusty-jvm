.PHONY: all classfiles testfiles test build

JAVA_DIR = ./java
TEST_OUTPUTS_DIR = ./jvm-outputs
JAVA_SOURCES = $(shell find $(JAVA_DIR) -name "*.java")
JAVAC_FLAGS = -source 1.2 -target 1.2
CLASS_FILES = $(patsubst %.java, %.class, $(JAVA_SOURCES))
TESTS = Jump Calc
TEST_OUTPUTS = $(patsubst %, $(TEST_OUTPUTS_DIR)/%.out, $(TESTS))

all: classfiles testfiles

classfiles: $(CLASS_FILES)

$(JAVA_DIR)/%.class: $(JAVA_DIR)/%.java Makefile
	javac $(JAVAC_FLAGS) $<

testfiles: $(TEST_OUTPUTS_DIR) $(TEST_OUTPUTS)

$(TEST_OUTPUTS_DIR):
	mkdir -p $(TEST_OUTPUTS_DIR)

$(TEST_OUTPUTS_DIR)/%.out: $(JAVA_DIR)/%.class Makefile
	(cd $(JAVA_DIR) && java $* > ../$@)

build:
	cargo build

test: testfiles
	@for test in $(TESTS); do \
		cargo run --release -- "$$test" | diff -u "$(TEST_OUTPUTS_DIR)/$$test.out" -; \
	done
