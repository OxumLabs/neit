
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
