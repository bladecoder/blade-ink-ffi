#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <stdbool.h>
#include <string.h>
#include <bink.h>
#include "common.h"

void choose_path_string_test() {
    uint32_t ret = BINK_OK;
    BINK_Story *story = NULL;
    char *err_msg = NULL;

    char *json_string = read_json_file("./inkfiles/runtime/jump-stitch.ink.json");
    if(json_string == NULL)
        exit(EXIT_FAILURE);

    ret = bink_story_new(&story, json_string, &err_msg);
    check_ret(ret, story, err_msg);
    free(json_string);

    ret = bink_story_choose_path_string(story, "two.sthree", &err_msg);
    check_ret(ret, story, err_msg);
    assert_cont(story, "Two.3");

    ret = bink_story_choose_path_string(story, "one.stwo", &err_msg);
    check_ret(ret, story, err_msg);
    assert_cont(story, "One.2");

    bink_story_free(story);
}

int main(void) {
    choose_path_string_test();

    puts("RT tests ok.\n");
}