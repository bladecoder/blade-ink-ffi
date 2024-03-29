#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <stdbool.h>
#include <string.h>
#include <bink.h>
#include "common.h"

void choose_path_string_test()
{
    uint32_t ret = BINK_OK;
    BINK_Story *story = NULL;
    char *err_msg = NULL;

    char *json_string = read_json_file("./inkfiles/runtime/jump-stitch.ink.json");
    if (json_string == NULL)
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

void set_get_var_test()
{
    int ret = BINK_OK;
    BINK_Story *story = NULL;
    char *err_msg = NULL;
    char *lines = NULL;

    char *json_string = read_json_file("./inkfiles/runtime/set-get-variables.ink.json");
    if (json_string == NULL)
        exit(EXIT_FAILURE);

    ret = bink_story_new(&story, json_string, &err_msg);
    check_ret(ret, story, err_msg);
    free(json_string);

    ret = bink_story_continue_maximally(story, &lines, &err_msg);
    check_ret(ret, story, err_msg);
    puts(lines);
    bink_cstring_free(lines);

    BINK_Value *v = NULL;
    ret = bink_var_get(story, "x", &v, &err_msg);
    check_ret(ret, story, err_msg);
    int x = 0;
    ret = bink_value_get_int(v, &x, &err_msg);
    check_ret(ret, story, err_msg);
    printf("x=%d\n", x);

    if (x != 10)
    {
        puts("Set/Get var test failed.\n");
        finish(EXIT_FAILURE, story, NULL);
    }
    bink_value_free(v);

    v = bink_value_new_int(15);
    check_ret(ret, story, err_msg);
    ret = bink_var_set(story, "x", v, &err_msg);
    check_ret(ret, story, err_msg);
    bink_value_free(v);

    ret = bink_var_get(story, "x", &v, &err_msg);
    check_ret(ret, story, err_msg);
    ret = bink_value_get_int(v, &x, &err_msg);
    check_ret(ret, story, err_msg);
    printf("x=%d\n", x);
    bink_value_free(v);

    if (x != 15)
    {
        puts("Set/Get var test failed.\n");
        finish(EXIT_FAILURE, story, NULL);
    }

    bink_story_free(story);
}

void var_callback(char *variable_name, BINK_Value *new_value, void *user_data)
{
    char *err_msg = NULL;
    int val = 0;

    int ret = bink_value_get_int(new_value, &val, &err_msg);
    check_ret(ret, NULL, err_msg);

    printf("callback: %s=%d\n", variable_name, val);

    if (*(int *)user_data != 10 || val != 5)
    {
        puts("Observe variable test failed.\n");
        exit(EXIT_FAILURE);
    }

    *(int *)user_data = 15;
}

void observe_variable_test()
{
    int ret = BINK_OK;
    BINK_Story *story = NULL;
    char *err_msg = NULL;
    char *lines = NULL;
    int user_data = 10;

    char *json_string = read_json_file("./inkfiles/runtime/variable-observers.ink.json");
    if (json_string == NULL)
        exit(EXIT_FAILURE);

    ret = bink_story_new(&story, json_string, &err_msg);
    free(json_string);
    check_ret(ret, story, err_msg);
    bink_observe_variable(story, "x", &var_callback, &user_data, &err_msg);

    ret = bink_story_continue_maximally(story, &lines, &err_msg);
    check_ret(ret, story, err_msg);
    puts(lines);
    bink_cstring_free(lines);

    if (user_data != 15)
    {
        puts("Observe variable test failed: User data not updated\n");
        exit(EXIT_FAILURE);
    }
}

BINK_Value *fun_callback(char *func_name, BINK_FunArgs *args, void *user_data)
{
    char *err_msg = NULL;
    BINK_Value *value1 = NULL;
    BINK_Value *value2 = NULL;
    int x = 0;
    int y = 0;

    int ret = bink_fun_args_get(args, 0, &value1, &err_msg);
    check_ret(ret, NULL, err_msg);
    ret = bink_fun_args_get(args, 1, &value2, &err_msg);
    check_ret(ret, NULL, err_msg);

    ret = bink_value_get_int(value1, &x, &err_msg);
    check_ret(ret, NULL, err_msg);
    ret = bink_value_get_int(value2, &y, &err_msg);
    check_ret(ret, NULL, err_msg);

    printf("Calling  %s, ret=%d\n", func_name, x - y);

    if (x - y != -1)
    {
        puts("External function test failed.\n");
        exit(EXIT_FAILURE);
    }

    return bink_value_new_int(x - y);
}

void external_function_test()
{
    int ret = BINK_OK;
    BINK_Story *story = NULL;
    char *err_msg = NULL;
    char *lines = NULL;
    int user_data = 10;

    char *json_string = read_json_file("./inkfiles/runtime/external-function-2-arg.ink.json");
    if (json_string == NULL)
        exit(EXIT_FAILURE);

    ret = bink_story_new(&story, json_string, &err_msg);
    free(json_string);
    check_ret(ret, story, err_msg);
    bink_bind_external_function(story, "externalFunction", &fun_callback, &user_data, &err_msg);

    ret = bink_story_continue_maximally(story, &lines, &err_msg);
    check_ret(ret, story, err_msg);
    puts(lines);

    if (strcmp(lines, "The value is -1.\n") != 0)
    {
        puts("External function test failed: Return valued not expected\n");
        finish(EXIT_FAILURE, NULL, NULL);
    }

    bink_cstring_free(lines);
}

int main(void)
{
    choose_path_string_test();
    set_get_var_test();
    observe_variable_test();
    external_function_test();

    puts("RT tests ok.\n");
}