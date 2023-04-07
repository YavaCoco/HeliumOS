#include <stdarg.h>
#include <debug.h>
#include <errno.h>
#include <stdio.h>

#include "internal_stdio.h"

int tpf(char const *template, ...)
{
  /* Implementation copied from printf() */
  va_list va;
  va_start(va, template);
  int ret = vprintf(template, va);
  va_end(va);
  return ret;
}

int prtrace_begin(char const *fname, char const *args, ...)
{
  va_list va;
  va_start(va, args);

  int r = 0;
#ifdef PRINT_FUNC_TRACE
  r += printf("begin %s(", fname);
  if(args)
    r += vprintf(args, va);
  r += printf(")\n");
#endif
  va_end(va);
  return r;
}

int prtrace_end(char const *fname, char const *status, char const *result, ...)
{
  va_list va;
  va_start(va, result);

  int r = 0;
#ifdef PRINT_FUNC_TRACE
  r += printf("end %s()", fname);
  if(status)
    printf(" -> %s", status);
  if(result)
  {
    printf("{");
    vprintf(result, va);
    printf("}");
  }
  printf("\n");
#endif
  va_end(va);
  return r;
}

int printf(char const *template, ...)
{
  va_list va;
  va_start(va, template);
  int ret = vprintf(template, va);
  va_end(va);
  return ret;
}

int vprintf(char const *template, va_list va)
{
  return vfprintf(stdout, template, va);
}


int fprintf(FILE *stream, char const *template, ...)
{
  va_list va;
  va_start(va, template);
  int ret = vfprintf(stream, template, va);
  va_end(va);
  return ret;
}

int vfprintf(FILE *stream, char const *template, va_list va)
{
  if(stream && !(stream->mode & MODE_W) && !(stream->mode & MODE_A))
  {
    errno = EOPNOTSUPP;
    return 0;
  }

  va_list va2;
  va_copy(va2, va);
  size_t len = vsnprintf(0, 0, template, va2);
  va_end(va2);

  char buf[len + 1];

  vsnprintf(buf, len + 1, template, va);

  int ret;

  if(stream)
  {
    if(stream->mode & MODE_W)
      fwrite(buf, sizeof(char), len+1, stream);
    else
      fappend(buf, sizeof(char), len+1, stream);
  }
  else
    // If stream is NULL; This is undefined behavior in libc, but in Helium
    // we will assume we want to print to debug console for emulators
    ret = dbg_write_string(buf);

  return ret ? ret : len;
}