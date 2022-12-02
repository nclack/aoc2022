#include <fcntl.h>
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/mman.h>
#include <sys/stat.h>

#define containerof(T, p, m) ((T *)((uint8_t *)(p)-offsetof(T, m)))

struct slice {
  char *beg, *end;
};

enum parse_kind { pk_single, pk_many, pk_u32, pk_error };

struct parse {
  struct parse_value {
    enum parse_kind kind;
    union {
      struct slice match;
      struct match_list_item *matches;
      uint32_t u32;
    };
  } value;
  struct slice rest;
  int is_match;
};

struct match_list_item {
  struct parse_value value;
  struct match_list_item *next;
};

struct parser {
  struct parse (*proc)(struct parser *ctx, struct slice input);
};

// parse runner

static struct parse eval(struct parser *ctx, struct slice input) {
  return ctx->proc(ctx, input);
}

// util

static void pslice(struct slice s) {
  char t = *s.end;
  *s.end = '\0';
  puts(s.beg);
  *s.end = t;
}

// predicates

static int isdigit(char c) { return '0' <= c && c <= '9'; }

// digits

static struct parse digits_(struct parser *unused_, struct slice input) {
  char *c = input.beg;
  while (c < input.end && isdigit(*c))
    ++c;
  return (struct parse){
      .value =
          {
              .kind = pk_single,
              .match = (struct slice){.beg = input.beg, .end = c},
          },
      .rest = (struct slice){.beg = c, .end = input.end},
      .is_match = (input.beg < c),
  };
}

static struct parser *digits() {
  struct parser *out = malloc(sizeof(*out));
  *out = (struct parser){.proc = digits_};
  return out;
}

// single char

struct onechar_s {
  struct parser parser;
  char target;
};

static struct parse onechar_(struct parser *ctx_, struct slice input) {
  struct onechar_s *ctx = containerof(struct onechar_s, ctx_, parser);
  char *c = input.beg;
  if (c < input.end && *c == ctx->target) {
    ++c;
  }
  return (struct parse){
      .value =
          {
              .kind = pk_single,
              .match = (struct slice){.beg = input.beg, .end = c},
          },
      .rest = (struct slice){.beg = c, .end = input.end},
      .is_match = (input.beg < c),
  };
}

static struct parser *onechar(char target) {
  struct onechar_s *out = malloc(sizeof(*out));
  *out = (struct onechar_s){.parser = {.proc = onechar_}, .target = target};
  return &out->parser;
}

// terminated

struct terminated_s {
  struct parser parser;
  struct parser *first, *second;
};

static struct parse terminated_(struct parser *ctx_, struct slice input) {
  struct terminated_s *ctx = containerof(struct terminated_s, ctx_, parser);
  struct parse a = eval(ctx->first, input);
  if (a.is_match) {
    struct parse b = eval(ctx->second, a.rest);
    if (b.is_match) {
      return (struct parse){.value = a.value, .rest = b.rest, .is_match = 1};
    }
  }
  return (struct parse){
      .value =
          {
              .kind = pk_single,
              .match = (struct slice){.beg = input.beg, .end = input.beg},
          },
      .rest = input,
      .is_match = 0,
  };
}

static struct parser *terminated(struct parser *target,
                                 struct parser *terminator) {
  struct terminated_s *ctx = malloc(sizeof(*ctx));
  *ctx = (struct terminated_s){
      .parser.proc = terminated_, .first = target, .second = terminator};
  return &ctx->parser;
}

// opt

struct opt_s {
  struct parser parser;
  struct parser *inner;
};

static struct parse opt_(struct parser *ctx_, struct slice input) {
  struct opt_s *ctx = containerof(struct opt_s, ctx_, parser);
  struct parse a = eval(ctx->inner, input);
  a.is_match = 1;
  return a;
}

static struct parser *opt(struct parser *target) {
  struct opt_s *ctx = malloc(sizeof(*ctx));
  *ctx = (struct opt_s){
      .parser.proc = opt_,
      .inner = target,
  };
  return &ctx->parser;
}

// many1

struct many1_s {
  struct parser parser;
  struct parser *inner;
};

static struct parse many1_(struct parser *ctx_, struct slice input) {
  struct many1_s *ctx = containerof(struct many1_s, ctx_, parser);
  struct parse a = {.rest = input};
  struct match_list_item *list = 0, *tail = 0;
  do {
    a = eval(ctx->inner, a.rest);
    if (a.is_match) {
      // add match to head
      struct match_list_item *node = malloc(sizeof(*node));
      *node = (struct match_list_item){.value = a.value, .next = 0};
      if (!list)
        list = node;
      if (tail)
        tail->next = node;
      tail = node;
    }
  } while (a.rest.beg < input.end && a.is_match);
  return (struct parse){
      .value =
          {
              .kind = pk_many,
              .matches = list,
          },
      .rest = a.rest,
      .is_match = (list != 0),
  };
}

static struct parser *many1(struct parser *target) {
  struct many1_s *ctx = malloc(sizeof(*ctx));
  *ctx = (struct many1_s){
      .parser.proc = many1_,
      .inner = target,
  };
  return &ctx->parser;
}

// map_u32

struct map_u32_s {
  struct parser parser;
  struct parser *inner;
  uint32_t (*func)(struct slice *s);
};

static struct parse_value apply_fu32(uint32_t (*func)(struct slice *s),
                                     struct parse_value *v) {
  switch (v->kind) {
  case pk_single: {
    return (struct parse_value){.kind = pk_u32, .u32 = func(&v->match)};
  }

  case pk_many: {
    struct match_list_item *list=0,*tail=0;
    for(struct match_list_item *cur=v->matches;cur;cur=cur->next) {
      struct match_list_item *buf = malloc(sizeof(*buf));
      buf->value=apply_fu32(func,&cur->value);
      if(!list) list=buf;
      if(tail) tail->next=buf;
      tail=buf;
    }
    return (struct parse_value){.kind = pk_many, .matches = list};
  }

  default:
    return (struct parse_value){.kind=pk_error};
  }
}

static struct parse map_u32_(struct parser *ctx_, struct slice input) {
  struct map_u32_s *ctx = containerof(struct map_u32_s, ctx_, parser);
  struct parse a = eval(ctx->inner, input);
  if (a.is_match) {
    return (struct parse) {
      .value=apply_fu32(ctx->func,&a.value),
      .rest=a.rest,
      .is_match=1
    };
  }
  return (struct parse){
    .value={.kind = pk_error,},
      .is_match = 0,
      .rest = input,
  };
}

static struct parser *map_u32(struct parser *target,
                              uint32_t (*func)(struct slice *)) {
  struct map_u32_s *ctx = malloc(sizeof(*ctx));
  *ctx = (struct map_u32_s){
      .parser.proc = map_u32_,
      .inner = target,
      .func = func,
  };
  return &ctx->parser;
}

//

static uint32_t as_u32(struct slice *s) {
  char t = *s->end;
  *s->end = '\0';
  int out = atoi(s->beg);
  *s->end = t;
  return out;
}

// IO

static struct slice input() {
  struct stat s;
  const int fd = open("../assets/day1.txt", O_RDWR);
  if (fd == -1)
    perror("open");
  fstat(fd, &s);
  char *buf = mmap(0, s.st_size, PROT_READ | PROT_WRITE, MAP_PRIVATE, fd, 0);
  return (struct slice){.beg = buf, .end = buf + s.st_size};
}

int main() {
  pslice(eval(terminated(
    map_u32(many1(terminated(digits(), opt(onechar('\n')))), as_u32), 
    opt(onechar('\n'))),
              input())
             .rest);
  return 0;
}