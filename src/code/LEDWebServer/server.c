//
// server.c
//

#include <stdio.h>
#include <stdlib.h>
#include <stdarg.h>
#include <ctype.h>
#include <string.h>
#include <unistd.h>
#include <fcntl.h>
#include <errno.h>
#include <netinet/in.h>
#include <sys/stat.h>
#include <sys/mman.h>

#include "Globals.h"
#include "utils.c"
#include "socket.c"
#include "rpiModel.c"
#include "rpiMemMap.c"
#include "rpiPwm.c"

#define PORT 12345 // A random number > 1024

#define ISspace(x) isspace((int)(x))

#define SERVER_STRING "Server: foohttpd/0.1.0\r\n"

static UInt32
getNumber(char **sp)
{
  char *s = *sp;
  UInt32 n = strtol(s, (char **) &s, 10);
  if (*s != ',') {
    fatal("Missing comma in LED cmd");
  }
  *sp = s + 1;
  return n;
}

static void
doLedCmd(char *cmd)
{
  Assert(cmd[0] == 'L');
  Assert(cmd[1] == ',');
  char *s = &cmd[2];
  UInt32 redFreq       = getNumber(&s);  Assert(s[-1] == ',');
  UInt32 redDutyCycle  = getNumber(&s);  Assert(s[-1] == ',');
  UInt32 blueFreq      = getNumber(&s);  Assert(s[-1] == ',');
  UInt32 blueDutyCycle = getNumber(&s);  Assert(s[-1] == ',');
  Assert(s == (cmd + strlen(cmd)));
  pwmSet(redFreq, redDutyCycle, blueFreq, blueDutyCycle);
}

static void
doCommand(char *cmd)
{
  // epf("doCommand(%s)", cmd);
  switch (cmd[0]) {
  case 'L': doLedCmd(cmd); break;

  default:
    epf("Bad AJAX: {%s}", cmd);
    return;
  }
  /*
      const int PWM = 1023;
      FIXME
      FIXME
      FIXME
  if (cmd[0] != 'D') {
    setPwmA(0);
    setPwmB(0);
    Assert(cmd[0] == 'U');
    return;
  }
  Assert(cmd[3] == '\0');
  switch (cmd[1]) {
  case 'F': setPwmB(PWM); setMotorDirectionB(0); break;
  case 'R': setPwmB(PWM); setMotorDirectionB(1); break;
  case '0': setPwmB(0);   break;
  default:  Assert(0);
  }
  switch (cmd[2]) {
  case 'F': setPwmA(PWM); setMotorDirectionA(1); break;
  case 'R': setPwmA(PWM); setMotorDirectionA(0); break;
  case '0': setPwmA(0);   break;
  default:  Assert(0);
  }
*/
}

static void
unimplemented(int clientSocket)
{
  const char msg[] =
    "HTTP/1.0 501 Method Not Implemented\r\n"
    SERVER_STRING
    "Content-Type: text/html\r\n"
    "\r\n"
    "<HTML><HEAD><TITLE>Method Not Implemented\r\n"
    "</TITLE></HEAD>\r\n"
    "<BODY><P>HTTP request method not supported.\r\n"
    "</BODY></HTML>\r\n";
  send(clientSocket, msg, strlen(msg), 0);
}

static void
notFound(int clientSocket)
{
  const char msg[] =
    "HTTP/1.0 404 NOT FOUND\r\n"
    SERVER_STRING
    "Content-Type: text/html\r\n"
    "\r\n"
    "<HTML><TITLE>Not Found</TITLE>\r\n"
    "<BODY><P>The server could not fulfill\r\n"
    "your request because the resource specified\r\n"
    "is unavailable or nonexistent.\r\n"
    "</BODY></HTML>\r\n";
  send(clientSocket, msg, strlen(msg), 0);
}

static const char *
lookupContentType(const char *filename)
{
  static const char *tab[][2] = {
    { "css",  "text/css" },
    { "html", "text/html" },
    { "ico",  "image/vnd.microsoft.icon" },
    { "js",   "text/javascript" },
    { "txt",  "text/plain" },
  };
  const char *ext = strrchr(filename, '.');
  if (ext == NULL) {
    return NULL;
  }
  Assert(*ext == '.');
  ++ext;
  for (int i = 0; i < (int) (sizeof(tab)/sizeof(tab[0])); ++i) {
    if (strcmp(ext, tab[i][0]) == 0) {
      return tab[i][1];
    }
  }
  return NULL;
}

static void
sendHeader(int clientSocket, const char *filename)
{
  char buf[4096];
  const char msg[] =
    "HTTP/1.0 200 OK\r\n"
    SERVER_STRING
    "Content-Type: %s\r\n"
    "\r\n";
  const char *ct = lookupContentType(filename);
  if (ct == NULL) {
    epf("Unknown content type for %s", filename);
    ct = "text/html";
  }
  sprintf(buf, msg, ct);
  send(clientSocket, buf, strlen(buf), 0);
}

static void
discardHeader(int clientSocket)
{
  int numchars;
  char buf[1024];
  do {
    numchars = getLineFromSocket(clientSocket, buf, sizeof(buf));
  } while ((numchars > 0) && (strcmp("\n", buf) != 0));
}

static void
sendFile(int clientSocket, const char *filename)
{
  char buf[0x10000];
  int fd = open(filename, O_RDONLY);
  int w, r;
  if (fd < 0) {
    notFound(clientSocket);
    return;
  }
  sendHeader(clientSocket, filename);
  for (;;) {
    r = read(fd, buf, sizeof(buf));
    if (r < 0) {
      fatal("Read() failed: %s", syserr());
    }
    if (r == 0) {
      close(fd);
      return;
    }
    w = send(clientSocket, buf, r, 0);
    if (w != r) {
      fatal("Write failed (%d != %d) : %s", w, r, syserr());
    }
  }
}

static void
processRequest(int clientSocket)
{
  char buf[4096];
  char path[1024];
  unsigned int n = getLineFromSocket(clientSocket, buf, sizeof(buf));
  Assert(n == strlen(buf));
  discardHeader(clientSocket);
  if (strncmp(buf, "GET ", 4) != 0) {
    unimplemented(clientSocket);
    return;
  }
  char *p = buf + 4;
  char *url = p;
  Assert(*url > ' ');  
  while (*p > ' ') {
    ++p;
  }
  Assert(*p == ' ');
  *p = '\0';
  if (strncmp(url, "/doCommand?cmd=", 15) == 0) {
    url += 15;
    doCommand(url);
    url = "/ok.txt";
  }
  sprintf(path, "htdocs%s", url);
  if (path[strlen(path) - 1] == '/') {
    strcat(path, "index.html");
  } else if (isDir(path)) {
    strcat(path, "/index.html");
  }
  // epf("path={%s}", path);
  sendFile(clientSocket, path);
  return;
}

static int
getServerSocket(void)
{
  struct sockaddr_in name;
  int httpd = socket(PF_INET, SOCK_STREAM, 0);
  if (httpd == -1) {
    fatal("socket() failed: %s", syserr());
  }
  if (setsockopt(httpd, SOL_SOCKET, SO_REUSEADDR,
		 &(int){1}, sizeof(int)) < 0) {
    fatal("setsockopt() failed: %s", syserr());
  }
  memset(&name, 0, sizeof(name));
  name.sin_family = AF_INET;
  name.sin_port = htons(PORT);
  name.sin_addr.s_addr = htonl(INADDR_ANY);
  if (bind(httpd, (struct sockaddr *)&name, sizeof(name)) < 0) {
    fatal("bind() failed: %s", syserr());
  }
  if (listen(httpd, 5) < 0) {
    fatal("listen() failed: %s", syserr());
  }
  return httpd;
}

int
main(void)
{
  g.modelNum = rPiModelNumber();
  int serverSocket = getServerSocket();
  epf("httpd running on port %d", PORT);
  pwmInit();
  pwmSet(1, 1, 1, 1);
  for (;;) {
    struct sockaddr_in client_name;
    unsigned int client_name_len = sizeof(client_name);
    int clientSocket = accept(serverSocket,
			      (struct sockaddr *)&client_name,
			      &client_name_len);
    if (clientSocket == -1) {
      fatal("accept() failed: %s", syserr());
    }
    processRequest(clientSocket);
    close(clientSocket);
  }
  close(serverSocket);
  return 0;
}
