#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <stdbool.h>
#include <string.h>
#include <bink.h>
#include "common.h"

int main(void) {
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

    puts("Tags test ok.\n");

    finish(EXIT_SUCCESS, story, err_msg);
}