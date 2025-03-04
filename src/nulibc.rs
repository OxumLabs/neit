pub static NULIBC: &'static str = r#"
#include <stdarg.h>
#if defined(UNIX) || defined(__linux__)
#include <unistd.h>
#endif
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#if defined(_WIN32) || defined(_WIN64)
#include <windows.h>
#else
#include <sys/syscall.h>
#endif

#define STDOUT 1
#define STDERR 2
#define STDIN 0

// Custom definition for size_t (unsigned long) to avoid conflict with mingw-w64's typedefs.
typedef unsigned long custom_size_t;

// Custom definition for pid_t (int) to avoid conflict with mingw-w64's typedefs.
typedef int custom_pid_t;

typedef struct {
    char *str;
    custom_size_t len;
} nstring;

custom_size_t nstrlen(nstring *str) {
    return str->len;
}

nstring nstr_new(const char *str) {
    nstring s;
    s.len = strlen(str);
    s.str = (char *)malloc(s.len + 1);
    strcpy(s.str, str);
    return s;
}

nstring nstrncpy(const nstring *s, custom_size_t start, custom_size_t length) {
    nstring result;
    if (!s || !s->str || start >= s->len) {
        result.str = NULL;
        result.len = 0;
        return result;
    }

    custom_size_t max_len = (start + length) > s->len ? s->len - start : length;
    result.str = (char *)malloc(max_len + 1);
    memcpy(result.str, s->str + start, max_len);
    result.str[max_len] = '\0';
    result.len = max_len;

    return result;
}

int strcmp(const char *str1, const char *str2) {
    while (*str1 && (*str1 == *str2)) {
        str1++;
        str2++;
    }
    return (unsigned char)*str1 - (unsigned char)*str2;
}

void write_char(int fd, char c) {
    #if defined(_WIN32) || defined(_WIN64)
    fwrite(&c, 1, 1, stdout);
    #endif
    #if defined(__unix__) || defined(__unix) || defined(__linux__) || defined(__APPLE__) || defined(__MACH__)
    write(fd, &c, 1);
    #endif
}

void write_str(int fd, const char *str) {
    while (*str) {
        #if defined(_WIN32) || defined(_WIN64)
        fwrite(str, 1, 1, stdout);
        #endif
        #if defined(__unix__) || defined(__unix) || defined(__linux__) || defined(__APPLE__) || defined(__MACH__)
        write(fd, str, 1);
        #endif
        str++;
    }
}

void write_num(int fd, int num) {
    char buffer[20];
    int i = 0;
    if (num == 0) {
        #if defined(_WIN32) || defined(_WIN64)
        fwrite("0", 1, 1, stdout);
        #endif
        #if defined(__unix__) || defined(__unix) || defined(__linux__) || defined(__APPLE__) || defined(__MACH__)
        write(fd, "0", 1);
        #endif
        return;
    }
    if (num < 0) {
        #if defined(_WIN32) || defined(_WIN64)
        fwrite("-", 1, 1, stdout);
        #endif
        #if defined(__unix__) || defined(__unix) || defined(__linux__) || defined(__APPLE__) || defined(__MACH__)
        write(fd, "-", 1);
        #endif
        num = -num;
    }
    while (num > 0) {
        buffer[i++] = (num % 10) + '0';
        num /= 10;
    }
    for (int j = i - 1; j >= 0; j--) {
        #if defined(_WIN32) || defined(_WIN64)
        fwrite(&buffer[j], 1, 1, stdout);
        #endif
        #if defined(__unix__) || defined(__unix) || defined(__linux__) || defined(__APPLE__) || defined(__MACH__)
        write(fd, &buffer[j], 1);
        #endif
    }
}

void write_unsigned(int fd, unsigned int num) {
    char buffer[20];
    int i = 0;
    if (num == 0) {
        #if defined(_WIN32) || defined(_WIN64)
        fwrite("0", 1, 1, stdout);
        #endif
        #if defined(__unix__) || defined(__unix) || defined(__linux__) || defined(__APPLE__) || defined(__MACH__)
        write(fd, "0", 1);
        #endif
        return;
    }
    while (num > 0) {
        buffer[i++] = (num % 10) + '0';
        num /= 10;
    }
    for (int j = i - 1; j >= 0; j--) {
        #if defined(_WIN32) || defined(_WIN64)
        fwrite(&buffer[j], 1, 1, stdout);
        #endif
        #if defined(__unix__) || defined(__unix) || defined(__linux__) || defined(__APPLE__) || defined(__MACH__)
        write(fd, &buffer[j], 1);
        #endif
    }
}

void write_hex(int fd, unsigned int num) {
    const char hex_chars[] = "0123456789abcdef";
    char buffer[10];
    int i = 0;
    if (num == 0) {
        #if defined(_WIN32) || defined(_WIN64)
        fwrite("0", 1, 1, stdout);
        #endif
        #if defined(__unix__) || defined(__unix) || defined(__linux__) || defined(__APPLE__) || defined(__MACH__)
        write(fd, "0", 1);
        #endif
        return;
    }
    while (num > 0) {
        buffer[i++] = hex_chars[num % 16];
        num /= 16;
    }
    for (int j = i - 1; j >= 0; j--) {
        #if defined(_WIN32) || defined(_WIN64)
        fwrite(&buffer[j], 1, 1, stdout);
        #endif
        #if defined(__unix__) || defined(__unix) || defined(__linux__) || defined(__APPLE__) || defined(__MACH__)
        write(fd, &buffer[j], 1);
        #endif
    }
}

void write_ptr(int fd, void *ptr) {
    // Write "0x" before the pointer value.
    #if defined(_WIN32) || defined(_WIN64)
    fwrite("0x", 2, 1, stdout);
    #endif
    #if defined(__unix__) || defined(__unix) || defined(__linux__) || defined(__APPLE__) || defined(__MACH__)
    write(fd, "0x", 2);
    #endif
    // Casting pointer to unsigned long long for 64-bit safety.
    write_hex(fd, (unsigned int)((unsigned long long)ptr));
}

void write_float(int fd, double num) {
    char buffer[50];
    int int_part = (int)num;
    double frac_part = num - int_part;

    write_num(fd, int_part);
    // Write decimal point.
    #if defined(_WIN32) || defined(_WIN64)
    fwrite(".", 1, 1, stdout);
    #endif
    #if defined(__unix__) || defined(__unix) || defined(__linux__) || defined(__APPLE__) || defined(__MACH__)
    write(fd, ".", 1);
    #endif

    // Multiply the fractional part to get 6 decimal places.
    frac_part *= 1000000;
    int frac_int = (int)(frac_part + 0.5); // rounding
    // Ensure leading zeros if necessary.
    char frac_buffer[10];
    snprintf(frac_buffer, sizeof(frac_buffer), "%06d", frac_int);
    write_str(fd, frac_buffer);
}

void write_long(int fd, long int num) {
    char buffer[32];
    int len = snprintf(buffer, sizeof(buffer), "%ld", num);
    if(len > 0) {
        #if defined(_WIN32) || defined(_WIN64)
        fwrite(buffer, 1, len, stdout);
        #endif
        #if defined(__unix__) || defined(__unix) || defined(__linux__) || defined(__APPLE__) || defined(__MACH__)
        write(fd, buffer, len);
        #endif
    }
}

void write_double(int fd, double num) {
    char buffer[64];
    int len = snprintf(buffer, sizeof(buffer), "%lf", num);
    if(len > 0) {
        #if defined(_WIN32) || defined(_WIN64)
        fwrite(buffer, 1, len, stdout);
        #endif
        #if defined(__unix__) || defined(__unix) || defined(__linux__) || defined(__APPLE__) || defined(__MACH__)
        write(fd, buffer, len);
        #endif
    }
}

void nprintf(int fd, const char *format, ...) {
    va_list args;
    va_start(args, format);
    const char *ptr = format;
    while (*ptr) {
        if (*ptr == '%' && *(ptr + 1) == 'd') {
            int num = va_arg(args, int);
            write_num(fd, num);
            ptr += 2;
        } else if (*ptr == '%' && *(ptr + 1) == 'u') {
            unsigned int num = va_arg(args, unsigned int);
            write_unsigned(fd, num);
            ptr += 2;
        } else if (*ptr == '%' && *(ptr + 1) == 's') {
            char *str = va_arg(args, char*);
            write_str(fd, str);
            ptr += 2;
        } else if (*ptr == '%' && *(ptr + 1) == 'c') {
            char c = (char)va_arg(args, int);
            write_char(fd, c);
            ptr += 2;
        } else if (*ptr == '%' && *(ptr + 1) == 'x') {
            unsigned int hex = va_arg(args, unsigned int);
            write_hex(fd, hex);
            ptr += 2;
        } else if (*ptr == '%' && *(ptr + 1) == 'p') {
            void *p = va_arg(args, void*);
            write_ptr(fd, p);
            ptr += 2;
        } else if (*ptr == '%' && *(ptr + 1) == 'f') {
            double num = va_arg(args, double);
            write_float(fd, num);
            ptr += 2;
        } else if (*ptr == '%' && *(ptr + 1) == '%' ) {
            write_char(fd, '%');
            ptr += 2;
        } else if (*ptr == '%' && *(ptr + 1) == 'l' && *(ptr + 2) == 'd') {
            long int num = va_arg(args, long int);
            write_long(fd, num);
            ptr += 3;
        } else if (*ptr == '%' && *(ptr + 1) == 'l' && *(ptr + 2) == 'f') {
            double num = va_arg(args, double);
            write_double(fd, num);
            ptr += 3;
        } else {
            write_char(fd, *ptr);
            ptr++;
        }
    }
    va_end(args);
}
typedef struct {
    int SUCCESS;
    int FAILURE;
    int INVALID_ARGUMENT;
    int COMMAND_NOT_FOUND;
    int PERMISSION_DENIED;
    int SIGNAL_TERMINATED;
    int SIGNAL_INT;
    int SEGFAULT;
    int OUT_OF_RANGE;
} ExitCode;

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

void nexit(int status) {
    exit(status);
}

int nsys(const char *command) {
   return system(command);
}

void __NCLRSCRN__() {
    #if defined(_WIN32) || defined(_WIN64)
        if (nsys("cls") == -1) {
            return;
        }
    #elif defined(__unix__) || defined(__unix) || defined(__linux__) || defined(__APPLE__) || defined(__MACH__)
        if (system("clear") == -1) {
            return;
        }
    #else
        return;
    #endif
}

typedef struct MemBlock {
    char* data;
    custom_size_t size;
    int free;
    struct MemBlock* next;
    char name[64];
} MemBlock;

typedef struct MemManager {
    custom_size_t total;
    custom_size_t used;
    MemBlock* blocks;
} MemManager;

MemManager* mem_init(custom_size_t total) {
    MemManager* mgr = (MemManager*)malloc(sizeof(MemManager));
    if (!mgr) return NULL;
    mgr->total = total;
    mgr->used = 0;
    mgr->blocks = NULL;
    return mgr;
}

MemBlock* mem_alloc(MemManager* mgr, custom_size_t size, const char* name) {
    if (!mgr || size == 0 || mgr->used + size > mgr->total || !name) return NULL;

    MemBlock* blk = (MemBlock*)malloc(sizeof(MemBlock));
    if (!blk) return NULL;

    blk->data = (char*)malloc(size);
    if (!blk->data) {
        free(blk);
        return NULL;
    }

    blk->size = size;
    blk->free = 0;
    blk->next = mgr->blocks;
    mgr->blocks = blk;
    mgr->used += size;

    strncpy(blk->name, name, sizeof(blk->name) - 1);
    blk->name[sizeof(blk->name) - 1] = '\0';

    return blk;
}

void mem_free(MemManager* mgr, MemBlock* blk) {
    if (!mgr || !blk || blk->free) return;

    blk->free = 1;
    mgr->used -= blk->size;
    free(blk->data);
    blk->data = NULL;
}

void mem_cleanup(MemManager* mgr) {
    if (!mgr) return;

    MemBlock* current = mgr->blocks;
    while (current) {
        MemBlock* next = current->next;
        free(current->data);
        free(current);
        current = next;
    }

    free(mgr);
}

nstring nstrcpy(const nstring *src) {
    if (!src || !src->str) return (nstring){.str = NULL, .len = 0};
    char *new_str = (char *)malloc(src->len + 1);
    for (custom_size_t i = 0; i < src->len; i++) {
        new_str[i] = src->str[i];
    }
    new_str[src->len] = '\0';
    return (nstring){.str = new_str, .len = src->len};
}

int nstr_cmp(const nstring *s1, const nstring *s2) {
    if (!s1 || !s2 || !s1->str || !s2->str) return 0;
    custom_size_t min_len = s1->len < s2->len ? s1->len : s2->len;
    for (custom_size_t i = 0; i < min_len; i++) {
        if (s1->str[i] != s2->str[i]) return s1->str[i] - s2->str[i];
    }
    return s1->len - s2->len;
}

nstring nstrcat(const nstring *s1, const nstring *s2) {
    if (!s1 || !s2 || !s1->str || !s2->str) return (nstring){.str = NULL, .len = 0};
    custom_size_t new_len = s1->len + s2->len;
    char *new_str = (char *)malloc(new_len + 1);
    memcpy(new_str, s1->str, s1->len);
    memcpy(new_str + s1->len, s2->str, s2->len);
    new_str[new_len] = '\0';
    return (nstring){.str = new_str, .len = new_len};
}
void ninput(nstring *ns) {
    size_t buffer_size = 64;  // Start with an initial buffer size
    size_t len = 0;          // Length of the string input
    char *buffer = (char *)malloc(buffer_size);
    int ch;

    if (!buffer) {
        fprintf(stderr, "Memory allocation failed.\n");
        return;
    }

    // Read input character by character
    while ((ch = getchar()) != '\n' && ch != EOF) {
        // Check if we need to expand the buffer
        if (len + 1 >= buffer_size) {
            buffer_size *= 2;
            char *new_buffer = (char *)realloc(buffer, buffer_size);
            if (!new_buffer) {
                free(buffer);
                fprintf(stderr, "Memory reallocation failed.\n");
                return;
            }
            buffer = new_buffer;
        }

        buffer[len++] = (char)ch;  // Store the character in the buffer
    }

    buffer[len] = '\0';  // Null-terminate the string

    // Assign the string to nstring
    ns->len = len;
    ns->str = buffer;
}

"#;

pub static NULIBCH: &'static str = r#"
#ifndef NULIBC_H
#define NULIBC_H

#include <stdarg.h>
#if defined(UNIX)
#include <unistd.h>
#endif 
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>

typedef int8_t  i8;
typedef int32_t i32;
typedef int64_t i64;
typedef float  f32;
typedef double f64;


#define STDOUT 1
#define STDERR 2
#define STDIN 0

#if defined(_WIN32) || defined(_WIN64)
#include <windows.h>
#else
#include <sys/syscall.h>
#endif

typedef unsigned long custom_size_t;
typedef int custom_pid_t;

typedef struct {
    char *str;
    custom_size_t len;
} nstring;

typedef struct MemBlock {
    char* data;
    custom_size_t size;
    int free;
    struct MemBlock* next;
    char name[64];
} MemBlock;

typedef struct MemManager {
    custom_size_t total;
    custom_size_t used;
    MemBlock* blocks;
} MemManager;

custom_size_t nstrlen(nstring *str);
nstring nstr_new(const char *str);
nstring nstrncpy(const nstring *s, custom_size_t start, custom_size_t length);
int strcmp(const char *str1, const char *str2);
void write_char(int fd, char c);
void write_str(int fd, const char *str);
void write_num(int fd, int num);
void write_unsigned(int fd, unsigned int num);
void write_hex(int fd, unsigned int num);
void write_ptr(int fd, void *ptr);
void write_float(int fd, double num);
void nprintf(int fd, const char *format, ...);
void nexit(int status);
int nsys(const char *command);
void __NCLRSCRN__();
MemManager* mem_init(custom_size_t total);
MemBlock* mem_alloc(MemManager* mgr, custom_size_t size, const char* name);
void mem_free(MemManager* mgr, MemBlock* blk);
void mem_cleanup(MemManager* mgr);
nstring nstrcpy(const nstring *src);
int nstr_cmp(const nstring *s1, const nstring *s2);
nstring nstrcat(const nstring *s1, const nstring *s2);
void ninput(nstring *nstring);
int file_exists(nstring filename);

#endif // NULIBC_H
"#;
