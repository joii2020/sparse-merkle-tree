// #define CKB_C_STDLIB_PRINTF
// #include <stdio.h>

#include <stddef.h>
#include <stdint.h>
#include <string.h>
#include <stdlib.h>
#include "blake2b_decl_only.h"
#include "ckb_smt.h"

smt_state_t* smt_state_new(uint32_t capacity) {
  smt_state_t *state = (smt_state_t *)malloc(sizeof(smt_state_t));
  smt_pair_t *buffer = (smt_pair_t *)malloc(sizeof(smt_pair_t) * capacity);
  memset(state, 0, sizeof(smt_state_t));
  memset(buffer, 0, sizeof(smt_pair_t) * capacity);
  smt_state_init(state, buffer, capacity);
  return state;
}

void smt_state_del(smt_state_t* state) {
  if (!state) {
    return;
  }
  if (state->pairs)
    free(state->pairs);
  free(state);
}

uint32_t smt_state_len(smt_state_t *state) {
  return state->len;
}
