# Targets to update gcc source code, configure gcc, and build gcc

GCC_VERSION := 11.2.0
GCC_NAME := gcc-$(GCC_VERSION)
GCC_LINK := https://ftp.gnu.org/gnu/gcc/$(GCC_NAME)/$(GCC_NAME).tar.gz

GCC_SRC_DIR := $(EXT_SRC_DIR)/gcc
GCC_BUILD_DIR := $(EXT_BUILD_DIR)/gcc

GCC_TARGET 		 := x86_64-elf
GCC_PREFIX 		 := $(BUILD_SYSROOT)
GCC_CONFIGURE_FLAGS := --enable-languages=c --without-headers

export PATH := ${PATH}:$(GCC_PREFIX)

GCC_PATCH_V := 1
GCC_PATCHES_LINK := \
	https://github.com/YavaCoco/helium-gcc-patches/releases/download/p$(GCC_PATCH_V)/$(GCC_VERSION)-$(GCC_TARGET).tar.gz

GCC_MAKEFILE := $(GCC_BUILD_DIR)/Makefile
GCC_BIN := $(GCC_PREFIX)/bin/$(GCC_TARGET)-gcc

$(GCC_SRC_DIR): $(BUILD_DIR)
	$(MKDIR) -p $(GCC_SRC_DIR)
	$(CURL) -o $(BUILD_DIR)/gcc.tar.gz $(GCC_LINK)
	$(CD) $(BUILD_DIR) && $(TAR) -xvf $(BUILD_DIR)/gcc.tar.gz
	$(MV) $(BUILD_DIR)/$(GCC_NAME)/* $(GCC_SRC_DIR)
	$(CURL) -L -o $(BUILD_DIR)/gcc-patches.tar.gz $(GCC_PATCHES_LINK)
	$(CD) $(BUILD_DIR) && $(TAR) -xvf $(BUILD_DIR)/gcc-patches.tar.gz
	$(CD) $(BUILD_DIR)/$(GCC_TARGET) && $(FIND) . -type f -exec $(MV) -f "{}" "$(GCC_SRC_DIR)/{}" \;

$(GCC_MAKEFILE): binutils $(GCC_SRC_DIR)
	$(MKDIR) -p $(GCC_BUILD_DIR)
	$(CD) $(GCC_BUILD_DIR) && $(GCC_SRC_DIR)/configure --target=$(GCC_TARGET) \
		--prefix=$(GCC_PREFIX) $(GCC_CONFIGURE_FLAGS)

$(GCC_BIN): $(GCC_MAKEFILE)
	$(CD) $(GCC_BUILD_DIR) && $(MAKE) all-gcc
	$(CD) $(GCC_BUILD_DIR) && $(MAKE) all-target-libgcc
	$(CD) $(GCC_BUILD_DIR) && $(MAKE) install-gcc
	$(CD) $(GCC_BUILD_DIR) && $(MAKE) install-target-libgcc

.PHONY: gcc-src-update gcc-configure gcc gcc-rm gcc-clean
gcc-src-update: $(GCC_SRC_DIR)
gcc-configure:  $(GCC_MAKEFILE)
gcc: $(GCC_BIN)

gcc-rm: gcc-clean
	$(RM) -rf $(GCC_SRC_DIR)
gcc-clean:
	$(RM) -rf  $(GCC_BUILD_DIR)
