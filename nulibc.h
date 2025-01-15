
#ifndef NULIBC
#define NULIBC

/* ============================================================
 * DEBUGGING SETTINGS
 * ============================================================ */

extern int DEBUG;  // Flag for enabling debugging output

/* ============================================================
 * FILE DESCRIPTORS
 * ============================================================ */

#define STDOUT 1  // File descriptor for standard output
#define STDERR 2  // File descriptor for standard error
#define STDIN  0  // File descriptor for standard input

/* ============================================================
 * MEMORY MANAGEMENT STRUCTURES AND FUNCTIONS
 * ============================================================ */

/* Custom type definition for `size_t` to represent sizes in the memory manager */
typedef unsigned long size_t;

/* Memory Block Structure */
typedef struct MemBlock {
    char *data;            // Pointer to the data block
    size_t size;           // Size of the block
    int free;              // Whether the block is free
    struct MemBlock *next; // Next memory block in the linked list
    char name[64];         // Name of the memory block
} MemBlock;

/* Memory Manager Structure */
typedef struct MemManager {
    size_t total;  // Total available memory
    size_t used;   // Memory used by allocated blocks
    MemBlock *blocks; // List of memory blocks managed
} MemManager;

/* Memory Management API Functions */

/**
 * Initializes a memory manager.
 * @param total: Total size of memory that can be managed.
 * @return: Pointer to the initialized MemManager.
 */
MemManager* mem_init(size_t total);

/**
 * Allocates a memory block of the specified size and associates it with a name.
 * @param mgr: Pointer to the memory manager.
 * @param size: Size of the memory block to be allocated.
 * @param name: Name to associate with the memory block.
 * @return: Pointer to the allocated MemBlock.
 */
MemBlock* mem_alloc(MemManager* mgr, size_t size, const char* name);

/**
 * Frees a memory block and marks it as free in the memory manager.
 * @param mgr: Pointer to the memory manager.
 * @param blk: Pointer to the memory block to be freed.
 */
void mem_free(MemManager* mgr, MemBlock* blk);

/**
 * Cleans up all allocated memory blocks and the memory manager.
 * @param mgr: Pointer to the memory manager to be cleaned up.
 */
void mem_cleanup(MemManager* mgr);

/**
 * Writes a string to a specified memory block.
 * @param blk: Pointer to the memory block.
 * @param str: String to write to the memory block.
 * @return: 0 on success, negative error code on failure.
 */
int mem_wrtstr(MemBlock* blk, const char* str);

/**
 * Writes an integer value to a specified memory block.
 * @param blk: Pointer to the memory block.
 * @param value: Integer value to write to the memory block.
 * @return: 0 on success, negative error code on failure.
 */
int mem_wrtint(MemBlock* blk, int value);

/**
 * Reads an integer value from a memory block by name.
 * @param mgr: Pointer to the memory manager.
 * @param name: Name associated with the memory block.
 * @return: Integer value read from the memory block, or -1 on failure.
 */
int mem_rdint(MemManager* mgr, const char* name);

/**
 * Reads a string from a memory block by name.
 * @param mgr: Pointer to the memory manager.
 * @param name: Name associated with the memory block.
 * @return: Pointer to the string, or NULL on failure.
 */
char* mem_rdstr(MemManager* mgr, const char* name);

/* ============================================================
 * STRING MANAGEMENT STRUCTURES AND FUNCTIONS
 * ============================================================ */

/* Custom string type `nstring` to represent a string with an associated length */
typedef struct {
    char *str;   // Pointer to the string data
    size_t len;  // Length of the string
} nstring;

/* String Management API Functions */

/**
 * Creates a new `nstring` from a given C string.
 * @param str: The C string to be converted into `nstring`.
 * @return: A new `nstring` structure containing the given string and its length.
 */
nstring nstr_new(const char *str);

/**
 * Frees an array of `nstring`s
 * @param array: The array of `nstring`s to free
 * @param count: The number of elements in the array
 */
void free_nstr_array(nstring *array, size_t count);

/**
 * Returns the length of an `nstring`.
 * @param str: The `nstring` whose length to calculate.
 * @return: The length of the string.
 */
size_t nstrlen(nstring *str);

/**
 * Copies the content of one `nstring` into a new `nstring` object.
 * @param src: The source `nstring` to copy from.
 * @return: A new `nstring` with the same content as `src`.
 */
nstring nstrcpy(const nstring *src);

/**
 * Compares two `nstring` objects.
 * @param s1: The first `nstring` to compare.
 * @param s2: The second `nstring` to compare.
 * @return: 0 if the `nstring` objects are equal, a negative value if `s1` is less than `s2`,
 *         or a positive value if `s1` is greater than `s2`.
 */
int nstr_cmp(const nstring *s1, const nstring *s2);

/**
 * Concatenates two `nstring` objects into a new `nstring`.
 * @param s1: The first `nstring` to concatenate.
 * @param s2: The second `nstring` to concatenate.
 * @return: A new `nstring` containing the concatenation of `s1` and `s2`.
 */
nstring nstrcat(const nstring *s1, const nstring *s2);

/**
 * Finds the first occurrence of a character in an `nstring`.
 * @param s: The `nstring` to search.
 * @param c: The character to find.
 * @return: The index of the first occurrence of `c` in `s`, or `-1` if not found.
 */
size_t nstrchr(const nstring *s, char c);

/**
 * Frees the memory allocated for an `nstring`.
 * @param s: The `nstring` to free.
 */
void stringfree(nstring *s);

/**
 * Duplicates a C string into a new `nstring`.
 * @param cstr: The C string to duplicate.
 * @return: A new `nstring` containing the content of `cstr`.
 */
nstring nstrdup(const char *cstr);

/**
 * Retrieves the character at a specified index from an `nstring`.
 * @param s: The `nstring` to access.
 * @param index: The index of the character to retrieve.
 * @return: The character at the specified index.
 */
char nstr_at_s(const nstring *s, size_t index);

/**
 * Splits a string at every instance of a specified character
 * @param input: The string to split
 * @param delimiter: The character to split at
 * @return: An array of `nstring`s, each containing a substring of the input string
 */
nstring* nstr_split_at_every(nstring input, char delimiter);

/* ============================================================
 * UTILITY FUNCTIONS
 * ============================================================ */

/**
 * Re-implementation of `printf` in nulibc.
 * @param fd: The file descriptor to write to (STDOUT, STDERR, STDIN).
 * @param format: The format string, followed by a variable number of arguments.
 */
void nprintf(int fd, const char *format, ...);

/**
 * Compares two C strings.
 * @param str1: The first string to compare.
 * @param str2: The second string to compare.
 * @return: 0 if the strings are equal, a negative value if `str1` is less than `str2`,
 *         or a positive value if `str1` is greater than `str2`.
 */
int strcmp(const char *str1, const char *str2);

/**
 * Exits the program with the specified status code
 * @param status: The status code to exit with
 */
void nexit(int status);

/**
 * @struct ExitCode
 * @brief Defines a set of common exit statuses for programs.
 *
 * This struct contains exit codes that can be used for signaling different types of termination reasons
 * for processes. The exit codes are organized as named constants to allow descriptive access to common
 * exit statuses.
 */
typedef struct {
    /** Exit status indicating success */
    int SUCCESS;
    
    /** Exit status indicating a general failure */
    int FAILURE;
    
    /** Exit status indicating invalid arguments were passed to the program */
    int INVALID_ARGUMENT;
    
    /** Exit status indicating the command was not found */
    int COMMAND_NOT_FOUND;
    
    /** Exit status indicating permission was denied */
    int PERMISSION_DENIED;
    
    /** Exit status indicating the process was terminated by a signal */
    int SIGNAL_TERMINATED;
    
    /** Exit status indicating the process was terminated by signal interrupt (SIGINT) */
    int SIGNAL_INT;
    
    /** Exit status indicating the process crashed due to a segmentation fault (SIGSEGV) */
    int SEGFAULT;
    
    /** Exit status indicating the program went out of range or encountered an invalid range */
    int OUT_OF_RANGE;
} ExitCode;
/** 
 * @var ExitStatus
 * @brief A static instance of ExitCode struct initialized with common exit status values.
 *
 * This constant is a pre-initialized instance of the ExitCode struct that contains common exit status
 * values that can be used across the program.
 */
static const ExitCode ExitStatus = {
    .SUCCESS = 0,
    .FAILURE = 1,
    .INVALID_ARGUMENT = 128,
    .COMMAND_NOT_FOUND = 127,
    .PERMISSION_DENIED = 126,
    .SIGNAL_TERMINATED = 137,
    .SIGNAL_INT = 130,
    .SEGFAULT = 11,
    .OUT_OF_RANGE = 255
};

/**
 * Clears the screen
 */
void __NCLRSCRN__();

int nsys(const char *command);
/**
 * Converts a C string to an `nstring`
 * @param cstr: The C string to convert
 * @return: An `nstring` containing the content of `cstr`
 */
nstring to_nstr(const char *cstr);

/**
 * Reads a line of input from the user and stores it in an `nstring`
 * @param nstr: The `nstring` to store the input in
 */
void ninput(nstring *nstr);

#endif

