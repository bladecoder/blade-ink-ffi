#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <stdbool.h>
#include <string.h>
#include <bink.h>
#include "common.h"

void dynamic_content_test(void) {
    uint32_t ret = BINK_OK;
    BINK_Story *story = NULL;
    BINK_Tags *tags = NULL;
    char *err_msg = NULL;

    char *json_string = read_json_file("./inkfiles/tags/tagsDynamicContent.ink.json");
    if(json_string == NULL)
        exit(EXIT_FAILURE);

    ret = bink_story_new(&story, json_string, &err_msg);
    check_ret(ret, story, err_msg);
    free(json_string);

    assert_cont(story, "tag\n");

    // Obtain and print tags
    size_t len = 0;
    ret = bink_story_get_current_tags(story, &tags, &len);
    check_ret(ret, story, NULL);

    if (len != 1) {
        printf("expected len==1, actual=%lu", len);
        finish(EXIT_FAILURE, story, NULL);
    }

    char *tag = NULL;
    ret = bink_tags_get(tags, 0, &tag);
    if (ret != BINK_OK) {
        puts("error getting tag 0");
        finish(EXIT_FAILURE, NULL, NULL);
    }

    printf("TAG: %s\n", tag);

    if (strcmp(tag, "pic8red.jpg") != 0 )
        finish(EXIT_FAILURE, NULL, NULL);

    bink_cstring_free(tag);

    puts("Dynamic Tags test ok.");

    bink_story_free(story);
}

void tags_test(void) {
    uint32_t ret = BINK_OK;
    BINK_Story *story = NULL;
    BINK_Tags *tags = NULL;
    char *err_msg = NULL;
    char *line = NULL;
    size_t len = 0;

    char *json_string = read_json_file("./inkfiles/tags/tags.ink.json");
    if(json_string == NULL)
        exit(EXIT_FAILURE);

    ret = bink_story_new(&story, json_string, &err_msg);
    check_ret(ret, story, err_msg);
    free(json_string);

    bool can_continue = false;
    ret = bink_story_can_continue(story, &can_continue);
    check_ret(ret, story, err_msg);
    if (!can_continue) finish(EXIT_FAILURE, story, NULL);

    ret = bink_story_cont(story, &line, &err_msg);
    check_ret(ret, story, err_msg);
    if (strcmp(line, "This is the content\n") != 0) finish(EXIT_FAILURE, story, NULL);
    bink_cstring_free(line);

    // get_current_tags after first content
    ret = bink_story_get_current_tags(story, &tags, &len);
    check_ret(ret, story, NULL);
    if (len != 2) finish(EXIT_FAILURE, story, NULL);
    char *tag0 = NULL, *tag1 = NULL;
    ret = bink_tags_get(tags, 0, &tag0);
    check_ret(ret, story, NULL);
    ret = bink_tags_get(tags, 1, &tag1);
    check_ret(ret, story, NULL);
    if (strcmp(tag0, "author: Joe") != 0) finish(EXIT_FAILURE, story, NULL);
    if (strcmp(tag1, "title: My Great Story") != 0) finish(EXIT_FAILURE, story, NULL);
    bink_cstring_free(tag0);
    bink_cstring_free(tag1);
    bink_tags_free(tags);

    // choose_path_string("knot")
    ret = bink_story_choose_path_string(story, "knot", &err_msg);
    check_ret(ret, story, err_msg);
    ret = bink_story_cont(story, &line, &err_msg);
    check_ret(ret, story, err_msg);
    if (strcmp(line, "Knot content\n") != 0) finish(EXIT_FAILURE, story, NULL);
    bink_cstring_free(line);
    ret = bink_story_get_current_tags(story, &tags, &len);
    check_ret(ret, story, NULL);
    if (len != 1) finish(EXIT_FAILURE, story, NULL);
    ret = bink_tags_get(tags, 0, &tag0);
    check_ret(ret, story, NULL);
    if (strcmp(tag0, "knot tag") != 0) finish(EXIT_FAILURE, story, NULL);
    bink_cstring_free(tag0);
    bink_tags_free(tags);

    // cont() again (should be empty string)
    ret = bink_story_cont(story, &line, &err_msg);
    check_ret(ret, story, err_msg);
    if (strcmp(line, "") != 0) finish(EXIT_FAILURE, story, NULL);
    bink_cstring_free(line);
    ret = bink_story_get_current_tags(story, &tags, &len);
    check_ret(ret, story, NULL);
    if (len != 1) finish(EXIT_FAILURE, story, NULL);
    char *end_tag = NULL;
    ret = bink_tags_get(tags, 0, &end_tag);
    check_ret(ret, story, NULL);
    if (strcmp(end_tag, "end of knot tag") != 0) finish(EXIT_FAILURE, story, NULL);
    bink_cstring_free(end_tag);
    bink_tags_free(tags);

    bink_story_free(story);

    puts("Tags test ok.");
}

int main(void)
{
    dynamic_content_test();
    tags_test();

    puts("All Tags tests ok.");
}