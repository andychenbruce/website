//
//  utils.c
//

static const char *syserr(void)
  __attribute__ ((unused))
  __attribute__ ((cold));

static const char *
syserr(void)
{
  return strerror(errno);
}

static void fatal(const char * const fmt, ...)
  __attribute__ ((format (printf, 1, 2)))
  __attribute__ ((unused))
  __attribute__ ((noreturn))
  __attribute__ ((cold));

static void
fatal(const char * const fmt, ...)
{
  va_list args;
  va_start(args, fmt);
  vfprintf(stderr, fmt, args);
  fprintf(stderr, "\n");
  va_end(args);
  exit(1);
}

static void epf(const char * const fmt, ...)
  __attribute__ ((unused));

static void
epf(const char * const fmt, ...)
{
  va_list args;
  va_start(args, fmt);
  vfprintf(stderr, fmt, args);
  fprintf(stderr, "\n");
  va_end(args);
}

static void xAssert(int x, const char *s, const char *file, int line)
  __attribute__ ((unused));

static void
xAssert(int x, const char *s, const char *file, int line)
{
  if (!x) {
    fatal("Assertion failed, %s line %d: %s", file, line, s);
  }
  return;
}

static void readFileToBuffer(const char *fn, char *buf, UInt bufSize)
  __attribute__ ((unused));
  
static void
readFileToBuffer(const char *fn, char *buf, UInt bufSize)
{
  int fd = open(fn, O_RDONLY);
  if (fd < 0) {
    fatal("Can't open %s: %s", fn, syserr());
  }
  int r = read(fd, buf, bufSize);
  if (r < 0) {
    fatal("Can't read %s: %s", fn, syserr());
  }
  close(fd);
}

static int isPrefix(const char *pfx, const char *str)
  __attribute__ ((unused));

static int
isPrefix(const char *pfx, const char *str)
{
  return strncmp(pfx, str, strlen(pfx)) == 0;
}
