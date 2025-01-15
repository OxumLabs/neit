
    #include <stdarg.h>
    #include <unistd.h>
    
    #if UNIX
    typedef unsigned long size_t;
    #endif
    typedef int pid_t;
    typedef struct{
        char *str;
        size_t len;
    }nstring;
    size_t nstrlen(nstring *str);
    nstring nstr_new(const char *str){
        nstring s;
    }
nstring nstrncpy(const nstring *s, size_t start, size_t length);
#include <unistd.h>
#include <stdarg.h>
#include <stdint.h>
#include <stdio.h>

#define STDOUT 1
#define STDERR 2
#define STDIN 0

#ifdef _WIN32
#include <windows.h>
#endif

void write_char(int fd, char c) {
    write(fd, &c, 1);
}

void write_str(int fd, const char *str) {
    if (str) {
        while (*str) {
            write(fd, str++, 1);
        }
    }
}

void write_num(int fd, int num) {
    char buffer[num];
    int i = 0;

    if (num == 0) {
        write(fd, "0", 1);
        return;
    }

    if (num < 0) {
        write(fd, "-", 1);
        num = -num;
    }

    while (num > 0) {
        buffer[i++] = (num % 10) + '0';
        num /= 10;
    }

    for (int j = i - 1; j >= 0; j--) {
        write(fd, &buffer[j], 1);
    }
}

void write_unsigned(int fd, unsigned int num) {
    char buffer[20];
    int i = 0;

    if (num == 0) {
        write(fd, "0", 1);
        return;
    }

    while (num > 0) {
        buffer[i++] = (num % 10) + '0';
        num /= 10;
    }

    for (int j = i - 1; j >= 0; j--) {
        write(fd, &buffer[j], 1);
    }
}

void write_hex(int fd, unsigned int num) {
    const char hex_chars[] = "0123456789abcdef";
    char buffer[10];
    int i = 0;

    if (num == 0) {
        write(fd, "0", 1);
        return;
    }

    while (num > 0) {
        buffer[i++] = hex_chars[num % 16];
        num /= 16;
    }

    for (int j = i - 1; j >= 0; j--) {
        write(fd, &buffer[j], 1);
    }
}

void write_ptr(int fd, void *ptr) {
    write(fd, "0x", 2);
    write_hex(fd, (unsigned long)ptr);
}

void write_float(int fd, double num) {
    char buffer[50];
    int int_part = (int)num;
    double frac_part = num - int_part;
    
    write_num(fd, int_part);
    write(fd, ".", 1);

    frac_part *= 1000000;
    int frac_int = (int)frac_part;
    write_num(fd, frac_int);
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
        }
        else if (*ptr == '%' && *(ptr +1) == 'z' && *(ptr + 2) == 'u') {
            unsigned int num = va_arg(args, unsigned int);
            write_unsigned(fd, num);
            ptr += 3;
        }
        else if (*ptr == '%' && *(ptr + 1) == 's') {
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
            void *ptr_value = va_arg(args, void*);
            write_ptr(fd, ptr_value);
            ptr += 2;
        } else if (*ptr == '%' && *(ptr + 1) == 'f') {
            double num = va_arg(args, double);
            write_float(fd, num);
            ptr += 2;
        } else if (*ptr == '%' && *(ptr + 1) == '%') {
            write_char(fd, '%');
            ptr += 2;
        } else {
            write_char(fd, *ptr);
            ptr++;
        }
    }

    va_end(args);
}

    int strcmp(const char *str1, const char *str2) {
        while (*str1 != '\0' && *str2 != '\0') {
            if (*str1 != *str2) {
                return (unsigned char)(*str1) - (unsigned char)(*str2);
            }
            str1++;
            str2++;
        }
        return (unsigned char)(*str1) - (unsigned char)(*str2);
    }
#include <unistd.h>
#include <sys/syscall.h>
#include <stdlib.h>

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

/**
 * Exit status built-ins
 */
void nexit(int status) {
    exit(status);
}

int nsys(const char *command) {
   system(command);
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

/*
=============================Memory Manager====================================
*/
#include <stdlib.h>
#include <string.h>

typedef struct MemBlock {
    char* data;
    size_t size;
    int free;
    struct MemBlock* next;
    char name[64];
} MemBlock;

typedef struct MemManager {
    size_t total;
    size_t used;
    MemBlock* blocks;
} MemManager;

MemManager* mem_init(size_t total) {
    MemManager* mgr = (MemManager*)malloc(sizeof(MemManager));
    if (!mgr) return NULL;
    mgr->total = total;
    mgr->used = 0;
    mgr->blocks = NULL;
    return mgr;
}

MemBlock* mem_alloc(MemManager* mgr, size_t size, const char* name) {
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

int mem_wrtstr(MemBlock* blk, const char* str) {
    if (!blk || blk->free || !str) return -1;

    size_t str_len = strlen(str);
    if (str_len + 1 > blk->size) return -2;

    strcpy(blk->data, str);
    return 0;
}

int mem_write_str_by_name(MemManager* mgr, const char* name, const char* str) {
    if (!mgr || !name || !str) return -1;

    MemBlock* blk = mgr->blocks;
    while (blk) {
        if (blk->free == 0 && strcmp(name, blk->name) == 0) {
            return mem_wrtstr(blk, str);
        }
        blk = blk->next;
    }

    size_t str_len = strlen(str) + 1;
    blk = mem_alloc(mgr, str_len, name);
    if (!blk) return -3;

    return mem_wrtstr(blk, str);
}

char* mem_rdstr(MemManager* mgr, const char* name) {
    if (!mgr || !name) return NULL;

    MemBlock* blk = mgr->blocks;
    while (blk) {
        if (blk->free == 0 && strcmp(name, blk->name) == 0) {
            return blk->data;
        }
        blk = blk->next;
    }
    return NULL;
}

int mem_wrtint(MemBlock* blk, int value) {
    if (!blk || blk->free) return -1;

    if (blk->size < sizeof(int)) return -2;

    memcpy(blk->data, &value, sizeof(int));
    return 0;
}

int mem_write_int_by_name(MemManager* mgr, const char* name, int value) {
    if (!mgr || !name) return -1;

    MemBlock* blk = mgr->blocks;
    while (blk) {
        if (blk->free == 0 && strcmp(name, blk->name) == 0) {
            return mem_wrtint(blk, value);
        }
        blk = blk->next;
    }

    blk = mem_alloc(mgr, sizeof(int), name);
    if (!blk) return -3;

    return mem_wrtint(blk, value);
}

int mem_rdint(MemManager* mgr, const char* name) {
    if (!mgr || !name) return -1;

    MemBlock* blk = mgr->blocks;
    while (blk) {
        if (blk->free == 0 && strcmp(name, blk->name) == 0) {
            int value;
            memcpy(&value, blk->data, sizeof(int));
            return value;
        }
        blk = blk->next;
    }
    return -1;
}
int mem_wrnstring(MemBlock* blk, nstring str) {
    if (!blk || blk->free || !str.str) return -1;
    size_t str_len = str.len + 1;
    if (str_len + 1 > blk->size) return -2;
    memcpy(blk->data, str.str, str_len);
    return 0;
}
nstring mem_rdnstring(MemManager* mgr, const char* name) {
    if (!mgr || !name) return (nstring){.str = NULL, .len = 0};
    MemBlock* blk = mgr->blocks;
    while (blk) {
        if (blk->free == 0 && strcmp(name, blk->name) == 0) {
            nstring str;
            memcpy(&str, blk->data, sizeof(nstring));
            return str;
        }
        blk = blk->next;
    }
    return (nstring){.str = NULL, .len = 0};
}


/*
=============================String Functions====================================
*/
size_t nstrlen(nstring *str) {
    return str->len;
}

nstring nstrcpy(const nstring *src) {
    if (!src || !src->str) return (nstring){.str = NULL, .len = 0};
    char *new_str = (char *)malloc(src->len + 1);
    for (size_t i = 0; i < src->len; i++) {
        new_str[i] = src->str[i];
    }
    new_str[src->len] = '\0';
    return (nstring){.str = new_str, .len = src->len};
}

int nstr_cmp(const nstring *s1, const nstring *s2) {
    if (!s1 || !s2 || !s1->str || !s2->str) return 0;
    size_t min_len = s1->len < s2->len ? s1->len : s2->len;
    for (size_t i = 0; i < min_len; i++) {
        if (s1->str[i] != s2->str[i]) return s1->str[i] - s2->str[i];
    }
    return s1->len - s2->len;
}

nstring nstrcat(const nstring *s1, const nstring *s2) {
    if (!s1 || !s2 || !s1->str || !s2->str) return (nstring){.str = NULL, .len = 0};
    size_t new_len = s1->len + s2->len;
    char *new_str = (char *)malloc(new_len + 1);
    for (size_t i = 0; i < s1->len; i++) {
        new_str[i] = s1->str[i];
    }
    for (size_t i = 0; i < s2->len; i++) {
        new_str[s1->len + i] = s2->str[i];
    }
    new_str[new_len] = '\0';
    return (nstring){.str = new_str, .len = new_len};
}

size_t nstrchr(const nstring *s, char c) {
    if (!s || !s->str) return (size_t)-1;
    for (size_t i = 0; i < s->len; i++) {
        if (s->str[i] == c) return i;
    }
    return (size_t)-1;
}

nstring to_nstr(const char *cstr) {
    if (!cstr) return (nstring){.str = NULL, .len = 0};
    size_t len = 0;
    while (cstr[len]) len++;
    char *new_str = (char *)malloc(len + 1);
    for (size_t i = 0; i < len; i++) {
        new_str[i] = cstr[i];
    }
    new_str[len] = '\0';
    return (nstring){.str = new_str, .len = len};
}

nstring nstrncpy(const nstring *s, size_t start, size_t length) {
    if (!s || !s->str || start >= s->len) return (nstring){.str = NULL, .len = 0};
    size_t max_len = (start + length) > s->len ? (s->len - start) : length;
    char *new_str = (char *)malloc(max_len + 1);
    for (size_t i = 0; i < max_len; i++) {
        new_str[i] = s->str[start + i];
    }
    new_str[max_len] = '\0';
    return (nstring){.str = new_str, .len = max_len};
}

void stringfree(nstring *s) {
    if (s && s->str) {
        free(s->str);
        s->str = NULL;
        s->len = 0;
    }
}

nstring nstrdup(const char *cstr) {
    if (!cstr) return (nstring){.str = NULL, .len = 0};
    size_t len = 0;
    while (cstr[len]) len++;
    char *new_str = (char *)malloc(len + 1);
    for (size_t i = 0; i < len; i++) {
        new_str[i] = cstr[i];
    }
    new_str[len] = '\0';
    return (nstring){.str = new_str, .len = len};
}

void ninput(nstring *nstr) {
    char *buffer = NULL;
    size_t bufsize = 0;
    ssize_t input_len = getline(&buffer, &bufsize, stdin);

    if (input_len == -1) {
        free(buffer);
        return;
    }

    if (input_len > 0 && buffer[input_len - 1] == '\n') {
        buffer[input_len - 1] = '\0';
        input_len--;
    }

    nstr->str = (char *)malloc((input_len + 1) * sizeof(char));
    if (!nstr->str) {
        free(buffer);
        return;
    }

    strcpy(nstr->str, buffer);
    nstr->len = input_len;

    free(buffer);
}


char nstr_at_s(const nstring *s, size_t index) {
    if (!s || !s->str || index >= s->len) return '\0';
    return s->str[index];
}

void nstr_set_s(nstring *s, size_t index, char c) {
    if (!s || !s->str || index >= s->len) return;
    s->str[index] = c;
}
void free_nstr_array(nstring *array, size_t count) {
    for (size_t i = 0; i <= count; i++) {
        free(array[i].str);
    }
    free(array);
}
nstring* nstr_split_at_every(nstring input, char delimiter) {
    const char *start = input.str;
    const char *delim_pos;
    size_t count = 0;

    while ((delim_pos = strchr(start, delimiter)) != NULL) {
        count++;
        start = delim_pos + 1;
    }

    nstring *result = (nstring *)malloc((count + 1) * sizeof(nstring));

    start = input.str;
    size_t index = 0;
    while ((delim_pos = strchr(start, delimiter)) != NULL) {
        result[index].str = (char *)malloc(delim_pos - start + 1);
        strncpy(result[index].str, start, delim_pos - start);
        result[index].str[delim_pos - start] = '\0';
        index++;
        start = delim_pos + 1;
    }

    result[index].str = strdup(start);

    return result;
}
