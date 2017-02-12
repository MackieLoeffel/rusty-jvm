.PHONY: all classfiles testfiles

JAVA_DIR = ./java
TEST_OUTPUTS_DIR = ./tests/outputs
JAVA_SOURCES = $(shell find $(JAVA_DIR) -name "*.java")
JAVAC_FLAGS = -source 1.2 -target 1.2
CLASS_FILES = $(patsubst %.java, %.class, $(JAVA_SOURCES))
TESTS = Jump
TEST_OUTPUTS = $(patsubst %, $(TEST_OUTPUTS_DIR)/%.out, $(TESTS))

all: classfiles testfiles

classfiles: $(CLASS_FILES)

$(JAVA_DIR)/%.class: $(JAVA_DIR)/%.java Makefile
	javac $(JAVAC_FLAGS) $<

testfiles: $(TEST_OUTPUTS)

$(TEST_OUTPUTS_DIR)/%.out: $(JAVA_DIR)/%.class Makefile
	(cd $(JAVA_DIR) && java $* > ../$@)
