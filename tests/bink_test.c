#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <stdbool.h>
#include <bink.h>
#include "common.h"

int main(void) {
    uint32_t ret = BINK_OK;
    BINK_Story *story = NULL;
    BINK_Choices *choices = NULL;
    char *err_msg = NULL;
    char *line = NULL;
    // char *json_string = "{\"inkVersion\":21,\"root\":[[\"^Line.\",\"\\n\",[\"done\",{\"#n\":\"g-0\"}],null],\"done\",null],\"listDefs\":{}}";

    char *json_string = read_json_file("./inkfiles/TheIntercept.ink.json");
    if(json_string == NULL)
        exit(EXIT_FAILURE);

    ret = bink_story_new(&story, json_string, &err_msg);
    check_ret(ret, story, err_msg);
    free(json_string);

    bool end = false;

    while(!end) {
        bool can_continue;
        ret = bink_story_can_continue(story, &can_continue);
        check_ret(ret, story, err_msg);

        while (can_continue) {
            ret = bink_story_cont(story, &line, &err_msg);
            check_ret(ret, story, err_msg);
            puts(line);
            bink_cstring_free(line);
            
            ret = bink_story_can_continue(story, &can_continue);
            check_ret(ret, story, err_msg);
        }

        // Obtain and print choices
        size_t len = 0;
        ret = bink_story_get_current_choices(story, &choices, &len);
        check_ret(ret, story, NULL);
        //printf("Num. choices: %lu\n", len);

        if (len !=0) {
            print_choices(choices, len);
            printf("\n");
            bink_choices_free(choices);

            // Always choose the first option
            ret = bink_story_choose_choice_index(story, 0, &err_msg);
            check_ret(ret, story, err_msg);
        } else {
            end = true;
        }

    }

    printf("Story ended ok.\n");

    finish(EXIT_SUCCESS, story, err_msg);
}