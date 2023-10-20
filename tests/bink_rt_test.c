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

void set_get_var_test() {
    int ret = BINK_OK;
    BINK_Story *story = NULL;
    char *err_msg = NULL;
    char *lines = NULL;

    char *json_string = read_json_file("./inkfiles/runtime/set-get-variables.ink.json");
    if(json_string == NULL)
        exit(EXIT_FAILURE);

    ret = bink_story_new(&story, json_string, &err_msg);
    check_ret(ret, story, err_msg);
    free(json_string);

    ret = bink_story_continue_maximally(story, &lines, &err_msg);
    check_ret(ret, story, err_msg);
    puts(lines);
    bink_cstring_free(lines);

    int x = 0;
    ret = bink_var_get_int(story, "x", &x, &err_msg);
    check_ret(ret, story, err_msg);
    printf("x=%d\n", x);

    if(x != 10) {
        puts("Set/Get var test failed.\n");
        finish(EXIT_FAILURE, story, NULL);
    }
    
    ret = bink_var_set_int(story, "x", 15, &err_msg);

    ret = bink_var_get_int(story, "x", &x, &err_msg);
    check_ret(ret, story, err_msg);
    printf("x=%d\n", x);

    if(x != 15) {
        puts("Set/Get var test failed.\n");
        finish(EXIT_FAILURE, story, NULL);
    }

    bink_story_free(story);
}

int main(void) {
    choose_path_string_test();
    set_get_var_test();

    puts("RT tests ok.\n");
}