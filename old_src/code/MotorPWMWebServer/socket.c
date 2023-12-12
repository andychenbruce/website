//
//  socket.c
//

static int
isDir(const char *path)
{
  struct stat st;
  if (stat(path, &st) == -1) {
    return 0;
  }
  return (st.st_mode & S_IFMT) == S_IFDIR;
}


static int
getOneByteFromSocket(int sock, int flag)
{
  char c = 0;
  int r = recv(sock, &c, 1, flag);
  if (r < 0) {
    fatal("recv failed: %s", syserr());
  }
  Assert((r == 0) || (r == 1));
  return c;
}

static int
getLineFromSocket(int sock, char *buf, int size)
{
  char *p = buf;
  while ((p - buf) < (size - 1)) {
    char c;
    c = getOneByteFromSocket(sock, 0);
    if (c == 0) {
      break;
    }
    if (c == '\r') {
      c = getOneByteFromSocket(sock, MSG_PEEK);
      if (c == '\n') {
	c = getOneByteFromSocket(sock, 0);
      }
      c = '\n';
    }
    *p++ = c;
    if (c == '\n') {
      break;
    }
  }
  *p = '\0';
  return p - buf;
}
